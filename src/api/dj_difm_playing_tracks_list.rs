use super::Query;
use crate::error::Result;
/// DIFM 电台 - 播放列表
/// 对应 Node.js module/dj_difm_playing_tracks_list.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// DIFM 电台 - 播放列表
    /// 对应 /dj/difm/playing/tracks/list
    pub async fn dj_difm_playing_tracks_list(&self, query: &Query) -> Result<ApiResponse> {
        let limit = match query.get("limit") {
            Some(v) if !v.is_empty() => json!(v),
            _ => json!(5),
        };
        let source = match query.get("source") {
            Some(v) if !v.is_empty() => json!(v),
            _ => json!(0),
        };
        let mut data = json!({
            "limit": limit,
            "source": source,
        });
        if let Some(channel_id) = query.get("channelId") {
            data["channelId"] = json!(channel_id);
        }
        self.request(
            "/api/dj/difm/playing/tracks/list",
            data,
            query.to_option(CryptoType::default()),
        )
        .await
    }
}
