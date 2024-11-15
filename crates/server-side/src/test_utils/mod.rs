use crate::test_utils::Error::InvalidInput;
use actix_web::body::MessageBody;
use actix_web::dev::ServiceResponse;

#[derive(Debug)]
pub enum Error {
    InvalidInput,
}

pub fn response_to_string(response: ServiceResponse) -> Result<String, Error> {
    let body = response
        .into_body()
        .try_into_bytes()
        .map_err(|_| InvalidInput);
    let body = body?;

    Ok(String::from_utf8_lossy(body.as_ref()).to_string())
}
