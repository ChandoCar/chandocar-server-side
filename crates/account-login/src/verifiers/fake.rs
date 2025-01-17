use crate::Error;
use crate::verifiers::LoginVerifier;

pub struct Fake;

impl LoginVerifier for Fake {
    async fn verify_login<'m>(&'m self, username: &'m str, password: &'m str) -> Result<bool, Error<'m>> {
        Ok(username == "username" && password == "password")
    }
}