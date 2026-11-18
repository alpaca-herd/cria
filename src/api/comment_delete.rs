use super::Query;
use crate::error::{NcmError, Result};
/// 删除评论
/// 对应 Node.js module/comment_delete.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 删除评论
    ///
    /// 对应 Node.js `module/comment_delete.js`；上游走安卓客户端 `xeapi`，不附加反作弊 token。
    ///
    /// Query 参数：
    /// - `type`：资源类型，取 `RESOURCE_TYPE_MAP` 的键（`0` 歌曲、`1` MV、`2` 歌单、
    ///   `3` 专辑、`4` 电台、`5` 视频、`6` 动态、`7` 声音），必填
    /// - `id`：资源 ID，必填
    /// - `cid`：要删除的评论 ID，必填
    ///
    /// 请求体只包含 `commentId` 与由类型前缀、`id` 拼接出的 `threadId`。
    ///
    /// 对应 POST /api/resource/comments/delete
    pub async fn comment_delete(&self, query: &Query) -> Result<ApiResponse> {
        let Some(resource_type) = query.get("type").filter(|value| !value.is_empty()) else {
            return Err(NcmError::InvalidParam("参数不足".to_string()));
        };
        let Some(id) = query.get("id").filter(|value| !value.is_empty()) else {
            return Err(NcmError::InvalidParam("参数不足".to_string()));
        };
        let Some(cid) = query.get("cid").filter(|value| !value.is_empty()) else {
            return Err(NcmError::InvalidParam("参数不足".to_string()));
        };
        let Some(prefix) = crate::util::config::RESOURCE_TYPE_MAP.get(resource_type) else {
            return Err(NcmError::InvalidParam("不支持的资源类型".to_string()));
        };

        let data = json!({
            "commentId": cid,
            "threadId": format!("{prefix}{id}"),
        });
        self.request(
            "/api/resource/comments/delete",
            data,
            query.to_option(CryptoType::Xeapi),
        )
        .await
    }
}
