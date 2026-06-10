use forge_query::facade::{
    readmit_lower_runtime_evidence, DeniedBasisCapability,
    ForgeQueryAdmittedConfiguredDomainHandle, ForgeQueryDeclarationEntryInspectionInput,
    ForgeQueryDomainOperatingContext, LowerRuntimeBasisEvidence, ScopedInspectionBasis,
};

use crate::bindings::query_native_retained_planar_facts::authoring::{
    retained_planar_facts_entry, RetainedPlanarFactsCase, RetainedPlanarFactsEntry,
};
use crate::bindings::query_native_retained_planar_facts::domain::RetainedPlanarFactsQueryDomain;
use crate::bindings::query_native_retained_planar_facts::facts::{
    retained_planar_facts, RetainedPlanarFactsFactError,
};
use crate::bindings::query_native_retained_planar_facts::inspection::RetainedPlanarFactsInspectionRow;
use crate::planar_contracts::contract_bundle::PlanarContractBundleValidationReceipt;
use crate::planar_contracts::motion_posture::PlanarMotionPostureReceipt;
use crate::planar_contracts::retained_planar_facts::{
    RetainedPlanarBranchLocalInspection, RetainedPlanarFactsBasis, RetainedPlanarFactsDenial,
    RetainedPlanarFactsDenialKind, RetainedPlanarFactsReceipt, RetainedPlanarFactsReplaySubject,
    RetainedPlanarHistoricalInspection,
};
use crate::planar_contracts::structural_identity::PlanarStructuralIdentityReceipt;
use crate::planar_contracts::topology_contract_completeness::PlanarTopologyContractCompletenessReceipt;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct RetainedPlanarFacts {
    builder: crate::planar_contracts::retained_planar_facts::RetainedPlanarFactsBuilder,
}

impl RetainedPlanarFacts {
    pub fn from_boolean_readiness(receipt: PlanarContractBundleValidationReceipt) -> Self {
        Self {
            builder: RetainedPlanarFactsBasis::builder().boolean_readiness_receipt(receipt),
        }
    }

    pub fn retain_planar_classification(mut self) -> Self {
        self.builder = self.builder.retain_planar_classification();
        self
    }

    pub fn retain_structural_identity(mut self, receipt: PlanarStructuralIdentityReceipt) -> Self {
        self.builder = self.builder.structural_identity_receipt(receipt);
        self
    }

    pub fn retain_motion_posture(mut self, receipt: PlanarMotionPostureReceipt) -> Self {
        self.builder = self.builder.motion_posture_receipt(receipt);
        self
    }

    pub fn retain_topology_contract(
        mut self,
        receipt: PlanarTopologyContractCompletenessReceipt,
    ) -> Self {
        self.builder = self.builder.topology_contract_receipt(receipt);
        self
    }

    pub fn compile<'a, WC>(
        self,
        contracts: &'a RetainedPlanarFactsContracts<WC>,
    ) -> Result<RetainedPlanarFactsPlan<'a, WC>, RetainedPlanarFactsDenial>
    where
        WC: ForgeQueryDomainOperatingContext<RetainedPlanarFactsQueryDomain>,
    {
        let basis = self.builder.build()?;
        let entry = retained_planar_facts_entry(RetainedPlanarFactsCase::from_basis(basis));
        Ok(RetainedPlanarFactsPlan { entry, contracts })
    }
}

pub struct RetainedPlanarFactsContracts<WC>
where
    WC: ForgeQueryDomainOperatingContext<RetainedPlanarFactsQueryDomain>,
{
    retained_handle: ForgeQueryAdmittedConfiguredDomainHandle<RetainedPlanarFactsQueryDomain, WC>,
}

