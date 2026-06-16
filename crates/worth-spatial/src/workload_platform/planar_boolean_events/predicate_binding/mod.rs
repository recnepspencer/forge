mod binding;
mod bound_pair;
mod bound_pair_basis;
mod classifier_input;
mod counters;
mod denial;
mod identity;
mod predicate_consumption_alignment;
mod segment_contract_alignment;

pub use binding::{
    PlanarBooleanEventPredicateBinding, PlanarBooleanEventPredicateBindingCompiledPlan,
    PlanarBooleanEventPredicateBindingPlan,
};
pub use bound_pair::PlanarBooleanPredicateBoundPair;
pub use classifier_input::PlanarBooleanEventClassifierInput;
pub use counters::PlanarBooleanEventPredicateBindingCounters;
pub use denial::{
    PlanarBooleanEventPredicateBindingDenial, PlanarBooleanEventPredicateBindingDenialKind,
};

pub(crate) use predicate_consumption_alignment::validate_predicate_consumption_alignment;
pub(crate) use segment_contract_alignment::aligned_segment_contracts;
