use super::Query;
use crate::error::Result;
/// 发送安全验证码
/// 对应 Node.js module/captcha_safe_sent.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 发送安全验证码
    ///
    /// `Query` 参数：`ctcode`（国家/地区码；缺失或空串时默认 `86`）。手机号等其余字段
    /// 由已登录 cookie 决定，不在请求体中传递。
    ///
    /// 对应 `/api/sms/captcha/safe/sent`，使用 eapi 加密。
    pub async fn captcha_safe_sent(&self, query: &Query) -> Result<ApiResponse> {
        let ctcode = query
            .get("ctcode")
            .filter(|v| !v.is_empty())
            .unwrap_or("86");
        let data = json!({
            "ctcode": ctcode
        });
        self.request(
            "/api/sms/captcha/safe/sent",
            data,
            query.to_option(CryptoType::Eapi),
        )
        .await
    }
}
