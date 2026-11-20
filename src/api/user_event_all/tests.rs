use crate::{ApiClient, Query};
use serde_json::json;
use std::io::{BufRead, BufReader, Read, Write};
use std::net::TcpListener;
use std::time::Duration;

#[compio::test]
async fn later_page_error_is_not_reported_as_success() {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let mut query = Query::new();
    query.domain = Some(format!("http://{}", listener.local_addr().unwrap()));

    let failed_page = json!({"code": 502, "message": "page unavailable"});
    let responses = [
        (
            "/weapi/nuser/account/get",
            "account=1; Path=/",
            json!({"code": 200, "account": {"id": 42}}),
        ),
        (
            "/eapi/event/get/42",
            "first=1; Path=/",
            json!({"code": 200, "more": true, "lasttime": 123, "events": [{"id": 1}]}),
        ),
        (
            "/eapi/event/get/42",
            "failed=1; Path=/",
            failed_page.clone(),
        ),
    ];
    let server = std::thread::spawn(move || {
        for (path, cookie, body) in responses {
            let (mut stream, _) = listener.accept().unwrap();
            stream
                .set_read_timeout(Some(Duration::from_secs(5)))
                .unwrap();
            let mut request = BufReader::new(&mut stream);
            let mut line = String::new();
            request.read_line(&mut line).unwrap();
            assert_eq!(line, format!("POST {path} HTTP/1.1\r\n"));
            let mut length = 0;
            loop {
                line.clear();
                assert_ne!(request.read_line(&mut line).unwrap(), 0);
                if line == "\r\n" {
                    break;
                }
                if let Some(value) = line.to_ascii_lowercase().strip_prefix("content-length:") {
                    length = value.trim().parse().unwrap();
                }
            }
            std::io::copy(&mut request.take(length), &mut std::io::sink()).unwrap();
            let body = body.to_string();
            write!(
                stream,
                "HTTP/1.1 200 OK\r\nSet-Cookie: {cookie}\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}",
                body.len()
            )
            .unwrap();
        }
    });

    let http = cyper::Client::builder().no_proxy().build().unwrap();
    let mut api = ApiClient::new(None, http);
    api.set_timeout(Duration::from_secs(5));
    let response = api.user_event_all(&query).await.unwrap();
    server.join().unwrap();

    assert_eq!(response.status, 502);
    assert_eq!(response.body, failed_page);
    assert_eq!(
        response.cookie,
        ["account=1; Path=/", "first=1; Path=/", "failed=1; Path=/"]
    );
}
