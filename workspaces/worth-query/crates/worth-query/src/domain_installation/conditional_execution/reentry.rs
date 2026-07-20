use crate::basis_lifecycle::BasisOperationLane;
use crate::domain_installation::operation_authority_chain::{
    mint_operation_phase_proof, operation_phase_basis, WorthQueryConditionalReentryPhase,
    WorthQueryOperationPhaseProof,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WorthQueryConditionalOutcomeClass {
    ComputedChanged,
    ComputedRevertedClean,
    DependencyUnchanged,
    Suppressed,
    DeferredByCondition,
    DeferredTemporal,
    DeferredOnDemand,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WorthQueryConditionalAdmissionDenial {
    ForeignOperation,
    ForeignRuntime,
    StaleInstallation,
    LoweringMismatch,
    SignalGraphMismatch,
    SignalNodeMismatch,
    SnapshotMismatch,
    AttemptMismatch,
    BoundCapabilityMismatch,
    AuthorityContinuity,
    SignalContractMismatch,
}

/// Query-owned retained provenance. It carries the opaque Bridge/Signal
/// evidence; the derived class is inspection-only and grants no authority.
pub struct WorthQueryConditionalProvenance {
    pub(crate) location: worth_query_installation::facade::WorthQueryConditionalNodeLocation,
    pub(crate) bridge: worth_runtime_bridge::facade::BridgeConditionalDecisionEvidence,
    pub(crate) _lowering:
        std::sync::Arc<worth_runtime_bridge::facade::BridgeInstalledConditionalLowering>,
    pub(crate) class: WorthQueryConditionalOutcomeClass,
    pub(crate) _admission: WorthQueryOperationPhaseProof<WorthQueryConditionalReentryPhase>,
}

pub struct WorthQueryConditionalSemanticObservation<'a> {
    bridge: &'a worth_runtime_bridge::facade::BridgeConditionalSemanticObservation,
}

pub(crate) struct WorthQueryConditionalAuthorityAdmission {
    lowering: std::sync::Arc<worth_runtime_bridge::facade::BridgeInstalledConditionalLowering>,
    binding_identity: std::sync::Arc<str>,
    capability_identity: u64,
}

impl WorthQueryConditionalSemanticObservation<'_> {
    pub const fn dependency_ordinal(&self) -> usize {
        self.bridge.dependency_ordinal()
    }

    pub fn previous(&self) -> Option<&worth_foundational::facade::ContractValidatedAspectArtifact> {
        self.bridge.previous()
    }

    pub fn current(&self) -> &worth_foundational::facade::ContractValidatedAspectArtifact {
        self.bridge.current()
    }
}

impl std::fmt::Debug for WorthQueryConditionalProvenance {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("WorthQueryConditionalProvenance")
            .field("location", &self.location)
            .field("class", &self.class)
            .field("signal_identity", &self.bridge.signal().identity())
            .finish()
    }
}

impl WorthQueryConditionalProvenance {
    pub fn location(&self) -> &worth_query_installation::facade::WorthQueryConditionalNodeLocation {
        &self.location
    }
    pub const fn class(&self) -> WorthQueryConditionalOutcomeClass {
        self.class
    }
    pub fn signal_identity(&self) -> &str {
        self.bridge.signal().identity()
    }
    pub fn artifact_reuse_admitted(&self) -> bool {
        self.bridge.signal().artifact_reuse_admitted()
    }
    pub fn declaration(
        &self,
    ) -> &worth_query_installation::facade::WorthQueryPortableConditionalNodeDeclaration {
        self._lowering.declaration()
    }
    pub fn condition(
        &self,
    ) -> &worth_query_installation::facade::WorthQueryConditionalEvaluationCondition {
        self.declaration().condition()
    }
    pub fn semantic_observation_count(&self) -> usize {
        self.bridge.semantic_observations().len()
    }
    pub fn semantic_observation(
        &self,
        dependency_ordinal: usize,
    ) -> Option<WorthQueryConditionalSemanticObservation<'_>> {
        self.bridge
            .semantic_observations()
            .iter()
            .find(|observation| observation.dependency_ordinal() == dependency_ordinal)
            .map(|bridge| WorthQueryConditionalSemanticObservation { bridge })
    }
}

