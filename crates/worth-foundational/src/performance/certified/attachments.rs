use worth_proof::{Artifact, Proof, TransitionOutcome};

use crate::canonicalization::{
    CanonicalBasisConstructionDenial, CanonicalBasisReadyArtifact, CanonicalizationRuleVersion,
};
use crate::performance::basis::{
    prepare_counter_backed_performance_receipt_for_canonical_basis,
    prepare_materialized_performance_report_for_canonical_basis,
};
use crate::performance::claims::FoundationalPerformanceClaimSurface;
use crate::performance::receipts::FoundationalCounterBackedPerformanceReceipt;
use crate::performance::reports::FoundationalMaterializedPerformanceReport;
use crate::performance::{
    performance_basis_rule_version, FoundationalPerformanceEvidenceStrength,
    FoundationalPerformanceExecutionTemperature, FoundationalPerformanceFallbackDebtPosture,
    FoundationalPerformanceFreshnessRetentionPosture,
    FoundationalPerformanceReportMaterializationBoundary, FoundationalPerformanceWorkClass,
};

use super::authority::FoundationalPerformanceCertifiedAttachmentAuthority;
use super::surfaces::{
    FoundationalCertifiedPerformanceBundle, FoundationalCertifiedPerformancePayload,
    FoundationalCertifiedPerformanceSourceDigest,
};
use super::vocabulary::{
    FoundationalCertifiedPerformanceAttachmentDenial, FoundationalCertifiedPerformanceClass,
    FoundationalCertifiedPerformanceSourceKind,
};

mod sealed {
    pub trait Sealed {}
}

pub trait FoundationalCertifiedPerformanceSource: sealed::Sealed + Sized {
    fn source_kind(&self) -> FoundationalCertifiedPerformanceSourceKind;
    fn certified_class(&self) -> FoundationalCertifiedPerformanceClass;
    fn prepare_source_basis(
        &self,
        version: CanonicalizationRuleVersion,
    ) -> TransitionOutcome<CanonicalBasisReadyArtifact, CanonicalBasisConstructionDenial>;
}

impl<Claim> sealed::Sealed for FoundationalCounterBackedPerformanceReceipt<Claim> where
    Claim: FoundationalPerformanceClaimSurface
{
}

impl<Claim> FoundationalCertifiedPerformanceSource
    for FoundationalCounterBackedPerformanceReceipt<Claim>
where
    Claim: FoundationalPerformanceClaimSurface,
{
    fn source_kind(&self) -> FoundationalCertifiedPerformanceSourceKind {
        FoundationalCertifiedPerformanceSourceKind::CurrentBasisCounterBackedExecutionReceipt
    }

    fn certified_class(&self) -> FoundationalCertifiedPerformanceClass {
        FoundationalCertifiedPerformanceClass::HotPathOperational
    }

    fn prepare_source_basis(
        &self,
        version: CanonicalizationRuleVersion,
    ) -> TransitionOutcome<CanonicalBasisReadyArtifact, CanonicalBasisConstructionDenial> {
        prepare_counter_backed_performance_receipt_for_canonical_basis(version, self)
    }
}

impl<Source> sealed::Sealed for FoundationalMaterializedPerformanceReport<Source> {}

impl<Source> FoundationalCertifiedPerformanceSource
    for FoundationalMaterializedPerformanceReport<Source>
{
    fn source_kind(&self) -> FoundationalCertifiedPerformanceSourceKind {
        FoundationalCertifiedPerformanceSourceKind::MaterializedSupportExpansionReport
    }

    fn certified_class(&self) -> FoundationalCertifiedPerformanceClass {
        FoundationalCertifiedPerformanceClass::SupportExpansionCompatibility
    }

    fn prepare_source_basis(
        &self,
        version: CanonicalizationRuleVersion,
    ) -> TransitionOutcome<CanonicalBasisReadyArtifact, CanonicalBasisConstructionDenial> {
        prepare_materialized_performance_report_for_canonical_basis(version, self)
    }
}

pub type FoundationalCertifiedPerformanceAttachmentOutcome<Source> = TransitionOutcome<
    FoundationalCertifiedPerformanceBundle<Source>,
    FoundationalCertifiedPerformanceAttachmentDenial,
>;

