use serde_derive::Serialize;

#[derive(Serialize, Clone)]
#[non_exhaustive]
pub enum Request {
    Pools,
}

impl From<&Request> for Request {
    fn from(request: &Request) -> Self { request.clone() }
}
