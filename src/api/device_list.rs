use super::Query;
use crate::error::Result;
/// 登录设备列表
/// 对应 Node.js module/device_list.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 登录设备列表
    ///
    /// `Query` 参数：无业务参数（仅使用 cookie 等通用选项）；固定传 `excStatus='9'`，
    /// 与上游一致。
    ///
    /// 对应 `/api/middle/user/device/list`，使用 eapi 加密。
    pub async fn device_list(&self, query: &Query) -> Result<ApiResponse> {
        let data = json!({
            "excStatus": "9"
        });
        self.request(
            "/api/middle/user/device/list",
            data,
            query.to_option(CryptoType::Eapi),
        )
        .await
    }
}
