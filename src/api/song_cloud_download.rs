use super::Query;
use crate::error::Result;
/// 从云盘获取歌曲下载链接
///
/// 对应 Node.js `module/song_cloud_download.js`（api-enhanced HEAD `75bf2e5`），
/// 显式使用 eapi 加密。
///
/// Query 参数：
/// - `id`：云盘歌曲 ID，写入 `songId`，缺省为 `"0"`。
///
/// 上游端点拼写为 `/api/cloud/dowonload`（`download` 的拼写错误），此处原样保留。
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 从云盘获取歌曲下载链接
    ///
    /// 对应 `/api/cloud/dowonload`。
    pub async fn song_cloud_download(&self, query: &Query) -> Result<ApiResponse> {
        let data = json!({
            "songId": query.get_or("id", "0"),
        });
        self.request(
            "/api/cloud/dowonload",
            data,
            query.to_option(CryptoType::Eapi),
        )
        .await
    }
}