pub struct WorthQueryDeferredDomainOperation<D, O, F, L: BasisOperationLane> {
    pub(crate) bound: super::super::WorthQueryBoundDomainOperation<D, O, F, L>,
    pub(crate) conditional: Vec<WorthQueryConditionalProvenance>,
    pub(crate) counters: super::super::WorthQueryOperationExecutionCounters,
}

impl<D, O, F, L: BasisOperationLane> std::fmt::Debug
    for WorthQueryDeferredDomainOperation<D, O, F, L>
{
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("WorthQueryDeferredDomainOperation")
            .field("binding_identity", &self.bound.binding_identity())
            .field("conditional", &self.conditional)
            .field("counters", &self.counters)
            .finish()
    }
}

pub struct WorthQueryDeferredWorkflowStage<D, O, F, L: BasisOperationLane> {
    pub(crate) run: super::super::WorthQueryWorkflowRun<D, O, F, L>,
    pub(crate) conditional: Vec<WorthQueryConditionalProvenance>,
}

impl<D, O, F, L: BasisOperationLane> std::fmt::Debug
    for WorthQueryDeferredWorkflowStage<D, O, F, L>
{
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("WorthQueryDeferredWorkflowStage")
            .field("run_identity", &self.run.identity())
            .field("conditional", &self.conditional)
            .finish()
    }
}

impl<D, O, F, L: BasisOperationLane> WorthQueryDeferredWorkflowStage<D, O, F, L> {
    pub fn run_identity(&self) -> &str {
        self.run.identity()
    }
    pub fn conditional_provenance(&self) -> &[WorthQueryConditionalProvenance] {
        &self.conditional
    }
    pub fn completed_stage_count(&self) -> usize {
        self.run.receipts().len()
    }
}

impl<D, O, F, L: BasisOperationLane> WorthQueryDeferredDomainOperation<D, O, F, L> {
    pub fn conditional_provenance(&self) -> &[WorthQueryConditionalProvenance] {
        &self.conditional
    }
    pub fn binding_identity(&self) -> &str {
        self.bound.binding_identity()
    }
    pub fn counters(&self) -> super::super::WorthQueryOperationExecutionCounters {
        self.counters
    }
}

pub(crate) fn admit_conditional_decision<D, O, F, L: BasisOperationLane>(
    bound: &super::super::WorthQueryBoundDomainOperation<D, O, F, L>,
    authority: WorthQueryConditionalAuthorityAdmission,
    bridge: worth_runtime_bridge::facade::BridgeConditionalDecisionEvidence,
    snapshot_identity: &str,
    bridge_snapshot_identity: Option<&worth_runtime_bridge::facade::TruthSnapshotIdentity>,
    execution_identity: &str,
    attempt: u64,
) -> Result<WorthQueryConditionalProvenance, WorthQueryConditionalAdmissionDenial> {
    if !bridge.retains_exact_lowering(&authority.lowering) {
        return Err(WorthQueryConditionalAdmissionDenial::LoweringMismatch);
    }
    if bridge.query_binding_identity() != authority.binding_identity.as_ref()
        || bridge.query_capability_identity() != authority.capability_identity
    {
        return Err(WorthQueryConditionalAdmissionDenial::BoundCapabilityMismatch);
    }
    if bridge.bridge_snapshot_identity() != bridge_snapshot_identity {
        return Err(WorthQueryConditionalAdmissionDenial::SnapshotMismatch);
    }
    let signal = bridge.signal();
    authority
        .lowering
        .validate_signal_decision_contract(signal)
        .map_err(|_| WorthQueryConditionalAdmissionDenial::SignalContractMismatch)?;
    if signal.graph_instance_id() != authority.lowering.signal_graph_instance_id() {
        return Err(WorthQueryConditionalAdmissionDenial::SignalGraphMismatch);
    }
    if signal.node() != authority.lowering.signal_node() {
        return Err(WorthQueryConditionalAdmissionDenial::SignalNodeMismatch);
    }
    if signal.snapshot_identity() != snapshot_identity {
        return Err(WorthQueryConditionalAdmissionDenial::SnapshotMismatch);
    }
    if signal.execution_identity() != execution_identity {
        return Err(WorthQueryConditionalAdmissionDenial::AttemptMismatch);
    }
    if signal.attempt() != attempt {
        return Err(WorthQueryConditionalAdmissionDenial::AttemptMismatch);
    }
    let class = classify_signal_decision(signal.class());
    let admission = mint_operation_phase_proof(
        signal.identity(),
        Some(bound.authority_proof().payload().identity()),
        operation_phase_basis(bound.authority_proof()).clone(),
    );
    Ok(WorthQueryConditionalProvenance {
        location: authority.lowering.location().clone(),
        _lowering: authority.lowering,
        bridge,
        class,
        _admission: admission,
    })
}

