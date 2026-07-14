mod allocation_scope;

#[cfg(test)]
mod allocation_scope_tests;

pub use allocation_scope::{
    AllocationAdmission, AllocationDenial, AllocationGrant, AllocationReceipt, AllocationRequest,
    AllocationRequestKind, FixedMetadataGrant,
};
