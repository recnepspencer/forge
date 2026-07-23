use std::sync::Arc;

use worth_foundational::facade::{
    admit_foundational_authority_identity, project_foundational_identity,
    FoundationalAuthorityIdentity, FoundationalIdentityKind, FoundationalProjectionIdentity,
};
use worth_proof::{AuthorityMarker, AuthorityWitness};

pub struct SignalConditionalDecisionIdentityKind;

impl FoundationalIdentityKind for SignalConditionalDecisionIdentityKind {}

struct SignalConditionalDecisionAuthority {
    _owner_seal: (),
}

impl AuthorityMarker for SignalConditionalDecisionAuthority {}

pub(in crate::data::conditional_execution) struct SignalConditionalDecisionAuthorityIdentity {
    _identity: FoundationalAuthorityIdentity<
        Arc<str>,
        SignalConditionalDecisionAuthority,
        SignalConditionalDecisionIdentityKind,
    >,
}

pub type SignalConditionalDecisionProjectionIdentity =
    FoundationalProjectionIdentity<Arc<str>, SignalConditionalDecisionIdentityKind>;

pub(in crate::data::conditional_execution) fn mint_signal_conditional_decision_identity(
    canonical_decision_basis: String,
) -> (
    SignalConditionalDecisionAuthorityIdentity,
    SignalConditionalDecisionProjectionIdentity,
) {
    let authority = signal_conditional_decision_authority();
    let canonical_decision_basis: Arc<str> = canonical_decision_basis.into();
    let identity =
        admit_foundational_authority_identity(Arc::clone(&canonical_decision_basis), authority);
    let projection = project_foundational_identity(
        &identity,
        canonical_decision_basis,
        signal_conditional_decision_authority(),
    );
    (
        SignalConditionalDecisionAuthorityIdentity {
            _identity: identity,
        },
        projection,
    )
}

fn signal_conditional_decision_authority() -> AuthorityWitness<SignalConditionalDecisionAuthority> {
    AuthorityWitness::from_authority_marker(SignalConditionalDecisionAuthority { _owner_seal: () })
}
