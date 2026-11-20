use super::Query;
use crate::error::Result;
/// 喜欢歌曲
///
/// 对应 Node.js `module/song_like.js`（api-enhanced HEAD `75bf2e5`），
/// 使用默认的 eapi 加密。
///
/// Query 参数：
/// - `id`：歌曲 ID，写入 `trackId`，缺省为 `"0"`；
/// - `uid`：用户 ID，写入 `userid`，缺省为空串；
/// - `like`：仅当值为字符串 `"false"` 时取消喜欢，其余值（含缺省）均表示喜欢。
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 喜欢/取消喜欢歌曲
    ///
    /// 对应 `/api/song/like`。
    pub async fn song_like(&self, query: &Query) -> Result<ApiResponse> {
        let like = query.get("like") != Some("false");
        let data = json!({
            "trackId": query.get_or("id", "0"),
            "userid": query.get_or("uid", ""),
            "like": like,
        });
        self.request(
            "/api/song/like",
            data,
            query.to_option(CryptoType::default()),
        )
        .await
    }
}
