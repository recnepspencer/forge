use worth_proof::TransitionOutcome;
use worth_runtime_bridge::facade::{
    BridgeAspectChangeWideningCause, BridgeAuthoritativePatchLoweringCounters,
    BridgeCommittedPatchEnvelope,
};

use super::authoritative_publication_witness::{
    admit_lowered_publication, begin_publication, resolve_publication, PublicationReadyRecipe,
    PublicationUnresolvedRecipe, RelationalPublicationBasis, RelationalPublicationRequest,
};
use crate::capabilities::CommitEnvelopeSource;
use crate::history::data::CommitId;
use crate::runtime::RelationalRuntime;

pub type RelationalBridgePublicationOutcome = TransitionOutcome<
    RelationalBridgePatchPublication,
    RelationalBridgePublicationDenial,
    RelationalBridgePublicationDeferred,
    RelationalBridgePublicationStale,
    RelationalBridgePublicationRebindRequired,
    RelationalBridgePublicationFailure,
>;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RelationalBridgePublicationDenial {
    error: worth_runtime_bridge::facade::BridgeRouteError,
    counters: BridgeAuthoritativePatchLoweringCounters,
}

impl RelationalBridgePublicationDenial {
    pub(super) const fn new(
        error: worth_runtime_bridge::facade::BridgeRouteError,
        counters: BridgeAuthoritativePatchLoweringCounters,
    ) -> Self {
        Self { error, counters }
    }

    pub fn error(&self) -> &worth_runtime_bridge::facade::BridgeRouteError {
        &self.error
    }

    pub fn kind(&self) -> worth_runtime_bridge::facade::BridgeRouteErrorKind {
        self.error.kind()
    }

    pub const fn counters(&self) -> BridgeAuthoritativePatchLoweringCounters {
        self.counters
    }
}

impl std::fmt::Display for RelationalBridgePublicationDenial {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.error.fmt(formatter)
    }
}

