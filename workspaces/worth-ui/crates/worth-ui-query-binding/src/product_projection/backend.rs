use std::cell::RefCell;
use std::collections::BTreeMap;
use std::rc::Rc;

use worth_query::facade::{foundation, runtime};

use super::WorthUiScalarProjectionSourceRecord;

pub(crate) type SharedSourceState = Rc<RefCell<WorthUiExternalScalarSourceState>>;

pub(crate) fn shared_source_state() -> SharedSourceState {
    Rc::new(RefCell::new(WorthUiExternalScalarSourceState::default()))
}

#[derive(Default)]
pub(crate) struct WorthUiExternalScalarSourceState {
    record: Option<WorthUiScalarProjectionSourceRecord>,
    live_targets: BTreeMap<runtime::WorthQueryLiveArtifactTarget, String>,
}

impl WorthUiExternalScalarSourceState {
    pub(crate) fn publish(&mut self, record: WorthUiScalarProjectionSourceRecord) {
        self.record = Some(record);
    }

    pub(crate) fn live_source_count(&self) -> usize {
        self.live_targets.len()
    }
}

pub(crate) struct WorthUiExternalScalarSourceBackend {
    state: SharedSourceState,
}

impl WorthUiExternalScalarSourceBackend {
    pub(crate) fn new(state: SharedSourceState) -> Self {
        Self { state }
    }
}

impl runtime::WorthQueryRuntimeBackend for WorthUiExternalScalarSourceBackend {
    fn support_profile(&self) -> runtime::WorthQueryRuntimeSupportProfile {
        use runtime::{
            WorthQueryAuthorityLane as Lane, WorthQueryRuntimeFacadeFamily as Family,
            WorthQueryRuntimeFamilySupport as Support,
        };
        runtime::WorthQueryRuntimeSupportProfile::new([
            Support::supported(
                Family::Read,
                [Lane::AuthoritativeTruth],
                [],
                ["external-scalar-native-read"],
            ),
            Support::supported(
                Family::Live,
                [Lane::AuthoritativeTruth],
                [],
                ["external-scalar-live-source"],
            ),
            Support::supported(
                Family::AsyncResource,
                [Lane::AsyncResourceState],
                [],
                ["bridge-async-source-binding"],
            ),
            Support::supported(
                Family::MixedCauseDelivery,
                [Lane::BridgeExternalState],
                [],
                ["bridge-owner-issued-revalidation"],
            ),
        ])
        .with_unsupported_batch_authority()
    }

    fn admit_live_view_declaration(
        &self,
        name: &str,
        request: &foundation::DeclarativeLiveQueryRequest,
        _schema_view: &runtime::QuerySchemaView,
    ) -> Result<runtime::LiveViewDeclarationAdmissionBoundaryReceipt, foundation::WorthQueryWorkspaceError>
    {
        use runtime::WorthQueryRuntimeSchemaAdapter;
        let adapter = ScalarSchemaAdapter;
        let receipt = adapter.build_live_view_declaration_admission_receipt(name, request);
        Ok(adapter.build_live_view_declaration_boundary_receipt(name, request, receipt))
    }

    fn declare_live_view(
        &mut self,
        name: String,
        request: foundation::DeclarativeLiveQueryRequest,
        _schema_view: runtime::QuerySchemaView,
    ) -> Result<foundation::WorthQueryLiveViewHandle, foundation::WorthQueryWorkspaceError> {
        let target =
            runtime::WorthQueryLiveArtifactTarget::from_source_adapter_declared_view_name(
                name.clone(),
            );
        self.state.borrow_mut().live_targets.insert(
            target,
            request.target_collection_identity().as_str().to_owned(),
        );
        Ok(foundation::WorthQueryLiveViewHandle::new(name))
    }

    fn close_live_view(
        &mut self,
        name: &str,
    ) -> Result<(), foundation::WorthQueryWorkspaceError> {
        let target =
            runtime::WorthQueryLiveArtifactTarget::from_source_adapter_declared_view_name(name);
        self.state.borrow_mut().live_targets.remove(&target);
        Ok(())
    }

    fn write(
        &mut self,
        _mutation: runtime::WorthQueryBackendAdmissibleMutation,
    ) -> Result<foundation::WorthQueryMutationReceipt, foundation::WorthQueryWorkspaceError> {
        Err(read_only_error("write"))
    }

