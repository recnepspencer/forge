use std::sync::Arc;

use crate::basis_lifecycle::BasisOperationLane;

use super::super::{
    WorthQueryBoundDomainOperation, WorthQueryConsumerSupportDimension,
    WorthQueryConsumerSupportPosture,
};
use super::denial::{
    WorthQueryCompatibilityCounters, WorthQueryCompatibilityDenial,
    WorthQueryCompatibilityDenialKind,
};

pub(super) fn require_execution_sharing<D, O, F, L: BasisOperationLane>(
    subject: &WorthQueryBoundDomainOperation<D, O, F, L>,
    candidate: &WorthQueryBoundDomainOperation<D, O, F, L>,
    counters: &mut WorthQueryCompatibilityCounters,
) -> Result<(), WorthQueryCompatibilityDenial> {
    let sharing_supported = [subject, candidate].into_iter().all(|bound| {
        counters.retained_authority_checks += 1;
        bound
            .consumer_support_profile()
            .posture(WorthQueryConsumerSupportDimension::Sharing)
            == WorthQueryConsumerSupportPosture::Supported
    });
    if !sharing_supported {
        return Err(sharing_denial(*counters));
    }

    let providers_match = same_arc_counted(subject.executor(), candidate.executor(), counters)
        && same_arc_counted(
            subject.workflow_executor(),
            candidate.workflow_executor(),
            counters,
        )
        && same_arc_counted(
            subject.workflow_parallel_admission_provider(),
            candidate.workflow_parallel_admission_provider(),
            counters,
        );
    if !providers_match {
        return Err(sharing_denial(*counters));
    }

    let artifacts_shareable = subject.conditional_nodes().iter().all(|node| {
        counters.retained_authority_checks += 1;
        use worth_query_installation::facade::{
            WorthQueryArtifactPosture, WorthQueryArtifactReuseEquivalence,
        };
        node.lowering.declaration().artifact() == WorthQueryArtifactPosture::ReusableWhenEquivalent
            && !matches!(
                node.lowering.declaration().artifact_reuse_equivalence(),
                WorthQueryArtifactReuseEquivalence::NotReusable
            )
    });

    if artifacts_shareable {
        Ok(())
    } else {
        Err(sharing_denial(*counters))
    }
}

fn sharing_denial(counters: WorthQueryCompatibilityCounters) -> WorthQueryCompatibilityDenial {
    WorthQueryCompatibilityDenial::plain(
        WorthQueryCompatibilityDenialKind::RelationshipRule,
        "execution sharing lacks support, exact providers, or reusable artifacts",
        counters,
    )
}

fn same_arc<T: ?Sized>(left: Option<&Arc<T>>, right: Option<&Arc<T>>) -> bool {
    match (left, right) {
        (Some(left), Some(right)) => Arc::ptr_eq(left, right),
        (None, None) => true,
        _ => false,
    }
}

fn same_arc_counted<T: ?Sized>(
    left: Option<&Arc<T>>,
    right: Option<&Arc<T>>,
    counters: &mut WorthQueryCompatibilityCounters,
) -> bool {
    counters.retained_authority_checks += 1;
    same_arc(left, right)
}
