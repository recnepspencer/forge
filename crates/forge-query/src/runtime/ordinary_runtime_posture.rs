use crate::ordinary_outcome::{
    ForgeQueryOrdinaryRuntimeAsyncPostureKind, ForgeQueryOrdinaryRuntimeBasisPostureKind,
    ForgeQueryOrdinaryRuntimeCausePostureKind, ForgeQueryOrdinaryRuntimePosture,
    ForgeQueryOrdinaryRuntimePostureKind, ForgeQueryOrdinaryRuntimeRemaskPostureKind,
};
use crate::subscription::QuerySubscriptionDeliveryCauseKind;

use super::{
    ForgeQueryRuntimeAsyncResultStateKind, ForgeQueryRuntimeDeliveryCoalescingKind,
    ForgeQueryRuntimeLiveSubscriptionState,
};

pub(in crate::runtime) fn project_live_subscription_ordinary_runtime_posture(
    state: &ForgeQueryRuntimeLiveSubscriptionState,
) -> ForgeQueryOrdinaryRuntimePosture {
    let cause_posture = state
        .last_delivery
        .as_ref()
        .map(|delivery| {
            let mixed_cause_delivery = delivery.mixed_cause_delivery();
            if mixed_cause_delivery.coalescing_kind()
                == ForgeQueryRuntimeDeliveryCoalescingKind::Coalesced
                || delivery.delivery_cause_kind() == QuerySubscriptionDeliveryCauseKind::MixedCause
            {
                ForgeQueryOrdinaryRuntimeCausePostureKind::MixedCause
            } else if matches!(
                delivery.delivery_cause_kind(),
                QuerySubscriptionDeliveryCauseKind::FreshnessOnly
                    | QuerySubscriptionDeliveryCauseKind::WindowEntry
                    | QuerySubscriptionDeliveryCauseKind::WindowExit
                    | QuerySubscriptionDeliveryCauseKind::Deadline
                    | QuerySubscriptionDeliveryCauseKind::PreviousValueTransition
            ) {
                ForgeQueryOrdinaryRuntimeCausePostureKind::TimeOnly
            } else {
                ForgeQueryOrdinaryRuntimeCausePostureKind::Ordinary
            }
        })
        .unwrap_or(ForgeQueryOrdinaryRuntimeCausePostureKind::Ordinary);
    let async_posture = state
        .async_result_state
        .as_ref()
        .map(|state| ordinary_async_posture_kind(state.kind()));
    let remask_posture = state
        .remask_posture
        .as_ref()
        .map(|posture| ordinary_remask_posture_kind(posture.reason_kind()));
    let basis_posture = state
        .async_result_state
        .as_ref()
        .map(|async_result_state| {
            if async_result_state.basis_digest() != state.installation.basis_binding_digest() {
                ForgeQueryOrdinaryRuntimeBasisPostureKind::BasisDrift
            } else if async_result_state.generation_digest()
                != state.active_lane_handle.checkpoint_identity_digest()
            {
                ForgeQueryOrdinaryRuntimeBasisPostureKind::GenerationDrift
            } else {
                ForgeQueryOrdinaryRuntimeBasisPostureKind::Stable
            }
        })
        .unwrap_or(ForgeQueryOrdinaryRuntimeBasisPostureKind::Stable);
    let kind = match state
        .remask_posture
        .as_ref()
        .map(|posture| posture.disposition_kind())
    {
        Some(super::ForgeQueryRuntimeRemaskDispositionKind::Remasked) => {
            ForgeQueryOrdinaryRuntimePostureKind::Remasked
        }
        Some(super::ForgeQueryRuntimeRemaskDispositionKind::Denied) => {
            ForgeQueryOrdinaryRuntimePostureKind::Denied
        }
        None => match async_posture {
            Some(ForgeQueryOrdinaryRuntimeAsyncPostureKind::Pending) => {
                ForgeQueryOrdinaryRuntimePostureKind::Pending
            }
            Some(ForgeQueryOrdinaryRuntimeAsyncPostureKind::Current) => {
                ForgeQueryOrdinaryRuntimePostureKind::Current
            }
            Some(ForgeQueryOrdinaryRuntimeAsyncPostureKind::Failed) => {
                ForgeQueryOrdinaryRuntimePostureKind::Failed
            }
            Some(ForgeQueryOrdinaryRuntimeAsyncPostureKind::Stale) => {
                ForgeQueryOrdinaryRuntimePostureKind::Stale
            }
            Some(ForgeQueryOrdinaryRuntimeAsyncPostureKind::Cancelled) => {
                ForgeQueryOrdinaryRuntimePostureKind::Cancelled
            }
            Some(ForgeQueryOrdinaryRuntimeAsyncPostureKind::Retried) => {
                ForgeQueryOrdinaryRuntimePostureKind::Retried
            }
            Some(ForgeQueryOrdinaryRuntimeAsyncPostureKind::Revalidating) => {
                ForgeQueryOrdinaryRuntimePostureKind::Revalidating
            }
            Some(ForgeQueryOrdinaryRuntimeAsyncPostureKind::Superseded) => {
                ForgeQueryOrdinaryRuntimePostureKind::Superseded
            }
            Some(ForgeQueryOrdinaryRuntimeAsyncPostureKind::Denied) => {
                ForgeQueryOrdinaryRuntimePostureKind::Denied
            }
            None => ForgeQueryOrdinaryRuntimePostureKind::Current,
        },
    };
    ForgeQueryOrdinaryRuntimePosture::new(
        kind,
        cause_posture,
        async_posture,
        basis_posture,
        remask_posture,
        state.installation.support_evidence(),
    )
}

