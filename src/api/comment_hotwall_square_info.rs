use super::Query;
use crate::error::Result;
/// 热评墙广场信息
/// 对应 Node.js module/comment_hotwall_square_info.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 热评墙广场信息
    ///
    /// 对应 Node.js `module/comment_hotwall_square_info.js`；上游走 weapi。
    ///
    /// Query 参数：无（登录态由 cookie 携带）。
    ///
    /// 请求体为空对象。
    ///
    /// 对应 POST /api/comment/hotwall/today/squareinfo/get
    pub async fn comment_hotwall_square_info(&self, query: &Query) -> Result<ApiResponse> {
        self.request(
            "/api/comment/hotwall/today/squareinfo/get",
            json!({}),
            query.to_option(CryptoType::Weapi),
        )
        .await
    }
}
