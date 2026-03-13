use crate::publication::data::diff::PatchRecord;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum DiffFragmentKind {
    Created,
    Updated,
    Deleted,
    RetainedForAudit,
}

impl From<&crate::publication::data::diff::PatchRecordKind> for DiffFragmentKind {
    fn from(value: &crate::publication::data::diff::PatchRecordKind) -> Self {
        match value {
            crate::publication::data::diff::PatchRecordKind::Created => Self::Created,
            crate::publication::data::diff::PatchRecordKind::Updated => Self::Updated,
            crate::publication::data::diff::PatchRecordKind::Deleted => Self::Deleted,
            crate::publication::data::diff::PatchRecordKind::RetainedForAudit => {
                Self::RetainedForAudit
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct DiffPreparationHeader {
    pub(crate) packet_index_floor: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct DiffPreparationPacket {
    pub(crate) header: DiffPreparationHeader,
    pub(crate) records: Vec<PatchRecord>,
}
