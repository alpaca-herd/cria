use super::Query;
use crate::error::Result;
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 查询免费听歌时长与权益状态
    ///
    /// 对应 Node.js `module/ad_listening_rights.js`，请求
    /// `POST /api/ad/homepage/free/tab/extend/v2`，使用 xeapi 加密；上游不携带
    /// 易盾反作弊 token。
    ///
    /// 无 Query 参数；入口固定为 `FREE_LISTEN_RN`。
    pub async fn ad_listening_rights(&self, query: &Query) -> Result<ApiResponse> {
        let data = json!({
            "entrance": "FREE_LISTEN_RN",
        });
        self.request(
            "/api/ad/homepage/free/tab/extend/v2",
            data,
            query.to_option(CryptoType::Xeapi),
        )
        .await
    }
}
