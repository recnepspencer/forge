use super::{batch_row, bridge_row, neighborhood_row, relational_row, row, signal_row};
use crate::application::{
    WorthQueryAdmittedConfiguredDomainHandle, WorthQueryCapabilityFamily,
    WorthQueryCapabilityStatus, WorthQueryConfigSectionFamily, WorthQueryDeclarationAspectContract,
    WorthQueryDeclarationAspectCoverage, WorthQueryDeclarationAspectFit,
    WorthQueryDeclarationFamilyMarker, WorthQueryDeclarationFamilyTaxonomy,
    WorthQueryDomainEntryMarker, WorthQueryDomainOperatingContext,
};
use crate::identity::hash_parts;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum WorthQueryDeclarationCapabilityVerb {
    Declare,
    RelationalTruthWitness,
    BridgeContinuationWitness,
    SignalCompatibilityWitness,
    NeighborhoodGroupingWitness,
    BatchGroupingWitness,
}

impl WorthQueryDeclarationCapabilityVerb {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Declare => "declare",
            Self::RelationalTruthWitness => "relational_truth_witness",
            Self::BridgeContinuationWitness => "bridge_continuation_witness",
            Self::SignalCompatibilityWitness => "signal_compatibility_witness",
            Self::NeighborhoodGroupingWitness => "neighborhood_grouping_witness",
            Self::BatchGroupingWitness => "batch_grouping_witness",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum WorthQueryDeclarationCapabilityStatus {
    Admitted,
    DeferredDebt,
    Unsupported,
    InvalidContext,
}

impl WorthQueryDeclarationCapabilityStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Admitted => "admitted",
            Self::DeferredDebt => "deferred_debt",
            Self::Unsupported => "unsupported",
            Self::InvalidContext => "invalid_context",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryDeclarationFamilySupportRow {
    verb: WorthQueryDeclarationCapabilityVerb,
    status: WorthQueryDeclarationCapabilityStatus,
    aspect_fit: WorthQueryDeclarationAspectFit,
    reason: &'static str,
}

impl WorthQueryDeclarationFamilySupportRow {
    pub(crate) fn new(
        verb: WorthQueryDeclarationCapabilityVerb,
        status: WorthQueryDeclarationCapabilityStatus,
        aspect_fit: WorthQueryDeclarationAspectFit,
        reason: &'static str,
    ) -> Self {
        Self {
            verb,
            status,
            aspect_fit,
            reason,
        }
    }

    pub fn verb(&self) -> WorthQueryDeclarationCapabilityVerb {
        self.verb
    }

    pub fn status(&self) -> WorthQueryDeclarationCapabilityStatus {
        self.status
    }

    pub fn aspect_fit(&self) -> WorthQueryDeclarationAspectFit {
        self.aspect_fit
    }

    pub fn reason(&self) -> &'static str {
        self.reason
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct WorthQueryDeclarationFamilySupportReport<
    D: WorthQueryDomainEntryMarker,
    F: WorthQueryDeclarationFamilyMarker<D>,
> {
    domain_key: &'static str,
    declaration_family_key: &'static str,
    declaration_taxonomy: WorthQueryDeclarationFamilyTaxonomy,
    aspect_contract: WorthQueryDeclarationAspectContract,
    aspect_coverage: WorthQueryDeclarationAspectCoverage,
    required_capability_families: Vec<WorthQueryCapabilityFamily>,
    required_config_sections: Vec<WorthQueryConfigSectionFamily>,
    rows: Vec<WorthQueryDeclarationFamilySupportRow>,
    support_digest: String,
    _marker: std::marker::PhantomData<(D, F)>,
}

impl<D: WorthQueryDomainEntryMarker, F: WorthQueryDeclarationFamilyMarker<D>> Clone
    for WorthQueryDeclarationFamilySupportReport<D, F>
{
    fn clone(&self) -> Self {
        Self {
            domain_key: self.domain_key,
            declaration_family_key: self.declaration_family_key,
            declaration_taxonomy: self.declaration_taxonomy,
            aspect_contract: self.aspect_contract.clone(),
            aspect_coverage: self.aspect_coverage.clone(),
            required_capability_families: self.required_capability_families.clone(),
            required_config_sections: self.required_config_sections.clone(),
            rows: self.rows.clone(),
            support_digest: self.support_digest.clone(),
            _marker: std::marker::PhantomData,
        }
    }
}

impl<D: WorthQueryDomainEntryMarker, F: WorthQueryDeclarationFamilyMarker<D>>
    WorthQueryDeclarationFamilySupportReport<D, F>
{
    pub(crate) fn new(
        domain_key: &'static str,
        declaration_family_key: &'static str,
        declaration_taxonomy: WorthQueryDeclarationFamilyTaxonomy,
        aspect_contract: WorthQueryDeclarationAspectContract,
        aspect_coverage: WorthQueryDeclarationAspectCoverage,
        required_capability_families: Vec<WorthQueryCapabilityFamily>,
        required_config_sections: Vec<WorthQueryConfigSectionFamily>,
        rows: Vec<WorthQueryDeclarationFamilySupportRow>,
        support_digest: String,
    ) -> Self {
        Self {
            domain_key,
            declaration_family_key,
            declaration_taxonomy,
            aspect_contract,
            aspect_coverage,
            required_capability_families,
            required_config_sections,
            rows,
            support_digest,
            _marker: std::marker::PhantomData,
        }
    }

    pub fn domain_key(&self) -> &'static str {
        self.domain_key
    }
    pub fn declaration_family_key(&self) -> &'static str {
        self.declaration_family_key
    }
    pub fn declaration_taxonomy(&self) -> WorthQueryDeclarationFamilyTaxonomy {
        self.declaration_taxonomy
    }
    pub fn aspect_contract(&self) -> &WorthQueryDeclarationAspectContract {
        &self.aspect_contract
    }
    pub fn aspect_coverage(&self) -> &WorthQueryDeclarationAspectCoverage {
        &self.aspect_coverage
    }
    pub fn required_capability_families(&self) -> &[WorthQueryCapabilityFamily] {
        &self.required_capability_families
    }
    pub fn required_config_sections(&self) -> &[WorthQueryConfigSectionFamily] {
        &self.required_config_sections
    }
    pub fn rows(&self) -> &[WorthQueryDeclarationFamilySupportRow] {
        &self.rows
    }
    pub fn support_digest(&self) -> &str {
        &self.support_digest
    }