impl<WC> RetainedPlanarFactsContracts<WC>
where
    WC: ForgeQueryDomainOperatingContext<RetainedPlanarFactsQueryDomain>,
{
    pub fn new(
        retained_handle: ForgeQueryAdmittedConfiguredDomainHandle<
            RetainedPlanarFactsQueryDomain,
            WC,
        >,
    ) -> Self {
        Self { retained_handle }
    }

    fn inspect_retained_planar_receipt(
        &self,
        receipt: &RetainedPlanarFactsReceipt,
    ) -> Result<
        forge_query::facade::ForgeQueryDeclarationEntryInspection<
            RetainedPlanarFactsQueryDomain,
            RetainedPlanarFactsEntry,
        >,
        RetainedPlanarFactsDenial,
    > {
        let entry = retained_planar_facts_entry(RetainedPlanarFactsCase::from_basis(
            receipt.basis().clone(),
        ));
        let progressed = self
            .retained_handle
            .declare_review_and_progress(entry)
            .map_err(|_| {
                RetainedPlanarFactsDenial::new(
                    RetainedPlanarFactsDenialKind::TruncatedRetainedBasis,
                    "retained planar replay requires a Query-progressed retained declaration entry before inspection",
                )
            })?;
        let checked = self
            .retained_handle
            .orchestrate_envelope_from_progressed_checked(progressed);
        self.retained_handle
            .inspect_declaration_entry(ForgeQueryDeclarationEntryInspectionInput::envelope_checked(
                checked,
            ))
            .map_err(|_| {
                RetainedPlanarFactsDenial::new(
                    RetainedPlanarFactsDenialKind::TruncatedRetainedBasis,
                    "retained planar replay requires inspectable Query declaration-entry artifacts",
                )
            })
    }
}

pub struct RetainedPlanarFactsPlan<'a, WC>
where
    WC: ForgeQueryDomainOperatingContext<RetainedPlanarFactsQueryDomain>,
{
    entry: RetainedPlanarFactsEntry,
    contracts: &'a RetainedPlanarFactsContracts<WC>,
}

impl<WC> RetainedPlanarFactsPlan<'_, WC>
where
    WC: ForgeQueryDomainOperatingContext<RetainedPlanarFactsQueryDomain>,
{
    pub fn inspected_retained_rows(&self) -> usize {
        RetainedPlanarFactsInspectionRow::from_basis(self.entry.case().basis()).len()
    }

    pub fn retain(self) -> Result<RetainedPlanarFactsReceipt, RetainedPlanarFactsFactError> {
        retained_planar_facts(&self.entry, &self.contracts.retained_handle)
    }
}

impl RetainedPlanarFactsReceipt {
    pub fn historical_inspection(&self) -> RetainedPlanarHistoricalInspectionBuilder<'_> {
        RetainedPlanarHistoricalInspectionBuilder {
            receipt: self,
            subject: None,
        }
    }

    pub fn branch_local_inspection(
        &self,
        scoped_basis: ScopedInspectionBasis,
        evidence: LowerRuntimeBasisEvidence,
    ) -> RetainedPlanarBranchLocalInspectionBuilder<'_> {
        RetainedPlanarBranchLocalInspectionBuilder {
            receipt: self,
            scoped_basis,
            evidence,
            subject: None,
        }
    }
}

pub struct RetainedPlanarHistoricalInspectionBuilder<'a> {
    receipt: &'a RetainedPlanarFactsReceipt,
    subject: Option<RetainedPlanarFactsReplaySubject>,
}

impl<'a> RetainedPlanarHistoricalInspectionBuilder<'a> {
    pub fn against_replay_subject(mut self, subject: RetainedPlanarFactsReplaySubject) -> Self {
        self.subject = Some(subject);
        self
    }

    pub fn inspect<WC>(
        self,
        contracts: &RetainedPlanarFactsContracts<WC>,
    ) -> Result<RetainedPlanarHistoricalInspection, RetainedPlanarFactsDenial>
    where
        WC: ForgeQueryDomainOperatingContext<RetainedPlanarFactsQueryDomain>,
    {
        let inspection = contracts.inspect_retained_planar_receipt(self.receipt)?;
        let subject = self
            .subject
            .unwrap_or_else(|| self.receipt.replay_subject());
        assert_query_inspection_matches_replay_subject(&inspection, &subject)?;
        self.receipt.historical_replay(&subject)
    }
}

