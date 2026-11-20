use super::Query;
use crate::error::Result;
/// 红心/取消红心歌曲（v1）
///
/// 对应 Node.js `module/like_v1.js`（api-enhanced HEAD `75bf2e5`）。
/// 与 `ApiClient::like` 不同，本接口使用 `/api/v1/radio/like`、xeapi 加密，
/// 并在请求前实时获取 v3 易盾反作弊 token。
///
/// Query 参数：
/// - `id`：歌曲 ID，写入 `trackId`，缺省为 `"0"`；
/// - `like`：仅当值为字符串 `"false"` 时表示取消红心，其余值（含缺省）均表示红心。
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 红心/取消红心歌曲（v1）
    ///
    /// 对应 `/api/v1/radio/like`。
    pub async fn like_v1(&self, query: &Query) -> Result<ApiResponse> {
        let like = query.get("like") != Some("false");
        let data = json!({
            "alg": "itembased",
            "trackId": query.get_or("id", "0"),
            "like": like,
            "time": "3",
        });
        let mut option = query.to_option(CryptoType::Xeapi);
        option.check_token = true;
        self.request("/api/v1/radio/like", data, option).await
    }
}
