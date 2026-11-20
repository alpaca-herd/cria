use super::Query;
use crate::error::Result;
/// DIFM 电台 - 收藏列表
/// 对应 Node.js module/dj_difm_subscribe_channels_get.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// DIFM 电台 - 收藏列表
    /// 对应 /dj/difm/subscribe/channels/get
    pub async fn dj_difm_subscribe_channels_get(&self, query: &Query) -> Result<ApiResponse> {
        let sources = query
            .get("sources")
            .filter(|v| !v.is_empty())
            .unwrap_or("[0]");
        let data = json!({ "sources": sources });
        self.request(
            "/api/dj/difm/subscribe/channels/get/v2",
            data,
            query.to_option(CryptoType::default()),
        )
        .await
    }
}
