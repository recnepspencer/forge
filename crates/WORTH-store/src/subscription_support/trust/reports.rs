use super::classification::SupportTrustClassificationCostSurface;
use super::epochs::{SupportCertificationCorpusVersion, SupportCertificationEpoch};
use super::failure::{SupportTrustFailure, SupportTrustFailureKind, SupportTrustRecoveryPosture};
use super::taxonomy::{
    SupportTrustClass, SupportTrustProvenance, SupportTrustStrength, SupportTrustUseBoundary,
};
use super::witnesses::{
    CertifiedSupportTrustWitness, DegradedSupportTrustWitness, ExactSupportTrustWitness,
    RebuildDerivedSupportTrustWitness, RejectedSupportTrustWitness, SupportTrustOperationalWitness,
};
use crate::subscription_support::SubscriptionSupportOperationalBasis;
use crate::subscription_support::{SubscriptionSupportFamilyId, SubscriptionSupportRole};
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct OperationalSupportTrustReport {
    witness: SupportTrustOperationalWitness,
    trust_class: SupportTrustClass,
    trust_strength: SupportTrustStrength,
    provenance: SupportTrustProvenance,
    use_boundary: SupportTrustUseBoundary,
    cost_surface: SupportTrustClassificationCostSurface,
}

impl OperationalSupportTrustReport {
    #[allow(dead_code)]
    pub(crate) fn from_exact_witness(witness: ExactSupportTrustWitness) -> Self {
        Self::from_exact_witness_with_cost(
            witness,
            SupportTrustClassificationCostSurface::phase1_zero(),
        )
    }

    pub(crate) fn from_exact_witness_with_cost(
        witness: ExactSupportTrustWitness,
        cost_surface: SupportTrustClassificationCostSurface,
    ) -> Self {
        let trust_class = SupportTrustClass::from_strength_provenance(witness.trust());
        Self {
            trust_strength: witness.trust().strength(),
            provenance: witness.trust().provenance(),
            witness: SupportTrustOperationalWitness::Exact(witness),
            trust_class,
            use_boundary: SupportTrustUseBoundary::StoreLocalOperational,
            cost_surface,
        }
    }

    pub(crate) fn from_degraded_witness(
        witness: DegradedSupportTrustWitness,
        provenance: SupportTrustProvenance,
        cost_surface: SupportTrustClassificationCostSurface,
    ) -> Self {
        Self {
            witness: SupportTrustOperationalWitness::Degraded(witness),
            trust_class: SupportTrustClass::DegradedSupportTrusted,
            trust_strength: SupportTrustStrength::Degraded,
            provenance,
            use_boundary: SupportTrustUseBoundary::StoreLocalOperational,
            cost_surface,
        }
    }

    pub(crate) fn from_rebuild_witness(
        witness: RebuildDerivedSupportTrustWitness,
        provenance: SupportTrustProvenance,
        cost_surface: SupportTrustClassificationCostSurface,
    ) -> Self {
        Self {
            witness: SupportTrustOperationalWitness::RebuildDerived(witness),
            trust_class: SupportTrustClass::RebuildDerivedSupport,
            trust_strength: SupportTrustStrength::RebuildOnly,
            provenance,
            use_boundary: SupportTrustUseBoundary::StoreLocalOperational,
            cost_surface,
        }
    }

    pub(crate) fn from_rejected_witness(
        witness: RejectedSupportTrustWitness,
        provenance: SupportTrustProvenance,
        cost_surface: SupportTrustClassificationCostSurface,
    ) -> Self {
        Self {
            witness: SupportTrustOperationalWitness::Rejected(witness),
            trust_class: SupportTrustClass::StaleSupportRejected,
            trust_strength: SupportTrustStrength::Rejected,
            provenance,
            use_boundary: SupportTrustUseBoundary::StoreLocalOperational,
            cost_surface,
        }
    }

    pub fn trust_strength(&self) -> SupportTrustStrength {
        self.trust_strength
    }

    pub fn provenance(&self) -> SupportTrustProvenance {
        self.provenance
    }

    pub fn trust_class(&self) -> SupportTrustClass {
        self.trust_class
    }

    pub fn use_boundary(&self) -> SupportTrustUseBoundary {
        self.use_boundary
    }

    pub fn cost_surface(&self) -> SupportTrustClassificationCostSurface {
        self.cost_surface
    }

    pub fn basis(&self) -> &SubscriptionSupportOperationalBasis {
        self.witness.basis()
    }

    #[allow(dead_code)]
    pub(crate) fn exact_witness(&self) -> Option<&ExactSupportTrustWitness> {
        self.witness.exact()
    }

