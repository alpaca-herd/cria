use super::Query;
use crate::error::{NcmError, Result};
/// 上报设备中心设备名称
/// 对应 Node.js module/deviceinfo_center_upload.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 上报设备中心设备名称
    ///
    /// 官方 PC 端在登录成功后调用此接口，把当前登录设备在「登录设备管理」中的名称
    /// 设为自定义名称。
    ///
    /// `Query` 参数：`deviceName`（设备名称；缺失或空串时回退到 `name`）。按上游语义
    /// 先做 `trim`，结果为空时返回参数错误。
    ///
    /// 对应 `/api/deviceinfo/center/upload`，使用 eapi 加密。
    pub async fn deviceinfo_center_upload(&self, query: &Query) -> Result<ApiResponse> {
        let device_name = query
            .get("deviceName")
            .filter(|v| !v.is_empty())
            .or_else(|| query.get("name").filter(|v| !v.is_empty()))
            .unwrap_or("")
            .trim();
        if device_name.is_empty() {
            return Err(NcmError::InvalidParam(
                "缺少必要参数: deviceName".to_string(),
            ));
        }
        let data = json!({
            "deviceName": device_name
        });
        self.request(
            "/api/deviceinfo/center/upload",
            data,
            query.to_option(CryptoType::Eapi),
        )
        .await
    }
}
