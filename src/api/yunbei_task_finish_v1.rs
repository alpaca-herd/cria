use super::Query;
use crate::error::Result;
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 云贝广告任务：完成一次任务并领取云贝
    ///
    /// 对应 Node.js `module/yunbei_task_finish_v1.js`，请求
    /// `POST /api/ad/power/yunbei/distribution/create`，使用 weapi 加密。
    ///
    /// Query 参数：
    /// - `yunbeiAmount`：本次可领云贝数，通常取自
    ///   `yunbei_task_list_v1` 返回的 `singleAmount`；空串或缺省时为数字 `150`。
    ///
    /// 上游对非空参数原样提交（字符串），只有空串/缺省时才使用数字默认值；
    /// 这里保持同样的类型与默认值语义。
    pub async fn yunbei_task_finish_v1(&self, query: &Query) -> Result<ApiResponse> {
        let yunbei_amount = query
            .get("yunbeiAmount")
            .filter(|value| !value.is_empty())
            .map_or_else(|| json!(150), |value| json!(value));
        let data = json!({
            "yunbeiAmount": yunbei_amount,
        });
        self.request(
            "/api/ad/power/yunbei/distribution/create",
            data,
            query.to_option(CryptoType::Weapi),
        )
        .await
    }
}
