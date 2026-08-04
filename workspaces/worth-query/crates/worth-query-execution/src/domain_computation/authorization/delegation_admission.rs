//! Bounded Query-owned capability delegation progression.

use std::collections::BTreeSet;

use worth_query_declaration::facade::application_capability::ApplicationCapabilityDelegationRule;
use worth_runtime_bridge::facade::BridgeAuthorizationRuntime;

use super::capability_observation::{
    observe_capability_policy, WorthQueryObservedCapabilityDecision,
};
use super::capability_registry::WorthQueryInstalledCapabilityPlan;
use super::decision_facts::WorthQueryDelegationDecisionFact;
use super::retained_capability_request::WorthQueryRetainedCapabilityRequest;
use super::{
    WorthQueryAuthorizationDecisionFact, WorthQueryAuthorizationTimeSample,
    WorthQueryOperationAuthorizationDenial, WorthQueryOperationAuthorizationDenialKind,
};

mod discovery;
mod transition;

struct DelegationFrame {
    child_policy: WorthQueryAuthorizationDecisionFact,
    grantor: worth_relational::facade::identity::EntityId,
    parent_grant: worth_relational::facade::identity::EntityId,
    discovery: worth_relational::facade::authorization::RelationalAuthorizationObservationEvidence,
    transition: worth_relational::facade::authorization::RelationalAuthorizationObservationEvidence,
}

pub(super) fn observe_capability(
    session_identity: crate::domain_computation::provider_session::WorthQueryGraphWorkSessionIdentity,
    relational: &worth_relational::facade::runtime::RelationalRuntime,
    snapshot: worth_relational::facade::snapshots::SnapshotHandle,
    bridge: &BridgeAuthorizationRuntime,
    installed: &WorthQueryInstalledCapabilityPlan,
    request: &WorthQueryRetainedCapabilityRequest,
    sample: &WorthQueryAuthorizationTimeSample,
    exact_grant: Option<worth_relational::facade::identity::EntityId>,
    expected: Option<&WorthQueryAuthorizationDecisionFact>,
) -> Result<WorthQueryObservedCapabilityDecision, WorthQueryOperationAuthorizationDenial> {
    let leaf = observe_capability_policy(
        session_identity,
        relational,
        snapshot.clone(),
        bridge,
        installed,
        request,
        sample,
        exact_grant,
    )?;
    let (leaf_policy, leaf_grant) = leaf.into_parts();
    let decision = observe_lineage(
        session_identity,
        relational,
        snapshot,
        bridge,
        installed,
        request,
        sample,
        leaf_grant,
        leaf_policy,
    )?;
    if expected.is_some_and(|expected| !expected.has_same_lineage(&decision)) {
        return Err(denial(
            WorthQueryOperationAuthorizationDenialKind::DelegationLineageChanged,
            installed.contract.name(),
        ));
    }
    Ok(WorthQueryObservedCapabilityDecision::new(
        decision, leaf_grant,
    ))
}

#[allow(clippy::too_many_arguments)]
fn observe_lineage(
    session_identity: crate::domain_computation::provider_session::WorthQueryGraphWorkSessionIdentity,
    relational: &worth_relational::facade::runtime::RelationalRuntime,
    snapshot: worth_relational::facade::snapshots::SnapshotHandle,
    bridge: &BridgeAuthorizationRuntime,
    installed: &WorthQueryInstalledCapabilityPlan,
    request: &WorthQueryRetainedCapabilityRequest,
    sample: &WorthQueryAuthorizationTimeSample,
    leaf_grant: worth_relational::facade::identity::EntityId,
    leaf_policy: WorthQueryAuthorizationDecisionFact,
) -> Result<WorthQueryAuthorizationDecisionFact, WorthQueryOperationAuthorizationDenial> {
    let mut visited = BTreeSet::from([leaf_grant]);
    let mut frames: Vec<DelegationFrame> = Vec::new();
    let mut current_grant = leaf_grant;
    let mut current_principal = request.principal;
    let mut current_policy = leaf_policy;
    loop {
        let observed = discovery::observe_parent(
            relational,
            snapshot.clone(),
            installed,
            current_grant,
            current_principal,
        )?;
        let Some(parent_grant) = observed.parent else {
            let mut decision = current_policy
                .with_delegation(WorthQueryDelegationDecisionFact::root(observed.evidence));
            while let Some(frame) = frames.pop() {
                decision = frame.child_policy.with_delegation(
                    WorthQueryDelegationDecisionFact::delegated(
                        frame.grantor,
                        frame.parent_grant,
                        frame.discovery,
                        frame.transition,
                        decision,
                    ),
                );
            }
            return Ok(decision);
        };
        if !visited.insert(parent_grant) {
            return Err(denial(
                WorthQueryOperationAuthorizationDenialKind::DelegationCycle,
                installed.contract.name(),
            ));
        }
        enforce_depth(
            installed.delegation.rule,
            frames.len() + 1,
            installed.contract.name(),
        )?;
        let transition = transition::observe_transition(
            relational,
            snapshot.clone(),
            installed,
            request,
            sample,
            current_grant,
            parent_grant,
        )?;
        let parent_request = request.for_principal(transition.grantor);
        let parent = observe_capability_policy(
            session_identity,
            relational,
            snapshot.clone(),
            bridge,
            installed,
            &parent_request,
            sample,
            Some(parent_grant),
        )?;
        let (parent_policy, observed_parent) = parent.into_parts();
        if observed_parent != parent_grant {
            return Err(denial(
                WorthQueryOperationAuthorizationDenialKind::DelegationRejected,
                installed.contract.name(),
            ));
        }
        frames.push(DelegationFrame {
            child_policy: current_policy,
            grantor: transition.grantor,
            parent_grant,
            discovery: observed.evidence,
            transition: transition.evidence,
        });
        current_grant = parent_grant;
        current_principal = transition.grantor;
        current_policy = parent_policy;
    }
}

fn enforce_depth(
    rule: ApplicationCapabilityDelegationRule,
    depth: usize,
    subject: &str,
) -> Result<(), WorthQueryOperationAuthorizationDenial> {
    let maximum = match rule {
        ApplicationCapabilityDelegationRule::Forbidden => {
            return Err(denial(
                WorthQueryOperationAuthorizationDenialKind::DelegationRejected,
                subject,
            ));
        }
        ApplicationCapabilityDelegationRule::NarrowAllDimensions { maximum_depth } => {
            maximum_depth.maximum() as usize
        }
    };
    if depth > maximum {
        return Err(denial(
            WorthQueryOperationAuthorizationDenialKind::DelegationDepthExceeded,
            subject,
        ));
    }
    Ok(())
}

fn denial(
    kind: WorthQueryOperationAuthorizationDenialKind,
    subject: impl Into<String>,
) -> WorthQueryOperationAuthorizationDenial {
    WorthQueryOperationAuthorizationDenial::new(kind, subject)
}
