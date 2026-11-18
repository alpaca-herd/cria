use super::Query;
use crate::error::Result;
/// 播客分类列表（新版）
/// 对应 Node.js module/dj_category_list.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 播客分类列表（新版），返回一级分类及其下全部子分类
    /// 对应 /dj/category/list
    pub async fn dj_category_list(&self, query: &Query) -> Result<ApiResponse> {
        let data = json!({});
        self.request(
            "/api/dj/radio/category/list",
            data,
            query.to_option(CryptoType::Xeapi),
        )
        .await
    }
}
