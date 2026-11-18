use super::Query;
use crate::error::Result;
/// 获取「不感兴趣」歌曲列表
/// 对应 Node.js module/dislike_list.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 获取「不感兴趣」歌曲列表
    /// 对应 /dislike/list
    pub async fn dislike_list(&self, query: &Query) -> Result<ApiResponse> {
        let data = json!({});
        self.request(
            "/api/music/dislike/get",
            data,
            query.to_option(CryptoType::Xeapi),
        )
        .await
    }
}
