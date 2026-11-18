use super::Query;
use crate::error::Result;
/// 我创建的播客声音
/// 对应 Node.js module/voicelist_my_created.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 我创建的播客声音
    /// 对应 /voicelist/my/created
    pub async fn voicelist_my_created(&self, query: &Query) -> Result<ApiResponse> {
        let limit = match query.get("limit") {
            Some(v) if !v.is_empty() => json!(v),
            _ => json!(20),
        };
        let data = json!({ "limit": limit });
        self.request(
            "/api/social/my/created/voicelist/v1",
            data,
            query.to_option(CryptoType::Weapi),
        )
        .await
    }
}
