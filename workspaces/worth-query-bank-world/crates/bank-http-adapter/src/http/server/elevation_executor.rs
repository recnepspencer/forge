use std::sync::Arc;
use std::time::Instant;

use bank_domain::estate::EstateAction;
use bank_domain::proposals::BankIdempotencyKey;
use tokio::sync::{mpsc, oneshot};
use tokio::task::JoinHandle;
use worth_query_host::facade::admission::authenticated_principal::WorthQueryCancellationSource;

use super::super::protocol::{
    BankHttpCredential, BankHttpElevationApprovalOutcome, BankHttpElevationRequestOutcome,
    BankHttpElevationRevocationOutcome, BankHttpMandatoryReviewOutcome,
};
use super::authentication::BankHttpApplicationAuthenticator;
use super::elevation_registry::{BankHttpElevationContext, BankHttpElevationRegistry};

mod approval;
mod mandatory_review;
mod outcome;
mod request;
mod revocation;

use approval::execute_approval;
use mandatory_review::execute_mandatory_review;
use outcome::*;
use request::execute_request;
use revocation::execute_revocation;

pub(super) struct AdmittedBankHttpElevationRequest {
    pub(super) request_id: String,
    pub(super) credential: BankHttpCredential,
    pub(super) idempotency_key: BankIdempotencyKey,
    pub(super) action: EstateAction,
    pub(super) context: BankHttpElevationContext,
    pub(super) deadline: Instant,
}

pub(super) struct AdmittedBankHttpElevationTransition {
    pub(super) request_id: String,
    pub(super) credential: BankHttpCredential,
    pub(super) idempotency_key: BankIdempotencyKey,
    pub(super) token: String,
    pub(super) deadline: Instant,
}

#[derive(Clone)]
pub(super) struct BankHttpElevationExecutor {
    sender: mpsc::Sender<ElevationCommand>,
}

enum ElevationCommand {
    Request {
        request: AdmittedBankHttpElevationRequest,
        cancellation: WorthQueryCancellationSource,
        response: oneshot::Sender<BankHttpElevationRequestOutcome>,
    },
    Approve {
        request: AdmittedBankHttpElevationTransition,
        cancellation: WorthQueryCancellationSource,
        response: oneshot::Sender<BankHttpElevationApprovalOutcome>,
    },
    Revoke {
        request: AdmittedBankHttpElevationTransition,
        cancellation: WorthQueryCancellationSource,
        response: oneshot::Sender<BankHttpElevationRevocationOutcome>,
    },
    Review {
        request: AdmittedBankHttpElevationTransition,
        cancellation: WorthQueryCancellationSource,
        response: oneshot::Sender<BankHttpMandatoryReviewOutcome>,
    },
}

impl BankHttpElevationExecutor {
    pub(super) fn start<A>(
        application: Arc<A>,
        queue_capacity: usize,
        registry_capacity: usize,
        lifetime: std::time::Duration,
    ) -> (Self, JoinHandle<()>)
    where
        A: BankHttpApplicationAuthenticator,
    {
        let (sender, receiver) = mpsc::channel(queue_capacity);
        let task = tokio::spawn(run(
            application,
            receiver,
            BankHttpElevationRegistry::new(registry_capacity, lifetime),
        ));
        (Self { sender }, task)
    }

    pub(super) async fn request(
        &self,
        request: AdmittedBankHttpElevationRequest,
    ) -> BankHttpElevationRequestOutcome {
        let request_id = request.request_id.clone();
        let deadline = request.deadline;
        let cancellation = WorthQueryCancellationSource::new();
        let _cancel_on_drop = CancelOnDrop(cancellation.clone());
        let (response, receiver) = oneshot::channel();
        if self
            .sender
            .try_send(ElevationCommand::Request {
                request,
                cancellation,
                response,
            })
            .is_err()
        {
            return request_denied(Some(request_id), saturated());
        }
        match tokio::time::timeout_at(deadline.into(), receiver).await {
            Ok(Ok(outcome)) => outcome,
            Ok(Err(_)) => request_denied(Some(request_id), unavailable()),
            Err(_) => request_denied(Some(request_id), deadline_exceeded()),
        }
    }

