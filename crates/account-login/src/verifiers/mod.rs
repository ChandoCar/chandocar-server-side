use crate::Error;

pub(crate) mod moodle;
pub(crate) mod fake;

// TODO: Remove the async if needed for aesthetic reasons
#[allow(async_fn_in_trait)]
pub trait LoginVerifier
{
    async fn verify_login<'m>(&'m self, username: &'m str, password: &'m str) -> Result<bool, Error<'m>>;
}