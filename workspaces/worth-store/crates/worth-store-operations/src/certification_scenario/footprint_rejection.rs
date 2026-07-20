use sha2::{Digest, Sha256};
use worth_store_offline_verifier::{
    BackupStructuralVerificationDenial, BackupVerificationBudget, OfflineInspectionBudget,
    StagedRecoveryPostVerificationDenial,
};

use crate::{
    AuthorizationReplayPolicy, AuthorizationRevocationObservation, BackupRestoreIntent,
    OperationalOperationId, OperationalTransitionId, RecoveryCutoverDenial,
};

use super::{
    certification_operator_assertion, CurrentScenarioStagingPort, ExactScenarioAuthorizationPort,
    OwnerBackedBackupScenario,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ScenarioFootprintMutationRejectionReceipt {
    plan_fingerprint: [u8; 32],
    declared_content_fingerprint: [u8; 32],
    injected_content_digest: [u8; 32],
    evidence_identity: [u8; 32],
}

pub fn certify_scenario_footprint_mutation_rejection(
    case: &str,
) -> ScenarioFootprintMutationRejectionReceipt {
    let scenario = OwnerBackedBackupScenario::materialize(case);
    let control = scenario.control_store();
    let source = scenario.execute_named(case, "footprint-source", &control);
    let target = scenario.workspace_root().join("footprint-target");
    std::fs::create_dir_all(&target).unwrap();
    let executed = BackupRestoreIntent::from_verified_backup(
        OperationalOperationId::new(format!("{case}/footprint-restore")).unwrap(),
        source.into_restore_source(),
        &target,
        scenario.operational_security_scope(),
        u64::MAX,
        64 * 1024,
    )
    .resolve()
    .lower()
    .unwrap()
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
        OperationalTransitionId::new(format!("{case}/footprint-ready")).unwrap(),
        scenario.authority(),
        21,
        AuthorizationRevocationObservation::NotRevoked { observed_at: 21 },
    )
    .unwrap()
    .execute(&CurrentScenarioStagingPort)
    .unwrap();
    let plan_fingerprint = executed.staged_media().plan_fingerprint();
    let declared_content_fingerprint = executed.staged_media().content_fingerprint();
    let injected = b"outside-the-lowered-owner-footprint";
    std::fs::write(
        executed
            .staged_media()
            .root()
            .join("undeclared-owner-write"),
        injected,
    )
    .unwrap();
    let denial = match executed.post_verify(BackupVerificationBudget::from_inspection(
        OfflineInspectionBudget::bounded(64 * 1024, u64::MAX).unwrap(),
    )) {
        Ok(_) => panic!("fresh structural verification accepted footprint escape"),
        Err(denial) => denial,
    };
    assert!(matches!(
        denial,
        RecoveryCutoverDenial::PostVerification(StagedRecoveryPostVerificationDenial::Structural(
            BackupStructuralVerificationDenial::Defects(_)
        ))
    ));
    let injected_content_digest = Sha256::digest(injected).into();
    let mut digest = Sha256::new();
    digest.update(b"worth-store-s10-footprint-mutation-rejection-v1");
    digest.update(plan_fingerprint);
    digest.update(declared_content_fingerprint);
    digest.update(injected_content_digest);
    ScenarioFootprintMutationRejectionReceipt {
        plan_fingerprint,
        declared_content_fingerprint,
        injected_content_digest,
        evidence_identity: digest.finalize().into(),
    }
}

impl ScenarioFootprintMutationRejectionReceipt {
    pub const fn plan_fingerprint(self) -> [u8; 32] {
        self.plan_fingerprint
    }
    pub const fn declared_content_fingerprint(self) -> [u8; 32] {
        self.declared_content_fingerprint
    }
    pub const fn injected_content_digest(self) -> [u8; 32] {
        self.injected_content_digest
    }
    pub const fn evidence_identity(self) -> [u8; 32] {
        self.evidence_identity
    }
}
