use crate::{
    error::{Error, Result},
    http::{Headers, HttpRequest, HttpResponse},
    transport::Transport,
};

pub(crate) struct ReqwestTransport {
    client: reqwest::Client,
}

impl ReqwestTransport {
    pub fn new(client: reqwest::Client) -> Self {
        Self { client }
    }
}

impl Transport for ReqwestTransport {

    // Map our HTTP types to Reqwest (Hyper)
    async fn send(&self, request: HttpRequest) -> Result<HttpResponse> {

        let mut request_builder = self.client.get(request.url);

        // Query params
        if request.query.iter().next().is_some() {
            let query_pairs = request.query.iter().collect::<Vec<_>>();
            request_builder = request_builder.query(&query_pairs);
        }

        // Headers
        for (k, v) in request.headers.iter() {
            request_builder = request_builder.header(k, v);
        }

        // Send
        let resp = request_builder
            .send()
            .await
            .map_err(|e| Error::Transport { source: Box::new(e) })?;

        let status = resp.status().as_u16();

        // Collect headers+
        let mut headers = Headers::new();
        for (name, value) in resp.headers().iter() {
            if let Ok(s) = value.to_str() {
                headers.insert(name.as_str(), s);
            }
        }

        // Get body
        let body = resp
            .bytes()
            .await
            .map_err(|e| Error::Transport { source: Box::new(e) })?
            .to_vec();

        Ok(HttpResponse { status, headers, body })
    }
}
