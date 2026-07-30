mod contract;
mod evidence;
mod executed_evidence;
mod expectation;
mod foundational_receipt;
mod hostile_evidence;
mod mismatch;
mod residency_source;
mod resource_envelope;
#[cfg(any(test, feature = "certification-test-support"))]
mod store_residency_fixture;
mod strength;

pub(crate) use contract::{counter_contract_kind_token, counter_expectation_strength_token};
pub use contract::{
    CounterContractDenial, CounterContractKind, PhysicalCounterContract, RequiredCounterContractSet,
};
pub use evidence::{
    admit_physical_counter_evidence, PhysicalCounterEvidenceReceipt, PhysicalCounterEvidenceRow,
    PhysicalResourceEnvelopeObservation,
};
pub use executed_evidence::{PhysicalCounterExecutionSources, PhysicalExecutedCounterEvidence};
pub(crate) use expectation::counter_expectation_kind_token;
pub use expectation::{
    CounterExpectationDenial, CounterExpectationKind, CounterExpectationStrength,
    PhysicalCounterExpectation,
};
pub(crate) use foundational_receipt::build_foundational_receipt;
pub use hostile_evidence::{
    reject_hostile_counter_evidence_for_readmission, HostileCounterEvidenceRow,
    HostileResourceEnvelopeObservation,
};
pub use mismatch::CounterMismatchEvidence;
pub use residency_source::PhysicalResidencyEvidenceSource;
pub use resource_envelope::PhysicalResourceEnvelope;
#[cfg(any(test, feature = "certification-test-support"))]
pub(crate) use store_residency_fixture::{
    observe_real_store_residency, CertificationResidencyWorkload,
};
pub(crate) use strength::classify_counter_strength;
pub use strength::{CounterStrengthJustification, CounterStrengthPosture, OverExactCounterDenied};
