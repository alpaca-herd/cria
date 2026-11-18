use super::Query;
use crate::error::{NcmError, Result};
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::{Value, json};

impl ApiClient {
    /// 看广告领取免费听歌时长 / 云贝等权益
    ///
    /// 对应 Node.js `module/ad_listening_rights_gain.js`，请求
    /// `POST /api/ad/listening/rights/gain`，使用 xeapi 加密并携带实时易盾反作弊
    /// token（上游 `createOption(query, 'xeapi', 'v3')`）。上游把权限参数序列化成
    /// `reqParam` JSON 字符串提交，成功后把原始响应体放进 `{ code: 200, data }`
    /// 返回。
    ///
    /// Query 参数（均应是调用方真实广告上下文中的值）：
    /// - `reqUid`：必填，广告平台响应里的 `req_id`；缺失或空串返回
    ///   `NcmError::InvalidParam`；
    /// - `contextInfo`：广告上下文 JSON 字符串，原样写入 `reqParam.contextInfo`；
    /// - `creativeType`：广告创意类型，缺省 `36`（激励视频）；
    /// - `generalRightsInfo`：广告下发的通用权益信息，原样写入；
    /// - `rightsGainMethod`：权益领取方式（1 曝光、2 曝光+点击、3 曝光+下载、
    ///   4 点击+停留、5 曝光或点击停留、6 拉新曝光/下载），缺省 `2`；
    /// - `exposureTime` / `clickTime`：真实曝光、点击时间戳；未提供时不写该字段；
    /// - `uid`：当前登录用户 ID，写入 `reqParam.userId`；
    /// - `extraRightsType`、`source`、`extraRightsGainMethod`、
    ///   `extraRightsGainDuration`、`nextRightsGainDuration`、`rightsGainType`、
    ///   `rightsGainDuration`、`gainMethodStep`、`installed`、`sniffTime`：
    ///   按 JS `parseInt` 语义转成数字，未提供时不写；
    /// - `playContinuously`：非空字符串即 `true`（与上游 JS 真值判断一致）；
    /// - `rightsExtJson`：权益扩展信息，空串时不写；
    /// - `appInfo`：应用信息 JSON 字符串，解析后写入，非法 JSON 返回参数错误。
    ///
    /// 与上游的刻意差异：上游在缺少 `reqUid` / `contextInfo` / `creativeType` 时会
    /// 自动调用 `ad_get`，用广告下发值补齐，并把 `exposureTime`、`clickTime`
    /// 以及方式 3/6 的 `sniffTime` 补成当前时间。本实现不自动拉广告、也不伪造
    /// 曝光/点击/嗅探时间，只接受调用方传入的真实广告上下文；缺少 `reqUid` 直接
    /// 返回参数错误。
    pub async fn ad_listening_rights_gain(&self, query: &Query) -> Result<ApiResponse> {
        let Some(req_uid) = query.get("reqUid").filter(|value| !value.is_empty()) else {
            return Err(NcmError::InvalidParam(
                "缺少必要参数：reqUid（真实广告请求返回的 req_id）".to_string(),
            ));
        };

        let rights_gain_method = query
            .get("rightsGainMethod")
            .filter(|value| !value.is_empty())
            .map(parse_int)
            .unwrap_or(JsInt::Number(2));
        let creative_type = query
            .get("creativeType")
            .map(parse_int)
            .unwrap_or(JsInt::Number(36));
        let play_continuously = query
            .get("playContinuously")
            .is_some_and(|value| !value.is_empty());
        let exposure_time = parse_int_field(query, "exposureTime");
        let click_time = parse_int_field(query, "clickTime");
        let app_info = match query.get("appInfo").filter(|value| !value.is_empty()) {
            Some(text) => Some(
                serde_json::from_str::<Value>(text)
                    .map_err(|_| NcmError::InvalidParam("appInfo 必须是合法的 JSON".to_string()))?,
            ),
            None => None,
        };

        let mut rights_param = json!({});
        rights_param["reqUid"] = Value::String(req_uid.to_string());
        if let Some(value) = exposure_time {
            rights_param["exposureTime"] = value.into_json();
        }
        if let Some(value) = parse_int_field(query, "uid") {
            rights_param["userId"] = value.into_json();
        }
        if let Some(value) = click_time {
            rights_param["clickTime"] = value.into_json();
        }
        if let Some(value) = parse_int_field(query, "extraRightsType") {
            rights_param["extraRightsType"] = value.into_json();
        }
        rights_param["playContinuously"] = Value::Bool(play_continuously);
        if let Some(value) = parse_int_field(query, "source") {
            rights_param["source"] = value.into_json();
        }
        rights_param["creativeType"] = creative_type.into_json();
        rights_param["rightsGainMethod"] = rights_gain_method.into_json();
        if let Some(value) = parse_int_field(query, "extraRightsGainMethod") {
            rights_param["extraRightsGainMethod"] = value.into_json();
        }
        if let Some(value) = parse_int_field(query, "extraRightsGainDuration") {
            rights_param["extraRightsGainDuration"] = value.into_json();
        }
        if let Some(value) = parse_int_field(query, "nextRightsGainDuration") {
            rights_param["nextRightsGainDuration"] = value.into_json();
        }
        if let Some(value) = parse_int_field(query, "rightsGainType") {
            rights_param["rightsGainType"] = value.into_json();
        }
        if let Some(value) = parse_int_field(query, "rightsGainDuration") {
            rights_param["rightsGainDuration"] = value.into_json();
        }
        if let Some(value) = parse_int_field(query, "gainMethodStep") {
            rights_param["gainMethodStep"] = value.into_json();
        }
        if let Some(value) = query.get("generalRightsInfo") {
            rights_param["generalRightsInfo"] = Value::String(value.to_string());
        }
        if let Some(value) = query.get("rightsExtJson").filter(|value| !value.is_empty()) {
            rights_param["rightsExtJson"] = Value::String(value.to_string());
        }
        if let Some(value) = app_info {
            rights_param["appInfo"] = value;
        }
        if let Some(value) = query.get("contextInfo") {
            rights_param["contextInfo"] = Value::String(value.to_string());
        }
        if let Some(value) = parse_int_field(query, "installed") {
            rights_param["installed"] = value.into_json();
        }
        if (rights_gain_method.is(3) || rights_gain_method.is(6))
            && let Some(value) = parse_int_field(query, "sniffTime")
        {
            rights_param["sniffTime"] = value.into_json();
        }

        let data = json!({
            "reqParam": rights_param.to_string(),
        });
        let mut option = query.to_option(CryptoType::Xeapi);
        option.check_token = true;
        let res = self
            .request("/api/ad/listening/rights/gain", data, option)
            .await?;

        Ok(ApiResponse {
            status: 200,
            body: json!({
                "code": 200,
                "data": res.body,
            }),
            cookie: vec![],
        })
    }
}

