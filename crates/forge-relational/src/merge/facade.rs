use crate::logic::runtime::RelationalRuntime;
use crate::merge::logic::MergeAccess;

impl RelationalRuntime {
    pub fn merge_access(&self) -> MergeAccess<'_> {
        MergeAccess::new(self)
    }
}
