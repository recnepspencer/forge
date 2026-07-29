mod account_authorization;
mod approved;
mod decision_snapshot;
mod denial;
mod effects;
mod engine;
mod idempotency;
mod journal_proposal;
mod proposal_completion;
mod proposal_identity;
mod snapshot;

pub use account_authorization::BankAccountAuthorization;
pub use approved::BankInvariantApprovedProposal;
pub use decision_snapshot::BankDecisionSnapshot;
pub use denial::BankProposalDenial;
pub use effects::BankProposedEffect;
pub use engine::BankProposalEngine;
pub use idempotency::{
    BankIdempotencyClaim, BankIdempotencyIntent, BankIdempotencyKey, BankIdempotencyKeyIdentity,
    BankOperationScopeBinding,
};
pub use snapshot::{BankSnapshot, BankSnapshotBuilder};

pub(crate) use journal_proposal::{append_balanced_transfer, ensure_open};
pub(crate) use proposal_completion::{complete_decision_proposal, complete_proposal};
pub(crate) use proposal_identity::CanonicalProposalPayload;
