use std::sync::Arc;

use worth_query::facade::runtime::WorthQueryWorkspace;

use super::{
    compatibility::{
        installation_authority_stop, query_replacement_stop, UiProjectionBindingCompatibilityProof,
    },
    UiCollectionProjectionBinding, UiProjectionBindingStopKind, UiProjectionBindingStopReceipt,
};

#[must_use]
#[derive(Debug)]
pub enum UiCollectionProjectionReplacementOutcome {
    Admitted(Box<UiCollectionProjectionReplacementReceipt>),
    Stopped(Box<UiCollectionProjectionReplacementStop>),
}

#[must_use = "the admitted successor is the only collection binding carrying preserved identity"]
#[derive(Debug)]
pub struct UiCollectionProjectionReplacementReceipt {
    successor: UiCollectionProjectionBinding,
    proof: UiProjectionBindingCompatibilityProof,
}

impl UiCollectionProjectionReplacementReceipt {
    pub fn proof(&self) -> &UiProjectionBindingCompatibilityProof {
        &self.proof
    }

    pub fn into_successor(self) -> UiCollectionProjectionBinding {
        self.successor
    }
}

#[must_use = "a failed collection replacement retains both bindings"]
#[derive(Debug)]
pub struct UiCollectionProjectionReplacementStop {
    predecessor: UiCollectionProjectionBinding,
    candidate: UiCollectionProjectionBinding,
    stop: UiProjectionBindingStopReceipt,
}

impl UiCollectionProjectionReplacementStop {
    pub fn stop(&self) -> &UiProjectionBindingStopReceipt {
        &self.stop
    }

    pub fn into_bindings(self) -> (UiCollectionProjectionBinding, UiCollectionProjectionBinding) {
        (self.predecessor, self.candidate)
    }
}

impl UiCollectionProjectionBinding {
    pub fn replace_with(
        self,
        candidate: UiCollectionProjectionBinding,
        workspace: &WorthQueryWorkspace,
    ) -> UiCollectionProjectionReplacementOutcome {
        if let Some((kind, summary)) = requirement_stop(&self, &candidate) {
            return stopped(self, candidate, kind, summary);
        }
        let predecessor_prepared = match prepare_for_replacement(&self, workspace) {
            Ok(prepared) => prepared,
            Err((kind, summary)) => return stopped(self, candidate, kind, summary),
        };
        let candidate_prepared = match prepare_for_replacement(&candidate, workspace) {
            Ok(prepared) => prepared,
            Err((kind, summary)) => return stopped(self, candidate, kind, summary),
        };
        let predecessor_identity =
            crate::UiQueryIdentityReportingProjection::from_query_reporting_text(
                predecessor_prepared.binding_identity_for_reporting(),
            );
        let successor_identity =
            crate::UiQueryIdentityReportingProjection::from_query_reporting_text(
                candidate_prepared.binding_identity_for_reporting(),
            );
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
        UiCollectionProjectionReplacementOutcome::Admitted(Box::new(
            UiCollectionProjectionReplacementReceipt { successor, proof },
        ))
    }
}

fn requirement_stop(
    predecessor: &UiCollectionProjectionBinding,
    candidate: &UiCollectionProjectionBinding,
) -> Option<(UiProjectionBindingStopKind, &'static str)> {
    if predecessor.view_identity() != candidate.view_identity() {
        return Some((
            UiProjectionBindingStopKind::SchemaMismatch,
            "the collection replacement targets a different installed view",
        ));
    }
    let predecessor = predecessor.requirement();
    let candidate = candidate.requirement();
    if predecessor.row_identity_field() != candidate.row_identity_field() {
        return Some((
            UiProjectionBindingStopKind::RowIdentityMismatch,
            "the collection replacement changes Query row identity",
        ));
    }
    if predecessor.native_family() != candidate.native_family() {
        return Some((
            UiProjectionBindingStopKind::NativeFamilyMismatch,
            "the collection replacement changes native value family",
        ));
    }
    if predecessor.selected_fields() != candidate.selected_fields() {
        return Some((
            UiProjectionBindingStopKind::SchemaMismatch,
            "the collection replacement selects different fields",
        ));
    }
    if predecessor.requires_complete_result() != candidate.requires_complete_result()
        || predecessor.permits_continuation() != candidate.permits_continuation()
    {
        return Some((
            UiProjectionBindingStopKind::PayloadShapeMismatch,
            "the collection replacement changes completeness or continuation cardinality",
        ));
    }
    None
}

fn prepare_for_replacement(
    binding: &UiCollectionProjectionBinding,
    workspace: &WorthQueryWorkspace,
) -> Result<
    crate::application_binding::WorthUiPreparedCollectionTextConsumer,
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
                    "the installed Query collection view is unavailable".to_owned(),
                ),
                Denial::InstalledDomainAuthorityMismatch => installation_authority_stop(
                    binding.runtime_provenance(),
                    workspace,
                    "collection",
                ),
                Denial::OperatingWorld(_) => (
                    UiProjectionBindingStopKind::RebindRequired,
                    "Query denied collection replacement entry to the operating world".to_owned(),
                ),
            }
        })?;
    gateway
        .prepare_consumer(binding.requirement())
        .map_err(preparation_stop)
}

fn preparation_stop(
    denial: crate::application_binding::WorthUiCollectionTextConsumerPreparationDenial,
) -> (UiProjectionBindingStopKind, String) {
    use crate::application_binding::WorthUiCollectionTextConsumerPreparationDenial as Denial;
    match denial {
        Denial::Binding(denial) => (
            UiProjectionBindingStopKind::RebindRequired,
            denial.detail().to_owned(),
        ),
        Denial::ConsumerContract(_) => (
            UiProjectionBindingStopKind::LifecycleMismatch,
            "Query consumer support no longer satisfies collection lifecycle".to_owned(),
        ),
        Denial::ProjectionShapeMismatch => (
            UiProjectionBindingStopKind::PayloadShapeMismatch,
            "Query collection producer shape changed".to_owned(),
        ),
        Denial::RowIdentityMismatch => (
            UiProjectionBindingStopKind::RowIdentityMismatch,
            "Query collection row identity changed".to_owned(),
        ),
        Denial::NativeRequest(denial) => native_request_stop(denial),
    }
}

fn native_request_stop(
    denial: crate::application_binding::WorthUiCollectionTextNativeRequestDenial,
) -> (UiProjectionBindingStopKind, String) {
    use crate::application_binding::WorthUiCollectionTextNativeRequestDenial as Denial;
    match denial {
        Denial::NativeFamilyMismatch => (
            UiProjectionBindingStopKind::NativeFamilyMismatch,
            "Query collection native value family changed".to_owned(),
        ),
        Denial::ProjectionRequest(_) | Denial::SelectionMismatch(_) => (
            UiProjectionBindingStopKind::SchemaMismatch,
            "Query collection selected fields changed".to_owned(),
        ),
    }
}

fn stopped(
    predecessor: UiCollectionProjectionBinding,
    candidate: UiCollectionProjectionBinding,
    kind: UiProjectionBindingStopKind,
    summary: impl Into<Arc<str>>,
) -> UiCollectionProjectionReplacementOutcome {
    let stop = UiProjectionBindingStopReceipt::replacement(
        kind,
        candidate.replacement_attempt_identity(),
        predecessor
            .core()
            .retained_query_binding_reporting_projection(),
        summary,
    );
    UiCollectionProjectionReplacementOutcome::Stopped(Box::new(
        UiCollectionProjectionReplacementStop {
            predecessor,
            candidate,
            stop,
        },
    ))
}
