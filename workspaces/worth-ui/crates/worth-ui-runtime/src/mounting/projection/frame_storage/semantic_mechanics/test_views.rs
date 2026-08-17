use std::{collections::BTreeMap, sync::Arc};

use worth_ui_host_contract::UiMountedInstanceIdentity;

use super::{UiMountedSemanticMechanicRows, UiMountedSemanticMechanicSource};

impl UiMountedSemanticMechanicSource {
    pub(in crate::mounting::projection::frame_storage) fn collection_layouts_for(
        &self,
        instance: UiMountedInstanceIdentity,
    ) -> BTreeMap<[u8; 32], Arc<worth_ui_text::UiQualifiedTextLayout>> {
        self.by_instance
            .get(&instance)
            .into_iter()
            .flat_map(UiMountedSemanticMechanicRows::iter)
            .filter_map(|row| {
                Some((
                    row.collection_row()?.correlation_digest(),
                    Arc::clone(row.qualified_layout()),
                ))
            })
            .collect()
    }
}
