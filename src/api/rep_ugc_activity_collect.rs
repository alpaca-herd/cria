use super::Query;
use crate::error::Result;
/// 云小编领取任务积分
/// 对应 Node.js module/rep_ugc_activity_collect.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 云小编领取任务积分；`activityId` 默认 5001（可由 rep/ugc/activity/get 获取）
    /// 对应 /rep/ugc/activity/collect
    pub async fn rep_ugc_activity_collect(&self, query: &Query) -> Result<ApiResponse> {
        let data = json!({
            "activityId": query
                .get("activityId")
                .filter(|v| !v.is_empty())
                .unwrap_or("5001"),
        });
        self.request(
            "/api/rep/ugc/activity/collect",
            data,
            query.to_option(CryptoType::Eapi),
        )
        .await
    }
}
