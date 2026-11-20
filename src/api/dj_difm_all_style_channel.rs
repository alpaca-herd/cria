use super::Query;
use crate::error::Result;
/// DIFM 电台 - 分类
/// 对应 Node.js module/dj_difm_all_style_channel.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// DIFM 电台 - 分类
    /// 对应 /dj/difm/all/style/channel
    pub async fn dj_difm_all_style_channel(&self, query: &Query) -> Result<ApiResponse> {
        let sources = query
            .get("sources")
            .filter(|v| !v.is_empty())
            .unwrap_or("[0]");
        let data = json!({ "sources": sources });
        self.request(
            "/api/dj/difm/all/style/channel/v2",
            data,
            query.to_option(CryptoType::default()),
        )
        .await
    }
}