    pub fn row(
        &self,
        verb: WorthQueryDeclarationCapabilityVerb,
    ) -> Option<&WorthQueryDeclarationFamilySupportRow> {
        self.rows.iter().find(|row| row.verb() == verb)
    }

    pub fn declare_status(&self) -> WorthQueryDeclarationCapabilityStatus {
        self.row(WorthQueryDeclarationCapabilityVerb::Declare)
            .expect("declare row must exist")
            .status()
    }
}

pub(crate) fn derive_family_support_report<
    D: WorthQueryDomainEntryMarker,
    C: WorthQueryDomainOperatingContext<D>,
    F: WorthQueryDeclarationFamilyMarker<D>,
>(
    handle: &WorthQueryAdmittedConfiguredDomainHandle<D, C>,
) -> WorthQueryDeclarationFamilySupportReport<D, F> {
    let taxonomy = F::taxonomy();
    let family_status = family_status::<D, C, F>(handle);
    let aspect_contract = F::aspect_contract();
    let aspect_coverage = F::aspect_coverage();
    let admitted_fit = aspect_coverage.fit_against(&aspect_contract);
    let required_capability_families = F::required_capability_families().to_vec();
    let required_config_sections = F::required_config_sections().to_vec();
    let rows = vec![
        row(
            WorthQueryDeclarationCapabilityVerb::Declare,
            family_status,
            admitted_fit,
        ),
        relational_row(family_status, taxonomy, admitted_fit),
        bridge_row(family_status, taxonomy, admitted_fit),
        signal_row(family_status, taxonomy, admitted_fit),
        neighborhood_row(family_status, taxonomy, admitted_fit),
        batch_row(family_status, taxonomy, admitted_fit),
    ];
    let support_digest = hash_parts(&[
        format!("domain:{}", handle.domain_key()),
        format!("handle:{}", handle.handle_identity_digest()),
        format!("family:{}", F::semantic_family_key()),
        format!("taxonomy:{taxonomy:?}"),
        format!("aspects:{aspect_contract:?}"),
        format!("aspect_coverage:{aspect_coverage:?}"),
        format!(
            "capabilities:{}",
            required_capability_families
                .iter()
                .map(WorthQueryCapabilityFamily::as_str)
                .collect::<Vec<_>>()
                .join(",")
        ),
        format!(
            "sections:{}",
            required_config_sections
                .iter()
                .map(WorthQueryConfigSectionFamily::as_str)
                .collect::<Vec<_>>()
                .join(",")
        ),
        rows.iter()
            .map(|row| {
                format!(
                    "{}:{}:{:?}:{}",
                    row.verb().as_str(),
                    row.status().as_str(),
                    row.aspect_fit(),
                    row.reason()
                )
            })
            .collect::<Vec<_>>()
            .join("|"),
    ]);
    WorthQueryDeclarationFamilySupportReport::new(
        handle.domain_key(),
        F::semantic_family_key(),
        taxonomy,
        aspect_contract,
        aspect_coverage,
        required_capability_families,
        required_config_sections,
        rows,
        support_digest,
    )
}

fn family_status<
    D: WorthQueryDomainEntryMarker,
    C: WorthQueryDomainOperatingContext<D>,
    F: WorthQueryDeclarationFamilyMarker<D>,
>(
    handle: &WorthQueryAdmittedConfiguredDomainHandle<D, C>,
) -> WorthQueryDeclarationCapabilityStatus {
    if F::required_capability_families()
        .iter()
        .copied()
        .any(|family| {
            handle.support_snapshot().capability_status(family)
                == Some(WorthQueryCapabilityStatus::DeferredDebt)
        })
    {
        return WorthQueryDeclarationCapabilityStatus::DeferredDebt;
    }
    if F::required_config_sections()
        .iter()
        .copied()
        .any(|section| {
            handle
                .support_snapshot()
                .section_postures()
                .iter()
                .find(|posture| posture.section() == section)
                .is_some_and(|posture| !posture.enabled())
        })
    {
        return WorthQueryDeclarationCapabilityStatus::InvalidContext;
    }
    if F::required_capability_families()
        .iter()
        .copied()
        .any(|family| {
            handle.support_snapshot().capability_status(family)
                == Some(WorthQueryCapabilityStatus::Unsupported)
        })
    {
        return WorthQueryDeclarationCapabilityStatus::Unsupported;
    }
    WorthQueryDeclarationCapabilityStatus::Admitted
}
