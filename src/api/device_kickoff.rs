use super::Query;
use crate::error::Result;
/// 强制下线设备
/// 对应 Node.js module/device_kickoff.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 强制下线设备
    ///
    /// `Query` 参数：`deviceKey`（登录设备列表中的设备 key；缺失时字段被省略，与上游
    /// `undefined` 不参与 JSON 序列化一致）、`captcha`（短信验证码；缺失或空串时传空串）。
    ///
    /// 对应 `/api/middle/user/security/device/kickoff`，使用 eapi 加密。
    pub async fn device_kickoff(&self, query: &Query) -> Result<ApiResponse> {
        let mut data = json!({
            "captcha": query.get_or("captcha", "")
        });
        if let Some(device_key) = query.get("deviceKey") {
            data["key"] = json!(device_key);
        }
        self.request(
            "/api/middle/user/security/device/kickoff",
            data,
            query.to_option(CryptoType::Eapi),
        )
        .await
    }
}
