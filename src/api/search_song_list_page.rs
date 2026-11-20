use super::Query;
use crate::error::Result;
/// 新版单曲搜索（分页）
///
/// 对应 Node.js `module/search_song_list_page.js`（api-enhanced HEAD `75bf2e5`），
/// 显式使用 weapi 加密。
///
/// Query 参数：
/// - `keyword`：搜索关键词，空串视为未提供；
/// - `keywords`：`keyword` 未提供时的回退关键词；
/// - `limit`：返回条数，缺省 `30`；
/// - `offset`：偏移量，缺省 `0`；
/// - `scene`：搜索场景，空串或缺省时为 `"normal"`。
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 新版单曲搜索（分页）
    ///
    /// 对应 `/api/search/song/list/page`。
    pub async fn search_song_list_page(&self, query: &Query) -> Result<ApiResponse> {
        let keyword = query
            .get("keyword")
            .filter(|value| !value.is_empty())
            .or_else(|| query.get("keywords").filter(|value| !value.is_empty()))
            .unwrap_or("");
        let scene = query
            .get("scene")
            .filter(|value| !value.is_empty())
            .unwrap_or("normal");
        let data = json!({
            "keyword": keyword,
            "limit": query.get_i64("limit", 30),
            "offset": query.get_i64("offset", 0),
            "scene": scene,
        });
        self.request(
            "/api/search/song/list/page",
            data,
            query.to_option(CryptoType::Weapi),
        )
        .await
    }
}
