use async_trait::async_trait;
use rocket::{
    fairing::{Fairing, Info, Kind},
    http::Header,
    Request, Response,
};

/// Fairing to enable CORS for all origins. The API we're proxying is
/// completely public, so there are no security concerns around CORS. We want
/// to enable requests from any browser origin.
pub struct Cors;

#[async_trait]
impl Fairing for Cors {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to responses",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(
        &self,
        _request: &'r Request<'_>,
        response: &mut Response<'r>,
    ) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new("Access-Control-Allow-Methods", "GET"));
    }
}
