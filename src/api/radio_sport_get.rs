use super::Query;
use crate::error::Result;
/// 跑步漫游
/// 对应 Node.js module/radio_sport_get.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 跑步漫游
    /// 对应 /radio/sport/get
    pub async fn radio_sport_get(&self, query: &Query) -> Result<ApiResponse> {
        let bpm = match query.get("bpm") {
            Some(v) if !v.is_empty() => json!(v),
            _ => json!(50),
        };
        let data = json!({ "bpm": bpm });
        self.request(
            "/api/radio/sport/get",
            data,
            query.to_option(CryptoType::default()),
        )
        .await
    }
}
