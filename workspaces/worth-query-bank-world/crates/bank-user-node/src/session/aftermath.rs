use bank_http_adapter::{
    BankHttpCredential, BankHttpEstateDisbursementRequest, BankHttpProtocolVersion,
    BankHttpRedoProgressionRequest, BankHttpUndoProgressionRequest,
};

use crate::protocol::{
    BankUserNodeDenialKind, BankUserNodeEstateDisbursementOutcome,
    BankUserNodeEstateDisbursementRequest, BankUserNodeRedoProgressionOutcome,
    BankUserNodeRedoProgressionRequest, BankUserNodeUndoProgressionOutcome,
    BankUserNodeUndoProgressionRequest,
};

use super::{denial, BankUserSession};

impl BankUserSession {
    pub(crate) async fn disburse_estate(
        &self,
        request: BankUserNodeEstateDisbursementRequest,
    ) -> BankUserNodeEstateDisbursementOutcome {
        let credential = match self.credential.lock().await.clone() {
            Some(credential) => credential,
            None => return disbursement_denied(BankUserNodeDenialKind::NoAuthenticatedSession),
        };
        let upstream = BankHttpEstateDisbursementRequest {
            protocol: BankHttpProtocolVersion::V1,
            request_id: request.request_id,
            credential: BankHttpCredential::from_authentik(&credential),
            controls: request.controls,
            idempotency_key: request.idempotency_key,
            estate: request.estate,
            source_account: request.source_account,
            destination_account: request.destination_account,
            beneficiary: request.beneficiary,
            amount_minor_units: request.amount_minor_units,
        };
        match self
            .forward(
                self.estate_disbursement_endpoint.clone(),
                &upstream,
                upstream.controls.deadline_milliseconds,
            )
            .await
        {
            Ok(response) => BankUserNodeEstateDisbursementOutcome::Forwarded { response },
            Err(kind) => disbursement_denied(kind),
        }
    }

    pub(crate) async fn progress_undo(
        &self,
        request: BankUserNodeUndoProgressionRequest,
    ) -> BankUserNodeUndoProgressionOutcome {
        let credential = match self.credential.lock().await.clone() {
            Some(credential) => credential,
            None => return undo_denied(BankUserNodeDenialKind::NoAuthenticatedSession),
        };
        let upstream = BankHttpUndoProgressionRequest {
            protocol: BankHttpProtocolVersion::V1,
            request_id: request.request_id,
            credential: BankHttpCredential::from_authentik(&credential),
            controls: request.controls,
            undo: request.undo,
            idempotency_key: request.idempotency_key,
        };
        match self
            .forward(
                self.undo_progression_endpoint.clone(),
                &upstream,
                upstream.controls.deadline_milliseconds,
            )
            .await
        {
            Ok(response) => BankUserNodeUndoProgressionOutcome::Forwarded { response },
            Err(kind) => undo_denied(kind),
        }
    }

    pub(crate) async fn progress_redo(
        &self,
        request: BankUserNodeRedoProgressionRequest,
    ) -> BankUserNodeRedoProgressionOutcome {
        let credential = match self.credential.lock().await.clone() {
            Some(credential) => credential,
            None => return redo_denied(BankUserNodeDenialKind::NoAuthenticatedSession),
        };
        let upstream = BankHttpRedoProgressionRequest {
            protocol: BankHttpProtocolVersion::V1,
            request_id: request.request_id,
            credential: BankHttpCredential::from_authentik(&credential),
            controls: request.controls,
            redo: request.redo,
        };
        match self
            .forward(
                self.redo_progression_endpoint.clone(),
                &upstream,
                upstream.controls.deadline_milliseconds,
            )
            .await
        {
            Ok(response) => BankUserNodeRedoProgressionOutcome::Forwarded { response },
            Err(kind) => redo_denied(kind),
        }
    }
}

fn disbursement_denied(kind: BankUserNodeDenialKind) -> BankUserNodeEstateDisbursementOutcome {
    BankUserNodeEstateDisbursementOutcome::Denied {
        denial: denial(kind),
    }
}

fn undo_denied(kind: BankUserNodeDenialKind) -> BankUserNodeUndoProgressionOutcome {
    BankUserNodeUndoProgressionOutcome::Denied {
        denial: denial(kind),
    }
}

fn redo_denied(kind: BankUserNodeDenialKind) -> BankUserNodeRedoProgressionOutcome {
    BankUserNodeRedoProgressionOutcome::Denied {
        denial: denial(kind),
    }
}
