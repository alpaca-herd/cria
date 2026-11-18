use super::Query;
use crate::error::Result;
/// 搜索建议（PC 端）
/// 对应 Node.js module/search_suggest_pc.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 搜索建议（PC 端）
    ///
    /// `Query` 参数：`keyword`（搜索词；缺失时传空串）。
    ///
    /// 对应 `/api/search/pc/suggest/keyword/get`，加密方式沿用上游默认的 eapi。
    pub async fn search_suggest_pc(&self, query: &Query) -> Result<ApiResponse> {
        let data = json!({
            "keyword": query.get_or("keyword", "")
        });
        self.request(
            "/api/search/pc/suggest/keyword/get",
            data,
            query.to_option(CryptoType::default()),
        )
        .await
    }
}
