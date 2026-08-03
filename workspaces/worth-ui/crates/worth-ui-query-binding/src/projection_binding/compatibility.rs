use std::sync::Arc;

use worth_query::facade::{
    domain::{
        WorthQueryCompatibilityCounters, WorthQueryCompatibilityDenialKind,
        WorthQueryReplacementDenial, WorthQueryReplacementWitness,
    },
    runtime::WorthQueryWorkspace,
};

use super::{
    UiProjectionBindingStopKind, UiProjectionBindingStopReceipt, UiScalarProjectionBinding,
};
use crate::WorthUiQueryWorkspaceExt;

#[must_use = "a compatibility proof carries Query's pair-bound replacement witness"]
pub struct UiProjectionBindingCompatibilityProof {
    query_witness: WorthQueryReplacementWitness,
    predecessor_binding_identity: Arc<str>,
    successor_binding_identity: Arc<str>,
}

impl UiProjectionBindingCompatibilityProof {
    pub(super) fn query_issued(
        query_witness: WorthQueryReplacementWitness,
        predecessor_binding_identity: impl Into<Arc<str>>,
        successor_binding_identity: impl Into<Arc<str>>,
    ) -> Self {
        Self {
            query_witness,
            predecessor_binding_identity: predecessor_binding_identity.into(),
            successor_binding_identity: successor_binding_identity.into(),
        }
    }

    pub fn predecessor_binding_identity_for_reporting(&self) -> &str {
        self.predecessor_binding_identity.as_ref()
    }

    pub fn successor_binding_identity_for_reporting(&self) -> &str {
        self.successor_binding_identity.as_ref()
    }

    pub fn query_counters(&self) -> WorthQueryCompatibilityCounters {
        self.query_witness.counters()
    }
}

impl std::fmt::Debug for UiProjectionBindingCompatibilityProof {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("UiProjectionBindingCompatibilityProof")
            .field("query_witness", &self.query_witness)
            .field(
                "predecessor_binding_identity",
                &self.predecessor_binding_identity,
            )
            .field(
                "successor_binding_identity",
                &self.successor_binding_identity,
            )
            .finish()
    }
}

#[must_use]
#[derive(Debug)]
pub enum UiScalarProjectionReplacementOutcome {
    Admitted(Box<UiScalarProjectionReplacementReceipt>),
    Stopped(Box<UiScalarProjectionReplacementStop>),
}

#[must_use = "the admitted successor is the only binding carrying preserved identity"]
#[derive(Debug)]
pub struct UiScalarProjectionReplacementReceipt {
    successor: UiScalarProjectionBinding,
    proof: UiProjectionBindingCompatibilityProof,
}

impl UiScalarProjectionReplacementReceipt {
    pub fn proof(&self) -> &UiProjectionBindingCompatibilityProof {
        &self.proof
    }

    pub fn into_successor(self) -> UiScalarProjectionBinding {
        self.successor
    }

    pub fn into_parts(
        self,
    ) -> (
        UiScalarProjectionBinding,
        UiProjectionBindingCompatibilityProof,
    ) {
        (self.successor, self.proof)
    }
}

#[must_use = "a failed replacement retains both bindings for recovery or cleanup"]
#[derive(Debug)]
pub struct UiScalarProjectionReplacementStop {
    predecessor: UiScalarProjectionBinding,
    candidate: UiScalarProjectionBinding,
    stop: UiProjectionBindingStopReceipt,
}

impl UiScalarProjectionReplacementStop {
    pub fn stop(&self) -> &UiProjectionBindingStopReceipt {
        &self.stop
    }

    pub fn into_bindings(self) -> (UiScalarProjectionBinding, UiScalarProjectionBinding) {
        (self.predecessor, self.candidate)
    }

    pub fn into_parts(
        self,
    ) -> (
        UiScalarProjectionBinding,
        UiScalarProjectionBinding,
        UiProjectionBindingStopReceipt,
    ) {
        (self.predecessor, self.candidate, self.stop)
    }
}

impl UiScalarProjectionBinding {
    pub fn replace_with(
        self,
        candidate: UiScalarProjectionBinding,
        workspace: &WorthQueryWorkspace,
    ) -> UiScalarProjectionReplacementOutcome {
        if self.requirement() != candidate.requirement() {
            return stopped(
                self,
                candidate,
                UiProjectionBindingStopKind::SchemaMismatch,
                "the scalar replacement requirement selects different schema",
            );
        }
        if self.view_identity() != candidate.view_identity() {
            return stopped(
                self,
                candidate,
                UiProjectionBindingStopKind::SchemaMismatch,
                "the scalar replacement targets a different installed view",
            );
        }
        let predecessor_prepared = match prepare_for_replacement(&self, workspace) {
            Ok(prepared) => prepared,
            Err((kind, summary)) => return stopped(self, candidate, kind, summary),
        };
        let candidate_prepared = match prepare_for_replacement(&candidate, workspace) {
            Ok(prepared) => prepared,
            Err((kind, summary)) => return stopped(self, candidate, kind, summary),
        };
        let predecessor_identity: Arc<str> =
            Arc::from(predecessor_prepared.binding_identity_for_reporting());
        let successor_identity: Arc<str> =
            Arc::from(candidate_prepared.binding_identity_for_reporting());
        let query_witness = match predecessor_prepared.replacement_witness_for(&candidate_prepared)
        {
            Ok(witness) => witness,
            Err(denial) => {
                let (kind, summary) = query_replacement_stop(denial);
                return stopped(self, candidate, kind, summary);
            }
        };
        let proof = UiProjectionBindingCompatibilityProof::query_issued(
            query_witness,
            predecessor_identity,
            successor_identity,
        );
        let successor = candidate.inherit_compatible_identity_from(self);
        UiScalarProjectionReplacementOutcome::Admitted(Box::new(
            UiScalarProjectionReplacementReceipt { successor, proof },
        ))
    }
}

