use super::PlanarM7ReadinessBasis;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum PlanarM7ReadinessFamily {
    BooleanReadinessBundle,
    PredicateAuthority,
    StructuralIdentity,
    MotionPosture,
    TopologyCompleteness,
    Precision,
    Transform,
    RetainedPlanarFacts,
    ProjectionConsumedFacts,
    RecoveryPosture,
    Diagnostics,
    CleanFailBoundary,
    SupportPosture,
}

impl PlanarM7ReadinessFamily {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::BooleanReadinessBundle => "boolean-readiness-bundle",
            Self::PredicateAuthority => "predicate-authority",
            Self::StructuralIdentity => "structural-identity",
            Self::MotionPosture => "motion-posture",
            Self::TopologyCompleteness => "topology-completeness",
            Self::Precision => "precision",
            Self::Transform => "transform",
            Self::RetainedPlanarFacts => "retained-planar-facts",
            Self::ProjectionConsumedFacts => "projection-consumed-facts",
            Self::RecoveryPosture => "recovery-posture",
            Self::Diagnostics => "diagnostics",
            Self::CleanFailBoundary => "clean-fail-boundary",
            Self::SupportPosture => "support-posture",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarM7ReadinessFamilyRow {
    family: PlanarM7ReadinessFamily,
    receipt_digest: String,
    declaration_digest: String,
    envelope_digest: String,
}

impl PlanarM7ReadinessFamilyRow {
    pub(crate) fn new(
        family: PlanarM7ReadinessFamily,
        receipt_digest: impl Into<String>,
        declaration_digest: impl Into<String>,
        envelope_digest: impl Into<String>,
    ) -> Self {
        Self {
            family,
            receipt_digest: receipt_digest.into(),
            declaration_digest: declaration_digest.into(),
            envelope_digest: envelope_digest.into(),
        }
    }

    pub fn family(&self) -> PlanarM7ReadinessFamily {
        self.family
    }

    pub fn receipt_digest(&self) -> &str {
        &self.receipt_digest
    }

    pub fn declaration_digest(&self) -> &str {
        &self.declaration_digest
    }

    pub fn envelope_digest(&self) -> &str {
        &self.envelope_digest
    }
}

pub(crate) fn m7_readiness_family_rows(
    basis: &PlanarM7ReadinessBasis,
) -> Vec<PlanarM7ReadinessFamilyRow> {
    let base = basis.boolean_readiness();
    let bundle_basis = base.basis();
    let mut rows = vec![
        PlanarM7ReadinessFamilyRow::new(
            PlanarM7ReadinessFamily::BooleanReadinessBundle,
            base.fact_digest(),
            base.declaration_digest(),
            base.envelope_digest(),
        ),
        PlanarM7ReadinessFamilyRow::new(
            PlanarM7ReadinessFamily::PredicateAuthority,
            predicate_authority_digest(basis),
            bundle_basis
                .predicate_receipts()
                .first()
                .map(|receipt| receipt.declaration_digest())
                .unwrap_or_default(),
            bundle_basis
                .predicate_receipts()
                .first()
                .map(|receipt| receipt.envelope_digest())
                .unwrap_or_default(),
        ),
        PlanarM7ReadinessFamilyRow::new(
            PlanarM7ReadinessFamily::StructuralIdentity,
            basis.structural_identity().structural_identity_digest(),
            basis.structural_identity().declaration_digest(),
            basis.structural_identity().envelope_digest(),
        ),
        PlanarM7ReadinessFamilyRow::new(
            PlanarM7ReadinessFamily::MotionPosture,
            basis.motion_posture().retained_motion_digest(),
            basis.motion_posture().declaration_digest(),
            basis.motion_posture().envelope_digest(),
        ),
        PlanarM7ReadinessFamilyRow::new(
            PlanarM7ReadinessFamily::TopologyCompleteness,
            bundle_basis.topology_contract_receipt().fact_digest(),
            bundle_basis
                .topology_contract_receipt()
                .declaration_digest(),
            bundle_basis.topology_contract_receipt().envelope_digest(),
        ),
        PlanarM7ReadinessFamilyRow::new(
            PlanarM7ReadinessFamily::Precision,
            bundle_basis.precision_receipt().fact_digest(),
            bundle_basis.precision_receipt().declaration_digest(),
            bundle_basis.precision_receipt().envelope_digest(),
        ),
        PlanarM7ReadinessFamilyRow::new(
            PlanarM7ReadinessFamily::Transform,
            basis
                .structural_identity()
                .canonical_transform_basis_digest(),
            basis.structural_identity().declaration_digest(),
            basis.structural_identity().envelope_digest(),
        ),
        PlanarM7ReadinessFamilyRow::new(
            PlanarM7ReadinessFamily::RetainedPlanarFacts,
            basis.retained_planar_facts().retained_fact_digest(),
            basis.retained_planar_facts().declaration_digest(),
            basis.retained_planar_facts().envelope_digest(),
        ),
        PlanarM7ReadinessFamilyRow::new(
            PlanarM7ReadinessFamily::ProjectionConsumedFacts,
            basis
                .projection_consumed_facts()
                .projection_consumption_digest(),
            basis.projection_consumed_facts().declaration_digest(),
            basis.projection_consumed_facts().envelope_digest(),
        ),
        PlanarM7ReadinessFamilyRow::new(
            PlanarM7ReadinessFamily::RecoveryPosture,
            basis.recovery_posture().recovery_posture_digest(),
            basis.recovery_posture().declaration_digest(),
            basis.recovery_posture().envelope_digest(),
        ),
        PlanarM7ReadinessFamilyRow::new(
            PlanarM7ReadinessFamily::Diagnostics,
            basis.diagnostics().diagnostic_bundle_digest(),
            basis.diagnostics().declaration_digest(),
            basis.diagnostics().envelope_digest(),
        ),
        PlanarM7ReadinessFamilyRow::new(
            PlanarM7ReadinessFamily::SupportPosture,
            basis.support_posture().digest_part(),
            "m7:boolean-execution:support",
            "m7:boolean-execution:support-gated",
        ),
    ];
    if let Some(clean_fail) = basis.clean_fail_boundary() {
        rows.push(PlanarM7ReadinessFamilyRow::new(
            PlanarM7ReadinessFamily::CleanFailBoundary,
            clean_fail.clean_fail_boundary_digest(),
            clean_fail.declaration_digest(),
            clean_fail.envelope_digest(),
        ));
    }
    rows.sort_by_key(|row| row.family());
    rows
}

fn predicate_authority_digest(basis: &PlanarM7ReadinessBasis) -> String {
    let mut digests = basis
        .boolean_readiness()
        .basis()
        .predicate_receipts()
        .iter()
        .map(|receipt| receipt.fact_digest().to_string())
        .collect::<Vec<_>>();
    digests.sort();
    digests.join("|")
}
