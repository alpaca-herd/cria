use super::Query;
use crate::error::Result;
/// 灰色歌曲的其他版本推荐
///
/// 对应 Node.js `module/song_copyright_rcmd.js`（api-enhanced HEAD `75bf2e5`），
/// 显式使用 eapi 加密。
///
/// Query 参数：
/// - `songid`：优先使用的歌曲 ID，空串视为未提供；
/// - `id`：`songid` 未提供时的回退歌曲 ID；
/// - 两者均缺省时写入 `"0"`。
///
/// 写入字段名为 `songid`（全小写），与上游一致。
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 灰色歌曲的其他版本推荐
    ///
    /// 对应 `/api/song/copyright/rcmd`。
    pub async fn song_copyright_rcmd(&self, query: &Query) -> Result<ApiResponse> {
        let song_id = query
            .get("songid")
            .filter(|value| !value.is_empty())
            .or_else(|| query.get("id"))
            .unwrap_or("0");
        let data = json!({
            "songid": song_id,
        });
        self.request(
            "/api/song/copyright/rcmd",
            data,
            query.to_option(CryptoType::Eapi),
        )
        .await
    }
}
