use super::Query;
use crate::error::Result;
/// 歌曲创作者信息
///
/// 对应 Node.js `module/song_creators.js`（api-enhanced HEAD `75bf2e5`），
/// 上游未显式指定加密方式，使用默认的 eapi。
///
/// Query 参数：
/// - `id`：歌曲 ID，写入 `songId`，缺省为 `"0"`。
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 歌曲创作者信息
    ///
    /// 对应 `/api/song/creators`。
    pub async fn song_creators(&self, query: &Query) -> Result<ApiResponse> {
        let data = json!({
            "songId": query.get_or("id", "0"),
        });
        self.request(
            "/api/song/creators",
            data,
            query.to_option(CryptoType::default()),
        )
        .await
    }
}
