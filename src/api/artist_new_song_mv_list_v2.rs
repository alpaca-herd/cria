use super::Query;
use crate::error::Result;
/// 获取关注歌手的新歌曲和 MV
/// 对应 Node.js module/artist_new_song_mv_list_v2.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 获取关注歌手的新歌曲和 MV
    /// 对应 /artist/new/song/mv/list/v2
    pub async fn artist_new_song_mv_list_v2(&self, query: &Query) -> Result<ApiResponse> {
        let start_timestamp = if let Some(v) = query.get("startTimestamp").filter(|v| !v.is_empty())
        {
            json!(v)
        } else if let Some(v) = query.get("before").filter(|v| !v.is_empty()) {
            json!(v)
        } else {
            json!(chrono::Utc::now().timestamp_millis())
        };
        let source_type = match query.get("sourceType") {
            Some(v) if !v.is_empty() => json!(v),
            _ => json!(1),
        };
        let limit = match query.get("limit") {
            Some(v) if !v.is_empty() => json!(v),
            _ => json!(10),
        };
        let first_request = match query.get("firstRequest") {
            Some(v) => json!(v),
            None => json!(true),
        };
        let data = json!({
            "startTimestamp": start_timestamp,
            "sourceType": source_type,
            "limit": limit,
            "firstRequest": first_request,
        });
        self.request(
            "/api/sub/artist/new/works/song-mv/list/v2",
            data,
            query.to_option(CryptoType::Eapi),
        )
        .await
    }
}