/// 上游对可选数字参数的写法是 `query.x ? parseInt(query.x) : undefined`：
/// 只有空串（JS falsy）才省略，非空串走 `parseInt`；解析失败时 JS 得到 `NaN`，
/// `JSON.stringify` 会把它写成 `null`。
#[derive(Clone, Copy)]
enum JsInt {
    Number(i64),
    NaN,
}

impl JsInt {
    fn into_json(self) -> Value {
        match self {
            JsInt::Number(value) => json!(value),
            JsInt::NaN => Value::Null,
        }
    }

    fn is(self, expected: i64) -> bool {
        matches!(self, JsInt::Number(value) if value == expected)
    }
}

fn parse_int_field(query: &Query, key: &str) -> Option<JsInt> {
    query
        .get(key)
        .filter(|value| !value.is_empty())
        .map(parse_int)
}

/// 复刻 JS `parseInt(value)` 的十进制前缀解析：跳过前导空白，允许正负号，
/// 读到第一个非数字为止；没有数字时为 `NaN`。超出 `i64` 时同样记为 `NaN`。
fn parse_int(text: &str) -> JsInt {
    let trimmed = text.trim_start();
    let (negative, digits) = match trimmed.strip_prefix('-') {
        Some(rest) => (true, rest),
        None => (false, trimmed.strip_prefix('+').unwrap_or(trimmed)),
    };
    let mut value: i64 = 0;
    let mut has_digit = false;
    for character in digits.chars() {
        let Some(digit) = character.to_digit(10) else {
            break;
        };
        has_digit = true;
        let Some(next) = value
            .checked_mul(10)
            .and_then(|value| value.checked_add(i64::from(digit)))
        else {
            return JsInt::NaN;
        };
        value = next;
    }
    if !has_digit {
        return JsInt::NaN;
    }
    JsInt::Number(if negative { -value } else { value })
}
