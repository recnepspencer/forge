pub(super) fn reconcile_focus_installation(
    slot: &mut crate::runtime::UiRuntimeServiceInstallation<
        crate::runtime::focus::UiFocusRuntimeState,
    >,
    policy: Option<crate::declaration::UiFocusPolicy>,
) {
    let owner = if let Some(policy) = policy {
        let mut owner = slot.take().unwrap_or_else(|| {
            crate::runtime::focus::UiFocusRuntimeState::new_session_restore_candidate_with_policy(
                policy,
            )
        });
        owner.apply_policy(policy);
        Some(owner)
    } else {
        if let Some(mut owner) = slot.take() {
            let _released = owner.shutdown();
        }
        None
    };
    *slot = crate::runtime::UiRuntimeServiceInstallation::from_optional(owner);
}

pub(super) fn reconcile_portal_installation(
    slot: &mut crate::runtime::UiRuntimeServiceInstallation<
        crate::runtime::portal::UiPortalRuntimeState,
    >,
    policy: Option<crate::declaration::UiPortalPolicy>,
) {
    let owner = if let Some(policy) = policy {
        let mut owner = slot.take().unwrap_or_else(|| {
            crate::runtime::portal::UiPortalRuntimeState::new_with_policy(
                crate::runtime::UiServiceStatePersistencePosture::SessionRestoreCandidate,
                policy,
            )
        });
        owner.apply_policy(policy);
        Some(owner)
    } else {
        if let Some(mut owner) = slot.take() {
            debug_assert_eq!(owner.shutdown().final_active_records(), 0);
        }
        None
    };
    *slot = crate::runtime::UiRuntimeServiceInstallation::from_optional(owner);
}

pub(super) fn reconcile_motion_installation(
    slot: &mut crate::runtime::UiRuntimeServiceInstallation<
        crate::runtime::motion::UiMotionRuntimeState,
    >,
    policy: Option<crate::declaration::UiMotionPolicy>,
) {
    let owner = if let Some(policy) = policy {
        let mut owner = slot.take().unwrap_or_else(|| {
            crate::runtime::motion::UiMotionRuntimeState::new_with_policy(
                crate::runtime::UiServiceStatePersistencePosture::Ephemeral,
                policy,
            )
        });
        owner.apply_policy(policy);
        Some(owner)
    } else {
        if let Some(mut owner) = slot.take() {
            debug_assert!(owner.shutdown().final_census().is_zero());
        }
        None
    };
    *slot = crate::runtime::UiRuntimeServiceInstallation::from_optional(owner);
}
