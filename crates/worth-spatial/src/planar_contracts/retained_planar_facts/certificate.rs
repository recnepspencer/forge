use forge_query::facade::{BasisFamily, LowerRuntimeBoundBasis, ScopedInspectionBasis};

use super::{
    retained_planar_fact_authority_entries, retained_planar_fact_digest, RetainedPlanarFactsBasis,
    RetainedPlanarFactsCounters, RetainedPlanarFactsDenial, RetainedPlanarFactsDenialKind,
};

#[derive(Clone, Debug, PartialEq)]
pub struct RetainedPlanarFactsReceipt {
    basis: RetainedPlanarFactsBasis,
    declaration_digest: String,
    progression_digest: String,
    route_plan_digest: String,
    query_receipt_digest: String,
    envelope_digest: String,
    retained_fact_digest: String,
    counters: RetainedPlanarFactsCounters,
}

impl RetainedPlanarFactsReceipt {
    pub(crate) fn new(
        basis: RetainedPlanarFactsBasis,
        declaration_digest: String,
        progression_digest: String,
        route_plan_digest: String,
        query_receipt_digest: String,
        envelope_digest: String,
        retained_fact_digest: String,
        counters: RetainedPlanarFactsCounters,
    ) -> Self {
        Self {
            basis,
            declaration_digest,
            progression_digest,
            route_plan_digest,
            query_receipt_digest,
            envelope_digest,
            retained_fact_digest,
            counters,
        }
    }

    pub(crate) fn retained_fact_digest_for(
        basis: &RetainedPlanarFactsBasis,
        declaration_digest: &str,
        progression_digest: &str,
        route_plan_digest: &str,
        query_receipt_digest: &str,
        envelope_digest: &str,
    ) -> String {
        let mut parts = retained_planar_fact_authority_entries(basis)
            .into_iter()
            .map(|entry| entry.digest_part())
            .collect::<Vec<_>>();
        parts.push(format!("declaration:{declaration_digest}"));
        parts.push(format!("progression:{progression_digest}"));
        parts.push(format!("route_plan:{route_plan_digest}"));
        parts.push(format!("query_receipt:{query_receipt_digest}"));
        parts.push(format!("envelope:{envelope_digest}"));
        retained_planar_fact_digest(&parts)
    }

