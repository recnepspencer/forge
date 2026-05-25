use crate::application::{
    ForgeQueryAdmittedConfiguredDomainHandle, ForgeQueryCapabilityFamily,
    ForgeQueryCapabilityStatus, ForgeQueryConfigSectionFamily, ForgeQueryDeclarationFamilyMarker,
    ForgeQueryDeclarationFamilyTaxonomy, ForgeQueryDomainEntryMarker,
    ForgeQueryDomainOperatingContext,
};
use crate::identity::hash_parts;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum ForgeQueryDeclarationCapabilityVerb {
    Declare,
    RelationalTruthWitness,
    BridgeContinuationWitness,
    SignalCompatibilityWitness,
    NeighborhoodGroupingWitness,
    BatchGroupingWitness,
}

impl ForgeQueryDeclarationCapabilityVerb {
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
pub enum ForgeQueryDeclarationCapabilityStatus {
    Admitted,
    DeferredDebt,
    Unsupported,
    InvalidContext,
}

impl ForgeQueryDeclarationCapabilityStatus {
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
pub struct ForgeQueryDeclarationFamilySupportRow {
    verb: ForgeQueryDeclarationCapabilityVerb,
    status: ForgeQueryDeclarationCapabilityStatus,
    reason: &'static str,
}

impl ForgeQueryDeclarationFamilySupportRow {
    pub(crate) fn new(
        verb: ForgeQueryDeclarationCapabilityVerb,
        status: ForgeQueryDeclarationCapabilityStatus,
        reason: &'static str,
    ) -> Self {
        Self {
            verb,
            status,
            reason,
        }
    }

    pub fn verb(&self) -> ForgeQueryDeclarationCapabilityVerb {
        self.verb
    }

    pub fn status(&self) -> ForgeQueryDeclarationCapabilityStatus {
        self.status
    }

