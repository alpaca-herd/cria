use super::Query;
use crate::error::Result;
/// 用户动态
/// 对应 Node.js module/user_event.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 用户动态
    /// 对应 /user/event
    pub async fn user_event(&self, query: &Query) -> Result<ApiResponse> {
        let data = json!({
            "getcounts": true,
            "time": query.get("lasttime").map_or(json!(-1), |value| json!(value)),
            "limit": query.get("limit").map_or(json!(30), |value| json!(value)),
            "total": false,
            "fromRN": "true"
        });
        self.request(
            &format!("/api/event/get/{}", query.get_or("uid", "0")),
            data,
            query.to_option(CryptoType::default()),
        )
        .await
    }
}