    pub fn basis(&self) -> &RetainedPlanarFactsBasis {
        &self.basis
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

    pub fn retained_fact_digest(&self) -> &str {
        &self.retained_fact_digest
    }

    pub fn counters(&self) -> RetainedPlanarFactsCounters {
        self.counters
    }

    pub fn replay_subject(&self) -> RetainedPlanarFactsReplaySubject {
        RetainedPlanarFactsReplaySubject {
            declaration_digest: self.declaration_digest.clone(),
            progression_digest: self.progression_digest.clone(),
            route_plan_digest: self.route_plan_digest.clone(),
            query_receipt_digest: self.query_receipt_digest.clone(),
            envelope_digest: self.envelope_digest.clone(),
            retained_fact_digest: self.retained_fact_digest.clone(),
        }
    }

    pub fn historical_replay(
        &self,
        subject: &RetainedPlanarFactsReplaySubject,
    ) -> Result<RetainedPlanarHistoricalInspection, RetainedPlanarFactsDenial> {
        assert_replay_subject_matches(self, subject)?;
        Ok(RetainedPlanarHistoricalInspection {
            retained_fact_digest: self.retained_fact_digest.clone(),
            historical_digest: retained_planar_fact_digest(&[
                "historical_retained_planar_replay".to_string(),
                format!("fact:{}", self.retained_fact_digest),
                format!("subject:{}", subject.declaration_digest),
                format!("progression:{}", subject.progression_digest),
                format!("route_plan:{}", subject.route_plan_digest),
                format!("query_receipt:{}", subject.query_receipt_digest),
                format!("envelope:{}", subject.envelope_digest),
            ]),
            counters: RetainedPlanarFactsCounters::historical_replay(
                retained_family_rows(self),
                retained_fact_rows(self),
                1,
            ),
        })
    }

    pub fn branch_local_replay(
        &self,
        subject: &RetainedPlanarFactsReplaySubject,
        bound_basis: &LowerRuntimeBoundBasis<ScopedInspectionBasis>,
    ) -> Result<RetainedPlanarBranchLocalInspection, RetainedPlanarFactsDenial> {
        assert_replay_subject_matches(self, subject)?;
        if !matches!(
            bound_basis.scoped_basis().family(),
            BasisFamily::BranchHead | BasisFamily::BranchSnapshot
        ) {
            return Err(RetainedPlanarFactsDenial::new(
                RetainedPlanarFactsDenialKind::UnsupportedBranchBasis,
                "retained planar branch-local replay requires a branch-head or branch-snapshot basis",
            ));
        }
        Ok(RetainedPlanarBranchLocalInspection {
            retained_fact_digest: self.retained_fact_digest.clone(),
            branch_basis_digest: bound_basis.scoped_basis().scoped_basis_digest().to_string(),
            branch_binding_digest: bound_basis.lower_runtime_binding_digest().to_string(),
            branch_local_digest: retained_planar_fact_digest(&[
                "branch_local_retained_planar_replay".to_string(),
                format!("fact:{}", self.retained_fact_digest),
                format!("subject:{}", subject.declaration_digest),
                format!("progression:{}", subject.progression_digest),
                format!("route_plan:{}", subject.route_plan_digest),
                format!("query_receipt:{}", subject.query_receipt_digest),
                format!(
                    "branch_basis:{}",
                    bound_basis.scoped_basis().scoped_basis_digest()
                ),
                format!(
                    "branch_binding:{}",
                    bound_basis.lower_runtime_binding_digest()
                ),
            ]),
            counters: RetainedPlanarFactsCounters::branch_local_replay(
                retained_family_rows(self),
                retained_fact_rows(self),
                1,
                1,
            ),
        })
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RetainedPlanarFactsReplaySubject {
    declaration_digest: String,
    progression_digest: String,
    route_plan_digest: String,
    query_receipt_digest: String,
    envelope_digest: String,
    retained_fact_digest: String,
}

impl RetainedPlanarFactsReplaySubject {
    pub fn new(
        declaration_digest: impl Into<String>,
        progression_digest: impl Into<String>,
        route_plan_digest: impl Into<String>,
        query_receipt_digest: impl Into<String>,
        envelope_digest: impl Into<String>,
        retained_fact_digest: impl Into<String>,
    ) -> Self {
        Self {
            declaration_digest: declaration_digest.into(),
            progression_digest: progression_digest.into(),
            route_plan_digest: route_plan_digest.into(),
            query_receipt_digest: query_receipt_digest.into(),
            envelope_digest: envelope_digest.into(),
            retained_fact_digest: retained_fact_digest.into(),
        }
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

    pub fn retained_fact_digest(&self) -> &str {
        &self.retained_fact_digest
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RetainedPlanarHistoricalInspection {
    retained_fact_digest: String,
    historical_digest: String,
    counters: RetainedPlanarFactsCounters,
}

impl RetainedPlanarHistoricalInspection {
    pub fn retained_fact_digest(&self) -> &str {
        &self.retained_fact_digest
    }

    pub fn historical_digest(&self) -> &str {
        &self.historical_digest
    }

    pub fn counters(&self) -> RetainedPlanarFactsCounters {
        self.counters
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RetainedPlanarBranchLocalInspection {
    retained_fact_digest: String,
    branch_basis_digest: String,
    branch_binding_digest: String,
    branch_local_digest: String,
    counters: RetainedPlanarFactsCounters,
}

impl RetainedPlanarBranchLocalInspection {
    pub fn retained_fact_digest(&self) -> &str {
        &self.retained_fact_digest
    }

    pub fn branch_basis_digest(&self) -> &str {
        &self.branch_basis_digest
    }

    pub fn branch_binding_digest(&self) -> &str {
        &self.branch_binding_digest
    }

    pub fn branch_local_digest(&self) -> &str {
        &self.branch_local_digest
    }

    pub fn counters(&self) -> RetainedPlanarFactsCounters {
        self.counters
    }
}

fn assert_replay_subject_matches(
    receipt: &RetainedPlanarFactsReceipt,
    subject: &RetainedPlanarFactsReplaySubject,
) -> Result<(), RetainedPlanarFactsDenial> {
    if subject.declaration_digest() != receipt.declaration_digest()
        || subject.progression_digest() != receipt.progression_digest()
        || subject.route_plan_digest() != receipt.route_plan_digest()
        || subject.query_receipt_digest() != receipt.query_receipt_digest()
        || subject.envelope_digest() != receipt.envelope_digest()
        || subject.retained_fact_digest() != receipt.retained_fact_digest()
    {
        return Err(RetainedPlanarFactsDenial::new(
            RetainedPlanarFactsDenialKind::TruncatedRetainedBasis,
            "retained planar replay requires replay-subject digests to match the retained fact receipt exactly",
        ));
    }
    Ok(())
}

fn retained_family_rows(receipt: &RetainedPlanarFactsReceipt) -> usize {
    receipt
        .basis()
        .boolean_readiness_receipt()
        .basis()
        .family_rows()
        .len()
}

fn retained_fact_rows(receipt: &RetainedPlanarFactsReceipt) -> usize {
    receipt
        .basis()
        .boolean_readiness_receipt()
        .basis()
        .family_rows()
        .iter()
        .map(|row| row.retained_fact_digests().len())
        .sum()
}