    fn write_batch(
        &mut self,
        _mutations: Vec<runtime::WorthQueryBackendAdmissibleMutation>,
    ) -> Result<Vec<foundation::WorthQueryMutationReceipt>, foundation::WorthQueryWorkspaceError>
    {
        Err(read_only_error("write batch"))
    }

    fn execute_intent(
        &mut self,
        _declaration: &runtime::WorthQueryIntentDeclaration,
    ) -> Result<runtime::WorthQueryIntentExecution, runtime::WorthQueryRuntimeError> {
        Err(runtime::WorthQueryRuntimeError::MissingIntentAuthority)
    }

    fn live_entities_for_target(
        &self,
        target: &runtime::WorthQueryLiveArtifactTarget,
    ) -> Vec<foundation::WorthQueryEntity> {
        let state = self.state.borrow();
        if !state.live_targets.contains_key(target) {
            return Vec::new();
        }
        state
            .record
            .clone()
            .map(WorthUiScalarProjectionSourceRecord::into_query_entity)
            .into_iter()
            .collect()
    }

    fn drain_live_patches_for_target(
        &mut self,
        _target: &runtime::WorthQueryLiveArtifactTarget,
    ) -> Vec<foundation::WorthQueryLivePatch> {
        Vec::new()
    }

    fn affected_live_view_targets(
        &self,
        _receipt: &foundation::WorthQueryMutationReceipt,
    ) -> Vec<runtime::WorthQueryLiveArtifactTarget> {
        Vec::new()
    }

    fn install_live_subscription(
        &mut self,
        view_name: &str,
        activation: &runtime::SubscriptionActivationInput,
    ) -> Result<runtime::SubscriptionActivationReceipt, foundation::WorthQueryWorkspaceError> {
        use runtime::WorthQueryRuntimeSubscriptionActivationAdapter;
        let mut adapter = ScalarSubscriptionAdapter;
        Ok(adapter
            .admit_activation(view_name, activation)?
            .activation_receipt()
            .clone())
    }

    fn admit_preview_basis(
        &self,
        _label: &runtime::WorthQuerySessionLabel,
        _effect_policy: runtime::WorthQueryEffectPolicy,
        _authority: &runtime::WorthQueryRuntimeEvidenceAuthority,
    ) -> Result<runtime::WorthQueryPreviewBasisAdmission, foundation::WorthQueryWorkspaceError> {
        Err(read_only_error("preview"))
    }

    fn inspect_write_receipt(
        &self,
        _receipt: &runtime::WorthQueryWriteReceipt,
        _authority: &runtime::WorthQueryRuntimeEvidenceAuthority,
    ) -> Result<runtime::WorthQueryRuntimeInspectionEvidence, foundation::WorthQueryWorkspaceError>
    {
        Err(read_only_error("write receipt inspection"))
    }
}

struct ScalarSchemaAdapter;

impl runtime::WorthQueryRuntimeSchemaAdapter for ScalarSchemaAdapter {
    fn admit_live_view(
        &self,
        name: &str,
        request: &foundation::DeclarativeLiveQueryRequest,
        _schema_view: &runtime::QuerySchemaView,
    ) -> Result<runtime::LiveViewDeclarationAdmissionBoundaryReceipt, foundation::WorthQueryWorkspaceError>
    {
        let receipt = self.build_live_view_declaration_admission_receipt(name, request);
        Ok(self.build_live_view_declaration_boundary_receipt(name, request, receipt))
    }
}

struct ScalarSubscriptionAdapter;

impl runtime::WorthQueryRuntimeSubscriptionActivationAdapter for ScalarSubscriptionAdapter {
    fn support_evidence_identity(&self) -> runtime::WorthQueryEvidenceIdentity {
        runtime::runtime_subscription_support_evidence_identity(
            "worth-ui-external-scalar-subscription",
        )
    }

    fn admit_activation(
        &mut self,
        view_name: &str,
        activation: &runtime::SubscriptionActivationInput,
    ) -> Result<runtime::SubscriptionActivationBoundaryReceipt, foundation::WorthQueryWorkspaceError>
    {
        let receipt = self.build_subscription_activation_receipt(view_name, activation);
        Ok(self.build_subscription_activation_boundary_receipt(
            view_name,
            activation,
            receipt,
        ))
    }
}

fn read_only_error(operation: &str) -> foundation::WorthQueryWorkspaceError {
    foundation::WorthQueryWorkspaceError::new(format!(
        "Worth UI external scalar source is read-only and does not admit {operation}"
    ))
}
