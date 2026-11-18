use super::Query;
use crate::error::{NcmError, Result};
/// 云小编提交任务
/// 对应 Node.js module/thinktank_audit_resource_update.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 云小编提交任务；`taskId` 由 thinktank/audit/resource/detail 获取，
    /// `judgement` 1 同意 / 2 否决 / 3 跳过（不算次数），`type` 默认 4
    /// 对应 /thinktank/audit/resource/update
    pub async fn thinktank_audit_resource_update(&self, query: &Query) -> Result<ApiResponse> {
        let Some(task_id) = query.get("taskId").filter(|v| !v.is_empty()) else {
            return Err(NcmError::InvalidParam("参数不足".to_string()));
        };
        let Some(judgement) = query.get("judgement").filter(|v| !v.is_empty()) else {
            return Err(NcmError::InvalidParam("参数不足".to_string()));
        };
        let data = json!({
            "type": query.get("type").filter(|v| !v.is_empty()).unwrap_or("4"),
            "taskId": task_id,
            "judgement": judgement,
        });
        self.request(
            "/api/thinktank/audit/resource/update",
            data,
            query.to_option(CryptoType::Eapi),
        )
        .await
    }
}
