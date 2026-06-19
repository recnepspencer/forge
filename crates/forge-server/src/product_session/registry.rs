use std::{
    collections::HashMap,
    sync::{
        atomic::{AtomicU64, Ordering},
        Arc, Mutex,
    },
};

use crate::{diagnostics::ForgeServerCounters, ForgeServerResolvedRequestContext};

use super::{
    clock::ForgeServerProductSessionClock,
    counters::ForgeServerProductSessionCounterSnapshot,
    lifecycle_gate::{ForgeServerProductSessionDenial, ForgeServerProductSessionDenialCode},
    ForgeServerProductSession, ForgeServerProductSessionCreationRequest,
    ForgeServerProductSessionExpiryPosture, ForgeServerProductSessionIdentity,
    ForgeServerProductSessionLifecycle,
};

#[derive(Clone, Debug)]
pub struct ForgeServerProductSessionRegistry {
    records: Arc<Mutex<HashMap<String, ForgeServerProductSession>>>,
    next_identity: Arc<AtomicU64>,
    counters: Arc<ForgeServerCounters>,
    clock: Arc<dyn ForgeServerProductSessionClock>,
}

impl ForgeServerProductSessionRegistry {
    pub(crate) fn new(
        counters: Arc<ForgeServerCounters>,
        clock: Arc<dyn ForgeServerProductSessionClock>,
    ) -> Self {
        Self {
            records: Arc::new(Mutex::new(HashMap::new())),
            next_identity: Arc::new(AtomicU64::new(1)),
            counters,
            clock,
        }
    }

    pub fn open_preview(
        &self,
        resolved_request_context: &ForgeServerResolvedRequestContext,
        request: ForgeServerProductSessionCreationRequest,
    ) -> ForgeServerProductSession {
        self.open(
            resolved_request_context,
            request,
            ForgeServerProductSessionLifecycle::ReadOnlyPreview,
        )
    }

    pub fn open_mutation(
        &self,
        resolved_request_context: &ForgeServerResolvedRequestContext,
        request: ForgeServerProductSessionCreationRequest,
    ) -> ForgeServerProductSession {
        self.open(
            resolved_request_context,
            request,
            ForgeServerProductSessionLifecycle::MutationDraft,
        )
    }

    pub fn lookup(
        &self,
        identity: &str,
        resolved_request_context: &ForgeServerResolvedRequestContext,
    ) -> Option<ForgeServerProductSession> {
        self.counters.increment_product_session_lookups_attempted();
        let mut records = self.records.lock().expect("product session registry lock");
        let session = records.get(identity).cloned()?;
        let refreshed = self.refresh_expiry(session);
        records.insert(identity.to_string(), refreshed.clone());
        let _ = resolved_request_context;
        Some(refreshed)
    }

    pub fn close(
        &self,
        identity: &str,
        resolved_request_context: &ForgeServerResolvedRequestContext,
    ) -> Result<ForgeServerProductSession, ForgeServerProductSessionDenial> {
        let session = self
            .lookup(identity, resolved_request_context)
            .ok_or_else(|| {
                self.counters
                    .increment_product_session_lookup_denied_missing();
                ForgeServerProductSessionDenial::new(
                    ForgeServerProductSessionDenialCode::UnknownProductSessionIdentity,
                    format!("product session `{identity}` was not found"),
                )
            })?;
        if session.tenant_id()
            != resolved_request_context
                .request_context()
                .workspace_target()
                .tenant_id()
            || session.workspace_id()
                != resolved_request_context
                    .request_context()
                    .workspace_target()
                    .workspace_id()
        {
            self.counters
                .increment_product_session_lookup_denied_foreign();
            return Err(ForgeServerProductSessionDenial::new(
                ForgeServerProductSessionDenialCode::ForeignProductSession,
                format!("product session `{identity}` does not belong to this workspace"),
            ));
        }
        let closed_at_epoch_millis = self.clock.current_time_millis();
        let closed = ForgeServerProductSession::new(
            session.identity().clone(),
            ForgeServerProductSessionLifecycle::Closed,
            ForgeServerProductSessionExpiryPosture::Closed {
                closed_at_epoch_millis,
            },
            session.operation_name(),
            session.tenant_id(),
            session.workspace_id(),
            session.branch_label(),
            session.basis_digest().map(str::to_string),
        );
        self.records
            .lock()
            .expect("product session registry lock")
            .insert(identity.to_string(), closed.clone());
        self.counters.increment_product_session_closes_recorded();
        Ok(closed)
    }

