use worth_primitives::{truth_digest_parts, TruthDigestScope};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ConflictIndependenceDisposition {
    Disjoint,
    CompatibleAspectOverlap,
    SerializableOnly,
    Denied,
}

impl ConflictIndependenceDisposition {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Disjoint => "disjoint",
            Self::CompatibleAspectOverlap => "compatible-aspect-overlap",
            Self::SerializableOnly => "serializable-only",
            Self::Denied => "denied",
        }
    }
}

pub(super) fn canonical_pair_digest(
    scope_label: &'static str,
    left_plan_digest: &str,
    right_plan_digest: &str,
    extra_parts: &[String],
) -> String {
    let mut plan_digests = [left_plan_digest.to_string(), right_plan_digest.to_string()];
    plan_digests.sort();
    let mut parts = vec![scope_label.to_string()];
    parts.extend(
        plan_digests
            .into_iter()
            .map(|digest| format!("plan:{digest}")),
    );
    parts.extend(extra_parts.iter().cloned());
    truth_digest_parts(TruthDigestScope::ArtifactIdentity, &parts)
}

pub(super) fn canonical_pair_parts(
    label: &'static str,
    left: String,
    right: String,
) -> [String; 2] {
    let mut values = [left, right];
    values.sort();
    [
        format!("{label}:{}", values[0]),
        format!("{label}:{}", values[1]),
    ]
}
