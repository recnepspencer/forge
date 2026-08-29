use crate::observation_contract::PlatformPulseQueryAdmissionDenial;

/// Concise product language for one real Query admission denial. The typed
/// denial remains available separately in the external observation stream.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PlatformPulseQueryDenialStory {
    denial: PlatformPulseQueryAdmissionDenial,
}

impl PlatformPulseQueryDenialStory {
    pub const fn new(denial: PlatformPulseQueryAdmissionDenial) -> Self {
        Self { denial }
    }

    pub const fn denial(self) -> PlatformPulseQueryAdmissionDenial {
        self.denial
    }

    pub const fn title(self) -> &'static str {
        "Query kept its boundary"
    }

    pub const fn explanation(self) -> &'static str {
        match self.denial {
            PlatformPulseQueryAdmissionDenial::SourceRevisionMismatch => {
                "UI route admitted · Query source revision denied"
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn product_copy_preserves_the_exact_audience_denial() {
        let story = PlatformPulseQueryDenialStory::new(
            PlatformPulseQueryAdmissionDenial::SourceRevisionMismatch,
        );
        assert_eq!(
            story.denial(),
            PlatformPulseQueryAdmissionDenial::SourceRevisionMismatch
        );
        assert!(story.explanation().contains("source revision denied"));
    }
}
