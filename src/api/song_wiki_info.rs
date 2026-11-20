use super::Query;
use crate::error::Result;
/// 歌曲百科信息
///
/// 对应 Node.js `module/song_wiki_info.js`（api-enhanced HEAD `75bf2e5`），
/// 显式使用 eapi 加密。
///
/// Query 参数：
/// - `id`：歌曲 ID，写入 `extJson.states.playingResource.current`，缺省为 `"0"`。
///
/// `extJson` 与上游一致，是 JSON 序列化后的字符串：
/// `{"states":{"playingResource":{"current":"<id>","scene":"songWiki"}}}`；
/// `positionCode` 固定为 `"songWikiMainPosition"`。
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 歌曲百科信息
    ///
    /// 对应 `/api/link/page/parent/relation/construct/info`。
    pub async fn song_wiki_info(&self, query: &Query) -> Result<ApiResponse> {
        let ext_json = json!({
            "states": {
                "playingResource": {
                    "current": query.get_or("id", "0"),
                    "scene": "songWiki",
                },
            },
        })
        .to_string();
        let data = json!({
            "extJson": ext_json,
            "positionCode": "songWikiMainPosition",
        });
        self.request(
            "/api/link/page/parent/relation/construct/info",
            data,
            query.to_option(CryptoType::Eapi),
        )
        .await
    }
}
