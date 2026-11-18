use super::Query;
use crate::error::Result;
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 云贝广告任务：获取「听歌得云贝」推荐歌曲
    ///
    /// 对应 Node.js `module/yunbei_task_recommend_song.js`，请求
    /// `POST /api/ad/power/yunbei/distribution/recommend/song`，使用 weapi 加密。
    ///
    /// Query 参数：
    /// - `offset`：分页偏移，空串或缺省时为 `0`；
    /// - `limit`：返回数量，空串或缺省时为 `10`（客户端一次取 10 首）。
    ///
    /// 上游返回 `songId`、`songName`、`artistName`、`albumUrl`、
    /// `songChorusStartTime`、`likeFlag`、`alg` 等推荐字段，其中 `alg` 形如
    /// `alg_payrec_yunBei_*`。
    pub async fn yunbei_task_recommend_song(&self, query: &Query) -> Result<ApiResponse> {
        let data = json!({
            "offset": query.get_i64("offset", 0),
            "limit": query.get_i64("limit", 10),
        });
        self.request(
            "/api/ad/power/yunbei/distribution/recommend/song",
            data,
            query.to_option(CryptoType::Weapi),
        )
        .await
    }
}
