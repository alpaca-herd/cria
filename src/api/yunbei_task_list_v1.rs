use super::Query;
use crate::error::Result;
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 云贝广告任务：查询今日任务完成状态
    ///
    /// 对应 Node.js `module/yunbei_task_list_v1.js`，请求
    /// `POST /api/ad/power/yunbei/distribution/list`（上游用 GET 描述该接口，
    /// 但同样走加密请求），使用 weapi 加密。
    ///
    /// 无 Query 参数。响应包含 `times`（今日已完成次数）、`amount`（今日累计云贝）
    /// 与 `singleAmount`（单次可得云贝）；上游说明单日上限 10 次、单次 150 云贝。
    pub async fn yunbei_task_list_v1(&self, query: &Query) -> Result<ApiResponse> {
        let data = json!({});
        self.request(
            "/api/ad/power/yunbei/distribution/list",
            data,
            query.to_option(CryptoType::Weapi),
        )
        .await
    }
}
