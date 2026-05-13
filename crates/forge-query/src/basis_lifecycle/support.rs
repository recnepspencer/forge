use crate::identity::hash_parts;

use super::counters::BasisEligibilityCounters;
use super::taxonomy::BasisFamily;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BasisSupportPosture {
    Admitted,
    Advisory,
    Denied,
    Deferred,
    Unsupported,
}

impl BasisSupportPosture {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Admitted => "admitted",
            Self::Advisory => "advisory",
            Self::Denied => "denied",
            Self::Deferred => "deferred",
            Self::Unsupported => "unsupported",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BasisLifecycleSupportRow {
    family: BasisFamily,
    operation_lane: &'static str,
    posture: BasisSupportPosture,
    row_digest: String,
}

impl BasisLifecycleSupportRow {
    pub(crate) fn new(
        family: BasisFamily,
        operation_lane: &'static str,
        posture: BasisSupportPosture,
    ) -> Self {
        let row_digest = hash_parts(&[
            format!("family:{}", family.as_str()),
            format!("lane:{operation_lane}"),
            format!("posture:{}", posture.as_str()),
        ]);
        Self {
            family,
            operation_lane,
            posture,
            row_digest,
        }
    }

    pub fn family(&self) -> BasisFamily {
        self.family
    }

    pub fn operation_lane(&self) -> &'static str {
        self.operation_lane
    }

    pub fn posture(&self) -> BasisSupportPosture {
        self.posture
    }

