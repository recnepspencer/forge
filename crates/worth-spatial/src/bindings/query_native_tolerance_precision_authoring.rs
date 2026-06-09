use forge_query::facade::{
    ForgeQueryAdmittedConfiguredDomainHandle, ForgeQueryDeclarationCanonicalEntry,
    ForgeQueryDeclarationInput, ForgeQueryDomainOperatingContext, ForgeQueryOrdinaryNextStep,
    ForgeQueryOrdinaryOutcome, ForgeQueryOrdinaryPosture, ForgeQueryOrdinaryPostureKind,
};
use worth_geom::facade::{
    PrimitiveConditioningWitness, PrimitiveRealizationExhaustionReport, PrimitiveRealizationReport,
    PrimitiveRealizationStrategy, PrimitiveStabilityClass,
};
use worth_primitives::PrimitiveConstructionFamilyKey;

use crate::bindings::query_native_tolerance_precision::{
    ToleranceAndPrecisionCertificationDeclarationFamily,
    ToleranceAndPrecisionCertificationQueryDomain,
};
use crate::bindings::query_native_tolerance_precision_facts::ToleranceAndPrecisionCertificationPayload;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ToleranceAndPrecisionCertificateKind {
    PrimitiveConstructionBirth,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CertifiedToleranceBound {
    precision_headroom_ratio: f64,
    support_normal_headroom_ratio: f64,
    normalization_scale_applied: f64,
}

impl CertifiedToleranceBound {
    pub fn new(
        precision_headroom_ratio: f64,
        support_normal_headroom_ratio: f64,
        normalization_scale_applied: f64,
    ) -> Self {
        Self {
            precision_headroom_ratio,
            support_normal_headroom_ratio,
            normalization_scale_applied,
        }
    }

    pub fn precision_headroom_ratio(&self) -> f64 {
        self.precision_headroom_ratio
    }

    pub fn support_normal_headroom_ratio(&self) -> f64 {
        self.support_normal_headroom_ratio
    }

    pub fn normalization_scale_applied(&self) -> f64 {
        self.normalization_scale_applied
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ToleranceAndPrecisionToleranceBasis {
    origin: [f64; 3],
    facing_vector: [f64; 3],
}

impl ToleranceAndPrecisionToleranceBasis {
    pub fn new(origin: [f64; 3], facing_vector: [f64; 3]) -> Self {
        Self {
            origin,
            facing_vector,
        }
    }

    pub fn origin(&self) -> [f64; 3] {
        self.origin
    }

    pub fn facing_vector(&self) -> [f64; 3] {
        self.facing_vector
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ToleranceAndPrecisionCertificationPosture {
    CertifiedStable,
    CertifiedAfterEscalation,
    Unsupported,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ToleranceAndPrecisionUnsupportedReason {
    CollapsedFeatureScale,
    DegenerateSupportNormals,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ToleranceAndPrecisionRealizationPosture {
    selected_strategy: PrimitiveRealizationStrategy,
    attempted_strategies: Vec<PrimitiveRealizationStrategy>,
    conditioning_witness: PrimitiveConditioningWitness,
    stability_class: PrimitiveStabilityClass,
}

impl ToleranceAndPrecisionRealizationPosture {
    pub fn from_realization_report(realization_report: PrimitiveRealizationReport) -> Self {
        Self {
            selected_strategy: realization_report.strategy(),
            attempted_strategies: realization_report.attempted_strategies().to_vec(),
            conditioning_witness: realization_report.conditioning_witness().clone(),
            stability_class: realization_report.stability_class(),
        }
    }

    pub fn from_exhaustion_report(exhaustion_report: PrimitiveRealizationExhaustionReport) -> Self {
        let selected_strategy = exhaustion_report
            .attempted_strategies()
            .last()
            .copied()
            .unwrap_or(PrimitiveRealizationStrategy::DirectWorld);
        Self {
            selected_strategy,
            attempted_strategies: exhaustion_report.attempted_strategies().to_vec(),
            conditioning_witness: exhaustion_report.conditioning_witness().clone(),
            stability_class: exhaustion_report.stability_class(),
        }
    }

    pub fn from_direct_planar_support(
        label: &'static str,
        vertex_positions: &[[f64; 3]],
        support_planes: &[worth_geom::facade::Plane],
    ) -> Self {
        Self::from_realization_report(worth_geom::facade::build_direct_realization_report(
            label,
            vertex_positions,
            support_planes,
        ))
    }

    pub fn selected_strategy(&self) -> PrimitiveRealizationStrategy {
        self.selected_strategy
    }

    pub fn attempted_strategies(&self) -> &[PrimitiveRealizationStrategy] {
        &self.attempted_strategies
    }

    pub fn conditioning_witness(&self) -> &PrimitiveConditioningWitness {
        &self.conditioning_witness
    }

    pub fn stability_class(&self) -> PrimitiveStabilityClass {
        self.stability_class
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct ToleranceAndPrecisionCertificationCase {
    family: PrimitiveConstructionFamilyKey,
    certificate_kind: ToleranceAndPrecisionCertificateKind,
    precision_policy_identity: String,
    tolerance_basis: ToleranceAndPrecisionToleranceBasis,
    realization_posture: ToleranceAndPrecisionRealizationPosture,
}

impl ToleranceAndPrecisionCertificationCase {
    pub fn primitive_construction_birth(
        family: PrimitiveConstructionFamilyKey,
        precision_policy_identity: impl Into<String>,
        tolerance_basis: ToleranceAndPrecisionToleranceBasis,
        realization_posture: ToleranceAndPrecisionRealizationPosture,
    ) -> Self {
        Self {
            family,
            certificate_kind: ToleranceAndPrecisionCertificateKind::PrimitiveConstructionBirth,
            precision_policy_identity: precision_policy_identity.into(),
            tolerance_basis,
            realization_posture,
        }
    }

    pub(crate) fn family(&self) -> PrimitiveConstructionFamilyKey {
        self.family
    }

    pub(crate) fn certificate_kind(&self) -> ToleranceAndPrecisionCertificateKind {
        self.certificate_kind
    }

    pub(crate) fn precision_policy_identity(&self) -> &str {
        &self.precision_policy_identity
    }

    pub(crate) fn tolerance_basis(&self) -> ToleranceAndPrecisionToleranceBasis {
        self.tolerance_basis
    }

    pub(crate) fn realization_posture(&self) -> &ToleranceAndPrecisionRealizationPosture {
        &self.realization_posture
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct PrimitiveConstructionToleranceAndPrecisionCertificationEntry {
    case: ToleranceAndPrecisionCertificationCase,
    payload: ToleranceAndPrecisionCertificationPayload,
}

impl PrimitiveConstructionToleranceAndPrecisionCertificationEntry {
    pub fn case(&self) -> &ToleranceAndPrecisionCertificationCase {
        &self.case
    }

    pub(crate) fn payload(&self) -> &ToleranceAndPrecisionCertificationPayload {
        &self.payload
    }
}

impl ForgeQueryDeclarationInput<ToleranceAndPrecisionCertificationQueryDomain>
    for PrimitiveConstructionToleranceAndPrecisionCertificationEntry
{
    type Family = ToleranceAndPrecisionCertificationDeclarationFamily;

    fn canonical_declaration_entries(&self) -> Vec<ForgeQueryDeclarationCanonicalEntry> {
        let witness = self.case.realization_posture.conditioning_witness();
        vec![
            ForgeQueryDeclarationCanonicalEntry::text(
                "geometry.tolerance.family",
                self.case.family.as_str(),
            ),
            ForgeQueryDeclarationCanonicalEntry::text(
                "geometry.tolerance.certificate_kind",
                "primitive_construction_birth",
            ),
            ForgeQueryDeclarationCanonicalEntry::text(
                "geometry.tolerance.policy_identity",
                self.case.precision_policy_identity.clone(),
            ),
            ForgeQueryDeclarationCanonicalEntry::text(
                "geometry.tolerance.origin",
                format!("{:?}", self.case.tolerance_basis.origin()),
            ),
            ForgeQueryDeclarationCanonicalEntry::text(
                "geometry.tolerance.facing_vector",
                format!("{:?}", self.case.tolerance_basis.facing_vector()),
            ),
            ForgeQueryDeclarationCanonicalEntry::text(
                "geometry.tolerance.selected_strategy",
                self.case.realization_posture.selected_strategy().as_str(),
            ),
            ForgeQueryDeclarationCanonicalEntry::text(
                "geometry.tolerance.stability_class",
                self.case.realization_posture.stability_class().as_str(),
            ),
            ForgeQueryDeclarationCanonicalEntry::text(
                "geometry.tolerance.precision_headroom_ratio",
                witness.precision_headroom_ratio().to_bits().to_string(),
            ),
            ForgeQueryDeclarationCanonicalEntry::text(
                "geometry.tolerance.support_normal_headroom_ratio",
                witness
                    .support_normal_headroom_ratio()
                    .to_bits()
                    .to_string(),
            ),
            ForgeQueryDeclarationCanonicalEntry::text(
                "geometry.tolerance.normalization_disposition",
                witness.normalization_disposition().as_str(),
            ),
        ]
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct ToleranceAndPrecisionCertificationFactReceipt {
    certificate_kind: ToleranceAndPrecisionCertificateKind,
    precision_policy_identity: String,
    tolerance_basis: ToleranceAndPrecisionToleranceBasis,
    certified_bound: CertifiedToleranceBound,
    certification_posture: ToleranceAndPrecisionCertificationPosture,
    fact_digest: String,
    escalation_trace: Vec<String>,
    unsupported_reason: Option<ToleranceAndPrecisionUnsupportedReason>,
}

impl ToleranceAndPrecisionCertificationFactReceipt {
    pub(crate) fn new(
        certificate_kind: ToleranceAndPrecisionCertificateKind,
        precision_policy_identity: String,
        tolerance_basis: ToleranceAndPrecisionToleranceBasis,
        certified_bound: CertifiedToleranceBound,
        certification_posture: ToleranceAndPrecisionCertificationPosture,
        fact_digest: String,
        escalation_trace: Vec<String>,
        unsupported_reason: Option<ToleranceAndPrecisionUnsupportedReason>,
    ) -> Self {
        Self {
            certificate_kind,
            precision_policy_identity,
            tolerance_basis,
            certified_bound,
            certification_posture,
            fact_digest,
            escalation_trace,
            unsupported_reason,
        }
    }

    pub fn certificate_kind(&self) -> ToleranceAndPrecisionCertificateKind {
        self.certificate_kind
    }

    pub fn precision_policy_identity(&self) -> &str {
        &self.precision_policy_identity
    }

    pub fn tolerance_basis(&self) -> ToleranceAndPrecisionToleranceBasis {
        self.tolerance_basis
    }

    pub fn certified_bound(&self) -> CertifiedToleranceBound {
        self.certified_bound
    }

    pub fn certification_posture(&self) -> ToleranceAndPrecisionCertificationPosture {
        self.certification_posture
    }

    pub fn fact_digest(&self) -> &str {
        &self.fact_digest
    }

    pub fn escalation_trace(&self) -> &[String] {
        &self.escalation_trace
    }

    pub fn unsupported_reason(&self) -> Option<ToleranceAndPrecisionUnsupportedReason> {
        self.unsupported_reason
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum ToleranceAndPrecisionCertificationFactError {
    OutcomeNotBound {
        kind: ForgeQueryOrdinaryPostureKind,
        reason: String,
        next_step: ForgeQueryOrdinaryNextStep,
    },
}

impl ToleranceAndPrecisionCertificationFactError {
    fn outcome_not_bound(posture: &ForgeQueryOrdinaryPosture) -> Self {
        Self::OutcomeNotBound {
            kind: posture.kind(),
            reason: posture.reason().to_string(),
            next_step: posture.next_step(),
        }
    }
}

pub fn primitive_construction_tolerance_and_precision_certification_entry(
    case: ToleranceAndPrecisionCertificationCase,
) -> PrimitiveConstructionToleranceAndPrecisionCertificationEntry {
    let payload = ToleranceAndPrecisionCertificationPayload::from_case(&case);
    PrimitiveConstructionToleranceAndPrecisionCertificationEntry { case, payload }
}

pub fn primitive_construction_tolerance_and_precision_certification_facts<C>(
    entry: &PrimitiveConstructionToleranceAndPrecisionCertificationEntry,
    handle: &ForgeQueryAdmittedConfiguredDomainHandle<
        ToleranceAndPrecisionCertificationQueryDomain,
        C,
    >,
) -> Result<
    ToleranceAndPrecisionCertificationFactReceipt,
    ToleranceAndPrecisionCertificationFactError,
>
where
    C: ForgeQueryDomainOperatingContext<ToleranceAndPrecisionCertificationQueryDomain>,
{
    match handle.orchestrate_declaration_entry_outcome(entry.clone()) {
        ForgeQueryOrdinaryOutcome::Bound(envelope) => {
            Ok(entry.payload().from_bound_envelope(&envelope))
        }
        ForgeQueryOrdinaryOutcome::Ambiguous(posture)
        | ForgeQueryOrdinaryOutcome::AspectConflict(posture)
        | ForgeQueryOrdinaryOutcome::AuthorityMismatch(posture)
        | ForgeQueryOrdinaryOutcome::BasisMismatch(posture)
        | ForgeQueryOrdinaryOutcome::Deferred(posture)
        | ForgeQueryOrdinaryOutcome::Denied(posture)
        | ForgeQueryOrdinaryOutcome::ExplicitNarrowingRequired(posture)
        | ForgeQueryOrdinaryOutcome::Failed(posture)
        | ForgeQueryOrdinaryOutcome::MissingRequiredAspect(posture)
        | ForgeQueryOrdinaryOutcome::RebindRequired(posture)
        | ForgeQueryOrdinaryOutcome::Refused(posture)
        | ForgeQueryOrdinaryOutcome::Stale(posture)
        | ForgeQueryOrdinaryOutcome::Unavailable(posture)
        | ForgeQueryOrdinaryOutcome::Unsupported(posture)
        | ForgeQueryOrdinaryOutcome::WrongHandle(posture)
        | ForgeQueryOrdinaryOutcome::WrongWorld(posture) => {
            Err(ToleranceAndPrecisionCertificationFactError::outcome_not_bound(&posture))
        }
    }
}
