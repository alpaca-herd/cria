use super::Query;
use crate::error::Result;
/// 获取当前登录用户可被上游枚举的全部动态
/// 对应 Node.js module/user_event_all.js
use crate::request::{ApiClient, ApiResponse};
use serde_json::{Value, json};
use std::collections::HashSet;

/// 上游每页固定的动态条数
const PAGE_SIZE: i64 = 100;
/// 上游允许的最大翻页数，超过视为异常而失败
const MAX_PAGES: usize = 1000;

impl ApiClient {
    /// 获取当前登录用户可被上游枚举的全部动态
    ///
    /// 对应 Node.js `module/user_event_all.js`。先用 user/account 接口解析当前登录用户，
    /// 再以每页 `PAGE_SIZE = 100`、最多 `MAX_PAGES = 1000` 页循环拉取 user/event：
    /// - `size` 只取第一页回包，按数值保存；非数值为 `null`
    /// - 按事件 `id` 去重，没有 `id` 的事件原样保留
    /// - 每一页的 cookie 依次累加到最终回包
    /// - 分页未返回业务码 200 时立即停止，保留该页回包及累计 cookie；状态取该页的
    ///   4xx/5xx 业务码，无法映射时返回 502。
    /// - `more` 为真但 `lasttime` 为空或重复，或页数达到上限时，返回上游同形的
    ///   502 错误回包（保留已收集的 cookie），而不是抛出只含部分结果的响应
    ///
    /// 与上游不同，CRIA 的请求错误（登录失效、网络或解密失败）会直接向上传播，
    /// 不在这里转换成 200/401/502 回包；只有本地翻页守卫使用合成回包。
    ///
    /// Query 参数：无必填项；cookie 必须为有效登录态。上游 `query` 中的 `lasttime`
    /// 和 `limit` 会被本方法覆盖。
    ///
    /// 对应 POST /api/event/get/{uid}
    pub async fn user_event_all(&self, query: &Query) -> Result<ApiResponse> {
        let account = self.user_account(query).await?;
        let mut cookies = account.cookie.clone();

        let uid = account
            .body
            .get("account")
            .and_then(|account| account.get("id"))
            .filter(|value| js_truthy(value))
            .or_else(|| {
                account
                    .body
                    .get("profile")
                    .and_then(|profile| profile.get("userId"))
                    .filter(|value| js_truthy(value))
            });
        let Some(uid) = uid else {
            return Ok(ApiResponse {
                status: 401,
                body: json!({
                    "code": 401,
                    "message": "A valid login cookie is required",
                }),
                cookie: cookies,
            });
        };
        let uid = js_string(uid);

        let mut events: Vec<Value> = Vec::new();
        let mut event_ids: HashSet<String> = HashSet::new();
        let mut cursors: HashSet<String> = HashSet::new();
        let mut lasttime = json!(-1);
        let mut more = true;
        let mut page_count = 0usize;
        let mut size: Option<f64> = None;

        let mut page_query = query.clone();
        page_query.params.insert("uid".to_string(), uid);
        page_query
            .params
            .insert("limit".to_string(), PAGE_SIZE.to_string());

        while more {
            page_count += 1;

            page_query.params.insert(
                "lasttime".to_string(),
                if lasttime.is_null() {
                    "-1".to_string()
                } else {
                    js_string(&lasttime)
                },
            );

            let mut page = self.user_event(&page_query).await?;
            cookies.append(&mut page.cookie);
            let code = page.body.get("code").and_then(|code| {
                code.as_i64()
                    .or_else(|| code.as_str().and_then(|text| text.parse().ok()))
            });
            if page.status != 200 || code != Some(200) {
                // 请求层会把部分业务错误码归一为 200；分页不能把这些回包当作成功。
                if page.status == 200 {
                    page.status = code.filter(|code| (400..600).contains(code)).unwrap_or(502);
                }
                page.cookie = cookies;
                return Ok(page);
            }

            if page_count == 1 {
                size = page.body.get("size").and_then(js_number);
            }

            more = page.body.get("more").map(js_truthy).unwrap_or(false);
            lasttime = page.body.get("lasttime").cloned().unwrap_or(Value::Null);

            if let Some(page_events) = page.body.get_mut("events").and_then(Value::as_array_mut) {
                for event in page_events.drain(..) {
                    let Some(event_id) = event
                        .get("id")
                        .filter(|value| !value.is_null())
                        .map(js_string)
                    else {
                        events.push(event);
                        continue;
                    };
                    if event_ids.insert(event_id) {
                        events.push(event);
                    }
                }
            }

            if more {
                let cursor = if lasttime.is_null() {
                    String::new()
                } else {
                    js_string(&lasttime)
                };
                if cursor.is_empty() || !cursors.insert(cursor) {
                    return Ok(ApiResponse {
                        status: 502,
                        body: json!({
                            "code": 502,
                            "message": "Upstream event pagination cursor stalled",
                        }),
                        cookie: cookies,
                    });
                }
            }

            if more && page_count >= MAX_PAGES {
                return Ok(ApiResponse {
                    status: 502,
                    body: json!({
                        "code": 502,
                        "message": format!(
                            "Upstream event pagination exceeded {MAX_PAGES} pages"
                        ),
                    }),
                    cookie: cookies,
                });
            }
        }

        let retrieved_count = events.len();
        let unavailable_count = size.map(|size| (size - retrieved_count as f64).max(0.0));
        let body = json!({
            "code": 200,
            "events": events,
            "size": size.map(number_value).unwrap_or(Value::Null),
            "retrievedCount": retrieved_count,
            "unavailableCount": unavailable_count.map(number_value).unwrap_or(Value::Null),
            "sizeMismatch": size
                .map(|size| size != retrieved_count as f64)
                .map(Value::Bool)
                .unwrap_or(Value::Null),
            "pageCount": page_count,
            "more": false,
            "lasttime": lasttime,
        });

        Ok(ApiResponse {
            status: 200,
            body,
            cookie: cookies,
        })
    }
}

