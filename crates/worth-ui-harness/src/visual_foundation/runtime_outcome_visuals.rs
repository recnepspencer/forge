use forge_query::facade::ForgeQueryRuntimeAsyncResultStateKind;
use worth_ui::facade::{
    IconId, RuntimeOutcomeAffordance, RuntimeOutcomeDenialPosture, RuntimeOutcomeFamily,
    RuntimeOutcomePresentation, RuntimeOutcomeProjectionDescriptor, RuntimeOutcomeProjectionId,
    RuntimeOutcomeRecoveryPosture, RuntimeOutcomeSourceReference, RuntimeOutcomeTone,
};

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum HarnessRuntimeOutcomeVisualRole {
    Active,
    Success,
    Warning,
    Danger,
    Disabled,
}

impl HarnessRuntimeOutcomeVisualRole {
    pub const REQUIRED: [Self; 5] = [
        Self::Active,
        Self::Success,
        Self::Warning,
        Self::Danger,
        Self::Disabled,
    ];
}

pub(crate) fn harness_runtime_outcome_projections() -> Vec<RuntimeOutcomeProjectionDescriptor> {
    HarnessRuntimeOutcomeVisualRole::REQUIRED
        .into_iter()
        .map(runtime_outcome_projection)
        .collect()
}

fn runtime_outcome_projection(
    role: HarnessRuntimeOutcomeVisualRole,
) -> RuntimeOutcomeProjectionDescriptor {
    let descriptor = RuntimeOutcomeProjectionDescriptor::new(
        projection_id(role),
        family(role),
        RuntimeOutcomeSourceReference::from_query_async_result_state_kind(query_kind(role)),
    )
    .with_presentation(presentation(role));
    match role {
        HarnessRuntimeOutcomeVisualRole::Warning | HarnessRuntimeOutcomeVisualRole::Danger => {
            descriptor.with_recovery_posture(RuntimeOutcomeRecoveryPosture::retry_hint())
        }
        HarnessRuntimeOutcomeVisualRole::Disabled => {
            descriptor.with_denial_posture(RuntimeOutcomeDenialPosture::structured_status())
        }
        _ => descriptor,
    }
}

fn projection_id(role: HarnessRuntimeOutcomeVisualRole) -> RuntimeOutcomeProjectionId {
    RuntimeOutcomeProjectionId::new(match role {
        HarnessRuntimeOutcomeVisualRole::Active => "harness.runtime_outcome.active",
        HarnessRuntimeOutcomeVisualRole::Success => "harness.runtime_outcome.success",
        HarnessRuntimeOutcomeVisualRole::Warning => "harness.runtime_outcome.warning",
        HarnessRuntimeOutcomeVisualRole::Danger => "harness.runtime_outcome.danger",
        HarnessRuntimeOutcomeVisualRole::Disabled => "harness.runtime_outcome.disabled",
    })
    .expect("valid harness runtime outcome projection id")
}

fn family(role: HarnessRuntimeOutcomeVisualRole) -> RuntimeOutcomeFamily {
    match role {
        HarnessRuntimeOutcomeVisualRole::Active => RuntimeOutcomeFamily::loading(),
        HarnessRuntimeOutcomeVisualRole::Success => RuntimeOutcomeFamily::ready(),
        HarnessRuntimeOutcomeVisualRole::Warning => RuntimeOutcomeFamily::stale(),
        HarnessRuntimeOutcomeVisualRole::Danger => RuntimeOutcomeFamily::failed(),
        HarnessRuntimeOutcomeVisualRole::Disabled => RuntimeOutcomeFamily::denied(),
    }
}

fn query_kind(role: HarnessRuntimeOutcomeVisualRole) -> ForgeQueryRuntimeAsyncResultStateKind {
    match role {
        HarnessRuntimeOutcomeVisualRole::Active => ForgeQueryRuntimeAsyncResultStateKind::Pending,
        HarnessRuntimeOutcomeVisualRole::Success => ForgeQueryRuntimeAsyncResultStateKind::Current,
        HarnessRuntimeOutcomeVisualRole::Warning => ForgeQueryRuntimeAsyncResultStateKind::Stale,
        HarnessRuntimeOutcomeVisualRole::Danger => ForgeQueryRuntimeAsyncResultStateKind::Failed,
        HarnessRuntimeOutcomeVisualRole::Disabled => ForgeQueryRuntimeAsyncResultStateKind::Denied,
    }
}

fn presentation(role: HarnessRuntimeOutcomeVisualRole) -> RuntimeOutcomePresentation {
    RuntimeOutcomePresentation::new()
        .with_label(label(role))
        .with_icon(icon_id(role))
        .with_tone(tone(role))
        .with_affordance(affordance(role))
}

fn label(role: HarnessRuntimeOutcomeVisualRole) -> &'static str {
    match role {
        HarnessRuntimeOutcomeVisualRole::Active => "Active",
        HarnessRuntimeOutcomeVisualRole::Success => "Ready",
        HarnessRuntimeOutcomeVisualRole::Warning => "Check",
        HarnessRuntimeOutcomeVisualRole::Danger => "Failed",
        HarnessRuntimeOutcomeVisualRole::Disabled => "Denied",
    }
}

fn tone(role: HarnessRuntimeOutcomeVisualRole) -> RuntimeOutcomeTone {
    match role {
        HarnessRuntimeOutcomeVisualRole::Active => RuntimeOutcomeTone::progress(),
        HarnessRuntimeOutcomeVisualRole::Success => RuntimeOutcomeTone::positive(),
        HarnessRuntimeOutcomeVisualRole::Warning => RuntimeOutcomeTone::advisory(),
        HarnessRuntimeOutcomeVisualRole::Danger => RuntimeOutcomeTone::destructive(),
        HarnessRuntimeOutcomeVisualRole::Disabled => RuntimeOutcomeTone::blocking(),
    }
}

fn affordance(role: HarnessRuntimeOutcomeVisualRole) -> RuntimeOutcomeAffordance {
    match role {
        HarnessRuntimeOutcomeVisualRole::Active => RuntimeOutcomeAffordance::wait(),
        HarnessRuntimeOutcomeVisualRole::Success => RuntimeOutcomeAffordance::none(),
        HarnessRuntimeOutcomeVisualRole::Warning => RuntimeOutcomeAffordance::inspect(),
        HarnessRuntimeOutcomeVisualRole::Danger => RuntimeOutcomeAffordance::retry(),
        HarnessRuntimeOutcomeVisualRole::Disabled => RuntimeOutcomeAffordance::inspect(),
    }
}

fn icon_id(role: HarnessRuntimeOutcomeVisualRole) -> IconId {
    IconId::new(match role {
        HarnessRuntimeOutcomeVisualRole::Active => "harness.icon.runtime.active",
        HarnessRuntimeOutcomeVisualRole::Success => "harness.icon.runtime.success",
        HarnessRuntimeOutcomeVisualRole::Warning => "harness.icon.runtime.warning",
        HarnessRuntimeOutcomeVisualRole::Danger => "harness.icon.runtime.danger",
        HarnessRuntimeOutcomeVisualRole::Disabled => "harness.icon.runtime.disabled",
    })
    .expect("valid harness runtime icon id")
}
