use super::Query;
use crate::error::Result;
/// 每日风格推荐场景配置设置
/// 对应 Node.js module/recommend_category_set.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::{Value, json};

/// 复刻 JavaScript `Number(value)` 的转换语义：
/// 空串为 0，十六进制按 16 进制解析，无法解析或非有限值（JSON.stringify 中）为 null。
fn js_number(raw: &str) -> Value {
    let trimmed = raw.trim();
    if trimmed.is_empty() {
        return json!(0);
    }
    if let Some(hex) = trimmed
        .strip_prefix("0x")
        .or_else(|| trimmed.strip_prefix("0X"))
    {
        return match u64::from_str_radix(hex, 16) {
            Ok(number) => json!(number),
            Err(_) => Value::Null,
        };
    }
    match trimmed.parse::<f64>() {
        Ok(number) if number.is_finite() => {
            if number.fract() == 0.0 && number >= i64::MIN as f64 && number <= i64::MAX as f64 {
                json!(number as i64)
            } else {
                serde_json::Number::from_f64(number)
                    .map(Value::Number)
                    .unwrap_or(Value::Null)
            }
        }
        _ => Value::Null,
    }
}

impl ApiClient {
    /// 每日风格推荐场景配置设置
    ///
    /// `Query` 参数：`tags`（逗号分隔的标签 ID 列表）、`category`（分类 ID）。
    /// 两者均按上游 `Number()` 语义转换：非法数值变为 JSON `null`；`tags` 缺失时
    /// 按上游 `String(undefined)` 语义得到 `[null]`，空串则按 `Number('')` 得到 0。
    /// 转换后的 `{tagIds, categoryId}` 再序列化为 JSON **字符串** 放进 `tags` 字段。
    ///
    /// 对应 `/api/homepage/daily/song/tag/save`，使用 xeapi 加密。
    pub async fn recommend_category_set(&self, query: &Query) -> Result<ApiResponse> {
        let tags_raw = query.get("tags").unwrap_or("undefined");
        let tag_ids: Vec<Value> = tags_raw.split(',').map(js_number).collect();
        let category_id = match query.get("category") {
            Some(value) => js_number(value),
            // Number(undefined) 得到 NaN，JSON.stringify 后为 null
            None => Value::Null,
        };
        let tags = serde_json::to_string(&json!({
            "tagIds": tag_ids,
            "categoryId": category_id
        }))
        .unwrap_or_default();
        let data = json!({
            "tags": tags
        });
        self.request(
            "/api/homepage/daily/song/tag/save",
            data,
            query.to_option(CryptoType::Xeapi),
        )
        .await
    }
}
