use crate::observation_contract::PlatformPulseQueryActionPreconditionDenial;

/// Concise product language for one real Query-backed action that stopped at its
/// own precondition before Query was asked. The typed precondition remains
/// available separately in the external observation stream. This story never
/// claims that Query denied anything.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PlatformPulseQueryDenialStory {
    denial: PlatformPulseQueryActionPreconditionDenial,
}

impl PlatformPulseQueryDenialStory {
    pub const fn new(denial: PlatformPulseQueryActionPreconditionDenial) -> Self {
        Self { denial }
    }

    pub const fn denial(self) -> PlatformPulseQueryActionPreconditionDenial {
        self.denial
    }

    pub const fn title(self) -> &'static str {
        "Route admitted, action stopped"
    }

    pub const fn explanation(self) -> &'static str {
        match self.denial {
            PlatformPulseQueryActionPreconditionDenial::SourceRevisionMismatch => {
                "UI route admitted · stale source revision · Query not asked"
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn product_copy_preserves_the_exact_precondition_without_claiming_query_denied() {
        let story = PlatformPulseQueryDenialStory::new(
            PlatformPulseQueryActionPreconditionDenial::SourceRevisionMismatch,
        );
        assert_eq!(
            story.denial(),
            PlatformPulseQueryActionPreconditionDenial::SourceRevisionMismatch
        );
        assert!(story.explanation().contains("stale source revision"));
        assert!(story.explanation().contains("Query not asked"));
        assert!(!story.title().contains("Query"));
    }
}
