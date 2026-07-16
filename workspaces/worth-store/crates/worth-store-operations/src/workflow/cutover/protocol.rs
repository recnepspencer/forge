use sha2::{Digest, Sha256};
use worth_store_authority::{
    CurrentAuthorityReadmissionReceipt, RecoveryAuthorityReadmissionDenial,
    RecoveryCutoverAuthorityOwner, RecoveryWriteFenceDenial, RecoveryWriteFencePlan,
    RecoveryWriteFencePort, RecoveryWriteFenceReceipt, RecoveryWriteFenceReleaseReceipt,
    StoreCurrentAuthorityWitness,
};
use worth_store_physical_isolation::{
    AtomicRecoveryPublicationReceipt, RecoveryPublicationDenial, RecoveryPublicationLoweredPlan,
    RecoveryPublicationOwner, RecoveryPublicationPlanRequest,
};

use crate::authorization::{
    authorize_lowered_plan, consume_authorization, AuthorizationReplayPolicy,
    AuthorizedOperationalPlan, ConsumedOperationalPlan, LoweredOperationalPlan,
};
use crate::owner_plan_dag::{
    CanonicalOwnerPlanDag, DestructiveOperationKind, OperationalPlanBinding, OwnerPlanEffect,
    OwnerPlanExecutionStage, OwnerPlanFootprint, OwnerPlanNode, OwnerPlanPrerequisite,
    StoreOwnerKind,
};
use crate::{
    AuthorizationConsumptionDenial, AuthorizationDenial, AuthorizationRevocationObservation,
    ExternalOperatorAssertion, OperationalAuthorizationPort, OperationalControlAppendDenial,
    OperationalControlRecord, OperationalControlStore, OperationalOperationId,
    OperationalTransitionId,
};

use super::post_verification::ResolvedRecoveryCutoverCore;
use super::RecoveryCutoverDenial;

pub(super) struct LoweredCutoverCore<K> {
    pub(super) operation_id: OperationalOperationId,
    pub(super) authorization: LoweredOperationalPlan<K>,
    pub(super) fence: RecoveryWriteFencePlan,
    pub(super) publication: RecoveryPublicationLoweredPlan,
    pub(super) explanation: crate::CanonicalOwnerPlanDagExplanation,
    pub(super) source_lease: Option<super::post_verification::RecoveryCutoverSourceLease>,
    operation_kind: DestructiveOperationKind,
    authority_posture: worth_store_authority::RecoveryAuthorityAdmissionPosture,
    admission_policy: worth_store_authority::RecoveryAuthorityAdmissionPolicy,
}

pub(super) struct AuthorizedCutoverCore<K> {
    operation_id: OperationalOperationId,
    authorization: AuthorizedOperationalPlan<K>,
    fence: RecoveryWriteFencePlan,
    publication: RecoveryPublicationLoweredPlan,
    source_lease: Option<super::post_verification::RecoveryCutoverSourceLease>,
    operation_kind: DestructiveOperationKind,
    authority_posture: worth_store_authority::RecoveryAuthorityAdmissionPosture,
    admission_policy: worth_store_authority::RecoveryAuthorityAdmissionPolicy,
}

pub(super) struct FencedCutoverCore<K> {
    operation_id: OperationalOperationId,
    consumed: ConsumedOperationalPlan<K>,
    fence: RecoveryWriteFenceReceipt,
    publication: RecoveryPublicationLoweredPlan,
    source_lease: Option<super::post_verification::RecoveryCutoverSourceLease>,
    operation_kind: DestructiveOperationKind,
    authority_posture: worth_store_authority::RecoveryAuthorityAdmissionPosture,
    admission_policy: worth_store_authority::RecoveryAuthorityAdmissionPolicy,
}

pub(super) struct PublishedCutoverCore<K> {
    pub(super) operation_id: OperationalOperationId,
    pub(super) consumed: ConsumedOperationalPlan<K>,
    pub(super) fence: RecoveryWriteFenceReceipt,
    pub(super) publication: AtomicRecoveryPublicationReceipt,
    pub(super) source_lease: Option<super::post_verification::RecoveryCutoverSourceLease>,
    pub(super) authority_posture: worth_store_authority::RecoveryAuthorityAdmissionPosture,
    pub(super) admission_policy: worth_store_authority::RecoveryAuthorityAdmissionPolicy,
}

pub(super) struct ReadmittedCutoverCore {
    pub(super) publication: AtomicRecoveryPublicationReceipt,
    pub(super) readmission: CurrentAuthorityReadmissionReceipt,
    pub(super) fence_release: RecoveryWriteFenceReleaseReceipt,
    pub(super) source_lease: Option<super::post_verification::RecoveryCutoverSourceLease>,
}

#[derive(Debug)]
pub enum RecoveryCutoverExecutionDenial {
    StaleAuthority,
    Authorization(AuthorizationConsumptionDenial),
    Fence(RecoveryWriteFenceDenial),
    Publication(RecoveryPublicationDenial),
    Readmission(RecoveryAuthorityReadmissionDenial),
    Control(OperationalControlAppendDenial),
    InvalidDispositionBasis,
    PublicationMayHaveStarted,
    SourceLease(worth_store_physical_isolation::RecoverySourceLeaseDenial),
    MissingSourceLeaseRegistry,
}