pub fn certify_hot_path_counter_backed_performance_receipt<Claim>(
    receipt: FoundationalCounterBackedPerformanceReceipt<Claim>,
    authority: worth_proof::AuthorityWitness<FoundationalPerformanceCertifiedAttachmentAuthority>,
) -> FoundationalCertifiedPerformanceAttachmentOutcome<
    FoundationalCounterBackedPerformanceReceipt<Claim>,
>
where
    Claim: FoundationalPerformanceClaimSurface,
{
    let claim = receipt.bundle().claim();
    if claim.evidence_strength()
        != FoundationalPerformanceEvidenceStrength::CounterBackedExecutionReceipt
    {
        return TransitionOutcome::denied(
            FoundationalCertifiedPerformanceAttachmentDenial::HotPathCertificationRequiresCounterBackedExecution,
        );
    }
    if claim.execution_temperature() != FoundationalPerformanceExecutionTemperature::HotPath
        || claim.freshness_retention()
            != FoundationalPerformanceFreshnessRetentionPosture::ExactBasisCurrent
        || claim.fallback_debt() != FoundationalPerformanceFallbackDebtPosture::Verified
    {
        return TransitionOutcome::denied(
            FoundationalCertifiedPerformanceAttachmentDenial::HotPathCertificationRequiresExactCurrentVerifiedHotPath,
        );
    }

    let has_required_exclusions = [
        FoundationalPerformanceWorkClass::ReplayReconstruction,
        FoundationalPerformanceWorkClass::SupportReportAssembly,
        FoundationalPerformanceWorkClass::ForensicParity,
    ]
    .into_iter()
    .all(|work| claim.excluded_work().contains(&work));
    if !has_required_exclusions {
        return TransitionOutcome::denied(
            FoundationalCertifiedPerformanceAttachmentDenial::HotPathCertificationRequiresExplicitOperationalExclusions,
        );
    }

    certify_performance_source(receipt, authority)
}

pub fn certify_support_expansion_performance_report<Source>(
    report: FoundationalMaterializedPerformanceReport<Source>,
    authority: worth_proof::AuthorityWitness<FoundationalPerformanceCertifiedAttachmentAuthority>,
) -> FoundationalCertifiedPerformanceAttachmentOutcome<
    FoundationalMaterializedPerformanceReport<Source>,
> {
    if report.materialization_boundary()
        != FoundationalPerformanceReportMaterializationBoundary::SupportExpansion
    {
        return TransitionOutcome::denied(
            FoundationalCertifiedPerformanceAttachmentDenial::SupportCertificationRequiresSupportExpansionBoundary,
        );
    }
    if report.supporting_evidence_rows().is_empty() {
        return TransitionOutcome::denied(
            FoundationalCertifiedPerformanceAttachmentDenial::SupportCertificationRequiresSupportRows,
        );
    }

    certify_performance_source(report, authority)
}

fn certify_performance_source<Source>(
    source: Source,
    authority: worth_proof::AuthorityWitness<FoundationalPerformanceCertifiedAttachmentAuthority>,
) -> FoundationalCertifiedPerformanceAttachmentOutcome<Source>
where
    Source: FoundationalCertifiedPerformanceSource,
{
    let source_basis = match source.prepare_source_basis(performance_basis_rule_version()) {
        TransitionOutcome::Success(ready) => ready,
        TransitionOutcome::Denied(_) => {
            unreachable!(
                "performance canonical basis preparation uses only denied for invalid shapes"
            )
        }
        TransitionOutcome::Deferred(_)
        | TransitionOutcome::Stale(_)
        | TransitionOutcome::RebindRequired(_)
        | TransitionOutcome::Failed(_) => {
            unreachable!(
                "performance canonical basis preparation is not deferred or runtime-failing"
            )
        }
    };
    let source_digest = FoundationalCertifiedPerformanceSourceDigest::from_basis(&source_basis);
    let source_kind = source.source_kind();
    let certified_class = source.certified_class();
    let proof = Proof::from_authority_witness(&authority);
    TransitionOutcome::success(FoundationalCertifiedPerformanceBundle::new(
        Artifact::with_proofs_and_current_basis(
            FoundationalCertifiedPerformancePayload::new(
                source,
                source_kind,
                certified_class,
                source_digest.clone(),
            ),
            proof,
            source_basis,
            authority,
        ),
    ))
}
