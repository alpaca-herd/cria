use super::Query;
use crate::error::Result;
/// 多级行政区划数据获取接口
/// 对应 Node.js module/lbs_city_code.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 多级行政区划数据获取接口
    /// 对应 /lbs/city/code
    pub async fn lbs_city_code(&self, query: &Query) -> Result<ApiResponse> {
        let biz_code = match query.get("bizCode") {
            Some(v) if !v.is_empty() => json!(v),
            _ => json!(""),
        };
        let data = json!({ "bizCode": biz_code });
        self.request(
            "/api/lbs/city/code",
            data,
            query.to_option(CryptoType::default()),
        )
        .await
    }
}
