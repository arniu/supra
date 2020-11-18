#[derive(Debug, Clone)]
pub struct Request {}

#[derive(Debug, Clone)]
pub struct Response {}

#[async_trait]
pub trait Extractor: Sized {
    type Error;

    async fn extract(req: &Request) -> Result<Self, Self::Error>;
}
