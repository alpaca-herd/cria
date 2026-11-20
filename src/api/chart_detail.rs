use super::Query;
use crate::error::Result;
/// 获取指定维度音乐排行榜详情
/// 对应 Node.js module/chart_detail.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 获取指定维度音乐排行榜详情
    /// 对应 /chart/detail
    pub async fn chart_detail(&self, query: &Query) -> Result<ApiResponse> {
        let mut data = json!({});
        if let Some(chart_code) = query.get("chartCode") {
            data["chartCode"] = json!(chart_code);
        }
        if let Some(target_id) = query.get("targetId") {
            data["targetId"] = json!(target_id);
        }
        if let Some(target_type) = query.get("targetType") {
            data["targetType"] = json!(target_type);
        }
        self.request(
            "/api/chart/detail",
            data,
            query.to_option(CryptoType::default()),
        )
        .await
    }
}
