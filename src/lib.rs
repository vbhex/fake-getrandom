pub fn getrandom(_buf: &mut [u8]) -> Result<(), Error> {
    // Do nothing and return Ok
    Ok(())
}

#[derive(Debug)]
pub struct Error;

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "getrandom error")
    }
}

impl std::error::Error for Error {}
