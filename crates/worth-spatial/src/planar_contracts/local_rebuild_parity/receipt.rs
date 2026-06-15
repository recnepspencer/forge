use super::{
    planar_local_rebuild_parity_authority_entries, planar_local_rebuild_parity_digest,
    PlanarLocalRebuildParityBasis, PlanarLocalRebuildParityCounters, PlanarLocalRebuildParityRow,
};

#[derive(Clone, Debug, PartialEq)]
pub struct PlanarLocalRebuildParityReceipt {
    basis: PlanarLocalRebuildParityBasis,
    parity_rows: Vec<PlanarLocalRebuildParityRow>,
    declaration_digest: String,
    progression_digest: String,
    route_plan_digest: String,
    query_receipt_digest: String,
    envelope_digest: String,
    parity_digest: String,
    counters: PlanarLocalRebuildParityCounters,
}

impl PlanarLocalRebuildParityReceipt {
    pub(crate) const SOURCE_RECEIPTS_CONSUMED: usize = 8;

    pub(crate) fn new(
        basis: PlanarLocalRebuildParityBasis,
        declaration_digest: String,
        progression_digest: String,
        route_plan_digest: String,
        query_receipt_digest: String,
        envelope_digest: String,
        parity_digest: String,
        counters: PlanarLocalRebuildParityCounters,
    ) -> Self {
        let parity_rows = PlanarLocalRebuildParityRow::from_basis(&basis);
        Self {
            basis,
            parity_rows,
            declaration_digest,
            progression_digest,
            route_plan_digest,
            query_receipt_digest,
            envelope_digest,
            parity_digest,
            counters,
        }
    }

    pub(crate) fn parity_digest_for(
        basis: &PlanarLocalRebuildParityBasis,
        declaration_digest: &str,
        progression_digest: &str,
        route_plan_digest: &str,
        query_receipt_digest: &str,
        envelope_digest: &str,
    ) -> String {
        let mut parts = planar_local_rebuild_parity_authority_entries(basis)
            .into_iter()
            .map(|entry| entry.digest_part())
            .collect::<Vec<_>>();
        parts.push(format!("declaration:{declaration_digest}"));
        parts.push(format!("progression:{progression_digest}"));
        parts.push(format!("route_plan:{route_plan_digest}"));
        parts.push(format!("query_receipt:{query_receipt_digest}"));
        parts.push(format!("envelope:{envelope_digest}"));
        planar_local_rebuild_parity_digest(&parts)
    }

    pub fn basis(&self) -> &PlanarLocalRebuildParityBasis {
        &self.basis
    }

    pub fn parity_rows(&self) -> &[PlanarLocalRebuildParityRow] {
        &self.parity_rows
    }

    pub fn declaration_digest(&self) -> &str {
        &self.declaration_digest
    }

    pub fn progression_digest(&self) -> &str {
        &self.progression_digest
    }

    pub fn route_plan_digest(&self) -> &str {
        &self.route_plan_digest
    }

    pub fn query_receipt_digest(&self) -> &str {
        &self.query_receipt_digest
    }

    pub fn envelope_digest(&self) -> &str {
        &self.envelope_digest
    }

    pub fn parity_digest(&self) -> &str {
        &self.parity_digest
    }

    pub fn counters(&self) -> PlanarLocalRebuildParityCounters {
        self.counters
    }
}
