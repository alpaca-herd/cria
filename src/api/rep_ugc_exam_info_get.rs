use super::Query;
use crate::error::{NcmError, Result};
/// 云小编考试状态
/// 对应 Node.js module/rep_ugc_exam_info_get.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 云小编考试状态；`examType` 取 musicalStyleEnter / languageEnter /
    /// oriSingerEnter / emotionEnter
    /// 对应 /rep/ugc/exam/info/get
    pub async fn rep_ugc_exam_info_get(&self, query: &Query) -> Result<ApiResponse> {
        let Some(exam_type) = query.get("examType").filter(|v| !v.is_empty()) else {
            return Err(NcmError::InvalidParam("参数不足".to_string()));
        };
        let data = json!({
            "examType": exam_type,
        });
        self.request(
            "/api/rep/ugc/exam/info/get",
            data,
            query.to_option(CryptoType::Eapi),
        )
        .await
    }
}
