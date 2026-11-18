use super::Query;
use crate::error::{NcmError, Result};
/// 修改本人动态的可见权限
/// 对应 Node.js module/event_privacy.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 修改本人动态的可见权限
    ///
    /// `Query` 参数：`evId`（动态 ID，必填，按字符串发送）、`privacy`（可见权限，必填，
    /// 仅允许 `0`、`1`、`2`、`6`，按数字发送）。任一参数缺失或非法时返回参数错误，
    /// 错误信息与上游一致。
    ///
    /// 对应 `/api/event/privacy/op`，加密方式沿用上游默认的 eapi。
    pub async fn event_privacy(&self, query: &Query) -> Result<ApiResponse> {
        let event_id = query.get("evId").unwrap_or("").trim();
        let privacy_raw = query.get("privacy").unwrap_or("").trim();
        let privacy = if privacy_raw.is_empty() {
            None
        } else {
            privacy_raw
                .parse::<f64>()
                .ok()
                .filter(|value| value.is_finite() && value.fract() == 0.0)
                .map(|value| value as i64)
                .filter(|value| matches!(value, 0 | 1 | 2 | 6))
        };
        let Some(privacy) = privacy else {
            return Err(NcmError::InvalidParam(
                "evId is required and privacy must be one of 0, 1, 2, 6".to_string(),
            ));
        };
        if event_id.is_empty() {
            return Err(NcmError::InvalidParam(
                "evId is required and privacy must be one of 0, 1, 2, 6".to_string(),
            ));
        }
        let data = json!({
            "eventId": event_id,
            "privacy": privacy
        });
        self.request(
            "/api/event/privacy/op",
            data,
            query.to_option(CryptoType::default()),
        )
        .await
    }
}