pub(super) fn lower<K>(
    mut resolved: ResolvedRecoveryCutoverCore,
    current: &StoreCurrentAuthorityWitness,
    operation: DestructiveOperationKind,
) -> Result<LoweredCutoverCore<K>, RecoveryCutoverDenial> {
    if resolved.current().frontier.authority() != current.authority_identity() {
        return Err(RecoveryCutoverDenial::StaleCurrentAuthority);
    }
    let cutover_basis = cutover_basis(&resolved, operation);
    let fence = RecoveryCutoverAuthorityOwner::lower_write_fence(
        current,
        cutover_basis,
        resolved.media().content_fingerprint(),
        resolved.delta().identity(),
    )
    .map_err(RecoveryCutoverDenial::Authority)?;
    let publication = RecoveryPublicationOwner::lower(RecoveryPublicationPlanRequest::new(
        &resolved.current().publication_directory,
        resolved.current().current_root,
        resolved.current().old_reachability,
        resolved.media().clone(),
        resolved.verified().root_generation(),
        cutover_basis,
    ))
    .map_err(RecoveryCutoverDenial::Isolation)?;
    let footprint =
        OwnerPlanFootprint::bounded(0, 1).ok_or(RecoveryCutoverDenial::InvalidFootprint)?;
    let fence_node = node(
        StoreOwnerKind::Authority,
        OwnerPlanEffect::EstablishWriteFence,
        OwnerPlanExecutionStage::Cutover,
        footprint,
        fence.fingerprint(),
        false,
    );
    let publication_effect = match operation {
        DestructiveOperationKind::AuthorityAffectingRepairCutover => {
            OwnerPlanEffect::ChangeReachability
        }
        _ => OwnerPlanEffect::PublishNonCurrentRoot,
    };
    let publication_node = node(
        StoreOwnerKind::PhysicalIsolation,
        publication_effect,
        OwnerPlanExecutionStage::Cutover,
        footprint,
        publication.fingerprint(),
        true,
    );
    let readmission_fingerprint: [u8; 32] =
        Sha256::digest([cutover_basis.as_slice(), b"readmit".as_slice()].concat()).into();
    let readmission_effect = match operation {
        DestructiveOperationKind::AuthorityAffectingRepairCutover => {
            OwnerPlanEffect::EstablishAuthorityPosture
        }
        _ => OwnerPlanEffect::ReadmitCurrentAuthority,
    };
    let readmission_node = node(
        StoreOwnerKind::Authority,
        readmission_effect,
        OwnerPlanExecutionStage::Readmission,
        footprint,
        readmission_fingerprint,
        true,
    );
    let edges = vec![
        OwnerPlanPrerequisite::new(fence_node.identity(), publication_node.identity(), true),
        OwnerPlanPrerequisite::new(
            publication_node.identity(),
            readmission_node.identity(),
            true,
        ),
    ];
    let dag =
        CanonicalOwnerPlanDag::admit(vec![fence_node, publication_node, readmission_node], edges)
            .map_err(RecoveryCutoverDenial::OwnerDag)?;
    let explanation = dag.explanation().clone();
    let binding = OperationalPlanBinding::bind(
        operation,
        dag,
        current.authority_identity(),
        resolved.security_scope(),
        resolved.verified().verification_identity(),
        publication.candidate_media_identity(),
        resolved.delta().identity(),
    );
    let authority_posture = resolved.authority_posture();
    let admission_policy = resolved.admission_policy();
    let source_lease = resolved.take_source_lease();
    Ok(LoweredCutoverCore {
        operation_id: resolved.operation_id().clone(),
        authorization: LoweredOperationalPlan::from_binding(binding),
        fence,
        publication,
        explanation,
        source_lease,
        operation_kind: operation,
        authority_posture,
        admission_policy,
    })
}

pub(super) fn authorize<K>(
    lowered: LoweredCutoverCore<K>,
    port: &impl OperationalAuthorizationPort,
    assertion: &ExternalOperatorAssertion,
    requested_at: u64,
    expires_at: u64,
    replay_policy: AuthorizationReplayPolicy,
    revocation: AuthorizationRevocationObservation,
) -> Result<AuthorizedCutoverCore<K>, AuthorizationDenial> {
    Ok(AuthorizedCutoverCore {
        operation_id: lowered.operation_id,
        authorization: authorize_lowered_plan(
            lowered.authorization,
            port,
            assertion,
            requested_at,
            expires_at,
            replay_policy,
            revocation,
        )?,
        fence: lowered.fence,
        publication: lowered.publication,
        source_lease: lowered.source_lease,
        operation_kind: lowered.operation_kind,
        authority_posture: lowered.authority_posture,
        admission_policy: lowered.admission_policy,
    })
}

