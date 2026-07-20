#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthQueryOrchestrationSurfaceCertificationReference {
    suite: &'static str,
    command: &'static str,
}

impl WorthQueryOrchestrationSurfaceCertificationReference {
    pub const fn new(suite: &'static str, command: &'static str) -> Self {
        Self { suite, command }
    }

    pub fn suite(&self) -> &'static str {
        self.suite
    }

    pub fn command(&self) -> &'static str {
        self.command
    }
}
