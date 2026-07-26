use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{mpsc, Arc, MutexGuard, TryLockError};
use std::thread;
use std::time::Duration;

use worth_query_installation::facade::WorthQueryInstallationGeneration;

use super::super::{
    installed_artifact_contract_for_managed_run, WorthQueryArtifactProductionAuthority,
    WorthQueryArtifactProductionAuthorityParts, WorthQueryArtifactProductionEvidence,
    WorthQueryArtifactProductionGeneration, WorthQueryArtifactProviderResource,
    WorthQueryMoveOnlyArtifactHandle, WorthQueryRuntimeArtifactLifecycle,
    WorthQueryRuntimeArtifactOwner, WorthQueryWorkflowArtifactRegistry,
    WorthQueryWorkflowArtifactRegistryEvidence,
};
use super::WorthQueryArtifactLifecycleSnapshotGate;
use crate::domain_computation::execution_runtime::WorthQueryExecutionRuntimeInstaller;
use crate::domain_computation::operation_binding::WorthQueryInstalledDomainExecutionAuthority;

#[test]
fn evidence_snapshot_excludes_real_registration_and_lifecycle_mutation() {
    let (authority, registry) = production_world();
    let disposals = Arc::new(AtomicUsize::new(0));
    let first = register(&authority, "first", Arc::clone(&disposals));
    let second = register(&authority, "second", Arc::clone(&disposals));
    prove_owner_release_waits_for_evidence(&registry, first, &second);

    let after_release = registry.evidence();
    assert_eq!(after_release.retained_artifact_count(), 1);
    assert_eq!(after_release.disposed_artifact_count(), 1);
    assert_eq!(after_release.provider_release_complete_count(), 1);

    let third =
        prove_registration_waits_for_snapshot(&authority, &registry, Arc::clone(&disposals));
    let after_registration = registry.evidence();
    assert_eq!(after_registration.produced_artifact_count(), 3);
    assert_eq!(after_registration.retained_artifact_count(), 2);
    assert_eq!(after_registration.disposed_artifact_count(), 1);

    drop(second);
    drop(third);
    assert_eq!(disposals.load(Ordering::Acquire), 3);
}

fn prove_owner_release_waits_for_evidence(
    registry: &Arc<WorthQueryWorkflowArtifactRegistry>,
    releasing: WorthQueryMoveOnlyArtifactHandle,
    blocker: &WorthQueryMoveOnlyArtifactHandle,
) {
    let releasing_owner = Arc::clone(&releasing.core.owner);
    let blocked_lifecycle = blocker.core.owner.lifecycle.lock();
    let (evidence_receiver, evidence_thread) = spawn_evidence_snapshot(registry);
    if !wait_for_exclusive_snapshot(&registry.snapshot_gate) {
        cleanup_missing_evidence_gate(blocked_lifecycle, evidence_receiver, evidence_thread);
    }

    let mutation_attempts = releasing_owner.lifecycle_gate_attempt_count();
    let (release_sender, release_receiver) = mpsc::channel();
    let release_thread = thread::spawn(move || {
        drop(releasing);
        release_sender
            .send(())
            .expect("release receiver remains live");
    });
    let blocked_release = BlockedOwnerRelease {
        blocked_lifecycle,
        evidence_receiver,
        evidence_thread,
        release_receiver,
        release_thread,
    };
    if !wait_for_owner_lifecycle_attempt(&releasing_owner, mutation_attempts) {
        cleanup_missing_owner_gate(blocked_release);
    }
    assert!(matches!(
        blocked_release.release_receiver.try_recv(),
        Err(mpsc::TryRecvError::Empty)
    ));
    let during_release = blocked_release.finish();
    assert_eq!(during_release.produced_artifact_count(), 2);
    assert_eq!(during_release.retained_artifact_count(), 2);
    assert_eq!(during_release.disposed_artifact_count(), 0);
    assert_eq!(during_release.provider_release_complete_count(), 0);
}

