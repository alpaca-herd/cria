use super::Query;
use crate::error::Result;
/// 我收藏的播客（Mlog）——按时间倒序分页
/// 对应 Node.js module/mlog_mycollect.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 我收藏的播客（Mlog），按时间倒序分页
    /// 对应 /mlog/mycollect
    pub async fn mlog_mycollect(&self, query: &Query) -> Result<ApiResponse> {
        let time = match query.get("time") {
            Some(v) if !v.is_empty() => json!(v),
            _ => json!("-1"),
        };
        let limit = match query.get("limit") {
            Some(v) if !v.is_empty() => json!(v),
            _ => json!(30),
        };
        let mlog_type = match query.get("type") {
            Some(v) if !v.is_empty() => json!(v),
            _ => json!(3),
        };
        let data = json!({
            "time": time,
            "limit": limit,
            "mlogType": mlog_type,
        });
        self.request(
            "/api/mlog/mycollect/bytime/get",
            data,
            query.to_option(CryptoType::Xeapi),
        )
        .await
    }
}
