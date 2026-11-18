use super::Query;
use crate::error::{NcmError, Result};
/// 评论默认文案
/// 对应 Node.js module/comment_default_text.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 评论默认文案
    ///
    /// 对应 Node.js `module/comment_default_text.js`；上游走 weapi。
    ///
    /// Query 参数：
    /// - `threadId`：评论线程 ID（如 `R_SO_4_<歌曲 ID>`），必填
    ///
    /// 请求体只携带 `threadId`。
    ///
    /// 对应 POST /api/comment/default/text
    pub async fn comment_default_text(&self, query: &Query) -> Result<ApiResponse> {
        let Some(thread_id) = query.get("threadId").filter(|value| !value.is_empty()) else {
            return Err(NcmError::InvalidParam("参数不足".to_string()));
        };

        let data = json!({ "threadId": thread_id });
        self.request(
            "/api/comment/default/text",
            data,
            query.to_option(CryptoType::Weapi),
        )
        .await
    }
}
