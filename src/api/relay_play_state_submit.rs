use super::Query;
use crate::error::{NcmError, Result};
/// 提交歌曲播放状态
///
/// 对应 Node.js `module/relay_play_state_submit.js`（api-enhanced HEAD `75bf2e5`），
/// 显式使用 weapi 加密。
///
/// Query 参数：
/// - `id`：资源 ID，必填；缺省或空串返回 `NcmError::InvalidParam`
///   （上游返回 400，消息为“缺少必要参数：id”）；
/// - `sessionId`：会话 ID，空串或缺省时随机生成 12 位大写字母/数字；
/// - `progress`：播放进度，按 `Number(progress) || 0` 转换，缺省 `0`；
/// - `playMode`：播放模式，缺省 `"list_loop"`；
/// - `type`：资源类型，缺省 `"song"`。
///
/// `progress`、`sessionId`、`playMode` 与资源信息序列化为字符串
/// `playStateSubmitReq` 后提交。
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 提交歌曲播放状态
    ///
    /// 对应 `/api/relay/play/state/submit`。
    pub async fn relay_play_state_submit(&self, query: &Query) -> Result<ApiResponse> {
        let Some(id) = query.get("id").filter(|value| !value.is_empty()) else {
            return Err(NcmError::InvalidParam("缺少必要参数：id".to_string()));
        };
        let session_id = query
            .get("sessionId")
            .filter(|value| !value.is_empty())
            .map(|value| value.to_string())
            .unwrap_or_else(generate_session_id);
        let progress = query
            .get("progress")
            .and_then(|value| value.trim().parse::<f64>().ok())
            .filter(|value| value.is_finite())
            .unwrap_or(0.0);
        // 与 JSON.stringify 一致：整数值写成 1000，而不是 1000.0。
        let progress = if progress.fract() == 0.0 {
            json!(progress as i64)
        } else {
            json!(progress)
        };
        let play_mode = query.get_or("playMode", "list_loop");
        let resource_type = query.get_or("type", "song");
        let play_state_submit_req = json!({
            "resource": {
                "id": id,
                "type": resource_type,
            },
            "progress": progress,
            "sessionId": session_id,
            "playMode": play_mode,
        })
        .to_string();
        let data = json!({
            "playStateSubmitReq": play_state_submit_req,
        });
        self.request(
            "/api/relay/play/state/submit",
            data,
            query.to_option(CryptoType::Weapi),
        )
        .await
    }
}

/// 生成 12 位大写字母或数字的会话 ID，对应上游 `generateSessionId`。
fn generate_session_id() -> String {
    use rand::RngExt;

    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
    let mut rng = rand::rng();
    (0..12)
        .map(|_| CHARSET[rng.random_range(0..CHARSET.len())] as char)
        .collect()
}
