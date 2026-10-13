use toolcraft_request::{Method, Request, RequestBody, RequestOptions};

#[tokio::test]
async fn generic_request_types_are_constructible() {
    let _client = Request::with_timeout(5).unwrap();
    let _options = RequestOptions {
        query: Some(vec![("key".to_owned(), "value".to_owned())]),
        headers: None,
        body: Some(RequestBody::Json(serde_json::json!({"ok": true}))),
    };
    let _method = Method::POST;
}
