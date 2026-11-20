use super::Query;
use crate::error::Result;
/// 每日推荐风格歌曲
/// 对应 Node.js module/recommend_category_songs.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 每日推荐风格歌曲
    ///
    /// `Query` 参数：无业务参数（仅使用 cookie 等通用选项）。
    ///
    /// 对应 `/api/homepage/category/daily/song/list`，使用 xeapi 加密。
    pub async fn recommend_category_songs(&self, query: &Query) -> Result<ApiResponse> {
        let data = json!({});
        self.request(
            "/api/homepage/category/daily/song/list",
            data,
            query.to_option(CryptoType::Xeapi),
        )
        .await
    }
}
