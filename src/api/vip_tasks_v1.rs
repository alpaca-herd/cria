use super::Query;
use crate::error::Result;
/// 会员任务 - 新版
/// 对应 Node.js module/vip_tasks_v1.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 获取会员任务；传入 id 时获取指定用户的会员任务
    /// 对应 /vip/task/v1
    pub async fn vip_tasks_v1(&self, query: &Query) -> Result<ApiResponse> {
        let mut data = json!({
            "taskType": "app_vip_task_center"
        });
        if let Some(id) = query.get("id") {
            data["userId"] = json!(id);
        }
        self.request(
            "/api/middle/vip/mission/user/progress/list",
            data,
            query.to_option(CryptoType::Xeapi),
        )
        .await
    }
}