    pub(super) async fn approve(
        &self,
        request: AdmittedBankHttpElevationTransition,
    ) -> BankHttpElevationApprovalOutcome {
        let request_id = request.request_id.clone();
        let deadline = request.deadline;
        let cancellation = WorthQueryCancellationSource::new();
        let _cancel_on_drop = CancelOnDrop(cancellation.clone());
        let (response, receiver) = oneshot::channel();
        if self
            .sender
            .try_send(ElevationCommand::Approve {
                request,
                cancellation,
                response,
            })
            .is_err()
        {
            return approval_denied(Some(request_id), saturated());
        }
        match tokio::time::timeout_at(deadline.into(), receiver).await {
            Ok(Ok(outcome)) => outcome,
            Ok(Err(_)) => approval_denied(Some(request_id), unavailable()),
            Err(_) => approval_denied(Some(request_id), deadline_exceeded()),
        }
    }

    pub(super) async fn revoke(
        &self,
        request: AdmittedBankHttpElevationTransition,
    ) -> BankHttpElevationRevocationOutcome {
        let request_id = request.request_id.clone();
        let deadline = request.deadline;
        let cancellation = WorthQueryCancellationSource::new();
        let _cancel_on_drop = CancelOnDrop(cancellation.clone());
        let (response, receiver) = oneshot::channel();
        if self
            .sender
            .try_send(ElevationCommand::Revoke {
                request,
                cancellation,
                response,
            })
            .is_err()
        {
            return revocation_denied(Some(request_id), saturated());
        }
        match tokio::time::timeout_at(deadline.into(), receiver).await {
            Ok(Ok(outcome)) => outcome,
            Ok(Err(_)) => revocation_denied(Some(request_id), unavailable()),
            Err(_) => revocation_denied(Some(request_id), deadline_exceeded()),
        }
    }

    pub(super) async fn review(
        &self,
        request: AdmittedBankHttpElevationTransition,
    ) -> BankHttpMandatoryReviewOutcome {
        let request_id = request.request_id.clone();
        let deadline = request.deadline;
        let cancellation = WorthQueryCancellationSource::new();
        let _cancel_on_drop = CancelOnDrop(cancellation.clone());
        let (response, receiver) = oneshot::channel();
        if self
            .sender
            .try_send(ElevationCommand::Review {
                request,
                cancellation,
                response,
            })
            .is_err()
        {
            return review_denied(Some(request_id), saturated());
        }
        match tokio::time::timeout_at(deadline.into(), receiver).await {
            Ok(Ok(outcome)) => outcome,
            Ok(Err(_)) => review_denied(Some(request_id), unavailable()),
            Err(_) => review_denied(Some(request_id), deadline_exceeded()),
        }
    }
}

struct CancelOnDrop(WorthQueryCancellationSource);

impl Drop for CancelOnDrop {
    fn drop(&mut self) {
        self.0.cancel();
    }
}

async fn run<A>(
    application: Arc<A>,
    mut receiver: mpsc::Receiver<ElevationCommand>,
    mut registry: BankHttpElevationRegistry,
) where
    A: BankHttpApplicationAuthenticator,
{
    while let Some(command) = receiver.recv().await {
        match command {
            ElevationCommand::Request {
                request,
                cancellation,
                response,
            } => {
                let _ = response.send(
                    execute_request(application.as_ref(), &mut registry, request, cancellation)
                        .await,
                );
            }
            ElevationCommand::Approve {
                request,
                cancellation,
                response,
            } => {
                let _ = response.send(
                    execute_approval(application.as_ref(), &mut registry, request, cancellation)
                        .await,
                );
            }
            ElevationCommand::Revoke {
                request,
                cancellation,
                response,
            } => {
                let _ = response.send(
                    execute_revocation(application.as_ref(), &mut registry, request, cancellation)
                        .await,
                );
            }
            ElevationCommand::Review {
                request,
                cancellation,
                response,
            } => {
                let _ = response.send(
                    execute_mandatory_review(
                        application.as_ref(),
                        &mut registry,
                        request,
                        cancellation,
                    )
                    .await,
                );
            }
        }
    }
}
