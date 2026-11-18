use super::Query;
use crate::error::Result;
/// 搜索联想（App 移动端）
/// 对应 Node.js module/search_suggest_mobile.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 搜索联想（App 移动端：直出歌曲卡 + 联想词 + 热搜/原唱标签）
    ///
    /// `Query` 参数：`keyword`（搜索词；缺失或空串时回退到 `keywords`，两者都缺失时传空串）。
    ///
    /// 上游注意：本接口参数名必须是 `keyword`，传旧版的 `s` 会返回 HTTP 400 空体；
    /// 与 `/api/search/suggest/keyword` 旧版接口不同。
    ///
    /// 对应 `/api/search/suggest/keyword/get`，使用 weapi 加密。
    pub async fn search_suggest_mobile(&self, query: &Query) -> Result<ApiResponse> {
        let keyword = query
            .get("keyword")
            .filter(|v| !v.is_empty())
            .or_else(|| query.get("keywords").filter(|v| !v.is_empty()))
            .unwrap_or("");
        let data = json!({
            "keyword": keyword
        });
        self.request(
            "/api/search/suggest/keyword/get",
            data,
            query.to_option(CryptoType::Weapi),
        )
        .await
    }
}
