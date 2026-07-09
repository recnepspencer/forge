#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryDeclarationReceiptExplanation {
    crossing_posture: &'static str,
    route_reference: Option<String>,
    retained_truths: Vec<String>,
    governing_reason: String,
}

impl WorthQueryDeclarationReceiptExplanation {
    pub(crate) fn new(
        crossing_posture: &'static str,
        route_reference: Option<String>,
        retained_truths: Vec<String>,
        governing_reason: String,
    ) -> Self {
        Self {
            crossing_posture,
            route_reference,
            retained_truths,
            governing_reason,
        }
    }

    pub fn crossing_posture(&self) -> &'static str {
        self.crossing_posture
    }

    pub fn route_reference(&self) -> Option<&str> {
        self.route_reference.as_deref()
    }

    pub fn retained_truths(&self) -> &[String] {
        &self.retained_truths
    }

    pub fn governing_reason(&self) -> &str {
        &self.governing_reason
    }
}