fn prepare_for_replacement(
    binding: &UiScalarProjectionBinding,
    workspace: &WorthQueryWorkspace,
) -> Result<
    crate::application_binding::WorthUiPreparedScalarTextConsumer,
    (UiProjectionBindingStopKind, String),
> {
    let gateway = binding
        .reference()
        .enter_attempt(workspace)
        .map_err(|denial| {
            use crate::WorthUiQueryOperationAttemptDenial as Denial;
            match denial {
                Denial::Installation(_) => (
                    UiProjectionBindingStopKind::MissingInstalledView,
                    "the installed Query view is unavailable".to_owned(),
                ),
                Denial::InstalledDomainAuthorityMismatch => {
                    installation_authority_stop(binding.runtime_provenance(), workspace, "scalar")
                }
                Denial::OperatingWorld(_) => (
                    UiProjectionBindingStopKind::RebindRequired,
                    "Query denied replacement entry to the operating world".to_owned(),
                ),
            }
        })?;
    gateway
        .prepare_consumer(binding.requirement().selected_field().declared_name())
        .map_err(preparation_stop)
}

pub(super) fn installation_authority_stop(
    expected_runtime: worth_query::facade::runtime::WorthQueryRuntimeProvenance,
    workspace: &WorthQueryWorkspace,
    shape: &str,
) -> (UiProjectionBindingStopKind, String) {
    match workspace.worth_ui() {
        Ok(current) if current.runtime_provenance() == expected_runtime => (
            UiProjectionBindingStopKind::RebindRequired,
            format!("the {shape} replacement belongs to a stale Query installation generation"),
        ),
        _ => (
            UiProjectionBindingStopKind::WrongWorld,
            format!("the {shape} replacement belongs to a different Query world"),
        ),
    }
}

fn preparation_stop(
    denial: crate::application_binding::WorthUiScalarTextConsumerPreparationDenial,
) -> (UiProjectionBindingStopKind, String) {
    use crate::application_binding::WorthUiScalarTextConsumerPreparationDenial as Denial;
    match denial {
        Denial::Binding(denial) => (
            UiProjectionBindingStopKind::RebindRequired,
            denial.detail().to_owned(),
        ),
        Denial::ConsumerContract(_) => (
            UiProjectionBindingStopKind::LifecycleMismatch,
            "Query consumer support no longer satisfies the scalar lifecycle".to_owned(),
        ),
        Denial::NativeRequest(denial) => {
            super::scalar_native_request_stop::scalar_native_request_stop(denial)
        }
    }
}

pub(super) fn query_replacement_stop(
    denial: WorthQueryReplacementDenial,
) -> (UiProjectionBindingStopKind, String) {
    use WorthQueryCompatibilityDenialKind as Kind;
    let kind = match denial.kind() {
        Kind::RuntimeAuthority => UiProjectionBindingStopKind::WrongWorld,
        Kind::NativeContractMismatched
        | Kind::NativeContractUnsupported
        | Kind::NativeMaskMismatched
        | Kind::NativeMaskUnsupported => UiProjectionBindingStopKind::NativeFamilyMismatch,
        Kind::NativeProducerShape => UiProjectionBindingStopKind::PayloadShapeMismatch,
        Kind::PortableOperationContract
        | Kind::PortableConditionalMismatched
        | Kind::PortableConditionalUnsupported => UiProjectionBindingStopKind::SchemaMismatch,
        Kind::DomainInstallation
        | Kind::DomainRebindAuthority
        | Kind::InstallationGeneration
        | Kind::InstallationFreshness
        | Kind::BasisMismatched
        | Kind::BasisUnsupported
        | Kind::BasisLifecycle
        | Kind::GraphAuthority
        | Kind::RequiredDomainAuthority
        | Kind::ConditionalLoweringSet
        | Kind::ConditionalLowering
        | Kind::RelationshipRule => UiProjectionBindingStopKind::RebindRequired,
    };
    (
        kind,
        format!("Query denied scalar replacement: {}", denial.detail()),
    )
}

fn stopped(
    predecessor: UiScalarProjectionBinding,
    candidate: UiScalarProjectionBinding,
    kind: UiProjectionBindingStopKind,
    summary: impl Into<Arc<str>>,
) -> UiScalarProjectionReplacementOutcome {
    let stop = UiProjectionBindingStopReceipt::replacement(
        kind,
        candidate.replacement_attempt_identity(),
        predecessor.core().query_binding_identity(),
        summary,
    );
    UiScalarProjectionReplacementOutcome::Stopped(Box::new(UiScalarProjectionReplacementStop {
        predecessor,
        candidate,
        stop,
    }))
}
