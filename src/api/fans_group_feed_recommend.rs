use super::Query;
use crate::error::Result;
/// 乐迷团推荐笔记
/// 对应 Node.js module/fans_group_feed_recommend.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 乐迷团推荐笔记
    /// 对应 /fans/group/feed/recommend
    pub async fn fans_group_feed_recommend(&self, query: &Query) -> Result<ApiResponse> {
        let fans_group_id = query
            .get("fansGroupId")
            .filter(|v| !v.is_empty())
            .or_else(|| query.get("groupId").filter(|v| !v.is_empty()))
            .unwrap_or("");
        let data = json!({
            "artistSelf": query
                .get("artistSelf")
                .filter(|v| !v.is_empty())
                .unwrap_or("0"),
            "fansGroupId": fans_group_id,
            "cursor": query.get("cursor").filter(|v| !v.is_empty()).unwrap_or("0"),
            "size": query.get("size").filter(|v| !v.is_empty()).unwrap_or("10"),
        });
        self.request(
            "/api/fans/group/feed/recommend/get",
            data,
            query.to_option(CryptoType::Eapi),
        )
        .await
    }
}