    pub fn row_digest(&self) -> &str {
        &self.row_digest
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BasisLifecycleSupportMatrix {
    rows: Vec<BasisLifecycleSupportRow>,
    matrix_digest: String,
}

impl BasisLifecycleSupportMatrix {
    pub fn rows(&self) -> &[BasisLifecycleSupportRow] {
        &self.rows
    }

    pub fn matrix_digest(&self) -> &str {
        &self.matrix_digest
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BasisLifecycleSupportDiscovery {
    requested_family: BasisFamily,
    requested_operation_lane: &'static str,
    posture: BasisSupportPosture,
    matched_row_digest: Option<String>,
    support_matrix_digest: String,
    discovery_digest: String,
    counters: BasisEligibilityCounters,
}

impl BasisLifecycleSupportDiscovery {
    fn new(
        requested_family: BasisFamily,
        requested_operation_lane: &'static str,
        decision: BasisSupportDecision,
        support_matrix_digest: String,
    ) -> Self {
        let matched_row_digest = decision
            .matched_row
            .as_ref()
            .map(|row| row.row_digest().to_string());
        let counters = BasisEligibilityCounters::support_lookup(decision.rows_consulted);
        let discovery_digest = hash_parts(&[
            "basis_lifecycle_support_discovery_v1".to_string(),
            format!("family:{}", requested_family.as_str()),
            format!("lane:{requested_operation_lane}"),
            format!("posture:{}", decision.posture.as_str()),
            format!(
                "matched_row:{}",
                matched_row_digest.as_deref().unwrap_or("unsupported")
            ),
            format!("matrix:{support_matrix_digest}"),
            format!("counters:{}", counters.digest()),
        ]);
        Self {
            requested_family,
            requested_operation_lane,
            posture: decision.posture,
            matched_row_digest,
            support_matrix_digest,
            discovery_digest,
            counters,
        }
    }

    pub fn requested_family(&self) -> BasisFamily {
        self.requested_family
    }

    pub fn requested_operation_lane(&self) -> &'static str {
        self.requested_operation_lane
    }

    pub fn posture(&self) -> BasisSupportPosture {
        self.posture
    }

    pub fn matched_row_digest(&self) -> Option<&str> {
        self.matched_row_digest.as_deref()
    }

    pub fn support_matrix_digest(&self) -> &str {
        &self.support_matrix_digest
    }

    pub fn discovery_digest(&self) -> &str {
        &self.discovery_digest
    }

    pub fn counters(&self) -> &BasisEligibilityCounters {
        &self.counters
    }
}

pub fn basis_lifecycle_support_matrix() -> BasisLifecycleSupportMatrix {
    let rows = support_rows()
        .iter()
        .map(|(family, lane, posture)| BasisLifecycleSupportRow::new(*family, lane, *posture))
        .collect::<Vec<_>>();
    let matrix_digest = hash_parts(
        &rows
            .iter()
            .map(|row| row.row_digest().to_string())
            .collect::<Vec<_>>(),
    );
    BasisLifecycleSupportMatrix {
        rows,
        matrix_digest,
    }
}

pub fn discover_basis_lifecycle_support(
    family: BasisFamily,
    operation_lane: &'static str,
) -> BasisLifecycleSupportDiscovery {
    let support_matrix = basis_lifecycle_support_matrix();
    BasisLifecycleSupportDiscovery::new(
        family,
        operation_lane,
        support_decision_for(family, operation_lane),
        support_matrix.matrix_digest().to_string(),
    )
}

pub(crate) fn support_posture_for(
    family: BasisFamily,
    operation_lane: &'static str,
) -> BasisSupportPosture {
    support_decision_for(family, operation_lane).posture
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct BasisSupportDecision {
    posture: BasisSupportPosture,
    matched_row: Option<BasisLifecycleSupportRow>,
    rows_consulted: usize,
}

fn support_decision_for(family: BasisFamily, operation_lane: &'static str) -> BasisSupportDecision {
    let mut rows_consulted = 0;
    for (row_family, row_lane, posture) in support_rows() {
        rows_consulted += 1;
        if *row_family == family && *row_lane == operation_lane {
            return BasisSupportDecision {
                posture: *posture,
                matched_row: Some(BasisLifecycleSupportRow::new(
                    *row_family,
                    *row_lane,
                    *posture,
                )),
                rows_consulted,
            };
        }
    }
    BasisSupportDecision {
        posture: BasisSupportPosture::Unsupported,
        matched_row: None,
        rows_consulted,
    }
}

fn support_rows() -> &'static [(BasisFamily, &'static str, BasisSupportPosture)] {
    use BasisFamily::*;
    use BasisSupportPosture::*;
    &[
        (CurrentHead, "observation", Admitted),
        (CurrentHead, "mutation_preparation", Admitted),
        (CurrentHead, "inspection", Admitted),
        (CurrentHead, "materialization", Admitted),
        (CurrentHead, "subscription_declaration", Admitted),
        (CurrentHead, "subscription_activation", Admitted),
        (CurrentHead, "certification", Admitted),
        (BranchHead, "observation", Admitted),
        (BranchHead, "mutation_preparation", Admitted),
        (BranchHead, "inspection", Admitted),
        (BranchHead, "subscription_declaration", Admitted),
        (BranchSnapshot, "observation", Admitted),
        (BranchSnapshot, "inspection", Admitted),
        (Preview, "preview_closeout", Admitted),
        (Preview, "inspection", Admitted),
        (PreviewDerived, "inspection", Advisory),
        (RuntimeSnapshot, "observation", Admitted),
        (RuntimeSnapshot, "inspection", Admitted),
        (HistoricalSnapshot, "observation", Admitted),
        (HistoricalSnapshot, "replay", Admitted),
        (HistoricalSnapshot, "inspection", Admitted),
        (TenantScoped, "observation", Admitted),
        (TenantScoped, "mutation_preparation", Admitted),
        (TenantScoped, "inspection", Admitted),
        (PolicyScoped, "observation", Admitted),
        (PolicyScoped, "mutation_preparation", Admitted),
        (PolicyScoped, "inspection", Admitted),
        (StoreBacked, "observation", Deferred),
        (StoreBacked, "replay", Deferred),
        (DurableReload, "certification", Deferred),
    ]
}

#[cfg(test)]
mod tests;
