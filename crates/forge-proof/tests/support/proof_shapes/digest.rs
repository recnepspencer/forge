#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProofShapeDigest {
    suite: &'static str,
    entries: Vec<&'static str>,
}

impl ProofShapeDigest {
    pub fn new(suite: &'static str, entries: Vec<&'static str>) -> Self {
        Self { suite, entries }
    }

    pub fn suite(&self) -> &'static str {
        self.suite
    }

    pub fn entries(&self) -> &[&'static str] {
        &self.entries
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BasisDigest {
    suite: &'static str,
    entries: Vec<&'static str>,
}

impl BasisDigest {
    pub fn new(suite: &'static str, entries: Vec<&'static str>) -> Self {
        Self { suite, entries }
    }

    pub fn suite(&self) -> &'static str {
        self.suite
    }

    pub fn entries(&self) -> &[&'static str] {
        &self.entries
    }
}