pub(super) fn ready<K>(
    authorized: AuthorizedCutoverCore<K>,
    control: &OperationalControlStore,
    transition: OperationalTransitionId,
    current: &StoreCurrentAuthorityWitness,
    fence_port: &impl RecoveryWriteFencePort,
    observed_at: u64,
    revocation: AuthorizationRevocationObservation,
) -> Result<FencedCutoverCore<K>, RecoveryCutoverExecutionDenial> {
    if authorized.authorization.binding().authority_identity() != current.authority_identity() {
        return Err(RecoveryCutoverExecutionDenial::StaleAuthority);
    }
    let operation_id = authorized.operation_id;
    let consumed = consume_authorization(
        control,
        operation_id.clone(),
        transition,
        authorized.authorization,
        None,
        observed_at,
        revocation,
    )
    .map_err(RecoveryCutoverExecutionDenial::Authorization)?;
    let fence =
        RecoveryCutoverAuthorityOwner::establish_write_fence(authorized.fence, current, fence_port)
            .map_err(RecoveryCutoverExecutionDenial::Fence)?;
    Ok(FencedCutoverCore {
        operation_id,
        consumed,
        fence,
        publication: authorized.publication,
        source_lease: authorized.source_lease,
        operation_kind: authorized.operation_kind,
        authority_posture: authorized.authority_posture,
        admission_policy: authorized.admission_policy,
    })
}

pub(super) fn publish<K>(
    fenced: FencedCutoverCore<K>,
    control: &impl crate::OperationalControlStorePort,
    transition: OperationalTransitionId,
) -> Result<PublishedCutoverCore<K>, RecoveryCutoverExecutionDenial> {
    let binding = crate::control_store::RecoveryPublicationControlBinding::from_prepared_cutover(
        recovery_operation_tag(fenced.operation_kind),
        fenced.fence,
        &fenced.publication,
        fenced.authority_posture,
        fenced.admission_policy,
    );
    control
        .append(&OperationalControlRecord::recovery_publication_prepared(
            fenced.fence.fenced_authority(),
            fenced.operation_id.clone(),
            transition,
            binding.clone(),
        ))
        .map_err(RecoveryCutoverExecutionDenial::Control)?;
    let publication = RecoveryPublicationOwner::publish(fenced.publication, fenced.fence)
        .map_err(RecoveryCutoverExecutionDenial::Publication)?;
    control
        .append(&OperationalControlRecord::recovery_publication_pending(
            fenced.fence.fenced_authority(),
            fenced.operation_id.clone(),
            OperationalTransitionId::recovery_publication_published(),
            binding,
        ))
        .map_err(RecoveryCutoverExecutionDenial::Control)?;
    Ok(PublishedCutoverCore {
        operation_id: fenced.operation_id,
        consumed: fenced.consumed,
        fence: fenced.fence,
        publication,
        source_lease: fenced.source_lease,
        authority_posture: fenced.authority_posture,
        admission_policy: fenced.admission_policy,
    })
}

fn cutover_basis(
    resolved: &ResolvedRecoveryCutoverCore,
    operation: DestructiveOperationKind,
) -> [u8; 32] {
    let mut digest = Sha256::new();
    digest.update(b"worth-store-recovery-cutover-basis-v1");
    digest.update([match operation {
        DestructiveOperationKind::BackupRestore => 1,
        DestructiveOperationKind::PointInTimeRecovery => 2,
        DestructiveOperationKind::Rollback => 3,
        DestructiveOperationKind::DerivedRepair => 4,
        DestructiveOperationKind::AuthorityAffectingRepair => 5,
        DestructiveOperationKind::BackupRestoreCutover => 6,
        DestructiveOperationKind::PointInTimeRecoveryCutover => 7,
        DestructiveOperationKind::RollbackCutover => 8,
        DestructiveOperationKind::AuthorityAffectingRepairCutover => 9,
    }]);
    digest.update(resolved.verified().verification_identity());
    digest.update(resolved.current().frontier.identity());
    digest.update(resolved.delta().identity());
    digest.update(resolved.authority_posture().identity());
    digest.update(resolved.admission_policy().identity());
    digest.finalize().into()
}
fn node(
    owner: StoreOwnerKind,
    effect: OwnerPlanEffect,
    stage: OwnerPlanExecutionStage,
    footprint: OwnerPlanFootprint,
    fingerprint: [u8; 32],
    irreversible: bool,
) -> OwnerPlanNode {
    OwnerPlanNode::from_owner_binding_at_stage(
        owner,
        effect,
        stage,
        footprint,
        1,
        irreversible,
        fingerprint,
        Sha256::digest(fingerprint).into(),
    )
}

const fn recovery_operation_tag(operation: DestructiveOperationKind) -> u8 {
    match operation {
        DestructiveOperationKind::BackupRestoreCutover => 1,
        DestructiveOperationKind::PointInTimeRecoveryCutover => 2,
        DestructiveOperationKind::RollbackCutover => 3,
        DestructiveOperationKind::AuthorityAffectingRepairCutover => 4,
        _ => 0,
    }
}
