use super::Query;
use crate::error::Result;
/// 动态搜索
/// 对应 Node.js module/event_search.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 动态搜索
    ///
    /// `Query` 参数：`keyword`（搜索词；缺失或空串时回退到 `keywords`）、
    /// `limit`（每页数量，默认 30）、`offset`（偏移量，默认 0）。
    ///
    /// 对应 `/api/event/search/list/get/v1`，使用 weapi 加密。
    pub async fn event_search(&self, query: &Query) -> Result<ApiResponse> {
        let keyword = query
            .get("keyword")
            .filter(|v| !v.is_empty())
            .or_else(|| query.get("keywords").filter(|v| !v.is_empty()))
            .unwrap_or("");
        let data = json!({
            "keyword": keyword,
            "limit": query.get_i64("limit", 30),
            "offset": query.get_i64("offset", 0)
        });
        self.request(
            "/api/event/search/list/get/v1",
            data,
            query.to_option(CryptoType::Weapi),
        )
        .await
    }
}
