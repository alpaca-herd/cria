use super::Query;
use crate::error::Result;
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 云小编每日抽奖
    ///
    /// 对应 Node.js `module/middle_play_do_lottery.js`，请求
    /// `POST /api/middle/play/do/lottery`，使用 eapi 加密。上游传入
    /// `createOption(query, 'eapi', 'v2')`，即每次请求前实时获取易盾反作弊 token；
    /// 本仓库的请求层只实现一个实时 token 入口，这里设置 `check_token`。
    ///
    /// Query 参数：
    /// - `activityId`：活动 ID，空串或缺省时为字符串 `"6501202"`；
    /// - `drawCount`：抽奖次数，空串或缺省时为字符串 `"1"`。
    pub async fn middle_play_do_lottery(&self, query: &Query) -> Result<ApiResponse> {
        let data = json!({
            "activityId": query
                .get("activityId")
                .filter(|value| !value.is_empty())
                .unwrap_or("6501202"),
            "drawCount": query
                .get("drawCount")
                .filter(|value| !value.is_empty())
                .unwrap_or("1"),
        });
        let mut option = query.to_option(CryptoType::Eapi);
        option.check_token = true;
        self.request("/api/middle/play/do/lottery", data, option)
            .await
    }
}