pub struct RetainedPlanarBranchLocalInspectionBuilder<'a> {
    receipt: &'a RetainedPlanarFactsReceipt,
    scoped_basis: ScopedInspectionBasis,
    evidence: LowerRuntimeBasisEvidence,
    subject: Option<RetainedPlanarFactsReplaySubject>,
}

impl<'a> RetainedPlanarBranchLocalInspectionBuilder<'a> {
    pub fn against_replay_subject(mut self, subject: RetainedPlanarFactsReplaySubject) -> Self {
        self.subject = Some(subject);
        self
    }

    pub fn inspect<WC>(
        self,
        contracts: &RetainedPlanarFactsContracts<WC>,
    ) -> Result<RetainedPlanarBranchLocalInspection, RetainedPlanarBranchLocalInspectionError>
    where
        WC: ForgeQueryDomainOperatingContext<RetainedPlanarFactsQueryDomain>,
    {
        let bound_basis = readmit_lower_runtime_evidence(self.scoped_basis, self.evidence)
            .map_err(RetainedPlanarBranchLocalInspectionError::LowerRuntimeBasis)?;
        let inspection = contracts
            .inspect_retained_planar_receipt(self.receipt)
            .map_err(RetainedPlanarBranchLocalInspectionError::RetainedPlanarFacts)?;
        let subject = self
            .subject
            .unwrap_or_else(|| self.receipt.replay_subject());
        assert_query_inspection_matches_replay_subject(&inspection, &subject)
            .map_err(RetainedPlanarBranchLocalInspectionError::RetainedPlanarFacts)?;
        self.receipt
            .branch_local_replay(&subject, &bound_basis)
            .map_err(RetainedPlanarBranchLocalInspectionError::RetainedPlanarFacts)
    }
}

fn assert_query_inspection_matches_replay_subject(
    inspection: &forge_query::facade::ForgeQueryDeclarationEntryInspection<
        RetainedPlanarFactsQueryDomain,
        RetainedPlanarFactsEntry,
    >,
    subject: &RetainedPlanarFactsReplaySubject,
) -> Result<(), RetainedPlanarFactsDenial> {
    if inspection.progression_digest().is_none()
        || inspection.route_plan_digest().is_none()
        || inspection.receipt_digest().is_none()
    {
        return Err(RetainedPlanarFactsDenial::new(
            RetainedPlanarFactsDenialKind::TruncatedRetainedBasis,
            "retained planar replay requires retained progression, route-plan, and receipt truth before interpretation",
        ));
    }
    if inspection.declaration_digest() != subject.declaration_digest()
        || inspection.progression_digest() != Some(subject.progression_digest())
        || inspection.route_plan_digest() != Some(subject.route_plan_digest())
        || inspection.receipt_digest() != Some(subject.query_receipt_digest())
        || inspection.envelope_digest() != subject.envelope_digest()
    {
        return Err(RetainedPlanarFactsDenial::new(
            RetainedPlanarFactsDenialKind::TruncatedRetainedBasis,
            format!(
                "retained planar replay requires Query inspection artifacts to match the retained replay subject: inspected=({}, {:?}, {:?}, {:?}, {}) subject=({}, {}, {}, {}, {})",
                inspection.declaration_digest(),
                inspection.progression_digest(),
                inspection.route_plan_digest(),
                inspection.receipt_digest(),
                inspection.envelope_digest(),
                subject.declaration_digest(),
                subject.progression_digest(),
                subject.route_plan_digest(),
                subject.query_receipt_digest(),
                subject.envelope_digest(),
            ),
        ));
    }
    Ok(())
}

#[derive(Clone, Debug, PartialEq)]
pub enum RetainedPlanarBranchLocalInspectionError {
    LowerRuntimeBasis(DeniedBasisCapability),
    RetainedPlanarFacts(RetainedPlanarFactsDenial),
}

impl RetainedPlanarBranchLocalInspectionError {
    pub fn reason(&self) -> &str {
        match self {
            Self::LowerRuntimeBasis(_) => {
                "retained planar branch-local replay requires readmitted lower-runtime evidence for the same branch-scoped inspection basis"
            }
            Self::RetainedPlanarFacts(error) => error.reason(),
        }
    }
}
