mod basis;
mod basis_identity;
mod certificate;
mod counters;
mod degeneracy;
mod denial;
mod measurement;
mod scale_comparison;
mod validation;

pub use basis::CertifiedSignedArea2DBasis;
pub use certificate::CertifiedSignedArea2DReceipt;
pub use counters::CertifiedSignedArea2DPerformanceCounters;
pub use degeneracy::{
    AreaDegeneracyClass, AreaDegeneracyPolicy, SignedAreaDegeneracyCause, SignedAreaOrientation,
    SignedAreaRepairAction,
};
pub use denial::{
    CertifiedSignedArea2DDenial, CertifiedSignedArea2DDenialBasisLocus,
    CertifiedSignedArea2DDenialKind,
};

pub(crate) use basis_identity::certified_signed_area_2d_identity_entries;
pub(crate) use measurement::{certify_signed_area, CertifiedSignedAreaMeasurement};