    pub(crate) fn operational_witness(&self) -> &SupportTrustOperationalWitness {
        &self.witness
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SupportTrustCertificationStamp {
    corpus_version: SupportCertificationCorpusVersion,
    certification_epoch: SupportCertificationEpoch,
    suite_version: String,
    family_id: SubscriptionSupportFamilyId,
    support_role: SubscriptionSupportRole,
    trust_strength: SupportTrustStrength,
    provenance: SupportTrustProvenance,
    row_id: String,
    evidence_bundle_digest: String,
}

impl SupportTrustCertificationStamp {
    pub fn new(
        corpus_version: SupportCertificationCorpusVersion,
        certification_epoch: SupportCertificationEpoch,
        suite_version: impl Into<String>,
        family_id: SubscriptionSupportFamilyId,
        support_role: SubscriptionSupportRole,
        trust_strength: SupportTrustStrength,
        provenance: SupportTrustProvenance,
        row_id: impl Into<String>,
        evidence_bundle_digest: impl Into<String>,
    ) -> Result<Self, SupportTrustFailure> {
        let suite_version = require_non_empty("suite version", suite_version)?;
        let row_id = require_non_empty("row id", row_id)?;
        let evidence_bundle_digest =
            require_non_empty("evidence bundle digest", evidence_bundle_digest)?;
        Ok(Self {
            corpus_version,
            certification_epoch,
            suite_version,
            family_id,
            support_role,
            trust_strength,
            provenance,
            row_id,
            evidence_bundle_digest,
        })
    }

    pub fn corpus_version(&self) -> &SupportCertificationCorpusVersion {
        &self.corpus_version
    }

    pub fn certification_epoch(&self) -> SupportCertificationEpoch {
        self.certification_epoch
    }

    pub fn family_id(&self) -> &SubscriptionSupportFamilyId {
        &self.family_id
    }

    pub fn support_role(&self) -> SubscriptionSupportRole {
        self.support_role
    }

    pub fn trust_strength(&self) -> SupportTrustStrength {
        self.trust_strength
    }

    pub fn provenance(&self) -> SupportTrustProvenance {
        self.provenance
    }

    pub fn suite_version(&self) -> &str {
        &self.suite_version
    }

    pub fn row_id(&self) -> &str {
        &self.row_id
    }

    pub fn evidence_bundle_digest(&self) -> &str {
        &self.evidence_bundle_digest
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct CertifiedSupportTrustReport {
    witness: CertifiedSupportTrustWitness,
    trust_class: SupportTrustClass,
    certification_stamp: SupportTrustCertificationStamp,
}

impl CertifiedSupportTrustReport {
    #[allow(dead_code)]
    pub(crate) fn from_operational_report(
        report: OperationalSupportTrustReport,
        certification_stamp: SupportTrustCertificationStamp,
    ) -> Result<Self, SupportTrustFailure> {
        if report
            .operational_witness()
            .freshness()
            .epoch()
            .certification()
            != Some(certification_stamp.certification_epoch())
        {
            return Err(SupportTrustFailure::new(
                SupportTrustFailureKind::SupportTrustCoverageMissing,
                SupportTrustRecoveryPosture::RerunCertification,
                "certification stamp epoch must match the trust freshness epoch",
            ));
        }
        if report.basis().family_id() != certification_stamp.family_id() {
            return Err(SupportTrustFailure::new(
                SupportTrustFailureKind::SupportTrustFamilyMismatch,
                SupportTrustRecoveryPosture::RerunCertification,
                "certification stamp family must match operational trust report family",
            ));
        }
        if report.basis().support_role() != certification_stamp.support_role() {
            return Err(SupportTrustFailure::new(
                SupportTrustFailureKind::SupportTrustRoleMismatch,
                SupportTrustRecoveryPosture::RerunCertification,
                "certification stamp support role must match operational trust report role",
            ));
        }
        if report.trust_strength() != certification_stamp.trust_strength()
            || report.provenance() != certification_stamp.provenance()
        {
            return Err(SupportTrustFailure::new(
                SupportTrustFailureKind::SupportTrustCoverageMissing,
                SupportTrustRecoveryPosture::RerunCertification,
                "certification stamp trust posture must match operational trust report posture",
            ));
        }
        Ok(Self {
            witness: CertifiedSupportTrustWitness::new(report.operational_witness().clone()),
            trust_class: report.trust_class,
            certification_stamp,
        })
    }

    pub fn trust_class(&self) -> SupportTrustClass {
        self.trust_class
    }

    pub fn trust_strength(&self) -> SupportTrustStrength {
        self.certification_stamp.trust_strength()
    }

    pub fn provenance(&self) -> SupportTrustProvenance {
        self.certification_stamp.provenance()
    }

    pub fn witness(&self) -> &CertifiedSupportTrustWitness {
        &self.witness
    }

    pub fn use_boundary(&self) -> SupportTrustUseBoundary {
        SupportTrustUseBoundary::CertifiedPlatform
    }

    pub fn certification_stamp(&self) -> &SupportTrustCertificationStamp {
        &self.certification_stamp
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct UncertifiedSupportTrustPosture {
    report: OperationalSupportTrustReport,
}

impl UncertifiedSupportTrustPosture {
    #[allow(dead_code)]
    pub(crate) fn new(report: OperationalSupportTrustReport) -> Self {
        Self { report }
    }

    pub fn report(&self) -> &OperationalSupportTrustReport {
        &self.report
    }
}

fn require_non_empty(
    label: &'static str,
    value: impl Into<String>,
) -> Result<String, SupportTrustFailure> {
    let value = value.into();
    if value.trim().is_empty() {
        return Err(SupportTrustFailure::new(
            SupportTrustFailureKind::SupportTrustCoverageMissing,
            SupportTrustRecoveryPosture::RerunCertification,
            format!("support trust certification {label} must be non-empty"),
        ));
    }
    Ok(value)
}
