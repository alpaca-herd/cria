use super::Query;
use crate::error::Result;
/// 首页轮播（v3）
/// 对应 Node.js module/banner_v3.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 首页轮播（v3）
    ///
    /// `Query` 参数：`clientType`（客户端类型；缺失或空串时默认 `android`）。
    ///
    /// 对应 `/api/banner/get/v3`，使用 weapi 加密。
    pub async fn banner_v3(&self, query: &Query) -> Result<ApiResponse> {
        let client_type = query
            .get("clientType")
            .filter(|v| !v.is_empty())
            .unwrap_or("android");
        let data = json!({
            "clientType": client_type
        });
        self.request(
            "/api/banner/get/v3",
            data,
            query.to_option(CryptoType::Weapi),
        )
        .await
    }
}
