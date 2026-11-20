use super::Query;
use crate::error::Result;
/// 按歌词搜索
///
/// 对应 Node.js `module/search_lyric.js`（api-enhanced HEAD `75bf2e5`），
/// 显式使用 weapi 加密。
///
/// Query 参数：
/// - `keyword`：搜索关键词，空串视为未提供；
/// - `keywords`：`keyword` 未提供时的回退关键词；
/// - `limit`：返回条数，缺省 `30`；
/// - `offset`：偏移量，缺省 `0`。
///
/// 关键词最终缺省为空串。
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 按歌词搜索
    ///
    /// 对应 `/api/search/resource/lyric`。
    pub async fn search_lyric(&self, query: &Query) -> Result<ApiResponse> {
        let keyword = query
            .get("keyword")
            .filter(|value| !value.is_empty())
            .or_else(|| query.get("keywords").filter(|value| !value.is_empty()))
            .unwrap_or("");
        let data = json!({
            "keyword": keyword,
            "limit": query.get_i64("limit", 30),
            "offset": query.get_i64("offset", 0),
        });
        self.request(
            "/api/search/resource/lyric",
            data,
            query.to_option(CryptoType::Weapi),
        )
        .await
    }
}
