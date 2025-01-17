// This lib provides a function to verify a user's login credentials with moodle.

#[allow(unused)]
mod verifiers;

pub use verifiers::LoginVerifier;

pub use verifiers::moodle::Moodle;
pub use verifiers::fake::Fake;

#[derive(Debug)]
pub enum Error<'m>
{
    UnexpectedResponse(&'m str),
    NetworkError(&'m str),
}
