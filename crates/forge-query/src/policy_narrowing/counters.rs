use crate::authorized_projection::AuthorizedProjectionCounters;
use crate::relationship_proof::RelationshipProofCounters;

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct PolicyNarrowingCounters {
    authorized_projection: AuthorizedProjectionCounters,
    relationship_proof: RelationshipProofCounters,
    validation_report_count: usize,
    narrowed_artifact_count: usize,
    unknown_narrowing_cost_denial_count: usize,
}

impl PolicyNarrowingCounters {
    pub(crate) fn new(
        authorized_projection: AuthorizedProjectionCounters,
        relationship_proof: RelationshipProofCounters,
    ) -> Self {
        Self {
            authorized_projection,
            relationship_proof,
            validation_report_count: 1,
            narrowed_artifact_count: 1,
            unknown_narrowing_cost_denial_count: 0,
        }
    }

    pub(crate) fn denied_unknown_cost() -> Self {
        Self {
            unknown_narrowing_cost_denial_count: 1,
            ..Self::default()
        }
    }

    pub fn authorized_projection(&self) -> &AuthorizedProjectionCounters {
        &self.authorized_projection
    }

    pub fn relationship_proof(&self) -> &RelationshipProofCounters {
        &self.relationship_proof
    }

    pub fn validation_report_count(&self) -> usize {
        self.validation_report_count
    }

    pub fn narrowed_artifact_count(&self) -> usize {
        self.narrowed_artifact_count
    }

    pub fn unknown_narrowing_cost_denial_count(&self) -> usize {
        self.unknown_narrowing_cost_denial_count
    }

    pub(crate) fn digest_parts(&self) -> Vec<String> {
        let mut parts = vec![
            format!("validation_report:{}", self.validation_report_count),
            format!("narrowed_artifact:{}", self.narrowed_artifact_count),
            format!(
                "unknown_narrowing_cost_denial:{}",
                self.unknown_narrowing_cost_denial_count
            ),
        ];
        parts.extend(self.authorized_projection.digest_parts());
        parts.extend(self.relationship_proof.digest_parts());
        parts
    }
}
