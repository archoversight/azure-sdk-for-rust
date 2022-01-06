#![doc = "generated by AutoRust 0.1.0"]
#![allow(unused_mut)]
#![allow(unused_variables)]
#![allow(unused_imports)]
use super::models;
#[derive(Clone)]
pub struct Client {
    endpoint: String,
    credential: std::sync::Arc<dyn azure_core::auth::TokenCredential>,
    scopes: Vec<String>,
    pipeline: azure_core::Pipeline,
}
#[derive(Clone)]
pub struct ClientBuilder {
    credential: std::sync::Arc<dyn azure_core::auth::TokenCredential>,
    endpoint: Option<String>,
    scopes: Option<Vec<String>>,
}
pub const DEFAULT_ENDPOINT: &str = azure_core::resource_manager_endpoint::AZURE_PUBLIC_CLOUD;
impl ClientBuilder {
    pub fn new(credential: std::sync::Arc<dyn azure_core::auth::TokenCredential>) -> Self {
        Self {
            credential,
            endpoint: None,
            scopes: None,
        }
    }
    pub fn endpoint(mut self, endpoint: impl Into<String>) -> Self {
        self.endpoint = Some(endpoint.into());
        self
    }
    pub fn scopes(mut self, scopes: &[&str]) -> Self {
        self.scopes = Some(scopes.iter().map(|scope| (*scope).to_owned()).collect());
        self
    }
    pub fn build(self) -> Client {
        let endpoint = self.endpoint.unwrap_or_else(|| DEFAULT_ENDPOINT.to_owned());
        let scopes = self.scopes.unwrap_or_else(|| vec![format!("{}/", endpoint)]);
        Client::new(endpoint, self.credential, scopes)
    }
}
impl Client {
    pub(crate) fn endpoint(&self) -> &str {
        self.endpoint.as_str()
    }
    pub(crate) fn token_credential(&self) -> &dyn azure_core::auth::TokenCredential {
        self.credential.as_ref()
    }
    pub(crate) fn scopes(&self) -> Vec<&str> {
        self.scopes.iter().map(String::as_str).collect()
    }
    pub(crate) async fn send(&self, request: impl Into<azure_core::Request>) -> Result<azure_core::Response, azure_core::Error> {
        let mut context = azure_core::Context::default();
        let mut request = request.into();
        self.pipeline.send(&mut context, &mut request).await
    }
    pub fn new(
        endpoint: impl Into<String>,
        credential: std::sync::Arc<dyn azure_core::auth::TokenCredential>,
        scopes: Vec<String>,
    ) -> Self {
        let endpoint = endpoint.into();
        let pipeline = azure_core::Pipeline::new(
            option_env!("CARGO_PKG_NAME"),
            option_env!("CARGO_PKG_VERSION"),
            azure_core::ClientOptions::default(),
            Vec::new(),
            Vec::new(),
        );
        Self {
            endpoint,
            credential,
            scopes,
            pipeline,
        }
    }
}
#[non_exhaustive]
#[derive(Debug, thiserror :: Error)]
#[allow(non_camel_case_types)]
pub enum Error {
    #[error(transparent)]
    PublishCloudEventEvents(#[from] publish_cloud_event_events::Error),
    #[error(transparent)]
    PublishCustomEventEvents(#[from] publish_custom_event_events::Error),
}
impl Client {
    pub fn publish_cloud_event_events(&self, events: impl Into<Vec<models::CloudEventEvent>>) -> publish_cloud_event_events::Builder {
        publish_cloud_event_events::Builder {
            client: self.clone(),
            events: events.into(),
        }
    }
    pub fn publish_custom_event_events(&self, events: impl Into<Vec<models::CustomEventEvent>>) -> publish_custom_event_events::Builder {
        publish_custom_event_events::Builder {
            client: self.clone(),
            events: events.into(),
        }
    }
}
pub mod publish_cloud_event_events {
    use super::models;
    #[derive(Debug, thiserror :: Error)]
    pub enum Error {
        #[error("HTTP status code {}", status_code)]
        DefaultResponse { status_code: http::StatusCode },
        #[error("Failed to parse request URL: {0}")]
        ParseUrl(url::ParseError),
        #[error("Failed to build request: {0}")]
        BuildRequest(http::Error),
        #[error("Failed to serialize request body: {0}")]
        Serialize(serde_json::Error),
        #[error("Failed to get access token: {0}")]
        GetToken(azure_core::Error),
        #[error("Failed to execute request: {0}")]
        SendRequest(azure_core::Error),
        #[error("Failed to get response bytes: {0}")]
        ResponseBytes(azure_core::StreamError),
        #[error("Failed to deserialize response: {0}, body: {1:?}")]
        Deserialize(serde_json::Error, bytes::Bytes),
    }
    #[derive(Clone)]
    pub struct Builder {
        pub(crate) client: super::Client,
        pub(crate) events: Vec<models::CloudEventEvent>,
    }
    impl Builder {
        pub fn into_future(self) -> futures::future::BoxFuture<'static, std::result::Result<(), Error>> {
            Box::pin(async move {
                let url_str = &format!("{}/api/events?overload=cloudEvent", self.client.endpoint(),);
                let mut url = url::Url::parse(url_str).map_err(Error::ParseUrl)?;
                let mut req_builder = http::request::Builder::new();
                req_builder = req_builder.method(http::Method::POST);
                let credential = self.client.token_credential();
                let token_response = credential
                    .get_token(&self.client.scopes().join(" "))
                    .await
                    .map_err(Error::GetToken)?;
                req_builder = req_builder.header(http::header::AUTHORIZATION, format!("Bearer {}", token_response.token.secret()));
                url.query_pairs_mut().append_pair("api-version", "2018-01-01");
                req_builder = req_builder.header("content-type", "application/json");
                let req_body = azure_core::to_json(&self.events).map_err(Error::Serialize)?;
                req_builder = req_builder.uri(url.as_str());
                let req = req_builder.body(req_body).map_err(Error::BuildRequest)?;
                let rsp = self.client.send(req).await.map_err(Error::SendRequest)?;
                let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                match rsp_status {
                    http::StatusCode::OK => Ok(()),
                    status_code => Err(Error::DefaultResponse { status_code }),
                }
            })
        }
    }
}
pub mod publish_custom_event_events {
    use super::models;
    #[derive(Debug, thiserror :: Error)]
    pub enum Error {
        #[error("HTTP status code {}", status_code)]
        DefaultResponse { status_code: http::StatusCode },
        #[error("Failed to parse request URL: {0}")]
        ParseUrl(url::ParseError),
        #[error("Failed to build request: {0}")]
        BuildRequest(http::Error),
        #[error("Failed to serialize request body: {0}")]
        Serialize(serde_json::Error),
        #[error("Failed to get access token: {0}")]
        GetToken(azure_core::Error),
        #[error("Failed to execute request: {0}")]
        SendRequest(azure_core::Error),
        #[error("Failed to get response bytes: {0}")]
        ResponseBytes(azure_core::StreamError),
        #[error("Failed to deserialize response: {0}, body: {1:?}")]
        Deserialize(serde_json::Error, bytes::Bytes),
    }
    #[derive(Clone)]
    pub struct Builder {
        pub(crate) client: super::Client,
        pub(crate) events: Vec<models::CustomEventEvent>,
    }
    impl Builder {
        pub fn into_future(self) -> futures::future::BoxFuture<'static, std::result::Result<(), Error>> {
            Box::pin(async move {
                let url_str = &format!("{}/api/events?overload=customEvent", self.client.endpoint(),);
                let mut url = url::Url::parse(url_str).map_err(Error::ParseUrl)?;
                let mut req_builder = http::request::Builder::new();
                req_builder = req_builder.method(http::Method::POST);
                let credential = self.client.token_credential();
                let token_response = credential
                    .get_token(&self.client.scopes().join(" "))
                    .await
                    .map_err(Error::GetToken)?;
                req_builder = req_builder.header(http::header::AUTHORIZATION, format!("Bearer {}", token_response.token.secret()));
                url.query_pairs_mut().append_pair("api-version", "2018-01-01");
                req_builder = req_builder.header("content-type", "application/json");
                let req_body = azure_core::to_json(&self.events).map_err(Error::Serialize)?;
                req_builder = req_builder.uri(url.as_str());
                let req = req_builder.body(req_body).map_err(Error::BuildRequest)?;
                let rsp = self.client.send(req).await.map_err(Error::SendRequest)?;
                let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                match rsp_status {
                    http::StatusCode::OK => Ok(()),
                    status_code => Err(Error::DefaultResponse { status_code }),
                }
            })
        }
    }
}
