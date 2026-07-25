mod reader;
mod successor;

pub(in crate::physical_runtime::record_serving) use reader::FreeSpaceReader;
pub(in crate::physical_runtime::record_serving) use successor::{
    plan_free_space_successor, FreeSpacePublicationPlan, FreeSpaceSuccessorRequest, FreeSpaceUpdate,
};
