mod basis;
mod basis_identity;
mod certificate;
mod counters;
mod denial;
mod face_pair;
mod overlap_rows;
mod policy;
mod validation;

pub use basis::CoplanarOverlapContractBasis;
pub use certificate::CoplanarOverlapContractReceipt;
pub use counters::CoplanarOverlapPerformanceCounters;
pub use denial::{
    CoplanarOverlapDenial, CoplanarOverlapDenialBasisLocus, CoplanarOverlapDenialKind,
};
pub use face_pair::CertifiedCoplanarOverlapFace2D;
pub use overlap_rows::{
    AmbiguousContactRow, ContainmentRelationRow, OverlapIslandRow, PolicyRequiredExitRow,
    SharedIntervalRow,
};
pub use policy::{
    CoplanarOverlapBooleanResult, CoplanarOverlapImprintAction, CoplanarOverlapPolicy,
};

pub(crate) use basis_identity::coplanar_overlap_contract_identity_entries;