    pub fn counter_snapshot(&self) -> ForgeServerProductSessionCounterSnapshot {
        let snapshot = self.counters.snapshot();
        ForgeServerProductSessionCounterSnapshot {
            sessions_created: snapshot.product_sessions_created,
            preview_sessions_created: snapshot.product_session_preview_creations,
            mutation_sessions_created: snapshot.product_session_mutation_creations,
            lookups_attempted: snapshot.product_session_lookups_attempted,
            lookups_denied_missing: snapshot.product_session_lookups_denied_missing,
            lookups_denied_foreign: snapshot.product_session_lookups_denied_foreign,
            lookups_denied_expired: snapshot.product_session_lookups_denied_expired,
            lookups_denied_closed: snapshot.product_session_lookups_denied_closed,
            lookups_denied_moved: snapshot.product_session_lookups_denied_moved,
            lookups_denied_preview_for_mutation: snapshot
                .product_session_lookups_denied_preview_for_mutation,
            closes_recorded: snapshot.product_session_closes_recorded,
        }
    }

    pub(crate) fn record_denial(&self, code: ForgeServerProductSessionDenialCode) {
        match code {
            ForgeServerProductSessionDenialCode::CoordinationRequestDenied
            | ForgeServerProductSessionDenialCode::CoordinationAdmissionDenied
            | ForgeServerProductSessionDenialCode::CoordinationReadinessDenied => {}
            ForgeServerProductSessionDenialCode::MissingProductSessionIdentity
            | ForgeServerProductSessionDenialCode::UnknownProductSessionIdentity => self
                .counters
                .increment_product_session_lookup_denied_missing(),
            ForgeServerProductSessionDenialCode::ForeignProductSession => self
                .counters
                .increment_product_session_lookup_denied_foreign(),
            ForgeServerProductSessionDenialCode::ExpiredProductSession => self
                .counters
                .increment_product_session_lookup_denied_expired(),
            ForgeServerProductSessionDenialCode::ClosedProductSession => self
                .counters
                .increment_product_session_lookup_denied_closed(),
            ForgeServerProductSessionDenialCode::PreviewSessionCannotMutate => self
                .counters
                .increment_product_session_lookup_denied_preview_for_mutation(),
            ForgeServerProductSessionDenialCode::SessionRebindRequired => self
                .counters
                .increment_product_session_lookup_denied_moved(),
        }
    }

    fn open(
        &self,
        resolved_request_context: &ForgeServerResolvedRequestContext,
        request: ForgeServerProductSessionCreationRequest,
        lifecycle: ForgeServerProductSessionLifecycle,
    ) -> ForgeServerProductSession {
        let created_at_epoch_millis = self.clock.current_time_millis();
        let identity = self.mint_identity(resolved_request_context);
        let session = ForgeServerProductSession::new(
            identity.clone(),
            lifecycle,
            ForgeServerProductSessionExpiryPosture::Active {
                expires_at_epoch_millis: created_at_epoch_millis
                    .saturating_add(request.expiry_seconds().saturating_mul(1000)),
            },
            request.operation_name(),
            resolved_request_context
                .request_context()
                .workspace_target()
                .tenant_id(),
            resolved_request_context
                .request_context()
                .workspace_target()
                .workspace_id(),
            resolved_request_context
                .request_context()
                .branch_target()
                .canonical_label(),
            request.basis_digest().map(str::to_string),
        );
        self.records
            .lock()
            .expect("product session registry lock")
            .insert(identity.as_str().to_string(), session.clone());
        self.counters.increment_product_sessions_created();
        match lifecycle {
            ForgeServerProductSessionLifecycle::ReadOnlyPreview => {
                self.counters.increment_product_session_preview_creations()
            }
            ForgeServerProductSessionLifecycle::MutationDraft => {
                self.counters.increment_product_session_mutation_creations()
            }
            ForgeServerProductSessionLifecycle::Closed => {}
        }
        session
    }

    fn mint_identity(
        &self,
        resolved_request_context: &ForgeServerResolvedRequestContext,
    ) -> ForgeServerProductSessionIdentity {
        let sequence = self.next_identity.fetch_add(1, Ordering::Relaxed);
        let workspace_target = resolved_request_context
            .request_context()
            .workspace_target();
        let branch_target = resolved_request_context.request_context().branch_target();
        ForgeServerProductSessionIdentity::new(format!(
            "product-session:{}:{}:{}:{sequence}",
            workspace_target.tenant_id(),
            workspace_target.workspace_id(),
            branch_target.canonical_label(),
        ))
    }

    fn refresh_expiry(&self, session: ForgeServerProductSession) -> ForgeServerProductSession {
        let ForgeServerProductSessionExpiryPosture::Active {
            expires_at_epoch_millis,
        } = session.expiry_posture()
        else {
            return session;
        };
        let observed_at_epoch_millis = self.clock.current_time_millis();
        if observed_at_epoch_millis < *expires_at_epoch_millis {
            return session;
        }
        ForgeServerProductSession::new(
            session.identity().clone(),
            session.lifecycle(),
            ForgeServerProductSessionExpiryPosture::Expired {
                expires_at_epoch_millis: *expires_at_epoch_millis,
                observed_at_epoch_millis,
            },
            session.operation_name(),
            session.tenant_id(),
            session.workspace_id(),
            session.branch_label(),
            session.basis_digest().map(str::to_string),
        )
    }
}
