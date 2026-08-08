//! Application-aftermath declaration surface.
//!
//! Domains declare correction authority and correction mechanism as separate
//! axes. Published law-14 posture names are derived only at installation.

mod contract;
mod correction_authority;
mod correction_mechanism;
mod postcondition;
mod reconciliation;

pub use contract::DeclaredApplicationAftermathContract;
pub use correction_authority::DeclaredCorrectionAuthority;
pub use correction_mechanism::{
    DeclaredCompensation, DeclaredCorrectionMechanism, DeclaredLoweringCorrespondenceRef,
    DeclaredPreImageDemand, DeclaredPreImageDemandDenial, DeclaredRecordedInverse,
};
pub use postcondition::DeclaredAftermathPostcondition;
pub use reconciliation::DeclaredReconciliationProcedure;