fn prove_registration_waits_for_snapshot(
    authority: &Arc<WorthQueryArtifactProductionAuthority>,
    registry: &Arc<WorthQueryWorkflowArtifactRegistry>,
    disposals: Arc<AtomicUsize>,
) -> WorthQueryMoveOnlyArtifactHandle {
    let exclusive_snapshot = registry.snapshot_gate.evidence_snapshot();
    let registration_attempts = registry.snapshot_gate.lifecycle_mutation_attempt_count();
    let (registration_sender, registration_receiver) = mpsc::channel();
    let registration_authority = Arc::clone(authority);
    let registration_disposals = Arc::clone(&disposals);
    let registration_thread = thread::spawn(move || {
        registration_sender
            .send(register(
                &registration_authority,
                "third",
                registration_disposals,
            ))
            .expect("registration receiver remains live");
    });
    let observed_registration_attempt =
        wait_for_lifecycle_mutation_attempt(&registry.snapshot_gate, registration_attempts);
    if !observed_registration_attempt {
        drop(exclusive_snapshot);
        let unguarded = registration_receiver
            .recv_timeout(Duration::from_secs(1))
            .expect("registration must finish during mutation failure cleanup");
        registration_thread
            .join()
            .expect("registration thread must remain in process");
        drop(unguarded);
        panic!("artifact registration did not enter the lifecycle snapshot gate");
    }
    assert!(matches!(
        registration_receiver.try_recv(),
        Err(mpsc::TryRecvError::Empty)
    ));
    drop(exclusive_snapshot);
    let third = registration_receiver
        .recv_timeout(Duration::from_secs(1))
        .expect("registration must finish after the snapshot");
    registration_thread
        .join()
        .expect("registration thread must remain in process");
    third
}

fn spawn_evidence_snapshot(
    registry: &Arc<WorthQueryWorkflowArtifactRegistry>,
) -> (
    mpsc::Receiver<WorthQueryWorkflowArtifactRegistryEvidence>,
    thread::JoinHandle<()>,
) {
    let (sender, receiver) = mpsc::channel();
    let registry = Arc::clone(registry);
    let thread = thread::spawn(move || {
        sender
            .send(registry.evidence())
            .expect("evidence receiver remains live");
    });
    (receiver, thread)
}

struct BlockedOwnerRelease<'a> {
    blocked_lifecycle: MutexGuard<'a, WorthQueryRuntimeArtifactLifecycle>,
    evidence_receiver: mpsc::Receiver<WorthQueryWorkflowArtifactRegistryEvidence>,
    evidence_thread: thread::JoinHandle<()>,
    release_receiver: mpsc::Receiver<()>,
    release_thread: thread::JoinHandle<()>,
}

impl BlockedOwnerRelease<'_> {
    fn finish(self) -> WorthQueryWorkflowArtifactRegistryEvidence {
        drop(self.blocked_lifecycle);
        let evidence = self
            .evidence_receiver
            .recv_timeout(Duration::from_secs(1))
            .expect("unblocked evidence snapshot must finish");
        self.evidence_thread
            .join()
            .expect("evidence thread must remain in process");
        self.release_receiver
            .recv_timeout(Duration::from_secs(1))
            .expect("lifecycle mutation must finish after the snapshot");
        self.release_thread
            .join()
            .expect("release thread must remain in process");
        evidence
    }
}

fn cleanup_missing_evidence_gate(
    blocked_lifecycle: MutexGuard<'_, WorthQueryRuntimeArtifactLifecycle>,
    evidence_receiver: mpsc::Receiver<WorthQueryWorkflowArtifactRegistryEvidence>,
    evidence_thread: thread::JoinHandle<()>,
) -> ! {
    drop(blocked_lifecycle);
    evidence_receiver
        .recv_timeout(Duration::from_secs(1))
        .expect("unguarded evidence snapshot must finish after lifecycle release");
    evidence_thread
        .join()
        .expect("unguarded evidence thread must remain in process");
    panic!("evidence snapshot did not acquire its exclusive gate");
}

