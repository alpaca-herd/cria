use super::Query;
use crate::error::Result;
/// 每日推荐歌曲 - v1
/// 对应 Node.js module/recommend_songs_v1.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 每日推荐歌曲 - v1（需要登录）
    ///
    /// `Query` 参数：`ispush`（可选；存在且非空时按原字符串透传，缺失或空串时传布尔 `false`，
    /// 与上游 `query.ispush || false` 一致）。
    ///
    /// 对应 `/api/v3/discovery/recommend/songs`，使用 xeapi 加密。
    pub async fn recommend_songs_v1(&self, query: &Query) -> Result<ApiResponse> {
        let ispush = match query.get("ispush") {
            Some(value) if !value.is_empty() => json!(value),
            _ => json!(false),
        };
        let data = json!({
            "ispush": ispush
        });
        self.request(
            "/api/v3/discovery/recommend/songs",
            data,
            query.to_option(CryptoType::Xeapi),
        )
        .await
    }
}
