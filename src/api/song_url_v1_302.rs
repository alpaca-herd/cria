use super::Query;
use crate::error::Result;
/// 获取客户端歌曲下载链接 - v1 (302 重定向)
/// 对应 Node.js module/song_url_v1_302.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::{json, Value};

fn extract_url(body: &Value) -> Option<String> {
    body.get("data").and_then(|data| {
        data.get("url")
            .and_then(Value::as_str)
            .filter(|url| !url.is_empty())
            .map(str::to_string)
            .or_else(|| {
                data.get(0)
                    .and_then(|item| item.get("url"))
                    .and_then(Value::as_str)
                    .filter(|url| !url.is_empty())
                    .map(str::to_string)
            })
    })
}

impl ApiClient {
    /// 获取客户端歌曲下载链接 (302 重定向)
    /// 对应 /song/url/v1/302
    pub async fn song_url_v1_302(&self, query: &Query) -> Result<ApiResponse> {
        let id = query.get_or("id", "0");
        let level = query.get_or("level", "standard");
        let mut option = query.to_option(CryptoType::default());
        if level == "vivid" {
            option.cookie_overrides = crate::util::config::VIVID_COOKIE_OVERRIDES.to_vec();
        }
        let response = self
            .request(
                "/api/song/enhance/download/url/v1",
                json!({ "id": &id, "immerseType": "c51", "level": &level }),
                option.clone(),
            )
            .await?;
        if let Some(url) = extract_url(&response.body) {
            return Ok(ApiResponse {
                status: 302,
                body: json!({ "redirectUrl": url }),
                cookie: response.cookie,
            });
        }

        let mut fallback_data = json!({
            "ids": format!("[{}]", id),
            "level": &level,
            "encodeType": if level == "vivid" { "mp3" } else { "flac" }
        });
        if level == "sky" {
            fallback_data["immerseType"] = json!("c51");
        }
        let fallback = self
            .request(
                "/api/song/enhance/player/url/v1",
                fallback_data,
                option,
            )
            .await?;
        match extract_url(&fallback.body) {
            Some(url) => Ok(ApiResponse {
                status: 302,
                body: json!({ "redirectUrl": url }),
                cookie: fallback.cookie,
            }),
            None => Ok(fallback),
        }
    }
}
