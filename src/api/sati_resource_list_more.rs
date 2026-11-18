use super::Query;
use crate::error::Result;
/// 助眠解压 - 查看同类推荐
/// 对应 Node.js module/sati_resource_list_more.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 助眠解压 - 查看同类推荐
    /// 对应 /sati/resource/list/more
    pub async fn sati_resource_list_more(&self, query: &Query) -> Result<ApiResponse> {
        let mut data = json!({});
        if let Some(id) = query.get("id") {
            data["id"] = json!(id);
        }
        self.request(
            "/api/voice/sati/resource/list/more/v1",
            data,
            query.to_option(CryptoType::default()),
        )
        .await
    }
}
