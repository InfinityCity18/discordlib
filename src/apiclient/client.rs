use reqwest::{
    header::{self, HeaderMap},
    Client,
};
use std::error::Error;

pub struct ApiClient {
    client: Client,
}

#[derive(Debug)]
pub struct ApiClientError<'a>(Box<dyn Error + Send + Sync + 'a>);

impl<'a, T> From<T> for ApiClientError<'a>
where
    T: Error + Send + Sync + 'a,
{
    fn from(value: T) -> Self {
        let bx = Box::new(value);
        ApiClientError(bx)
    }
}

impl ApiClient {
    pub async fn new(token: &str) -> Result<Self, ApiClientError> {
        let mut headers = HeaderMap::new();

        let mut auth = header::HeaderValue::from_str(token)?;
        auth.set_sensitive(true);

        headers.insert(header::AUTHORIZATION, auth);

        let client = Client::builder().default_headers(headers).build()?;

        Ok(Self { client })
    }
}
