use std::error::Error;

#[derive(Debug)]
pub struct ApiClientError<'a>(Box<dyn Error + Send + Sync + 'a>);

impl<'a, T> From<T> for ApiClientError<'a>
where
    T: Error + Send + Sync + 'a,
{
    fn from(value: T) -> Self {
        ApiClientError(Box::new(value))
    }
}
