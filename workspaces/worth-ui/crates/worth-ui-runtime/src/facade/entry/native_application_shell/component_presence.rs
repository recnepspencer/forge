use std::collections::HashSet;

use super::WorthUiNativeApplicationShell;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum UiNativeComponentPresenceDenial {
    DuplicateComponent,
    UnknownComponent,
    AlreadyInRequestedState,
    RuntimeTransition,
    RollbackIndeterminate,
}

impl WorthUiNativeApplicationShell {
    pub(crate) fn apply_component_presence(
        &mut self,
        changes: &[crate::facade::entry::UiNativeComponentPresenceChange],
    ) -> Result<(), UiNativeComponentPresenceDenial> {
        let indices = self.validate_component_presence(changes)?;
        let mut applied: Vec<(usize, bool)> = Vec::with_capacity(indices.len());
        for (index, change) in indices.iter().copied().zip(changes) {
            if self
                .set_component_presence(index, change.present())
                .is_err()
            {
                for (rollback_index, rollback_change) in applied.into_iter().rev() {
                    if self
                        .set_component_presence(rollback_index, !rollback_change)
                        .is_err()
                    {
                        return Err(UiNativeComponentPresenceDenial::RollbackIndeterminate);
                    }
                }
                return Err(UiNativeComponentPresenceDenial::RuntimeTransition);
            }
            applied.push((index, change.present()));
        }
        Ok(())
    }

    fn validate_component_presence(
        &self,
        changes: &[crate::facade::entry::UiNativeComponentPresenceChange],
    ) -> Result<Vec<usize>, UiNativeComponentPresenceDenial> {
        let mut seen = HashSet::with_capacity(changes.len());
        changes
            .iter()
            .map(|change| {
                if !seen.insert(change.authored_semantic_identity()) {
                    return Err(UiNativeComponentPresenceDenial::DuplicateComponent);
                }
                let index = self
                    .mounted_row_indices
                    .get(change.authored_semantic_identity())
                    .copied()
                    .ok_or(UiNativeComponentPresenceDenial::UnknownComponent)?;
                if self.mounted_rows[index].mounted.is_some() == change.present() {
                    return Err(UiNativeComponentPresenceDenial::AlreadyInRequestedState);
                }
                Ok(index)
            })
            .collect()
    }

    fn set_component_presence(&mut self, index: usize, present: bool) -> Result<(), ()> {
        let row = self.mounted_rows.get_mut(index).ok_or(())?;
        if present {
            let handle = self
                .session
                .mounted_graph_node(row.graph_node)
                .map_err(|_| ())?;
            row.mounted = Some(
                self.session
                    .mount_instance(handle, self.surface)
                    .map_err(|_| ())?,
            );
        } else {
            let mounted = row.mounted.take().ok_or(())?;
            self.session.unmount_instance(mounted).map_err(|_| ())?;
        }
        Ok(())
    }
}
