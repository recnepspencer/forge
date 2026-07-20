use std::cell::Cell;

use sha2::{Digest, Sha256};
use worth_store_authority::ControlStoreFencingAuthority;

use crate::{
    AuthorityAffectingRepairExecutionDenial, AuthorizationReplayPolicy,
    AuthorizationRevocationObservation, LoweredAuthorityAffectingRepairOwnerPlanDag,
    OperationalControlStore, OperationalOperationId, OperationalTransitionId,
    OwnerPlanNodeIdentity, RepairExecutionBoundary, RepairExecutionBoundaryMoment,
    RepairExecutionControlPort, RepairExecutionInterrupted,
};

use super::{
    certification_operator_assertion, CurrentScenarioStagingPort, ExactScenarioAuthorizationPort,
    ExactScenarioControlSelection, OwnerBackedBackupScenario,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ScenarioRepairOwnerRecoveryReceipt {
    owner_nodes: u64,
    recovered_cuts: u64,
    evidence_identity: [u8; 32],
}

pub fn certify_scenario_repair_owner_recovery(case: &str) -> ScenarioRepairOwnerRecoveryReceipt {
    let probe = repair_world(&format!("{case}/probe"));
    let owner_nodes = probe.lowered.explanation().node_count();
    assert!(owner_nodes >= 5);
    drop(probe);
    let moments = [
        RepairExecutionBoundaryMoment::BeforeOwnerEffect,
        RepairExecutionBoundaryMoment::AfterOwnerEffectBeforeReceipt,
        RepairExecutionBoundaryMoment::AfterReceiptPersistence,
    ];
    let mut digest = Sha256::new();
    digest.update(b"worth-store-s10-repair-owner-recovery-v1");
    let mut recovered_cuts = 0_u64;
    for node_index in 0..owner_nodes as usize {
        for (moment_index, moment) in moments.into_iter().enumerate() {
            let identity = format!("{case}/node-{node_index}/moment-{moment_index}");
            digest.update(exercise_cut(&identity, node_index, moment));
            recovered_cuts += 1;
        }
    }
    ScenarioRepairOwnerRecoveryReceipt {
        owner_nodes,
        recovered_cuts,
        evidence_identity: digest.finalize().into(),
    }
}

pub(super) struct RepairWorld {
    pub(super) scenario: OwnerBackedBackupScenario,
    pub(super) control: OperationalControlStore,
    pub(super) lowered: LoweredAuthorityAffectingRepairOwnerPlanDag,
}

pub(super) fn repair_world(case: &str) -> RepairWorld {
    let scenario = OwnerBackedBackupScenario::materialize(case);
    let control = scenario.control_store();
    let source = scenario
        .execute_named(case, "repair-recovery-source", &control)
        .into_restore_source();
    let target = scenario.workspace_root().join("repair-recovery-target");
    std::fs::create_dir_all(&target).unwrap();
    let lowered = crate::workflow::certification_authority_repair_from_backup_observation(
        OperationalOperationId::new(format!("{case}/repair")).unwrap(),
        source,
        &target,
    )
    .unwrap()
    .lower_owners()
    .unwrap();
    RepairWorld {
        scenario,
        control,
        lowered,
    }
}

fn exercise_cut(case: &str, node_index: usize, moment: RepairExecutionBoundaryMoment) -> [u8; 32] {
    let RepairWorld {
        scenario,
        control,
        lowered,
    } = repair_world(case);
    let node = lowered.explanation().nodes()[node_index].identity();
    let restart = lowered.clone();
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
            OperationalTransitionId::new(format!("{case}/ready")).unwrap(),
            scenario.authority(),
            21,
            AuthorizationRevocationObservation::NotRevoked { observed_at: 21 },
        )
        .unwrap()
        .execute_with_control(
            &CurrentScenarioStagingPort,
            &InterruptAt::once(node, moment),
        )
        .unwrap_err();
    assert!(matches!(
        denial,
        AuthorityAffectingRepairExecutionDenial::Interrupted(_)
    ));

    drop(control);
    let reopened = scenario.control_store();
    let handle = repair_handles(&scenario, &reopened)
        .into_iter()
        .next()
        .expect("interrupted owner effect must leave a recovery handle");
    let executed = restart
        .recover_ready(&handle, &reopened, scenario.authority())
        .unwrap()
        .execute(&CurrentScenarioStagingPort)
        .unwrap();
    assert!(repair_handles(&scenario, &reopened).is_empty());
    let mut digest = Sha256::new();
    digest.update(node.fingerprint());
    digest.update([moment_tag(moment)]);
    digest.update(executed.staged_media().plan_fingerprint());
    digest.update(executed.staged_media().content_fingerprint());
    digest.finalize().into()
}

pub(super) fn repair_handles(
    scenario: &OwnerBackedBackupScenario,
    control: &OperationalControlStore,
) -> Vec<crate::IndeterminateRepairRecoveryHandle> {
    let selection = ExactScenarioControlSelection::current(scenario.authority(), control);
    let fencing = ControlStoreFencingAuthority::for_current_store(scenario.authority(), &selection);
    let crate::ControlStoreTrustPosture::Selected(selected) = control.inspect_generations(&fencing)
    else {
        panic!("repair control history must remain selected");
    };
    selected.indeterminate_repair_recovery_handles().to_vec()
}

pub(super) struct InterruptAt {
    node: OwnerPlanNodeIdentity,
    moment: RepairExecutionBoundaryMoment,
    fired: Cell<bool>,
}

impl InterruptAt {
    pub(super) const fn once(
        node: OwnerPlanNodeIdentity,
        moment: RepairExecutionBoundaryMoment,
    ) -> Self {
        Self {
            node,
            moment,
            fired: Cell::new(false),
        }
    }
}

impl RepairExecutionControlPort for InterruptAt {
    fn observe(&self, boundary: RepairExecutionBoundary) -> Result<(), RepairExecutionInterrupted> {
        if !self.fired.get() && boundary.node() == self.node && boundary.moment() == self.moment {
            self.fired.set(true);
            Err(RepairExecutionInterrupted::at(boundary))
        } else {
            Ok(())
        }
    }
}

const fn moment_tag(moment: RepairExecutionBoundaryMoment) -> u8 {
    match moment {
        RepairExecutionBoundaryMoment::BeforeOwnerEffect => 1,
        RepairExecutionBoundaryMoment::AfterOwnerEffectBeforeReceipt => 2,
        RepairExecutionBoundaryMoment::AfterReceiptPersistence => 3,
    }
}

impl ScenarioRepairOwnerRecoveryReceipt {
    pub const fn owner_nodes(self) -> u64 {
        self.owner_nodes
    }
    pub const fn recovered_cuts(self) -> u64 {
        self.recovered_cuts
    }
    pub const fn evidence_identity(self) -> [u8; 32] {
        self.evidence_identity
    }
}
