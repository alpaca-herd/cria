use super::Query;
use crate::error::{NcmError, Result};
/// 举报评论
/// 对应 Node.js module/comment_report.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 举报评论
    ///
    /// 对应 Node.js `module/comment_report.js`；上游使用默认的 eapi 加密，
    /// 且不附加反作弊 token。`threadId` 固定按歌曲评论前缀 `R_SO_4_` 拼接，
    /// 不接受资源类型参数。
    ///
    /// Query 参数：
    /// - `id`：歌曲 ID，必填
    /// - `cid`：被举报的评论 ID，必填
    /// - `reason`：举报理由 ID 或理由文本，必填
    ///
    /// 对应 POST /api/report/reportcomment
    pub async fn comment_report(&self, query: &Query) -> Result<ApiResponse> {
        let Some(id) = query.get("id").filter(|value| !value.is_empty()) else {
            return Err(NcmError::InvalidParam("参数不足".to_string()));
        };
        let Some(cid) = query.get("cid").filter(|value| !value.is_empty()) else {
            return Err(NcmError::InvalidParam("参数不足".to_string()));
        };
        let Some(reason) = query.get("reason").filter(|value| !value.is_empty()) else {
            return Err(NcmError::InvalidParam("参数不足".to_string()));
        };

        let data = json!({
            "threadId": format!("R_SO_4_{id}"),
            "commentId": cid,
            "reason": reason,
        });
        self.request(
            "/api/report/reportcomment",
            data,
            query.to_option(CryptoType::Eapi),
        )
        .await
    }
}