impl std::error::Error for RelationalBridgePublicationDenial {}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RelationalBridgePublicationDeferred {
    CommitVisibilityPending,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RelationalBridgePublicationStale {
    RuntimeAuthority,
    CommitNotRetained,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RelationalBridgePublicationRebindRequired {
    GraphRole,
}

pub type RelationalBridgePublicationFailure = std::convert::Infallible;

/// Relational-owned proof that a Bridge envelope was derived from one admitted
/// canonical authoritative patch, rather than assembled from detached items.
pub struct RelationalBridgePatchPublication {
    envelope: BridgeCommittedPatchEnvelope,
    proof: PublicationReadyRecipe,
    _commit_identity: crate::identity_authority::RelationalSourceTruthAuthorityIdentity<
        u64,
        crate::identity_authority::RelationalCommitIdentityKind,
    >,
}

impl RelationalBridgePatchPublication {
    fn mint(proof: PublicationReadyRecipe, envelope: BridgeCommittedPatchEnvelope) -> Self {
        let commit_identity = worth_foundational::facade::admit_foundational_authority_identity(
            proof.payload().commit_id.0,
            crate::identity_authority::relational_source_truth_authority(),
        );
        Self {
            envelope,
            proof,
            _commit_identity: commit_identity,
        }
    }

    pub fn bridge_envelope(&self) -> &BridgeCommittedPatchEnvelope {
        &self.envelope
    }

    pub fn lowering_counters(&self) -> &BridgeAuthoritativePatchLoweringCounters {
        self.envelope.patch_summary().authoritative_lowering()
    }

    pub fn runtime_instance_id(&self) -> u64 {
        self.proof.payload().runtime_instance_id
    }

    pub fn commit_id(&self) -> CommitId {
        self.proof.payload().commit_id
    }

    pub fn graph_role(&self) -> &str {
        &self.proof.payload().graph_role
    }

    pub fn adapter_identity(&self) -> &str {
        &self.proof.strong_basis().value().adapter_identity
    }

    pub fn source_basis(&self) -> &str {
        &self.proof.strong_basis().value().source_basis
    }

    pub fn partition_role(&self) -> Option<&worth_foundational::facade::TruthPartitionRole> {
        self.proof.payload().partition_role.as_ref()
    }

    pub fn relational_partition_id(&self) -> Option<crate::identity::data::PartitionId> {
        self.proof.payload().relational_partition_id
    }

    pub(crate) fn into_bridge_envelope(self) -> BridgeCommittedPatchEnvelope {
        self.envelope
    }
}

/// Runtime-affine admission for the one supported loss of precision. The
/// opaque token cannot be assembled from a widening label or copied fields.
pub struct RelationalOpaqueAspectWideningAdmission {
    runtime_instance_id: u64,
    graph_role: std::sync::Arc<str>,
    cause: BridgeAspectChangeWideningCause,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RelationalOpaqueAspectWideningAdmissionDenial {
    InvalidGraphRole,
}

impl RelationalRuntime {
    pub fn admit_opaque_aspect_bridge_widening(
        &self,
        graph_role: impl Into<std::sync::Arc<str>>,
    ) -> Result<
        RelationalOpaqueAspectWideningAdmission,
        RelationalOpaqueAspectWideningAdmissionDenial,
    > {
        let graph_role = graph_role.into();
        if graph_role.is_empty()
            || graph_role.trim() != graph_role.as_ref()
            || graph_role.chars().any(char::is_whitespace)
        {
            return Err(RelationalOpaqueAspectWideningAdmissionDenial::InvalidGraphRole);
        }
        Ok(RelationalOpaqueAspectWideningAdmission {
            runtime_instance_id: self.runtime_instance_id(),
            graph_role,
            cause: BridgeAspectChangeWideningCause::OpaquePayloadToWholeAspect,
        })
    }

    pub fn publish_commit_for_bridge(
        &self,
        commit_id: CommitId,
        graph_role: impl Into<std::sync::Arc<str>>,
    ) -> RelationalBridgePublicationOutcome {
        self.publish_commit_for_bridge_inner(commit_id, graph_role.into(), None, None)
    }

    pub fn publish_commit_for_bridge_partition(
        &self,
        commit_id: CommitId,
        graph_role: impl Into<std::sync::Arc<str>>,
        relational_partition_id: crate::identity::data::PartitionId,
        partition_role: worth_foundational::facade::TruthPartitionRole,
    ) -> RelationalBridgePublicationOutcome {
        self.publish_commit_for_bridge_inner(
            commit_id,
            graph_role.into(),
            Some((relational_partition_id, partition_role)),
            None,
        )
    }

    pub fn publish_commit_for_bridge_with_widening(
        &self,
        commit_id: CommitId,
        graph_role: impl Into<std::sync::Arc<str>>,
        admission: &RelationalOpaqueAspectWideningAdmission,
    ) -> RelationalBridgePublicationOutcome {
        if admission.runtime_instance_id != self.runtime_instance_id() {
            return TransitionOutcome::Stale(RelationalBridgePublicationStale::RuntimeAuthority);
        }
        let graph_role = graph_role.into();
        if admission.graph_role != graph_role {
            return TransitionOutcome::RebindRequired(
                RelationalBridgePublicationRebindRequired::GraphRole,
            );
        }
        self.publish_commit_for_bridge_inner(commit_id, graph_role, None, Some(admission.cause))
    }

    pub(crate) fn publish_commit_for_bridge_graph_role(
        &self,
        commit_id: CommitId,
        graph_role: std::sync::Arc<str>,
    ) -> RelationalBridgePublicationOutcome {
        self.publish_commit_for_bridge_inner(commit_id, graph_role, None, None)
    }

    pub(crate) fn publish_commit_for_bridge_graph_partition(
        &self,
        commit_id: CommitId,
        graph_role: std::sync::Arc<str>,
        relational_partition_id: crate::identity::data::PartitionId,
        partition_role: worth_foundational::facade::TruthPartitionRole,
    ) -> RelationalBridgePublicationOutcome {
        self.publish_commit_for_bridge_inner(
            commit_id,
            graph_role,
            Some((relational_partition_id, partition_role)),
            None,
        )
    }

    fn publish_commit_for_bridge_inner(
        &self,
        commit_id: CommitId,
        graph_role: std::sync::Arc<str>,
        partition: Option<(
            crate::identity::data::PartitionId,
            worth_foundational::facade::TruthPartitionRole,
        )>,
        admitted_widening: Option<BridgeAspectChangeWideningCause>,
    ) -> RelationalBridgePublicationOutcome {
        let unresolved = begin_publication(RelationalPublicationRequest {
            runtime_instance_id: self.runtime_instance_id(),
            commit_id,
            graph_role,
            relational_partition_id: partition.as_ref().map(|(partition, _)| *partition),
            partition_role: partition.map(|(_, role)| role),
            widening: admitted_widening,
        });
        if unresolved.payload().graph_role.trim().is_empty() {
            return TransitionOutcome::Denied(RelationalBridgePublicationDenial::new(
                worth_runtime_bridge::facade::BridgeRouteError::new(
                    worth_runtime_bridge::facade::BridgeRouteErrorKind::UnsupportedProducerEnvelope,
                    "Relational Bridge publication requires an explicit logical graph role",
                ),
                BridgeAuthoritativePatchLoweringCounters::default(),
            ));
        }
        let Some(envelope) = self.commit_envelope(unresolved.payload().commit_id) else {
            return if unresolved.payload().commit_id >= self.history().next_commit_id() {
                TransitionOutcome::Deferred(
                    RelationalBridgePublicationDeferred::CommitVisibilityPending,
                )
            } else {
                TransitionOutcome::Stale(RelationalBridgePublicationStale::CommitNotRetained)
            };
        };
        self.lower_retained_publication(unresolved, envelope)
    }

    fn lower_retained_publication(
        &self,
        unresolved: PublicationUnresolvedRecipe,
        envelope: &crate::history::data::CanonicalCommitEnvelope,
    ) -> RelationalBridgePublicationOutcome {
        let source_basis =
            publication_source_basis(self.runtime_instance_id(), &unresolved, envelope);
        let adapter_identity =
            super::identities::relational_bridge_adapter_identity(self.runtime_instance_id());
        let resolved = resolve_publication(
            unresolved,
            RelationalPublicationBasis {
                version_id: envelope.commit.version_id.0,
                branch_id: std::sync::Arc::from(envelope.commit.branch_id.0.clone()),
                adapter_identity: adapter_identity.clone(),
                source_basis: source_basis.clone(),
            },
        );
        let source = publication_source_provenance(
            self.runtime_instance_id(),
            resolved.payload(),
            adapter_identity,
            source_basis,
        );
        let metadata =
            worth_runtime_bridge::facade::BridgeProducerMetadata::registered_authoritative_source()
                .with_authoritative_source(source);
        let projection = super::partition_projection::project_patch_partition(
            &envelope.patch,
            resolved.payload().relational_partition_id,
        );
        let outcome = super::patch_envelopes::publication_patch_to_bridge_envelope_with_widening(
            super::patch_envelopes::RelationalBridgePatchPublicationRequest {
                commit_id: envelope.commit.commit_id,
                branch_id: &envelope.commit.branch_id,
                snapshot_identity: super::identities::bridge_snapshot_identity_for_commit(
                    envelope.commit.commit_id,
                    envelope.commit.version_id,
                ),
                patch: &projection.patch,
                admitted_widening: resolved.payload().widening,
                producer_metadata: metadata,
                source_record_patches_examined: projection.records_examined,
                source_record_patches_filtered_out: projection.records_filtered_out,
            },
        );
        match outcome {
            TransitionOutcome::Success(envelope) => {
                TransitionOutcome::Success(RelationalBridgePatchPublication::mint(
                    admit_lowered_publication(resolved),
                    envelope,
                ))
            }
            TransitionOutcome::Denied(denial) => TransitionOutcome::Denied(denial),
        }
    }
}

fn publication_source_basis(
    runtime_instance_id: u64,
    unresolved: &PublicationUnresolvedRecipe,
    envelope: &crate::history::data::CanonicalCommitEnvelope,
) -> std::sync::Arc<str> {
    let source_basis: std::sync::Arc<str> = std::sync::Arc::from(format!(
            "runtime={};commit={};version={};branch={};graph-role={};relational-partition={};truth-partition={}",
            runtime_instance_id,
            envelope.commit.commit_id.0,
            envelope.commit.version_id.0,
            envelope.commit.branch_id.0,
            unresolved.payload().graph_role,
            unresolved
                .payload()
                .relational_partition_id
                .map_or_else(|| "all".to_string(), |partition| partition.as_u32().to_string()),
            unresolved
                .payload()
                .partition_role
                .as_ref()
                .map_or("all", |role| role.as_str()),
        ));
    source_basis
}

fn publication_source_provenance(
    runtime_instance_id: u64,
    request: &RelationalPublicationRequest,
    adapter_identity: std::sync::Arc<str>,
    source_basis: std::sync::Arc<str>,
) -> worth_runtime_bridge::facade::BridgeAuthoritativeSourceProvenance {
    match request.partition_role.clone() {
            Some(partition_role) => worth_runtime_bridge::facade::BridgeAuthoritativeSourceProvenance::from_owner_partition_publication(
                runtime_instance_id,
                request.graph_role.clone(),
                adapter_identity,
                source_basis,
                partition_role,
            ),
            None => worth_runtime_bridge::facade::BridgeAuthoritativeSourceProvenance::from_owner_publication(
                runtime_instance_id,
                request.graph_role.clone(),
                adapter_identity,
                source_basis,
            ),
    }
}
