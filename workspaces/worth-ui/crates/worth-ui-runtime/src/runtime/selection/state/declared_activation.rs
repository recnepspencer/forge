impl super::UiSelectionRuntimeState {
    pub(crate) fn request_for_declared_activation(
        &self,
        owner: crate::runtime::selection::UiSelectionOwnerIdentity,
        incarnation: crate::runtime::selection::UiSelectionOwnerIncarnation,
        key: crate::runtime::selection::UiSelectionStableKey,
        replacement: Option<&crate::runtime::selection::UiSelectionRegistration>,
    ) -> Result<
        crate::runtime::selection::UiSelectionRequest,
        crate::runtime::selection::UiSelectionRequestDenial,
    > {
        use crate::runtime::selection::{
            UiSelectionPolicy, UiSelectionRequest, UiSelectionRequestDenial,
        };

        let (policy, has_anchor, selected) = match self.owners.get(&owner) {
            Some(record) if record.incarnation == incarnation => (
                record.policy,
                record.anchor.is_some(),
                record.selected.contains(&key),
            ),
            Some(_) if replacement.is_none() => {
                return Err(UiSelectionRequestDenial::StaleOwnerIncarnation);
            }
            _ => {
                let registration = replacement.ok_or(UiSelectionRequestDenial::UnknownOwner)?;
                (registration.policy(), false, false)
            }
        };

        Ok(match policy {
            UiSelectionPolicy::Single => UiSelectionRequest::SelectSingle(key),
            UiSelectionPolicy::Multiple if selected => UiSelectionRequest::Remove(key),
            UiSelectionPolicy::Multiple => UiSelectionRequest::Add(key),
            UiSelectionPolicy::MultipleWithRange if has_anchor => UiSelectionRequest::SelectRange {
                target: key,
                extend: false,
            },
            UiSelectionPolicy::MultipleWithRange => UiSelectionRequest::SelectSingle(key),
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::runtime::selection::state_test_fixture::{incarnation, key, owner, registration};
    use crate::runtime::selection::{
        UiSelectionCatalogPosture, UiSelectionPolicy, UiSelectionRequest, UiSelectionRuntimeState,
    };

    #[test]
    fn declared_multiple_activation_adds_then_removes_the_same_item() {
        let owner = owner();
        let target = key(81);
        let mut state = UiSelectionRuntimeState::new_session_restore_candidate();
        state
            .synchronize(registration(
                owner,
                UiSelectionPolicy::Multiple,
                vec![target],
                UiSelectionCatalogPosture::Complete,
            ))
            .unwrap();

        let add = state
            .request_for_declared_activation(owner, incarnation(), target, None)
            .unwrap();
        assert_eq!(add, UiSelectionRequest::Add(target));
        state.apply(owner, incarnation(), add).unwrap();

        assert_eq!(
            state
                .request_for_declared_activation(owner, incarnation(), target, None)
                .unwrap(),
            UiSelectionRequest::Remove(target)
        );
    }

    #[test]
    fn declared_range_activation_uses_the_owner_held_anchor() {
        let owner = owner();
        let first = key(91);
        let target = key(93);
        let mut state = UiSelectionRuntimeState::new_session_restore_candidate();
        state
            .synchronize(registration(
                owner,
                UiSelectionPolicy::MultipleWithRange,
                vec![first, key(92), target],
                UiSelectionCatalogPosture::Complete,
            ))
            .unwrap();

        let anchor = state
            .request_for_declared_activation(owner, incarnation(), first, None)
            .unwrap();
        assert_eq!(anchor, UiSelectionRequest::SelectSingle(first));
        state.apply(owner, incarnation(), anchor).unwrap();

        assert_eq!(
            state
                .request_for_declared_activation(owner, incarnation(), target, None)
                .unwrap(),
            UiSelectionRequest::SelectRange {
                target,
                extend: false,
            }
        );
    }
}
