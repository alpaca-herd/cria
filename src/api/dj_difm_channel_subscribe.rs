use super::Query;
use crate::error::Result;
/// DIFM 电台 - 收藏频道
/// 对应 Node.js module/dj_difm_channel_subscribe.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// DIFM 电台 - 收藏频道
    /// 对应 /dj/difm/channel/subscribe
    pub async fn dj_difm_channel_subscribe(&self, query: &Query) -> Result<ApiResponse> {
        let mut data = json!({});
        if let Some(id) = query.get("id") {
            data["id"] = json!(id);
        }
        self.request(
            "/api/dj/difm/channel/subscribe",
            data,
            query.to_option(CryptoType::default()),
        )
        .await
    }
}
