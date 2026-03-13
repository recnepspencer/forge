use crate::publication::data::diff::PatchRecord;
use crate::transactions::data::RecordRef;

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

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct DiffFragmentIdentity {
    pub(crate) target: RecordRef,
    pub(crate) kind: DiffFragmentKind,
    pub(crate) packet_index: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct DiffPreparationPacket {
    pub(crate) packet_index: usize,
    pub(crate) identity: DiffFragmentIdentity,
    pub(crate) record: PatchRecord,
}
