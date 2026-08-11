//! Bank-owned operational authority for one application preview session.

use bank_domain::schema::BankSchema;
use worth_query_host::facade::{
    admission::authenticated_principal::WorthQueryRequestScope,
    primary_graph::{
        WorthQueryApplicationPreviewBasis, WorthQueryApplicationPreviewSession,
        WorthQueryPrimaryGraphApplicationRuntime,
    },
};

use super::BankApplicationQueryDenial;

/// Opaque Bank authority for admitting preview reads.
///
/// The underlying Query session never crosses the Bank facade. Callers can
/// only use this value through Bank preview operations.
///
/// ```compile_fail,E0451
/// use bank_server::BankPreviewSession;
///
/// let _ = BankPreviewSession { query: panic!("foreign Query session") };
/// ```
///
/// The wrapper cannot be coerced to Query's operational authority:
///
/// ```compile_fail,E0308
/// use bank_domain::schema::BankSchema;
/// use bank_server::BankPreviewSession;
/// use worth_query_host::facade::primary_graph::WorthQueryApplicationPreviewSession;
///
/// fn raw_query_session(
///     session: &BankPreviewSession,
/// ) -> &WorthQueryApplicationPreviewSession<BankSchema> {
///     session
/// }
/// ```
///
/// Nor does it expose the former raw-authority accessor:
///
/// ```compile_fail,E0599
/// use bank_domain::schema::BankSchema;
/// use bank_server::BankPreviewSession;
/// use worth_query_host::facade::primary_graph::WorthQueryApplicationPreviewSession;
///
/// fn raw_query_session(
///     session: &BankPreviewSession,
/// ) -> &WorthQueryApplicationPreviewSession<BankSchema> {
///     session.query()
/// }
/// ```
pub struct BankPreviewSession {
    query: WorthQueryApplicationPreviewSession<BankSchema>,
}

/// Closed Bank description of a released preview authority.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BankPreviewSessionDiscardReceipt {
    discarded: bool,
}

impl BankPreviewSession {
    pub(crate) const fn from_query(query: WorthQueryApplicationPreviewSession<BankSchema>) -> Self {
        Self { query }
    }

    pub(crate) fn admit_basis(
        &self,
        application: &WorthQueryPrimaryGraphApplicationRuntime<BankSchema>,
        request: &WorthQueryRequestScope,
    ) -> Result<WorthQueryApplicationPreviewBasis<BankSchema>, BankApplicationQueryDenial> {
        application
            .admit_application_preview_basis(&self.query, request)
            .map_err(BankApplicationQueryDenial::from_admission)
    }

    pub fn discard(self) -> Result<BankPreviewSessionDiscardReceipt, BankApplicationQueryDenial> {
        self.query
            .discard()
            .map(|receipt| BankPreviewSessionDiscardReceipt {
                discarded: receipt.discarded(),
            })
            .map_err(BankApplicationQueryDenial::from_preview_session)
    }
}

impl BankPreviewSessionDiscardReceipt {
    pub const fn discarded(self) -> bool {
        self.discarded
    }
}
