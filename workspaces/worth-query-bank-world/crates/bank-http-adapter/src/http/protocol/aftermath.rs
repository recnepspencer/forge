use serde::{Deserialize, Serialize};

use super::{
    BankHttpCommitDescription, BankHttpCommitDisposition, BankHttpCredential, BankHttpDenial,
    BankHttpMutationControls, BankHttpProtocolVersion,
};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct BankHttpEstateDisbursementRequest {
    pub protocol: BankHttpProtocolVersion,
    pub request_id: String,
    pub credential: BankHttpCredential,
    pub controls: BankHttpMutationControls,
    pub idempotency_key: String,
    pub estate: String,
    pub source_account: String,
    pub destination_account: String,
    pub beneficiary: String,
    pub amount_minor_units: i64,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(tag = "outcome", rename_all = "snake_case")]
pub enum BankHttpEstateDisbursementOutcome {
    Applied {
        request_id: String,
        disposition: BankHttpCommitDisposition,
        commit: BankHttpCommitDescription,
        recovery: String,
    },
    Denied {
        request_id: Option<String>,
        denial: BankHttpDenial,
    },
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct BankHttpUndoProgressionRequest {
    pub protocol: BankHttpProtocolVersion,
    pub request_id: String,
    pub credential: BankHttpCredential,
    pub controls: BankHttpMutationControls,
    pub undo: String,
    pub idempotency_key: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(tag = "outcome", rename_all = "snake_case")]
pub enum BankHttpUndoProgressionOutcome {
    Applied {
        request_id: String,
        disposition: BankHttpCommitDisposition,
        commit: BankHttpCommitDescription,
        redo: String,
    },
    Reconciled {
        request_id: String,
    },
    Denied {
        request_id: Option<String>,
        denial: BankHttpDenial,
    },
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct BankHttpRedoProgressionRequest {
    pub protocol: BankHttpProtocolVersion,
    pub request_id: String,
    pub credential: BankHttpCredential,
    pub controls: BankHttpMutationControls,
    pub redo: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(tag = "outcome", rename_all = "snake_case")]
pub enum BankHttpRedoProgressionOutcome {
    Applied {
        request_id: String,
        disposition: BankHttpCommitDisposition,
        commit: BankHttpCommitDescription,
    },
    Denied {
        request_id: Option<String>,
        denial: BankHttpDenial,
    },
}