    pub fn reason(&self) -> &'static str {
        self.reason
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryDeclarationFamilySupportReport<
    D: ForgeQueryDomainEntryMarker,
    F: ForgeQueryDeclarationFamilyMarker<D>,
> {
    domain_key: &'static str,
    declaration_family_key: &'static str,
    declaration_taxonomy: ForgeQueryDeclarationFamilyTaxonomy,
    required_capability_families: Vec<ForgeQueryCapabilityFamily>,
    required_config_sections: Vec<ForgeQueryConfigSectionFamily>,
    rows: Vec<ForgeQueryDeclarationFamilySupportRow>,
    support_digest: String,
    _marker: std::marker::PhantomData<(D, F)>,
}

impl<D: ForgeQueryDomainEntryMarker, F: ForgeQueryDeclarationFamilyMarker<D>>
    ForgeQueryDeclarationFamilySupportReport<D, F>
{
    pub(crate) fn new(
        domain_key: &'static str,
        declaration_family_key: &'static str,
        declaration_taxonomy: ForgeQueryDeclarationFamilyTaxonomy,
        required_capability_families: Vec<ForgeQueryCapabilityFamily>,
        required_config_sections: Vec<ForgeQueryConfigSectionFamily>,
        rows: Vec<ForgeQueryDeclarationFamilySupportRow>,
        support_digest: String,
    ) -> Self {
        Self {
            domain_key,
            declaration_family_key,
            declaration_taxonomy,
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
    pub fn declaration_taxonomy(&self) -> ForgeQueryDeclarationFamilyTaxonomy {
        self.declaration_taxonomy
    }
    pub fn required_capability_families(&self) -> &[ForgeQueryCapabilityFamily] {
        &self.required_capability_families
    }
    pub fn required_config_sections(&self) -> &[ForgeQueryConfigSectionFamily] {
        &self.required_config_sections
    }
    pub fn rows(&self) -> &[ForgeQueryDeclarationFamilySupportRow] {
        &self.rows
    }
    pub fn support_digest(&self) -> &str {
        &self.support_digest
    }

    pub fn row(
        &self,
        verb: ForgeQueryDeclarationCapabilityVerb,
    ) -> Option<&ForgeQueryDeclarationFamilySupportRow> {
        self.rows.iter().find(|row| row.verb() == verb)
    }

    pub fn declare_status(&self) -> ForgeQueryDeclarationCapabilityStatus {
        self.row(ForgeQueryDeclarationCapabilityVerb::Declare)
            .expect("declare row must exist")
            .status()
    }
}

pub(crate) fn derive_family_support_report<
    D: ForgeQueryDomainEntryMarker,
    C: ForgeQueryDomainOperatingContext<D>,
    F: ForgeQueryDeclarationFamilyMarker<D>,
>(
    handle: &ForgeQueryAdmittedConfiguredDomainHandle<D, C>,
) -> ForgeQueryDeclarationFamilySupportReport<D, F> {
    let taxonomy = F::taxonomy();
    let family_status = family_status::<D, C, F>(handle);
    let required_capability_families = F::required_capability_families().to_vec();
    let required_config_sections = F::required_config_sections().to_vec();
    let rows = vec![
        row(ForgeQueryDeclarationCapabilityVerb::Declare, family_status),
        relational_row(family_status, taxonomy),
        bridge_row(family_status, taxonomy),
        signal_row(family_status, taxonomy),
        neighborhood_row(family_status, taxonomy),
        batch_row(family_status, taxonomy),
    ];
    let support_digest = hash_parts(&[
        format!("domain:{}", handle.domain_key()),
        format!("handle:{}", handle.handle_identity_digest()),
        format!("family:{}", F::semantic_family_key()),
        format!("taxonomy:{taxonomy:?}"),
        format!(
            "capabilities:{}",
            required_capability_families
                .iter()
                .map(ForgeQueryCapabilityFamily::as_str)
                .collect::<Vec<_>>()
                .join(",")
        ),
        format!(
            "sections:{}",
            required_config_sections
                .iter()
                .map(ForgeQueryConfigSectionFamily::as_str)
                .collect::<Vec<_>>()
                .join(",")
        ),
        rows.iter()
            .map(|row| {
                format!(
                    "{}:{}:{}",
                    row.verb().as_str(),
                    row.status().as_str(),
                    row.reason()
                )
            })
            .collect::<Vec<_>>()
            .join("|"),
    ]);
    ForgeQueryDeclarationFamilySupportReport::new(
        handle.domain_key(),
        F::semantic_family_key(),
        taxonomy,
        required_capability_families,
        required_config_sections,
        rows,
        support_digest,
    )
}

fn family_status<
    D: ForgeQueryDomainEntryMarker,
    C: ForgeQueryDomainOperatingContext<D>,
    F: ForgeQueryDeclarationFamilyMarker<D>,
>(
    handle: &ForgeQueryAdmittedConfiguredDomainHandle<D, C>,
) -> ForgeQueryDeclarationCapabilityStatus {
    if F::required_capability_families()
        .iter()
        .copied()
        .any(|family| {
            handle.support_snapshot().capability_status(family)
                == Some(ForgeQueryCapabilityStatus::DeferredDebt)
        })
    {
        return ForgeQueryDeclarationCapabilityStatus::DeferredDebt;
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
        return ForgeQueryDeclarationCapabilityStatus::InvalidContext;
    }
    if F::required_capability_families()
        .iter()
        .copied()
        .any(|family| {
            handle.support_snapshot().capability_status(family)
                == Some(ForgeQueryCapabilityStatus::Unsupported)
        })
    {
        return ForgeQueryDeclarationCapabilityStatus::Unsupported;
    }
    ForgeQueryDeclarationCapabilityStatus::Admitted
}

fn row(
    verb: ForgeQueryDeclarationCapabilityVerb,
    status: ForgeQueryDeclarationCapabilityStatus,
) -> ForgeQueryDeclarationFamilySupportRow {
    let reason = match status {
        ForgeQueryDeclarationCapabilityStatus::Admitted => {
            "family capability is admitted for this operating world"
        }
        ForgeQueryDeclarationCapabilityStatus::DeferredDebt => {
            "required family capability remains deferred debt in this Query build"
        }
        ForgeQueryDeclarationCapabilityStatus::Unsupported => {
            "required family capability is unsupported in this operating world"
        }
        ForgeQueryDeclarationCapabilityStatus::InvalidContext => {
            "required family config sections must be enabled before admission"
        }
    };
    ForgeQueryDeclarationFamilySupportRow::new(verb, status, reason)
}

fn relational_row(
    family_status: ForgeQueryDeclarationCapabilityStatus,
    taxonomy: ForgeQueryDeclarationFamilyTaxonomy,
) -> ForgeQueryDeclarationFamilySupportRow {
    witness_row(
        ForgeQueryDeclarationCapabilityVerb::RelationalTruthWitness,
        family_status,
        taxonomy.primary_authority_family()
            == crate::application::ForgeQueryDeclarationPrimaryAuthorityFamily::RelationalTruth,
        "family is not structurally relational-truth",
    )
}

fn bridge_row(
    family_status: ForgeQueryDeclarationCapabilityStatus,
    taxonomy: ForgeQueryDeclarationFamilyTaxonomy,
) -> ForgeQueryDeclarationFamilySupportRow {
    witness_row(
        ForgeQueryDeclarationCapabilityVerb::BridgeContinuationWitness,
        family_status,
        taxonomy.primary_authority_family()
            == crate::application::ForgeQueryDeclarationPrimaryAuthorityFamily::BridgeContinuation,
        "family is not structurally bridge-continuation",
    )
}

fn signal_row(
    family_status: ForgeQueryDeclarationCapabilityStatus,
    taxonomy: ForgeQueryDeclarationFamilyTaxonomy,
) -> ForgeQueryDeclarationFamilySupportRow {
    match taxonomy.signal_compatibility() {
        crate::application::ForgeQuerySignalCompatibilityPosture::Compatible => witness_row(
            ForgeQueryDeclarationCapabilityVerb::SignalCompatibilityWitness,
            family_status,
            true,
            "",
        ),
        crate::application::ForgeQuerySignalCompatibilityPosture::Deferred => {
            ForgeQueryDeclarationFamilySupportRow::new(
                ForgeQueryDeclarationCapabilityVerb::SignalCompatibilityWitness,
                ForgeQueryDeclarationCapabilityStatus::DeferredDebt,
                "signal compatibility for this family remains explicitly deferred",
            )
        }
        crate::application::ForgeQuerySignalCompatibilityPosture::NotCompatible => {
            ForgeQueryDeclarationFamilySupportRow::new(
                ForgeQueryDeclarationCapabilityVerb::SignalCompatibilityWitness,
                ForgeQueryDeclarationCapabilityStatus::Unsupported,
                "family is not structurally signal-compatible",
            )
        }
    }
}

fn neighborhood_row(
    family_status: ForgeQueryDeclarationCapabilityStatus,
    taxonomy: ForgeQueryDeclarationFamilyTaxonomy,
) -> ForgeQueryDeclarationFamilySupportRow {
    witness_row(
        ForgeQueryDeclarationCapabilityVerb::NeighborhoodGroupingWitness,
        family_status,
        matches!(
            taxonomy.grouped_posture(),
            crate::application::ForgeQueryGroupedDeclarationPosture::NeighborhoodCapable
                | crate::application::ForgeQueryGroupedDeclarationPosture::NeighborhoodAndBatchCapable
        ),
        "family is not structurally neighborhood-capable",
    )
}

fn batch_row(
    family_status: ForgeQueryDeclarationCapabilityStatus,
    taxonomy: ForgeQueryDeclarationFamilyTaxonomy,
) -> ForgeQueryDeclarationFamilySupportRow {
    witness_row(
        ForgeQueryDeclarationCapabilityVerb::BatchGroupingWitness,
        family_status,
        matches!(
            taxonomy.grouped_posture(),
            crate::application::ForgeQueryGroupedDeclarationPosture::BatchCapable
                | crate::application::ForgeQueryGroupedDeclarationPosture::NeighborhoodAndBatchCapable
        ),
        "family is not structurally batch-capable",
    )
}

fn witness_row(
    verb: ForgeQueryDeclarationCapabilityVerb,
    family_status: ForgeQueryDeclarationCapabilityStatus,
    structurally_available: bool,
    unsupported_reason: &'static str,
) -> ForgeQueryDeclarationFamilySupportRow {
    if !structurally_available {
        return ForgeQueryDeclarationFamilySupportRow::new(
            verb,
            ForgeQueryDeclarationCapabilityStatus::Unsupported,
            unsupported_reason,
        );
    }
    row(verb, family_status)
}
