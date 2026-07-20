use sha2::{Digest, Sha256};
use worth_store_physical_integrity::IntegrityRepairClassificationReceipt;

use super::{S10HostileProgramDenial, S10HostileProgramEvidence, S10HostileProgramRequirement};
use crate::courtroom::operational_recovery::S10OperationalScenarioKind;

impl S10HostileProgramEvidence {
    pub fn authority_repair(
        classification: IntegrityRepairClassificationReceipt,
        restarting_scan: worth_store_offline_verifier::RestartingOfflineScanReceipt,
    ) -> Result<Self, S10HostileProgramDenial> {
        let classified_regions = classification.classified_regions();
        if classified_regions < 100 {
            return Err(S10HostileProgramDenial::RepairBreadthBelowHundreds);
        }
        let mut source = Sha256::new();
        source.update(b"worth-store-s10-authority-repair-hostile-observation-v1");
        source.update(classification.plan_fingerprint());
        source.update(classified_regions.to_be_bytes());
        source.update(classification.quarantined_regions().to_be_bytes());
        source.update(restarting_scan.receipt_identity());
        Ok(Self::bind(
            S10OperationalScenarioKind::AuthorityRepairRollback,
            source.finalize().into(),
            classified_regions,
            S10HostileProgramRequirement::RepairBreadth.mask()
                | S10HostileProgramRequirement::BoundedRestartingOfflineScan.mask(),
            [0; 32],
        ))
    }

    #[allow(clippy::too_many_arguments)]
    pub fn authority_repair_complete(
        classification: IntegrityRepairClassificationReceipt,
        restarting_scan: worth_store_offline_verifier::RestartingOfflineScanReceipt,
        source_denials: worth_store_operations::certification_scenario::ScenarioRepairSourceDenialReceipt,
        canonical_dag: worth_store_operations::certification_scenario::ScenarioCanonicalOwnerDagPermutationReceipt,
        owner_recovery: worth_store_operations::certification_scenario::ScenarioRepairOwnerRecoveryReceipt,
        cancellation_recovery: worth_store_operations::certification_scenario::ScenarioRepairCancellationRecoveryReceipt,
        mutants: worth_store_operations::certification_scenario::ScenarioRepairMutantRejectionReceipt,
        rollback: &worth_store_operations::CompletedRetainedAuthorityRollback,
    ) -> Result<Self, S10HostileProgramDenial> {
        let classified_regions = classification.classified_regions();
        validate_repair_program(
            classified_regions,
            source_denials,
            canonical_dag,
            owner_recovery,
            cancellation_recovery,
            mutants,
            rollback,
        )?;
        let mut source = Sha256::new();
        source.update(b"worth-store-s10-authority-repair-hostile-observation-v2");
        source.update(classification.plan_fingerprint());
        source.update(classified_regions.to_be_bytes());
        source.update(restarting_scan.receipt_identity());
        source.update(source_denials.evidence_identity());
        source.update(canonical_dag.evidence_identity());
        source.update(owner_recovery.evidence_identity());
        source.update(cancellation_recovery.evidence_identity());
        source.update(mutants.evidence_identity());
        source.update(rollback.evidence_identity());
        Ok(Self::bind(
            S10OperationalScenarioKind::AuthorityRepairRollback,
            source.finalize().into(),
            classified_regions,
            complete_repair_requirements(),
            [0; 32],
        ))
    }
}

#[allow(clippy::too_many_arguments)]
fn validate_repair_program(
    classified_regions: u64,
    source_denials: worth_store_operations::certification_scenario::ScenarioRepairSourceDenialReceipt,
    canonical_dag: worth_store_operations::certification_scenario::ScenarioCanonicalOwnerDagPermutationReceipt,
    owner_recovery: worth_store_operations::certification_scenario::ScenarioRepairOwnerRecoveryReceipt,
    cancellation: worth_store_operations::certification_scenario::ScenarioRepairCancellationRecoveryReceipt,
    mutants: worth_store_operations::certification_scenario::ScenarioRepairMutantRejectionReceipt,
    rollback: &worth_store_operations::CompletedRetainedAuthorityRollback,
) -> Result<(), S10HostileProgramDenial> {
    if classified_regions < 100 {
        return Err(S10HostileProgramDenial::RepairBreadthBelowHundreds);
    }
    if source_denials.stale_authority_denial_identity() == [0; 32]
        || source_denials.cross_scope_denial_identity() == [0; 32]
    {
        return Err(S10HostileProgramDenial::RepairSourceProgramIncomplete);
    }
    if canonical_dag.node_count() < 5 || canonical_dag.edge_count() < 4 {
        return Err(S10HostileProgramDenial::OwnerDagProgramIncomplete);
    }
    if owner_recovery.owner_nodes() < 5
        || owner_recovery.recovered_cuts() < owner_recovery.owner_nodes() * 3
        || cancellation.scheduler_cancellation_identity() == [0; 32]
        || cancellation.revocation_cancellation_identity() == [0; 32]
        || cancellation.backend_resume_identity() == [0; 32]
    {
        return Err(S10HostileProgramDenial::RepairRecoveryProgramIncomplete);
    }
    if mutants.footprint_rejection_identity() == [0; 32]
        || mutants.omitted_receipt_rejection_identity() == [0; 32]
    {
        return Err(S10HostileProgramDenial::RepairMutantProgramIncomplete);
    }
    if rollback.publication().publication_identity()
        != rollback.readmission().publication_identity()
        || rollback.fence_release().disposition()
            != worth_store_authority::RecoveryWriteFenceDisposition::Readmitted
        || rollback.source_lease_release().lease_identity() == [0; 32]
        || rollback.source_lease_release().source_identity() == [0; 32]
    {
        return Err(S10HostileProgramDenial::RetainedRollbackProgramIncomplete);
    }
    Ok(())
}

const fn complete_repair_requirements() -> u32 {
    S10HostileProgramRequirement::RepairBreadth.mask()
        | S10HostileProgramRequirement::BoundedRestartingOfflineScan.mask()
        | S10HostileProgramRequirement::RepairSourceAuthorityDenials.mask()
        | S10HostileProgramRequirement::CanonicalOwnerDagPermutation.mask()
        | S10HostileProgramRequirement::CrashEveryOwnerEffect.mask()
        | S10HostileProgramRequirement::RevocationCancellationRecovery.mask()
        | S10HostileProgramRequirement::FootprintAndReceiptMutants.mask()
        | S10HostileProgramRequirement::RetainedAuthorityRollback.mask()
}
