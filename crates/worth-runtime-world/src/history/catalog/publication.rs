//! Recovery of canonical performed facts uses the history entry's original
//! exclusive delivery lane. It neither retries owner work nor moves a cell.

use std::sync::Arc;

use crate::history::{ExplicitCommitHistoryProtectionObligation, PublicationDeliveryClaim};
use crate::identity::CompositeCommitIdentity;

use super::super::retention::{CompositeHistoryProtectionObligation, HistoryProtectionClass};
use super::support::{lock_state, validate_owner};
use super::{lock_index, CompositeHistoryCatalog, CompositeHistoryCatalogDenial};

impl CompositeHistoryCatalog {
    pub(crate) fn claim_performed_publication(
        &self,
        identity: &CompositeCommitIdentity,
    ) -> Result<Option<PublicationDeliveryClaim>, CompositeHistoryCatalogDenial> {
        let state = lock_state(&self.state);
        validate_owner(&state, identity.owner_identity())?;
        let Some(publication) = state
            .entries
            .get(identity)
            .and_then(Option::as_ref)
            .and_then(|entry| entry.publication.as_ref())
        else {
            return Ok(None);
        };
        if publication.facts().is_none() {
            return Ok(None);
        }
        lock_index(&state.reachability).increment_direct_protection(identity)?;
        let history = ExplicitCommitHistoryProtectionObligation::issued(
            CompositeHistoryProtectionObligation::new(
                Arc::clone(&state.reachability),
                identity.clone(),
                HistoryProtectionClass::ExplicitObligation,
            ),
        );
        Ok(publication.claim_delivery(history))
    }
}
