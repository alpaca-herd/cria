use super::Query;
use crate::error::Result;
/// 插播相似歌曲
///
/// 对应 Node.js `module/song_simi_get.js`（api-enhanced HEAD `75bf2e5`），
/// 显式使用 eapi 加密。
///
/// Query 参数：
/// - `id`：歌曲 ID，写入 `resourceId`，缺省为 `"0"`。
///
/// 固定参数：`positionCode` 为 `"toolBarRcmdSong"`，`resourceType` 为 `"song"`。
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 插播相似歌曲
    ///
    /// 对应 `/api/link/position/show/resource`。
    pub async fn song_simi_get(&self, query: &Query) -> Result<ApiResponse> {
        let data = json!({
            "positionCode": "toolBarRcmdSong",
            "resourceId": query.get_or("id", "0"),
            "resourceType": "song",
        });
        self.request(
            "/api/link/position/show/resource",
            data,
            query.to_option(CryptoType::Eapi),
        )
        .await
    }
}
