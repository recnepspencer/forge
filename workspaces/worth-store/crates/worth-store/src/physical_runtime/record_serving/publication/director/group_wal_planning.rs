use worth_proof::NonEmpty;

use super::RecordPublicationDirector;
use crate::physical_runtime::{PreparedPhysicalMutation, RecordAppendDenial};

impl RecordPublicationDirector {
    pub(super) fn plan_prepared_group_for_wal(
        &self,
        members: NonEmpty<PreparedPhysicalMutation>,
    ) -> Result<
        NonEmpty<PreparedPhysicalMutation>,
        (NonEmpty<PreparedPhysicalMutation>, RecordAppendDenial),
    > {
        let mut source = members.into_vec().into_iter();
        let mut planned = Vec::new();
        while let Some(member) = source.next() {
            match self.plan_prepared_data_for_wal(member) {
                Ok(member) => planned.push(member),
                Err((member, denial)) => {
                    planned.push(member);
                    planned.extend(source);
                    return Err((require_nonempty(planned), denial));
                }
            }
        }
        Ok(require_nonempty(planned))
    }
}

fn require_nonempty(members: Vec<PreparedPhysicalMutation>) -> NonEmpty<PreparedPhysicalMutation> {
    NonEmpty::try_from_vec(members)
        .unwrap_or_else(|_| unreachable!("group planning preserves nonempty membership"))
}
