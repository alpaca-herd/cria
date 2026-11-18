use super::Query;
use crate::error::Result;
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::{Value, json};

const DEFAULT_TYPE_IDS: &str = r#"["400002_0"]"#;

impl ApiClient {
    /// 获取广告
    ///
    /// 对应 Node.js `module/ad_get.js`，请求 `POST /api/ad/get`，使用 xeapi 加密
    /// 并携带实时易盾反作弊 token（上游 `createOption(query, 'xeapi', 'v3')`）。
    ///
    /// Query 参数：
    /// - `type_ids`：广告位 ID 的 JSON 字符串，空串或缺省时为
    ///   `["400002_0"]`。
    ///
    /// 与上游一致，响应体被重新包装为 `{ code: 200, ads, message, extra: { reqId } }`：
    /// `ads` / `message` 在原始响应中缺失或为 JS falsy 值时写成 `null`；`reqId`
    /// 依次读取首个广告的 `adExtMap.req_id`（字符串时先做 JSON 解析）、
    /// `adLogId.requestId`、`reqId`、`extJson.contextInfo.req_id`，都没有时为空串。
    /// 上游这个包装不返回原始 Set-Cookie，这里同样返回空 cookie。
    pub async fn ad_get(&self, query: &Query) -> Result<ApiResponse> {
        let type_ids = query
            .get("type_ids")
            .filter(|value| !value.is_empty())
            .unwrap_or(DEFAULT_TYPE_IDS);
        let data = json!({
            "type_ids": type_ids,
        });
        let mut option = query.to_option(CryptoType::Xeapi);
        option.check_token = true;
        let res = self.request("/api/ad/get", data, option).await?;

        let raw = &res.body;
        let ads = truthy_owned(raw.get("ads")).unwrap_or(Value::Null);
        let message = truthy_owned(raw.get("message")).unwrap_or(Value::Null);
        let req_id = extract_req_id(&ads);

        Ok(ApiResponse {
            status: 200,
            body: json!({
                "code": 200,
                "ads": ads,
                "message": message,
                "extra": { "reqId": req_id },
            }),
            cookie: vec![],
        })
    }
}

/// JS 真值语义：`null`、`false`、`0`、空串为假，空对象/空数组为真。
fn js_truthy(value: &Value) -> bool {
    match value {
        Value::Null => false,
        Value::Bool(value) => *value,
        Value::Number(number) => number.as_f64().is_some_and(|value| value != 0.0),
        Value::String(value) => !value.is_empty(),
        Value::Array(_) | Value::Object(_) => true,
    }
}

fn truthy_owned(value: Option<&Value>) -> Option<Value> {
    value.filter(|value| js_truthy(value)).cloned()
}

/// 对应上游 `Object.values(ads)[0]`：对象取第一个值，数组取首元素。
fn first_ad(ads: &Value) -> Option<&Value> {
    match ads {
        Value::Object(map) => map.values().next(),
        Value::Array(items) => items.first(),
        _ => None,
    }
}

/// 与上游 `typeof value === 'string' ? JSON.parse(value) : value` 一致；
/// 解析失败时上游在 try/catch 中忽略该分支，这里返回 `None`。
fn parse_json_text(value: &Value) -> Option<Value> {
    match value {
        Value::String(text) => serde_json::from_str(text).ok(),
        other => Some(other.clone()),
    }
}

fn extract_req_id(ads: &Value) -> Value {
    let Some(ad) = first_ad(ads) else {
        return Value::String(String::new());
    };
    if let Some(req_id) = ad
        .get("adExtMap")
        .and_then(parse_json_text)
        .and_then(|ext_map| truthy_owned(ext_map.get("req_id")))
    {
        return req_id;
    }
    if let Some(req_id) = truthy_owned(ad.get("adLogId").and_then(|log| log.get("requestId"))) {
        return req_id;
    }
    if let Some(req_id) = truthy_owned(ad.get("reqId")) {
        return req_id;
    }
    if let Some(req_id) = ad
        .get("extJson")
        .and_then(parse_json_text)
        .and_then(|ext| truthy_owned(ext.get("contextInfo").and_then(|info| info.get("req_id"))))
    {
        return req_id;
    }
    Value::String(String::new())
}
