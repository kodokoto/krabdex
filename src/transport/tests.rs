use crate::transport::reqwest_transport::ReqwestTransport;
use crate::{
    http::{HttpRequest, Method},
    transport::Transport,
};
use httpmock::prelude::*;
use url::Url;

#[tokio::test]
async fn reqwest_transport_sends_query_and_headers() {
    let server = MockServer::start();

    let m = server.mock(|when, then| {
        when.method(GET)
            .path("/hello")
            .query_param("limit", "20")
            .query_param("offset", "40")
            .header("x-test", "abc");

        then.status(200)
            .header("x-reply", "ok")
            .body("{\"ok\":true}");
    });

    let client = reqwest::Client::builder().build().unwrap();
    let transport = ReqwestTransport::new(client);

    let mut req = HttpRequest::new(
        Method::Get,
        Url::parse(&format!("{}/hello", server.base_url())).unwrap(),
    );
    req.query.set("limit", "20");
    req.query.set("offset", "40");
    req.headers.insert("x-test", "abc");

    let resp = transport.send(req).await.unwrap();

    m.assert();
    assert_eq!(resp.status, 200);
    assert_eq!(resp.headers.get("x-reply"), Some("ok"));
    assert_eq!(String::from_utf8_lossy(&resp.body), "{\"ok\":true}");
}

#[tokio::test]
async fn reqwest_transport_sends_no_body_for_get() {
    let server = MockServer::start();

    let m = server.mock(|when, then| {
        when.method(GET).path("/echo");
        then.status(200).body("ok");
    });

    let client = reqwest::Client::builder().build().unwrap();
    let transport = ReqwestTransport::new(client);

    let req = HttpRequest::new(
        Method::Get,
        Url::parse(&format!("{}/echo", server.base_url())).unwrap(),
    );

    let resp = transport.send(req).await.unwrap();

    m.assert();
    assert_eq!(resp.status, 200);
    assert_eq!(String::from_utf8_lossy(&resp.body), "ok");
}

#[tokio::test]
async fn reqwest_transport_maps_network_errors() {
    let client = reqwest::Client::builder().build().unwrap();
    let transport = ReqwestTransport::new(client);

    let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
    let port = listener.local_addr().unwrap().port();
    drop(listener);

    let req = HttpRequest::new(
        Method::Get,
        Url::parse(&format!("http://127.0.0.1:{port}/boom")).unwrap(),
    );

    let err = transport.send(req).await.unwrap_err();

    match err {
        crate::error::Error::Transport { .. } => {}
        other => panic!("expected Transport error, got {other:?}"),
    }
}
