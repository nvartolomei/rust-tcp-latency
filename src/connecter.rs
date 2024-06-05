struct Connecter {}

impl Connecter {
    pub fn new() -> Self {
        Connecter {}
    }

    pub fn connect(&self) -> Result<(), Box<dyn Error>> {
        // Connect to the server
        Ok(())
    }
}