fn ordinary_async_posture_kind(
    kind: ForgeQueryRuntimeAsyncResultStateKind,
) -> ForgeQueryOrdinaryRuntimeAsyncPostureKind {
    match kind {
        ForgeQueryRuntimeAsyncResultStateKind::Pending => {
            ForgeQueryOrdinaryRuntimeAsyncPostureKind::Pending
        }
        ForgeQueryRuntimeAsyncResultStateKind::Current => {
            ForgeQueryOrdinaryRuntimeAsyncPostureKind::Current
        }
        ForgeQueryRuntimeAsyncResultStateKind::Failed => {
            ForgeQueryOrdinaryRuntimeAsyncPostureKind::Failed
        }
        ForgeQueryRuntimeAsyncResultStateKind::Stale => {
            ForgeQueryOrdinaryRuntimeAsyncPostureKind::Stale
        }
        ForgeQueryRuntimeAsyncResultStateKind::Cancelled => {
            ForgeQueryOrdinaryRuntimeAsyncPostureKind::Cancelled
        }
        ForgeQueryRuntimeAsyncResultStateKind::Retried => {
            ForgeQueryOrdinaryRuntimeAsyncPostureKind::Retried
        }
        ForgeQueryRuntimeAsyncResultStateKind::Revalidating => {
            ForgeQueryOrdinaryRuntimeAsyncPostureKind::Revalidating
        }
        ForgeQueryRuntimeAsyncResultStateKind::Superseded => {
            ForgeQueryOrdinaryRuntimeAsyncPostureKind::Superseded
        }
        ForgeQueryRuntimeAsyncResultStateKind::Denied => {
            ForgeQueryOrdinaryRuntimeAsyncPostureKind::Denied
        }
    }
}

fn ordinary_remask_posture_kind(
    kind: super::ForgeQueryRuntimeRemaskReasonKind,
) -> ForgeQueryOrdinaryRuntimeRemaskPostureKind {
    match kind {
        super::ForgeQueryRuntimeRemaskReasonKind::PolicyDrift => {
            ForgeQueryOrdinaryRuntimeRemaskPostureKind::PolicyDrift
        }
        super::ForgeQueryRuntimeRemaskReasonKind::TenantDrift => {
            ForgeQueryOrdinaryRuntimeRemaskPostureKind::TenantDrift
        }
        super::ForgeQueryRuntimeRemaskReasonKind::RelationshipProofDrift => {
            ForgeQueryOrdinaryRuntimeRemaskPostureKind::RelationshipProofDrift
        }
        super::ForgeQueryRuntimeRemaskReasonKind::SchemaContextDrift => {
            ForgeQueryOrdinaryRuntimeRemaskPostureKind::SchemaContextDrift
        }
    }
}
