use std::collections::BTreeMap;

use crate::runtime::ForgeQueryRuntimeFacadeFamily;

use super::super::support_snapshot::{ForgeQuerySupportSnapshot, ForgeQuerySupportSnapshotRow};
use super::error::{ForgeQuerySupportPinningError, ForgeQuerySupportPinningErrorKind};

#[derive(Debug)]
pub(crate) struct SupportPinSnapshotIndex<'a> {
    by_family: BTreeMap<&'a str, &'a ForgeQuerySupportSnapshotRow>,
}

impl<'a> SupportPinSnapshotIndex<'a> {
    pub(crate) fn new(
        snapshot: &'a ForgeQuerySupportSnapshot,
    ) -> Result<Self, ForgeQuerySupportPinningError> {
        let mut by_family = BTreeMap::new();
        for row in snapshot.rows() {
            if let Some(family) = row.facade_family() {
                if by_family.insert(family, row).is_some() {
                    return Err(ForgeQuerySupportPinningError::with_family(
                        ForgeQuerySupportPinningErrorKind::DuplicateSnapshotFamily,
                        "support pin snapshot contains duplicate facade family rows",
                        family,
                    ));
                }
            }
        }
        Ok(Self { by_family })
    }

    pub(crate) fn required_row(
        &self,
        family: ForgeQueryRuntimeFacadeFamily,
    ) -> Result<&'a ForgeQuerySupportSnapshotRow, ForgeQuerySupportPinningError> {
        self.by_family.get(family.as_str()).copied().ok_or_else(|| {
            ForgeQuerySupportPinningError::with_family(
                ForgeQuerySupportPinningErrorKind::SnapshotRowMissing,
                "support pin required row is absent from snapshot",
                family.as_str(),
            )
        })
    }

    pub(crate) fn optional_row(
        &self,
        family: ForgeQueryRuntimeFacadeFamily,
    ) -> Option<&'a ForgeQuerySupportSnapshotRow> {
        self.by_family.get(family.as_str()).copied()
    }
}
