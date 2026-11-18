use super::Query;
use crate::error::Result;
/// 每日风格推荐场景配置列表获取
/// 对应 Node.js module/recommend_category_configs.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 每日风格推荐场景配置列表获取
    ///
    /// `Query` 参数：无业务参数（仅使用 cookie 等通用选项）。
    ///
    /// 对应 `/api/homepage/daily/song/config/get`，使用 xeapi 加密。
    pub async fn recommend_category_configs(&self, query: &Query) -> Result<ApiResponse> {
        let data = json!({});
        self.request(
            "/api/homepage/daily/song/config/get",
            data,
            query.to_option(CryptoType::Xeapi),
        )
        .await
    }
}
