use crate::basis_lifecycle::BasisFamily;
use crate::identity::hash_parts;

use super::counters::EffectLifecycleCounters;
use super::taxonomy::EffectFamily;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EffectSupportPosture {
    Admitted,
    Denied,
    RebindRequired,
    Deferred,
    Unsupported,
}

impl EffectSupportPosture {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Admitted => "admitted",
            Self::Denied => "denied",
            Self::RebindRequired => "rebind_required",
            Self::Deferred => "deferred",
            Self::Unsupported => "unsupported",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EffectLifecycleSupportRow {
    basis_family: BasisFamily,
    effect_family: EffectFamily,
    posture: EffectSupportPosture,
    row_digest: String,
}

impl EffectLifecycleSupportRow {
    pub(crate) fn new(
        basis_family: BasisFamily,
        effect_family: EffectFamily,
        posture: EffectSupportPosture,
    ) -> Self {
        let row_digest = hash_parts(&[
            format!("basis_family:{}", basis_family.as_str()),
            format!("effect_family:{}", effect_family.as_str()),
            format!("posture:{}", posture.as_str()),
        ]);
        Self {
            basis_family,
            effect_family,
            posture,
            row_digest,
        }
    }

    pub fn basis_family(&self) -> BasisFamily {
        self.basis_family
    }

    pub fn effect_family(&self) -> EffectFamily {
        self.effect_family
    }

    pub fn posture(&self) -> EffectSupportPosture {
        self.posture
    }

    pub fn row_digest(&self) -> &str {
        &self.row_digest
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EffectLifecycleSupportMatrix {
    rows: Vec<EffectLifecycleSupportRow>,
    matrix_digest: String,
}

impl EffectLifecycleSupportMatrix {
    pub fn rows(&self) -> &[EffectLifecycleSupportRow] {
        &self.rows
    }

    pub fn matrix_digest(&self) -> &str {
        &self.matrix_digest
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EffectLifecycleSupportDiscovery {
    requested_basis_family: BasisFamily,
    requested_effect_family: EffectFamily,
    posture: EffectSupportPosture,
    matched_row_digest: Option<String>,
    support_matrix_digest: String,
    discovery_digest: String,
    counters: EffectLifecycleCounters,
}

impl EffectLifecycleSupportDiscovery {
    fn new(
        requested_basis_family: BasisFamily,
        requested_effect_family: EffectFamily,
        decision: EffectSupportDecision,
        support_matrix_digest: String,
    ) -> Self {
        let matched_row_digest = decision
            .matched_row
            .as_ref()
            .map(|row| row.row_digest().to_string());
        let counters = EffectLifecycleCounters::support_lookup(decision.rows_consulted);
        let discovery_digest = hash_parts(&[
            "effect_lifecycle_support_discovery_v1".to_string(),
            format!("basis_family:{}", requested_basis_family.as_str()),
            format!("effect_family:{}", requested_effect_family.as_str()),
            format!("posture:{}", decision.posture.as_str()),
            format!(
                "matched_row:{}",
                matched_row_digest.as_deref().unwrap_or("unsupported")
            ),
            format!("matrix:{support_matrix_digest}"),
            format!("counters:{}", counters.digest()),
        ]);
        Self {
            requested_basis_family,
            requested_effect_family,
            posture: decision.posture,
            matched_row_digest,
            support_matrix_digest,
            discovery_digest,
            counters,
        }
    }

    pub fn requested_basis_family(&self) -> BasisFamily {
        self.requested_basis_family
    }

    pub fn requested_effect_family(&self) -> EffectFamily {
        self.requested_effect_family
    }

    pub fn posture(&self) -> EffectSupportPosture {
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

    pub fn counters(&self) -> &EffectLifecycleCounters {
        &self.counters
    }
}

pub fn effect_lifecycle_support_matrix() -> EffectLifecycleSupportMatrix {
    let rows = support_rows()
        .iter()
        .map(|(basis_family, effect_family, posture)| {
            EffectLifecycleSupportRow::new(*basis_family, *effect_family, *posture)
        })
        .collect::<Vec<_>>();
    let matrix_digest = hash_parts(
        &rows
            .iter()
            .map(|row| row.row_digest().to_string())
            .collect::<Vec<_>>(),
    );
    EffectLifecycleSupportMatrix {
        rows,
        matrix_digest,
    }
}

pub fn discover_effect_lifecycle_support(
    basis_family: BasisFamily,
    effect_family: EffectFamily,
) -> EffectLifecycleSupportDiscovery {
    let support_matrix = effect_lifecycle_support_matrix();
    EffectLifecycleSupportDiscovery::new(
        basis_family,
        effect_family,
        support_decision_for(basis_family, effect_family),
        support_matrix.matrix_digest().to_string(),
    )
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct EffectSupportDecision {
    posture: EffectSupportPosture,
    matched_row: Option<EffectLifecycleSupportRow>,
    rows_consulted: usize,
}

impl EffectSupportDecision {
    pub(crate) fn posture(&self) -> EffectSupportPosture {
        self.posture
    }

    pub(crate) fn rows_consulted(&self) -> usize {
        self.rows_consulted
    }
}

pub(crate) fn support_decision_for(
    basis_family: BasisFamily,
    effect_family: EffectFamily,
) -> EffectSupportDecision {
    let mut rows_consulted = 0;
    for (row_basis_family, row_effect_family, posture) in support_rows() {
        rows_consulted += 1;
        if *row_basis_family == basis_family && *row_effect_family == effect_family {
            return EffectSupportDecision {
                posture: *posture,
                matched_row: Some(EffectLifecycleSupportRow::new(
                    *row_basis_family,
                    *row_effect_family,
                    *posture,
                )),
                rows_consulted,
            };
        }
    }

    EffectSupportDecision {
        posture: EffectSupportPosture::Unsupported,
        matched_row: None,
        rows_consulted,
    }
}

fn support_rows() -> &'static [(BasisFamily, EffectFamily, EffectSupportPosture)] {
    use BasisFamily::*;
    use EffectFamily::*;
    use EffectSupportPosture::*;
    &[
        (CurrentHead, Mutation, Admitted),
        (CurrentHead, Merge, Admitted),
        (CurrentHead, Writeback, Admitted),
        (BranchHead, Mutation, Admitted),
        (BranchHead, Merge, Admitted),
        (BranchHead, Writeback, Admitted),
        (TenantScoped, Mutation, Admitted),
        (TenantScoped, Merge, Denied),
        (TenantScoped, Writeback, Admitted),
        (PolicyScoped, Mutation, Admitted),
        (PolicyScoped, Merge, Denied),
        (PolicyScoped, Writeback, Admitted),
        (Preview, Writeback, RebindRequired),
        (PreviewDerived, Writeback, RebindRequired),
        (StoreBacked, Writeback, Deferred),
        (DurableReload, Writeback, Deferred),
    ]
}
