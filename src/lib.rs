pub fn getrandom(_buf: &mut [u8]) -> Result<(), ()> {
    // Do nothing and return Ok
    Ok(())
}

mod getrandom {
    #[derive(Debug)]
    pub struct Error;
}
