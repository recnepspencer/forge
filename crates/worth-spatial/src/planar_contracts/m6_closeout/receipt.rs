use super::{
    M6PlanarCloseoutBasis, M6PlanarCloseoutCounters, M6PremetabossFamily,
    M6ShortcutDeletionFamily,
};
use crate::planar_contracts::contract_bundle::PlanarContractBundleBooleanResult;
use crate::planar_contracts::contract_bundle::{
    planar_contract_bundle_digest, PlanarContractBundleImprintAction,
};

#[derive(Clone, Debug, PartialEq)]
pub struct M6PlanarCloseoutReceipt {
    basis: M6PlanarCloseoutBasis,
    declaration_digest: String,
    envelope_digest: String,
    closeout_digest: String,
    counters: M6PlanarCloseoutCounters,
}

impl M6PlanarCloseoutReceipt {
    pub(crate) fn new(
        basis: M6PlanarCloseoutBasis,
        declaration_digest: impl Into<String>,
        envelope_digest: impl Into<String>,
    ) -> Self {
        let declaration_digest = declaration_digest.into();
        let envelope_digest = envelope_digest.into();
        let closeout_digest = closeout_digest_for(&basis, &declaration_digest, &envelope_digest);
        let counters = M6PlanarCloseoutCounters::certified(
            basis.premetaboss_rows().len(),
            basis.legacy_deletion_rows().len(),
            1,
            basis.closeout_rows(),
            basis.legacy_deletion_rows().len(),
        );
        Self {
            basis,
            declaration_digest,
            envelope_digest,
            closeout_digest,
            counters,
        }
    }

    pub fn proves_all_premetaboss_families(&self) -> bool {
        self.basis.premetaboss_rows().len() == M6PremetabossFamily::ALL.len()
    }

    pub fn proves_no_kernel_local_planar_shortcuts(&self) -> bool {
        self.basis.legacy_deletion_rows().len() == M6ShortcutDeletionFamily::ALL.len()
    }

    pub fn proves_query_owned_runtime_lanes(&self) -> bool {
        self.basis.query_boundary().declaration_digest() == self.basis.readiness().declaration_digest()
            && self.basis.query_boundary().envelope_digest() == self.basis.readiness().envelope_digest()
    }

    pub fn boolean_result(&self) -> Option<PlanarContractBundleBooleanResult> {
        None
    }

    pub fn imprint_action(&self) -> Option<PlanarContractBundleImprintAction> {
        None
    }

    pub fn basis(&self) -> &M6PlanarCloseoutBasis {
        &self.basis
    }

    pub fn declaration_digest(&self) -> &str {
        &self.declaration_digest
    }

    pub fn envelope_digest(&self) -> &str {
        &self.envelope_digest
    }

    pub fn closeout_digest(&self) -> &str {
        &self.closeout_digest
    }

    pub fn counters(&self) -> M6PlanarCloseoutCounters {
        self.counters
    }
}

fn closeout_digest_for(
    basis: &M6PlanarCloseoutBasis,
    declaration_digest: &str,
    envelope_digest: &str,
) -> String {
    let mut parts = vec![
        format!("m7_readiness:{}", basis.readiness().readiness_digest()),
        format!("query_declaration:{declaration_digest}"),
        format!("query_envelope:{envelope_digest}"),
    ];
    parts.extend(basis.premetaboss_rows().iter().map(|row| {
        format!(
            "premetaboss:{}:{}",
            row.family().as_str(),
            row.evidence_digest()
        )
    }));
    parts.extend(basis.legacy_deletion_rows().iter().map(|row| {
        format!(
            "legacy_deletion:{}:{}",
            row.family().as_str(),
            row.evidence_digest()
        )
    }));
    parts.sort();
    planar_contract_bundle_digest(&parts)
}
