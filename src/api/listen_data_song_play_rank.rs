use super::Query;
use crate::error::Result;
/// 听歌足迹 - 歌曲播放排行 (Top20)
/// 对应 Node.js module/listen_data_song_play_rank.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 听歌足迹 - 歌曲播放排行 (Top20)
    /// 对应 /listen/data/song/play/rank
    pub async fn listen_data_song_play_rank(&self, query: &Query) -> Result<ApiResponse> {
        let name_type = match query.get("type") {
            Some(v) if !v.is_empty() => json!(v),
            _ => json!("month"),
        };
        let mut data = json!({ "type": name_type });
        if let Some(end_time) = query.get("endTime") {
            data["endTime"] = json!(end_time);
        }
        self.request(
            "/api/content/activity/listen/data/song/play/rank",
            data,
            query.to_option(CryptoType::default()),
        )
        .await
    }
}
