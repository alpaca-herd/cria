use super::Query;
use crate::error::{NcmError, Result};
/// 云小编考试提交
/// 对应 Node.js module/rep_ugc_exam_submit.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 云小编考试提交；`questionId` 由 rep/ugc/exam/question/single/get 获取，
    /// `answer` 为 A（对）或 B（错）
    /// 对应 /rep/ugc/exam/submit
    pub async fn rep_ugc_exam_submit(&self, query: &Query) -> Result<ApiResponse> {
        let Some(exam_type) = query.get("examType").filter(|v| !v.is_empty()) else {
            return Err(NcmError::InvalidParam("参数不足".to_string()));
        };
        let Some(task_id) = query.get("taskId").filter(|v| !v.is_empty()) else {
            return Err(NcmError::InvalidParam("参数不足".to_string()));
        };
        let Some(question_id) = query.get("questionId").filter(|v| !v.is_empty()) else {
            return Err(NcmError::InvalidParam("参数不足".to_string()));
        };
        let Some(answer) = query.get("answer").filter(|v| !v.is_empty()) else {
            return Err(NcmError::InvalidParam("参数不足".to_string()));
        };
        let data = json!({
            "examType": exam_type,
            "taskId": task_id,
            "questionId": question_id,
            "answer": answer,
        });
        self.request(
            "/api/rep/ugc/exam/submit",
            data,
            query.to_option(CryptoType::Eapi),
        )
        .await
    }
}
