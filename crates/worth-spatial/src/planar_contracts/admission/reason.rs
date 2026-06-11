#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum PlanarAdmissionReason {
    ExactPlanarContractAdmitted,
    DownstreamContractLaneAdmitted,
    CoplanarOverlapRequiresPolicy,
    DirtyOrUnboundedInputDenied,
    OrdinaryRuntimeLaneUnsupported,
    PredicateUncertaintyReserved,
    OutsideFamilyResponsibility,
}

impl PlanarAdmissionReason {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ExactPlanarContractAdmitted => {
                "exact planar contract family is admitted through Query-backed spatial authority"
            }
            Self::DownstreamContractLaneAdmitted => {
                "downstream contract lane is admitted only as an explicit Query-backed support row"
            }
            Self::CoplanarOverlapRequiresPolicy => {
                "coplanar overlap contract is policy-visible boolean-readiness input, not boolean execution"
            }
            Self::DirtyOrUnboundedInputDenied => {
                "dirty or unbounded planar input must clean-fail before predicate or boolean work"
            }
            Self::OrdinaryRuntimeLaneUnsupported => {
                "this surface is not admitted as an ordinary runtime lane"
            }
            Self::PredicateUncertaintyReserved => {
                "predicate uncertainty is reserved unless the family owns certified predicate authority"
            }
            Self::OutsideFamilyResponsibility => {
                "runtime concern is outside this planar contract family responsibility"
            }
        }
    }
}