pub(crate) fn admit_conditional_authority<D, O, F, L: BasisOperationLane>(
    bound: &super::super::WorthQueryBoundDomainOperation<D, O, F, L>,
    node: &super::WorthQueryInstalledConditionalNode,
) -> Result<WorthQueryConditionalAuthorityAdmission, WorthQueryConditionalAdmissionDenial> {
    if node.operation_identity != bound.definition().canonical_identity() {
        return Err(WorthQueryConditionalAdmissionDenial::ForeignOperation);
    }
    if node.runtime_authority
        != bound
            .operation()
            .domain_authority()
            .runtime_authority()
            .as_u64()
    {
        return Err(WorthQueryConditionalAdmissionDenial::ForeignRuntime);
    }
    if node.installation_generation != bound.operation().installation_generation().ordinal()
        || !bound.installation_is_current()
    {
        return Err(WorthQueryConditionalAdmissionDenial::StaleInstallation);
    }
    let authority_basis = operation_phase_basis(bound.authority_proof());
    let graph_authorities = bound.conditional_graph_authorities();
    node.lowering
        .validate_query_authority_continuity(
            &authority_basis.operation_identity,
            authority_basis.runtime_authority,
            authority_basis.installation_generation,
            &graph_authorities,
        )
        .map_err(|_| WorthQueryConditionalAdmissionDenial::AuthorityContinuity)?;
    Ok(WorthQueryConditionalAuthorityAdmission {
        lowering: std::sync::Arc::clone(&node.lowering),
        binding_identity: bound.binding_identity().into(),
        capability_identity: bound.capability_identity(),
    })
}

fn classify_signal_decision(
    class: worth_signal::facade::SignalConditionalDecisionClass,
) -> WorthQueryConditionalOutcomeClass {
    match class {
        worth_signal::facade::SignalConditionalDecisionClass::ComputedChanged => {
            WorthQueryConditionalOutcomeClass::ComputedChanged
        }
        worth_signal::facade::SignalConditionalDecisionClass::ComputedRevertedClean => {
            WorthQueryConditionalOutcomeClass::ComputedRevertedClean
        }
        worth_signal::facade::SignalConditionalDecisionClass::DependencyUnchanged => {
            WorthQueryConditionalOutcomeClass::DependencyUnchanged
        }
        worth_signal::facade::SignalConditionalDecisionClass::SuppressedBeforeCompute => {
            WorthQueryConditionalOutcomeClass::Suppressed
        }
        worth_signal::facade::SignalConditionalDecisionClass::DeferredByCondition => {
            WorthQueryConditionalOutcomeClass::DeferredByCondition
        }
        worth_signal::facade::SignalConditionalDecisionClass::DeferredTemporal => {
            WorthQueryConditionalOutcomeClass::DeferredTemporal
        }
        worth_signal::facade::SignalConditionalDecisionClass::DeferredOnDemand => {
            WorthQueryConditionalOutcomeClass::DeferredOnDemand
        }
    }
}
