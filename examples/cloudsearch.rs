//! 匿名搜索示例：`cargo run --example cloudsearch -- 海阔天空`
//!
//! 需要网络。演示客户端构造、请求与响应体读取；登录态与 cookie 持久化见 README。

use cria::{ApiClient, NcmError, Query};

#[compio::main]
async fn main() -> Result<(), NcmError> {
    let keywords = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "海阔天空".to_string());

    // 代理、连接池等网络配置都在注入的 cyper::Client 上完成。
    let http = cyper::Client::builder().build()?;
    let api = ApiClient::new(None, http);

    let query = Query::new()
        .param("keywords", &keywords)
        .param("limit", "5");
    let response = api.cloudsearch(&query).await?;

    let songs = response
        .body
        .pointer("/result/songs")
        .and_then(|songs| songs.as_array())
        .cloned()
        .unwrap_or_default();
    for song in songs {
        let name = song["name"].as_str().unwrap_or_default();
        let artists: Vec<&str> = song["ar"]
            .as_array()
            .into_iter()
            .flatten()
            .filter_map(|artist| artist["name"].as_str())
            .collect();
        println!("{} - {} (id {})", name, artists.join(" / "), song["id"]);
    }
    Ok(())
}
