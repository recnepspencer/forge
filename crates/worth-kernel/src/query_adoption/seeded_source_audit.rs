use super::classification::WorthQueryAdoptionForbiddenPattern;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryAdoptionSeededFinding {
    source_name: String,
    pattern: WorthQueryAdoptionForbiddenPattern,
    localized_phrase: String,
}

impl WorthQueryAdoptionSeededFinding {
    fn new(
        source_name: impl Into<String>,
        pattern: WorthQueryAdoptionForbiddenPattern,
        localized_phrase: impl Into<String>,
    ) -> Self {
        Self {
            source_name: source_name.into(),
            pattern,
            localized_phrase: localized_phrase.into(),
        }
    }

    pub fn source_name(&self) -> &str {
        &self.source_name
    }

    pub const fn pattern(&self) -> WorthQueryAdoptionForbiddenPattern {
        self.pattern
    }

    pub fn localized_phrase(&self) -> &str {
        &self.localized_phrase
    }
}

pub fn detect_seeded_forbidden_patterns(
    source_name: &str,
    source_text: &str,
) -> Vec<WorthQueryAdoptionSeededFinding> {
    seeded_pattern_phrases()
        .into_iter()
        .filter_map(|(pattern, phrases)| {
            phrases
                .iter()
                .find(|phrase| source_text.contains(**phrase))
                .map(|phrase| WorthQueryAdoptionSeededFinding::new(source_name, pattern, *phrase))
        })
        .collect()
}

fn seeded_pattern_phrases() -> Vec<(WorthQueryAdoptionForbiddenPattern, [&'static str; 3])> {
    use WorthQueryAdoptionForbiddenPattern::{
        DirectSupportPostureAssumption, ForgedEvidenceRow, LowerAuthorityIdentityReconstruction,
        SyntheticReceipt, TestFixtureTruthPromotion,
    };

    vec![
        (
            SyntheticReceipt,
            ["SyntheticReceipt", "synthetic receipt", "manual receipt"],
        ),
        (
            ForgedEvidenceRow,
            [
                "ForgedEvidenceRow",
                "hand-built evidence row",
                "forged evidence",
            ],
        ),
        (
            DirectSupportPostureAssumption,
            [
                "support_posture_assumed",
                "direct support posture",
                "unpinned support",
            ],
        ),
        (
            LowerAuthorityIdentityReconstruction,
            [
                "from_raw_identity",
                "lower-authority identity",
                "identity reconstruction",
            ],
        ),
        (
            TestFixtureTruthPromotion,
            ["fixture_truth", "fixture as truth", "private fixture truth"],
        ),
    ]
}
