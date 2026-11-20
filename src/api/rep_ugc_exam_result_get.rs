use super::Query;
use crate::error::{NcmError, Result};
/// 云小编考试结果
/// 对应 Node.js module/rep_ugc_exam_result_get.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 云小编考试结果；`taskId` 首次由 rep/ugc/exam/start 获取，
    /// 之后由 rep/ugc/exam/info/get 获取
    /// 对应 /rep/ugc/exam/result/get
    pub async fn rep_ugc_exam_result_get(&self, query: &Query) -> Result<ApiResponse> {
        let Some(exam_type) = query.get("examType").filter(|v| !v.is_empty()) else {
            return Err(NcmError::InvalidParam("参数不足".to_string()));
        };
        let Some(task_id) = query.get("taskId").filter(|v| !v.is_empty()) else {
            return Err(NcmError::InvalidParam("参数不足".to_string()));
        };
        let data = json!({
            "examType": exam_type,
            "taskId": task_id,
        });
        self.request(
            "/api/rep/ugc/exam/result/get",
            data,
            query.to_option(CryptoType::Eapi),
        )
        .await
    }
}