/// JavaScript 的 `Boolean(value)` 语义
fn js_truthy(value: &Value) -> bool {
    match value {
        Value::Null => false,
        Value::Bool(value) => *value,
        Value::Number(value) => value.as_f64().is_none_or(|value| value != 0.0),
        Value::String(value) => !value.is_empty(),
        Value::Array(_) | Value::Object(_) => true,
    }
}

/// JavaScript 的 `String(value)` 语义（数字按整数值去掉 `.0`）
fn js_string(value: &Value) -> String {
    match value {
        Value::Null => "null".to_string(),
        Value::Bool(value) => value.to_string(),
        Value::Number(value) => js_number_to_string(value),
        Value::String(value) => value.clone(),
        Value::Array(values) => values
            .iter()
            .map(|value| match value {
                Value::Null => String::new(),
                other => js_string(other),
            })
            .collect::<Vec<_>>()
            .join(","),
        Value::Object(_) => "[object Object]".to_string(),
    }
}

/// JavaScript 的 `Number(value)` 语义中与上游动态统计相关的分支
fn js_number(value: &Value) -> Option<f64> {
    match value {
        Value::Number(value) => value.as_f64(),
        Value::String(value) => {
            let trimmed = value.trim();
            if trimmed.is_empty() {
                Some(0.0)
            } else {
                trimmed
                    .parse::<f64>()
                    .ok()
                    .filter(|value| value.is_finite())
            }
        }
        Value::Bool(value) => Some(if *value { 1.0 } else { 0.0 }),
        Value::Null | Value::Array(_) | Value::Object(_) => None,
    }
}

/// 把数值还原成 JSON 数字，整数值不携带小数点
fn number_value(value: f64) -> Value {
    if value.fract() == 0.0 && value >= i64::MIN as f64 && value <= i64::MAX as f64 {
        json!(value as i64)
    } else {
        json!(value)
    }
}

/// JavaScript 的 `String(number)`：整数值不带 `.0`，与上游游标拼接保持一致
fn js_number_to_string(value: &serde_json::Number) -> String {
    if let Some(value) = value.as_i64() {
        return value.to_string();
    }
    if let Some(value) = value.as_u64() {
        return value.to_string();
    }
    let value = value.as_f64().unwrap_or(0.0);
    if value.fract() == 0.0 && value.is_finite() && value.abs() < 1e21 {
        format!("{value:.0}")
    } else {
        value.to_string()
    }
}

#[cfg(test)]
mod tests;
