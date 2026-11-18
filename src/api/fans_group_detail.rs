use super::Query;
use crate::error::Result;
/// 乐迷团详情
/// 对应 Node.js module/fans_group_detail.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 乐迷团详情
    /// 对应 /fans/group/detail
    pub async fn fans_group_detail(&self, query: &Query) -> Result<ApiResponse> {
        let group_id = query
            .get("groupId")
            .filter(|v| !v.is_empty())
            .or_else(|| query.get("id").filter(|v| !v.is_empty()))
            .unwrap_or("");
        let data = json!({
            "groupId": group_id,
            "scene": query.get_or("scene", ""),
        });
        self.request(
            "/api/social/fansgroup/bff/detail/get",
            data,
            query.to_option(CryptoType::Eapi),
        )
        .await
    }
}
