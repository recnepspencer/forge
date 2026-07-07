mod collinear_classification;
mod counters;
mod denial;
mod endpoint_normalization;
mod event_extraction;
mod event_grouping;
mod event_ledger;
mod interval_events;
mod interval_normalization;
mod pair_enumeration;
mod point_deduplication;
mod point_events;
mod policy;
mod predicate_binding;
mod segment_carriers;
mod segment_identity;
mod shared_endpoint_events;

pub use collinear_classification::{
    PlanarBooleanCollinearIntervalBasis, PlanarBooleanCollinearRelation,
    PlanarBooleanCollinearRelationCounters, PlanarBooleanCollinearRelationDenial,
    PlanarBooleanCollinearRelationDenialKind, PlanarBooleanCollinearRelationExtraction,
    PlanarBooleanCollinearRelationExtractionCompiledPlan,
    PlanarBooleanCollinearRelationExtractionPlan, PlanarBooleanCollinearRelationKind,
    PlanarBooleanCollinearRelationReceipt, PlanarBooleanCollinearTouchPoint,
};
pub use counters::PlanarBooleanPointEventExtractionCounters;
pub use denial::{PlanarBooleanEventExtractionDenial, PlanarBooleanEventExtractionDenialKind};
#[cfg(test)]
pub(crate) use endpoint_normalization::{
    normalize_endpoint_order, validate_segment_endpoint_admissibility,
};
pub use endpoint_normalization::{
    PlanarBooleanNormalizedEndpoint, PlanarBooleanNormalizedEndpointPair,
};
pub use event_extraction::{
    PlanarBooleanEventExtractionCounters, PlanarBooleanEventExtractionPhaseStop,
    PlanarBooleanEventExtractionPhaseStopError,
};
#[cfg(test)]
pub(crate) use event_grouping::PlanarBooleanEventGroupInput;
pub(crate) use event_grouping::{group_interval_events, group_point_events};
pub use event_grouping::{
    PlanarBooleanEventGroup, PlanarBooleanEventGroupKind, PlanarBooleanEventGroupingCounters,
};
#[cfg(test)]
pub(crate) use event_ledger::PlanarBooleanEventLedgerReceiptInput;
pub use event_ledger::{
    PlanarBooleanEventLedger, PlanarBooleanEventLedgerAssemblyCompiledPlan,
    PlanarBooleanEventLedgerAssemblyPlan, PlanarBooleanEventLedgerCounters,
    PlanarBooleanEventLedgerDenial, PlanarBooleanEventLedgerDenialKind,
    PlanarBooleanEventLedgerLookupExecutionDenial,
    PlanarBooleanEventLedgerLookupExecutionDenialKind,
    PlanarBooleanEventLedgerLookupExecutionPacket, PlanarBooleanEventLedgerLookupExecutionWitness,
    PlanarBooleanEventLedgerReceipt, PlanarBooleanOrderedEventSet,
};
pub use interval_events::{
    PlanarBooleanIntervalEvent, PlanarBooleanIntervalEventExtraction,
    PlanarBooleanIntervalEventExtractionCompiledPlan, PlanarBooleanIntervalEventExtractionCounters,
    PlanarBooleanIntervalEventExtractionDenial, PlanarBooleanIntervalEventExtractionDenialKind,
    PlanarBooleanIntervalEventExtractionPlan, PlanarBooleanIntervalEventExtractionReceipt,
    PlanarBooleanIntervalEventKind, PlanarBooleanNormalizedInterval, PlanarBooleanSourceInterval,
    PlanarBooleanSourceIntervalSense,
};
pub(crate) use interval_normalization::{
    canonical_parameter_range, interval_has_collapsed, normalized_parameter_range,
};
pub(crate) use pair_enumeration::enumerate_segment_pairs;
#[cfg(test)]
pub(crate) use pair_enumeration::PlanarBooleanSegmentCandidateIndexProductInput;
pub use pair_enumeration::{
    PlanarBooleanCandidateBroadPhaseReason, PlanarBooleanCandidateEnvelopeBasis,
    PlanarBooleanCandidateIndexFallbackPosture, PlanarBooleanCandidateIndexLifecycleOutcome,
    PlanarBooleanCandidateIndexStrategy, PlanarBooleanSegmentCandidateIndexProduct,
    PlanarBooleanSegmentCandidateRowReceipt, PlanarBooleanSegmentPairEnumerationCounters,
    PlanarBooleanSegmentPairEnumerationDenial, PlanarBooleanSegmentPairEnumerationDenialKind,
    PlanarBooleanSegmentPairEnumerationReceipt, PlanarBooleanSegmentPairWorkItem,
};
pub(crate) use point_deduplication::PlanarBooleanDeduplicatedPointEventSet;
pub use point_events::{
    PlanarBooleanPointEvent, PlanarBooleanPointEventCoordinateFact,
    PlanarBooleanPointEventExtraction, PlanarBooleanPointEventExtractionCompiledPlan,
    PlanarBooleanPointEventExtractionDenial, PlanarBooleanPointEventExtractionDenialKind,
    PlanarBooleanPointEventExtractionPlan, PlanarBooleanPointEventExtractionReceipt,
    PlanarBooleanPointEventKind, PlanarBooleanPointEventSegmentParameterFact,
};
pub use policy::{
    PlanarBooleanEventExtractionPolicyExit, PlanarBooleanEventExtractionPolicyExitKind,
};
pub use predicate_binding::{
    PlanarBooleanEventClassifierInput, PlanarBooleanEventPredicateBinding,
    PlanarBooleanEventPredicateBindingCompiledPlan, PlanarBooleanEventPredicateBindingCounters,
    PlanarBooleanEventPredicateBindingDenial, PlanarBooleanEventPredicateBindingDenialKind,
    PlanarBooleanEventPredicateBindingPlan, PlanarBooleanPredicateBoundPair,
};
#[cfg(test)]
pub(crate) use segment_carriers::PlanarBooleanSegmentCarrierInput;
pub use segment_carriers::{
    PlanarBooleanLoopRole, PlanarBooleanSegmentCarrier, PlanarBooleanSegmentCarrierEndpointFacts,
    PlanarBooleanSegmentCarrierOperandSource, PlanarBooleanSegmentCarrierSet,
    PlanarBooleanSegmentCarrierSetDenial, PlanarBooleanSegmentCarrierSetDenialKind,
};
pub use segment_identity::{
    PlanarBooleanCanonicalSegment, PlanarBooleanCanonicalSegmentSet,
    PlanarBooleanCanonicalSegmentSetDenial, PlanarBooleanCanonicalSegmentSetDenialKind,
};
pub use shared_endpoint_events::PlanarBooleanSharedEndpointEvent;
