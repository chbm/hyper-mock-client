//! # hyper-mock-client
//!
//! `hyper-mock-client` is a hyper::client mock to test tower::services such as axum::router
//!
//! ## Example
//!
//! ```
//! mod tests {
//!    use hyper::{Request,Body, StatusCode};
//!    use axum::{Router};
//!    use hyper_mock_client::MockClient;
//!
//!    let app = super::MakeApiApp(); 
//!    let mut client = MockClient::new(app);
//!
//!    let resp = client.get("/status").await;
//!
//! ```

use axum::Router;
use axum::response::Response;
use hyper::service::Service;
use hyper::{Uri, Request, Body};


/// The mock client, instantiate with `new`  
pub struct MockClient {
    svc: Router,
}

impl MockClient {
    /// Creates a new mock client for the service
    pub fn new(svc: Router) -> MockClient {
        MockClient { svc }
    }
    
    /// Simplified GET request
    pub async fn get(&mut self, uri: Uri) -> Response {
        let req = Request::builder().uri(uri.to_string()).body(Body::empty()).unwrap();
        self.request(req).await
    }

    /// Full `hyper::Request` request
    pub async fn request(&mut self, req: Request<Body>) -> Response {
        self.svc.call(req).await.unwrap()
    }
}

#[cfg(test)]
mod tests {
    use axum::{
        Router,
        routing::{get,post,put,delete}, 
        http::StatusCode, body::Bytes,
    };
    use hyper::Body;

    use super::*;

    async fn ok_handler() -> Result<String, StatusCode> {
        Ok("pong".to_string())
    }
    async fn created_handler() -> Result<String, StatusCode> {
        Err(StatusCode::CREATED)
    }
    async fn echo_handler(body: Bytes) -> Result<Bytes, StatusCode> {
        Ok(body)
    }
    async fn error_handler() -> Result<String, StatusCode> {
        Err(StatusCode::NOT_ACCEPTABLE)
    }
    
    #[tokio::test]
    async fn test_basic() {
        let app = Router::new()
            .route("/ping", get(ok_handler))
            .route("/ping", post(created_handler))
            .route("/ping", put(echo_handler))
            .route("/ping", delete(error_handler));

        let mut client = MockClient::new(app);
        
        let path = Uri::from_static("/ping");
        let resp = client.get(path).await;
        assert_eq!( resp.status(), StatusCode::OK);
        let b = hyper::body::to_bytes(resp).await.unwrap();
        assert_eq!( b , "pong" );

        let path = Uri::from_static("/ping");
        let req = Request::post(path).body(Body::empty()).unwrap();
        let resp = client.request(req).await;
        assert_eq!( resp.status(), StatusCode::CREATED);

        let path = Uri::from_static("/ping");
        let bod = Body::from("ola");
        let req = Request::put(path).body(bod).unwrap();
        let resp = client.request(req).await;
        assert_eq!( resp.status(), StatusCode::OK);
        assert_eq!( hyper::body::to_bytes(resp).await.unwrap(), Bytes::from("ola"));

        let path = Uri::from_static("/ping");
        let req = Request::delete(path).body(Body::empty()).unwrap();
        let resp = client.request(req).await;
        assert_eq!( resp.status(), StatusCode::NOT_ACCEPTABLE);

    }
}
