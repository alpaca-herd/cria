use super::Query;
use crate::error::Result;
/// 发送验证码（v1）
/// 对应 Node.js module/captcha_sent_v1.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 发送验证码（v1，PC 登录场景）
    ///
    /// `Query` 参数：`phone`（手机号；缺失时字段被省略，与上游 `undefined` 不参与
    /// JSON 序列化一致）、`ctcode`（国家/地区码；缺失或空串时默认 `86`）。
    /// 固定字段 `secrete=music_middleuser_pclogin`、`scene='0'` 与上游保持一致。
    ///
    /// 对应 `/api/middle/captcha/sent/v1`，使用 eapi 加密。
    pub async fn captcha_sent_v1(&self, query: &Query) -> Result<ApiResponse> {
        let ctcode = query
            .get("ctcode")
            .filter(|v| !v.is_empty())
            .unwrap_or("86");
        let mut data = json!({
            "ctcode": ctcode,
            "secrete": "music_middleuser_pclogin",
            "scene": "0"
        });
        if let Some(phone) = query.get("phone") {
            data["cellphone"] = json!(phone);
        }
        self.request(
            "/api/middle/captcha/sent/v1",
            data,
            query.to_option(CryptoType::Eapi),
        )
        .await
    }
}
