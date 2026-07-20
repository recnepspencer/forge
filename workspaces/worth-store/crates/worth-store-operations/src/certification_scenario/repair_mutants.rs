use sha2::{Digest, Sha256};
use worth_store_authority::ControlStoreFencingAuthority;
use worth_store_offline_verifier::{
    BackupStructuralVerificationDenial, BackupVerificationBudget, OfflineInspectionBudget,
    StagedRecoveryPostVerificationDenial,
};

use crate::{
    AuthorityAffectingRepairExecutionDenial, AuthorizationReplayPolicy,
    AuthorizationRevocationObservation, ControlStoreSelectionIndeterminate,
    ControlStoreTrustPosture, OperationalControlHistoryViolationKind, OperationalControlRecord,
    OperationalControlStorePort, OperationalTransitionId, RecoveryCutoverDenial,
    RepairExecutionBoundaryMoment, StoreOwnerKind,
};

use super::{
    certification_operator_assertion, repair_owner_recovery, CurrentScenarioStagingPort,
    ExactScenarioAuthorizationPort, ExactScenarioControlSelection,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ScenarioRepairMutantRejectionReceipt {
    footprint_rejection_identity: [u8; 32],
    omitted_receipt_rejection_identity: [u8; 32],
    evidence_identity: [u8; 32],
}

pub fn certify_scenario_repair_mutant_rejections(
    case: &str,
) -> ScenarioRepairMutantRejectionReceipt {
    let footprint_rejection_identity = reject_out_of_footprint_write(case);
    let omitted_receipt_rejection_identity = reject_omitted_owner_receipt(case);
    let mut digest = Sha256::new();
    digest.update(b"worth-store-s10-repair-mutant-rejections-v1");
    digest.update(footprint_rejection_identity);
    digest.update(omitted_receipt_rejection_identity);
    ScenarioRepairMutantRejectionReceipt {
        footprint_rejection_identity,
        omitted_receipt_rejection_identity,
        evidence_identity: digest.finalize().into(),
    }
}

fn reject_out_of_footprint_write(case: &str) -> [u8; 32] {
    let name = format!("{case}/footprint");
    let repair_owner_recovery::RepairWorld {
        scenario,
        control,
        lowered,
    } = repair_owner_recovery::repair_world(&name);
    let executed = lowered
        .authorize(
            &ExactScenarioAuthorizationPort,
            &certification_operator_assertion(),
            20,
            80,
            AuthorizationReplayPolicy::SingleUse,
            AuthorizationRevocationObservation::NotRevoked { observed_at: 20 },
        )
        .unwrap()
        .ready(
            &control,
            OperationalTransitionId::new(format!("{name}/ready")).unwrap(),
            scenario.authority(),
            21,
            AuthorizationRevocationObservation::NotRevoked { observed_at: 21 },
        )
        .unwrap()
        .execute(&CurrentScenarioStagingPort)
        .unwrap();
    let plan = executed.staged_media().plan_fingerprint();
    let content = executed.staged_media().content_fingerprint();
    let mutant = b"undeclared-repair-owner-page";
    std::fs::write(executed.staged_media().root().join("mutant.page"), mutant).unwrap();
    let denial = match executed.post_verify(verification_budget()) {
        Ok(_) => panic!("post-verification accepted an undeclared repair-owner write"),
        Err(denial) => denial,
    };
    assert!(matches!(
        denial,
        RecoveryCutoverDenial::PostVerification(StagedRecoveryPostVerificationDenial::Structural(
            BackupStructuralVerificationDenial::Defects(_)
        ))
    ));
    let mut digest = Sha256::new();
    digest.update(plan);
    digest.update(content);
    digest.update(Sha256::digest(mutant));
    digest.finalize().into()
}

fn reject_omitted_owner_receipt(case: &str) -> [u8; 32] {
    let name = format!("{case}/omitted-receipt");
    let repair_owner_recovery::RepairWorld {
        scenario,
        control,
        lowered,
    } = repair_owner_recovery::repair_world(&name);
    let backend = lowered
        .explanation()
        .nodes()
        .iter()
        .find(|node| node.owner() == StoreOwnerKind::PhysicalBackend)
        .unwrap()
        .identity();
    let denial = lowered
        .authorize(
            &ExactScenarioAuthorizationPort,
            &certification_operator_assertion(),
            20,
            80,
            AuthorizationReplayPolicy::SingleUse,
            AuthorizationRevocationObservation::NotRevoked { observed_at: 20 },
        )
        .unwrap()
        .ready(
            &control,
            OperationalTransitionId::new(format!("{name}/ready")).unwrap(),
            scenario.authority(),
            21,
            AuthorizationRevocationObservation::NotRevoked { observed_at: 21 },
        )
        .unwrap()
        .execute_with_control(
            &CurrentScenarioStagingPort,
            &super::repair_owner_recovery::InterruptAt::once(
                backend,
                RepairExecutionBoundaryMoment::AfterOwnerEffectBeforeReceipt,
            ),
        )
        .unwrap_err();
    assert!(matches!(
        denial,
        AuthorityAffectingRepairExecutionDenial::Interrupted(_)
    ));
    let handle = repair_owner_recovery::repair_handles(&scenario, &control)
        .into_iter()
        .next()
        .unwrap();
    let operation = handle.operation_id().clone();
    let plan = handle.plan_fingerprint();
    assert!(handle
        .started_owner_nodes()
        .iter()
        .any(|started| started.node_fingerprint() == backend.fingerprint()));
    assert!(!handle
        .durable_owner_receipts()
        .iter()
        .any(|receipt| receipt.node_fingerprint() == backend.fingerprint()));
    OperationalControlStorePort::append(
        &control,
        &OperationalControlRecord::repair_disposition_recorded(
            scenario.authority().authority_identity(),
            operation,
            OperationalTransitionId::new(format!("{name}/forged-completion")).unwrap(),
            plan,
            1,
            [0xb1; 32],
        ),
    )
    .unwrap();
    let selection = ExactScenarioControlSelection::current(scenario.authority(), &control);
    let fencing = ControlStoreFencingAuthority::for_current_store(scenario.authority(), &selection);
    let ControlStoreTrustPosture::Indeterminate(
        ControlStoreSelectionIndeterminate::InvalidHistory(violation),
    ) = control.inspect_generations(&fencing)
    else {
        panic!("completion with an omitted owner receipt must poison control selection");
    };
    assert_eq!(
        violation.kind(),
        &OperationalControlHistoryViolationKind::RepairCompletedBeforeAllOwnerReceipts
    );
    let mut digest = Sha256::new();
    digest.update(plan);
    digest.update(backend.fingerprint());
    digest.update(violation.record_index().to_be_bytes());
    digest.finalize().into()
}

fn verification_budget() -> BackupVerificationBudget {
    BackupVerificationBudget::from_inspection(
        OfflineInspectionBudget::bounded(128 * 1024, u64::MAX).unwrap(),
    )
}

impl ScenarioRepairMutantRejectionReceipt {
    pub const fn footprint_rejection_identity(self) -> [u8; 32] {
        self.footprint_rejection_identity
    }
    pub const fn omitted_receipt_rejection_identity(self) -> [u8; 32] {
        self.omitted_receipt_rejection_identity
    }
    pub const fn evidence_identity(self) -> [u8; 32] {
        self.evidence_identity
    }
}
