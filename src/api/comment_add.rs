use super::Query;
use crate::error::{NcmError, Result};
/// 发送评论
/// 对应 Node.js module/comment_add.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 发送评论
    ///
    /// 对应 Node.js `module/comment_add.js`；上游走安卓客户端 `xeapi`，并强制使用
    /// v3 反作弊 token（本库以 `check_token` 表达该变体）。
    ///
    /// Query 参数：
    /// - `type`：资源类型，取 `RESOURCE_TYPE_MAP` 的键（`0` 歌曲、`1` MV、`2` 歌单、
    ///   `3` 专辑、`4` 电台、`5` 视频、`6` 动态、`7` 声音），必填
    /// - `id`：资源 ID，必填
    /// - `content`：评论内容，必填
    ///
    /// `threadId` 由资源类型前缀与 `id` 直接拼接；请求体固定携带
    /// `resourceType=0`、`expressionPicId=-1`、`bubbleId=-1`。
    ///
    /// 对应 POST /api/resource/comments/add
    pub async fn comment_add(&self, query: &Query) -> Result<ApiResponse> {
        let Some(resource_type) = query.get("type").filter(|value| !value.is_empty()) else {
            return Err(NcmError::InvalidParam("参数不足".to_string()));
        };
        let Some(id) = query.get("id").filter(|value| !value.is_empty()) else {
            return Err(NcmError::InvalidParam("参数不足".to_string()));
        };
        let Some(content) = query.get("content") else {
            return Err(NcmError::InvalidParam("参数不足".to_string()));
        };
        let Some(prefix) = crate::util::config::RESOURCE_TYPE_MAP.get(resource_type) else {
            return Err(NcmError::InvalidParam("不支持的资源类型".to_string()));
        };

        let data = json!({
            "threadId": format!("{prefix}{id}"),
            "content": content,
            "resourceType": "0",
            "expressionPicId": "-1",
            "bubbleId": "-1",
        });
        let mut option = query.to_option(CryptoType::Xeapi);
        option.check_token = true;
        self.request("/api/resource/comments/add", data, option)
            .await
    }
}
