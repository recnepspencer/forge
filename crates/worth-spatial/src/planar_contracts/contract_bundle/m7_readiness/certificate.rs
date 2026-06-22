use super::family_set::m7_readiness_family_rows;
use super::{PlanarM7ReadinessBasis, PlanarM7ReadinessCounters, PlanarM7ReadinessFamilyRow};
use crate::planar_contracts::contract_bundle::{
    planar_contract_bundle_digest, PlanarContractBundleBooleanResult,
    PlanarContractBundleImprintAction,
};
use crate::planar_contracts::local_frame::PlanarLocalFrameCertificateReceipt;
use crate::planar_contracts::precision_basis::PlanarPrecisionCertificateReceipt;

#[derive(Clone, Debug, PartialEq)]
pub struct PlanarM7ReadinessReceipt {
    basis: PlanarM7ReadinessBasis,
    family_rows: Vec<PlanarM7ReadinessFamilyRow>,
    declaration_digest: String,
    envelope_digest: String,
    readiness_digest: String,
    counters: PlanarM7ReadinessCounters,
}

impl PlanarM7ReadinessReceipt {
    pub(crate) fn new(
        basis: PlanarM7ReadinessBasis,
        declaration_digest: impl Into<String>,
        envelope_digest: impl Into<String>,
    ) -> Self {
        let family_rows = m7_readiness_family_rows(&basis);
        let declaration_digest = declaration_digest.into();
        let envelope_digest = envelope_digest.into();
        let readiness_digest =
            readiness_digest_for(&family_rows, &declaration_digest, &envelope_digest);
        let counters = PlanarM7ReadinessCounters::certified(
            family_rows.len(),
            basis
                .retained_planar_facts()
                .counters()
                .retained_fact_rows_inspected(),
            basis
                .projection_consumed_facts()
                .counters()
                .projection_consumption_breadth(),
            1,
        );
        Self {
            basis,
            family_rows,
            declaration_digest,
            envelope_digest,
            readiness_digest,
            counters,
        }
    }

    pub fn is_acceptable_m7_input(&self) -> bool {
        true
    }

    pub fn boolean_result(&self) -> Option<PlanarContractBundleBooleanResult> {
        None
    }

    pub fn imprint_action(&self) -> Option<PlanarContractBundleImprintAction> {
        None
    }

    pub fn family_rows(&self) -> &[PlanarM7ReadinessFamilyRow] {
        &self.family_rows
    }

    pub fn readiness_digest(&self) -> &str {
        &self.readiness_digest
    }

    pub fn declaration_digest(&self) -> &str {
        &self.declaration_digest
    }

    pub fn envelope_digest(&self) -> &str {
        &self.envelope_digest
    }

    pub fn counters(&self) -> PlanarM7ReadinessCounters {
        self.counters
    }

    pub fn precision_receipt(&self) -> &PlanarPrecisionCertificateReceipt {
        self.basis.boolean_readiness().basis().precision_receipt()
    }

    pub fn local_frame_receipt(&self) -> &PlanarLocalFrameCertificateReceipt {
        self.basis.boolean_readiness().basis().local_frame_receipt()
    }

    pub fn precision_fact_digest(&self) -> &str {
        self.precision_receipt().fact_digest()
    }

    pub fn local_frame_fact_digest(&self) -> &str {
        self.local_frame_receipt().fact_digest()
    }

    pub fn topology_basis_identity(&self) -> &str {
        self.basis
            .boolean_readiness()
            .basis()
            .topology_basis_identity()
    }

    pub fn movement_rotation_posture_identity(&self) -> &str {
        self.basis
            .boolean_readiness()
            .basis()
            .movement_rotation_posture_identity()
    }
}

fn readiness_digest_for(
    rows: &[PlanarM7ReadinessFamilyRow],
    declaration_digest: &str,
    envelope_digest: &str,
) -> String {
    let mut parts = rows
        .iter()
        .map(|row| {
            format!(
                "{}:{}:{}:{}",
                row.family().as_str(),
                row.receipt_digest(),
                row.declaration_digest(),
                row.envelope_digest()
            )
        })
        .collect::<Vec<_>>();
    parts.push(format!("query_declaration:{declaration_digest}"));
    parts.push(format!("query_envelope:{envelope_digest}"));
    parts.sort();
    planar_contract_bundle_digest(&parts)
}
