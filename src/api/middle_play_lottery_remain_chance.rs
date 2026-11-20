use super::Query;
use crate::error::Result;
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 云小编抽奖：查询剩余抽奖次数
    ///
    /// 对应 Node.js `module/middle_play_lottery_remain_chance.js`，请求
    /// `POST /api/middle/play/lottery/remain/chance`，使用 eapi 加密；上游不携带
    /// 易盾反作弊 token。
    ///
    /// Query 参数：
    /// - `activityId`：活动 ID，空串或缺省时为字符串 `"6501202"`。
    pub async fn middle_play_lottery_remain_chance(&self, query: &Query) -> Result<ApiResponse> {
        let data = json!({
            "activityId": query
                .get("activityId")
                .filter(|value| !value.is_empty())
                .unwrap_or("6501202"),
        });
        self.request(
            "/api/middle/play/lottery/remain/chance",
            data,
            query.to_option(CryptoType::Eapi),
        )
        .await
    }
}
