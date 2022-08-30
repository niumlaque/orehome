pub struct Verbose {
    verbose: bool,
}

impl Verbose {
    pub fn new(verbose: bool) -> Self {
        Self { verbose }
    }

    pub fn get(&self) -> bool {
        self.verbose
    }
}
