use super::Query;
use crate::error::Result;
/// 助眠解压 - 收藏
/// 对应 Node.js module/sati_resource_sub.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 助眠解压 - 收藏（`cancel` 为真时取消收藏）
    /// 对应 /sati/resource/sub
    pub async fn sati_resource_sub(&self, query: &Query) -> Result<ApiResponse> {
        let cancel = match query.get("cancel") {
            Some(v) if !v.is_empty() => json!(v),
            _ => json!(false),
        };
        let mut data = json!({ "cancel": cancel });
        if let Some(id) = query.get("id") {
            data["id"] = json!(id);
        }
        self.request(
            "/api/voice/sati/resource/sub",
            data,
            query.to_option(CryptoType::default()),
        )
        .await
    }
}
