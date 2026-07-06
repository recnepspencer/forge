use worth_spatial::facade::boolean_readiness_workload::{
    PlanarBooleanReadinessStageCoverage, PlanarBooleanReadinessWorkloadDenialKind,
    PlanarBooleanReadinessWorkloadReceipt,
};

use super::error::PlanarBooleanEntryBasisError;
use super::query::query_backed_planar_boolean_entry_basis;
use std::collections::BTreeMap;
use std::sync::{Mutex, OnceLock};

#[derive(Clone, Debug, PartialEq)]
pub struct PlanarBooleanEntryBasis {
    readiness_receipt: PlanarBooleanReadinessWorkloadReceipt,
    stage_coverage: PlanarBooleanReadinessStageCoverage,
    query_intent: String,
    query_declaration_digest: String,
    query_envelope_digest: String,
    query_handle_digest: String,
}

impl PlanarBooleanEntryBasis {
    pub fn bind(
        readiness_receipt: PlanarBooleanReadinessWorkloadReceipt,
        query_intent: impl Into<String>,
    ) -> Result<Self, PlanarBooleanEntryBasisError> {
        let query_intent = query_intent.into();
        if query_intent.trim().is_empty() {
            return Err(PlanarBooleanEntryBasisError::MissingQueryDeclaration);
        }
        let cache_key = entry_basis_cache_key(&readiness_receipt, query_intent.trim());
        if let Some(basis) = entry_basis_cache()
            .lock()
            .expect("planar boolean entry basis cache should not be poisoned")
            .get(&cache_key)
            .cloned()
        {
            return Ok(basis);
        }
        let query_receipt =
            query_backed_planar_boolean_entry_basis(&readiness_receipt, query_intent.trim())?;
        let basis = Self {
            stage_coverage: readiness_receipt.stage_coverage().clone(),
            readiness_receipt,
            query_intent: query_intent.trim().to_string(),
            query_declaration_digest: query_receipt.declaration_digest().to_string(),
            query_envelope_digest: query_receipt.envelope_digest().to_string(),
            query_handle_digest: query_receipt.handle_digest().to_string(),
        };
        entry_basis_cache()
            .lock()
            .expect("planar boolean entry basis cache should not be poisoned")
            .insert(cache_key, basis.clone());
        Ok(basis)
    }

    pub fn readiness_receipt(&self) -> &PlanarBooleanReadinessWorkloadReceipt {
        &self.readiness_receipt
    }

    pub fn readiness_receipt_identity(&self) -> &str {
        self.readiness_receipt
            .m7_readiness_receipt()
            .readiness_digest()
    }

    pub fn readiness_workload_digest(&self) -> &str {
        self.readiness_receipt.workload_digest()
    }

    pub fn readiness_declaration_digest(&self) -> &str {
        self.readiness_receipt
            .m7_readiness_receipt()
            .declaration_digest()
    }

    pub fn readiness_envelope_digest(&self) -> &str {
        self.readiness_receipt
            .m7_readiness_receipt()
            .envelope_digest()
    }

    pub fn stage_coverage(&self) -> &PlanarBooleanReadinessStageCoverage {
        &self.stage_coverage
    }

    pub fn blocker_family(&self) -> Option<PlanarBooleanReadinessWorkloadDenialKind> {
        None
    }

    pub fn denial_identity(&self) -> Option<&str> {
        None
    }

    pub fn query_intent(&self) -> &str {
        &self.query_intent
    }

    pub fn query_declaration_digest(&self) -> &str {
        &self.query_declaration_digest
    }

    pub fn query_envelope_digest(&self) -> &str {
        &self.query_envelope_digest
    }

    pub fn query_handle_digest(&self) -> &str {
        &self.query_handle_digest
    }
}

fn entry_basis_cache() -> &'static Mutex<BTreeMap<String, PlanarBooleanEntryBasis>> {
    static CACHE: OnceLock<Mutex<BTreeMap<String, PlanarBooleanEntryBasis>>> = OnceLock::new();
    CACHE.get_or_init(|| Mutex::new(BTreeMap::new()))
}

fn entry_basis_cache_key(
    readiness: &PlanarBooleanReadinessWorkloadReceipt,
    query_intent: &str,
) -> String {
    format!(
        "{}::{}::{}",
        readiness.m7_readiness_receipt().readiness_digest(),
        readiness.workload_digest(),
        query_intent
    )
}
