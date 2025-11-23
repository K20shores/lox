pub struct Scanner {
    source: String,
}

impl Scanner {
    pub fn new(source: String) -> Self {
        Self { source }
    }

    pub fn scan(&self) {
        println!("Scanning `{}`", self.source);
    }
}
