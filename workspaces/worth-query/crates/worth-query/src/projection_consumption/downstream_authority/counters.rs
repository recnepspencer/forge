#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct ConsumedProjectionAuthorityCounters {
    relationship_checks: usize,
    requirement_checks: usize,
    source_reference_checks: usize,
    consumed_fact_visits: usize,
    authority_constructions: usize,
}

impl ConsumedProjectionAuthorityCounters {
    pub fn relationship_checks(&self) -> usize {
        self.relationship_checks
    }

    pub fn source_reference_checks(&self) -> usize {
        self.source_reference_checks
    }

    pub fn requirement_checks(&self) -> usize {
        self.requirement_checks
    }

    pub fn consumed_fact_visits(&self) -> usize {
        self.consumed_fact_visits
    }

    pub fn authority_constructions(&self) -> usize {
        self.authority_constructions
    }

    pub(super) fn checked(
        relationship_checks: usize,
        requirement_checks: usize,
        source_reference_checks: usize,
        consumed_fact_visits: usize,
    ) -> Self {
        Self {
            relationship_checks,
            requirement_checks,
            source_reference_checks,
            consumed_fact_visits,
            authority_constructions: 1,
        }
    }

    pub(super) fn denied(
        relationship_checks: usize,
        requirement_checks: usize,
        source_reference_checks: usize,
    ) -> Self {
        Self {
            relationship_checks,
            requirement_checks,
            source_reference_checks,
            consumed_fact_visits: 0,
            authority_constructions: 0,
        }
    }
}