fn cleanup_missing_owner_gate(blocked_release: BlockedOwnerRelease<'_>) -> ! {
    let _ = blocked_release.finish();
    panic!("owner release did not enter the lifecycle snapshot gate");
}

fn production_world() -> (
    Arc<WorthQueryArtifactProductionAuthority>,
    Arc<WorthQueryWorkflowArtifactRegistry>,
) {
    let runtime = WorthQueryExecutionRuntimeInstaller::new()
        .install(
            WorthQueryInstallationGeneration::initial(),
            std::iter::empty(),
        )
        .expect("artifact evidence world must install")
        .into_parts()
        .0;
    let domain_authority = WorthQueryInstalledDomainExecutionAuthority::mint(
        runtime.authority_identity(),
        "WORTH.tests.affinity.owner",
        runtime.installed_packages().generation(),
        runtime.retain_current_generation(),
    );
    let registry = Arc::new(WorthQueryWorkflowArtifactRegistry::new(
        "registry-evidence-run".into(),
    ));
    let authority =
        WorthQueryArtifactProductionAuthority::mint(WorthQueryArtifactProductionAuthorityParts {
            contract: installed_artifact_contract_for_managed_run(),
            domain_authority,
            operation_identity: "registry-evidence-operation".into(),
            binding_identity: "registry-evidence-binding".into(),
            run_identity: "registry-evidence-run".into(),
            stage_identity: "producer".into(),
            basis_identity: "registry-evidence-basis".into(),
            registry: Arc::clone(&registry),
            production_generation: WorthQueryArtifactProductionGeneration::initial(),
        });
    (authority, registry)
}

fn register(
    authority: &Arc<WorthQueryArtifactProductionAuthority>,
    label: &str,
    disposals: Arc<AtomicUsize>,
) -> WorthQueryMoveOnlyArtifactHandle {
    let admission = WorthQueryArtifactProductionAuthority::admit(
        authority,
        WorthQueryArtifactProductionEvidence::new(
            format!("{label}-provenance"),
            format!("{label}-dependency"),
        ),
    );
    WorthQueryArtifactProductionAuthority::register_exact(
        authority,
        admission,
        EvidenceArtifact(disposals),
    )
    .expect("exact installed artifact authority must register")
}

fn wait_for_exclusive_snapshot(gate: &WorthQueryArtifactLifecycleSnapshotGate) -> bool {
    for _ in 0..100_000 {
        match gate.state.try_read() {
            Err(TryLockError::WouldBlock) => return true,
            Ok(guard) => drop(guard),
            Err(TryLockError::Poisoned(_)) => {
                panic!("artifact lifecycle snapshot gate must remain available")
            }
        }
        thread::yield_now();
    }
    false
}

fn wait_for_lifecycle_mutation_attempt(
    gate: &WorthQueryArtifactLifecycleSnapshotGate,
    prior: usize,
) -> bool {
    for _ in 0..100_000 {
        if gate.lifecycle_mutation_attempt_count() > prior {
            return true;
        }
        thread::yield_now();
    }
    false
}

fn wait_for_owner_lifecycle_attempt(owner: &WorthQueryRuntimeArtifactOwner, prior: usize) -> bool {
    for _ in 0..100_000 {
        if owner.lifecycle_gate_attempt_count() > prior {
            return true;
        }
        thread::yield_now();
    }
    false
}

struct EvidenceArtifact(Arc<AtomicUsize>);

impl WorthQueryArtifactProviderResource for EvidenceArtifact {
    const PROVIDER_FAMILY: &'static str = "WORTH.tests.affinity.provider";

    fn canonical_semantic_projection(&self) -> Vec<u8> {
        b"registry-evidence-artifact".to_vec()
    }

    fn retained_bytes(&self) -> usize {
        32
    }

    fn dispose(&mut self) {
        self.0.fetch_add(1, Ordering::AcqRel);
    }
}
