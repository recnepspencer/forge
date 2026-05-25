use crate::application::{
    ForgeQueryAdmittedConfiguredDomainHandle, ForgeQueryCapabilityFamily,
    ForgeQueryConfigSectionFamily, ForgeQueryDeclarationInput, ForgeQueryDomainEntryMarker,
    ForgeQueryDomainOperatingContext,
};

const WORKFLOW_ONLY: &[ForgeQueryCapabilityFamily] =
    &[ForgeQueryCapabilityFamily::WorkflowOrchestration];
const HISTORY_ONLY: &[ForgeQueryCapabilityFamily] =
    &[ForgeQueryCapabilityFamily::HistoricalEvaluation];
const RELATIONAL_SECTION_ONLY: &[ForgeQueryConfigSectionFamily] =
    &[ForgeQueryConfigSectionFamily::Relational];

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ForgeQueryDeclarationRelationalTruthClaim {
    AuthoritativeCurrentTruth,
    Identity,
    Lineage,
    HistoricalTruth,
    InvariantTruth,
    GroupedTruth,
    StrategyTruth,
}

impl ForgeQueryDeclarationRelationalTruthClaim {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::AuthoritativeCurrentTruth => "authoritative_current_truth",
            Self::Identity => "identity",
            Self::Lineage => "lineage",
            Self::HistoricalTruth => "historical_truth",
            Self::InvariantTruth => "invariant_truth",
            Self::GroupedTruth => "grouped_truth",
            Self::StrategyTruth => "strategy_truth",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ForgeQueryDeclarationRelationalAuthorityFamily {
    Runtime,
    History,
    GroupedTruth,
    CommitStrategies,
    BridgeSource,
}

impl ForgeQueryDeclarationRelationalAuthorityFamily {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Runtime => "runtime",
            Self::History => "history",
            Self::GroupedTruth => "grouped_truth",
            Self::CommitStrategies => "commit_strategies",
            Self::BridgeSource => "bridge_source",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ForgeQueryDeclarationRelationalTruthContract {
    truth_claim: ForgeQueryDeclarationRelationalTruthClaim,
    authority_family: ForgeQueryDeclarationRelationalAuthorityFamily,
    required_capability_families: &'static [ForgeQueryCapabilityFamily],
    required_config_sections: &'static [ForgeQueryConfigSectionFamily],
    reason: &'static str,
}

impl ForgeQueryDeclarationRelationalTruthContract {
    pub fn authoritative_current_truth() -> Self {
        Self {
            truth_claim: ForgeQueryDeclarationRelationalTruthClaim::AuthoritativeCurrentTruth,
            authority_family: ForgeQueryDeclarationRelationalAuthorityFamily::Runtime,
            required_capability_families: WORKFLOW_ONLY,
            required_config_sections: RELATIONAL_SECTION_ONLY,
            reason: "the declaration lowers into authoritative relational runtime truth",
        }
    }

    pub fn grouped_truth() -> Self {
        Self {
            truth_claim: ForgeQueryDeclarationRelationalTruthClaim::GroupedTruth,
            authority_family: ForgeQueryDeclarationRelationalAuthorityFamily::GroupedTruth,
            required_capability_families: WORKFLOW_ONLY,
            required_config_sections: RELATIONAL_SECTION_ONLY,
            reason: "the declaration lowers into grouped relational truth materialization",
        }
    }

    pub fn historical_truth() -> Self {
        Self {
            truth_claim: ForgeQueryDeclarationRelationalTruthClaim::HistoricalTruth,
            authority_family: ForgeQueryDeclarationRelationalAuthorityFamily::History,
            required_capability_families: HISTORY_ONLY,
            required_config_sections: RELATIONAL_SECTION_ONLY,
            reason: "the declaration lowers into relational historical truth evaluation",
        }
    }

    pub fn strategy_truth() -> Self {
        Self {
            truth_claim: ForgeQueryDeclarationRelationalTruthClaim::StrategyTruth,
            authority_family: ForgeQueryDeclarationRelationalAuthorityFamily::CommitStrategies,
            required_capability_families: WORKFLOW_ONLY,
            required_config_sections: RELATIONAL_SECTION_ONLY,
            reason: "the declaration lowers into relational commit strategy authority",
        }
    }

    pub fn bridge_source_current_truth() -> Self {
        Self {
            truth_claim: ForgeQueryDeclarationRelationalTruthClaim::AuthoritativeCurrentTruth,
            authority_family: ForgeQueryDeclarationRelationalAuthorityFamily::BridgeSource,
            required_capability_families: WORKFLOW_ONLY,
            required_config_sections: RELATIONAL_SECTION_ONLY,
            reason: "the declaration lowers into a bridge-consumable relational source binding",
        }
    }

    pub fn truth_claim(self) -> ForgeQueryDeclarationRelationalTruthClaim {
        self.truth_claim
    }

    pub fn authority_family(self) -> ForgeQueryDeclarationRelationalAuthorityFamily {
        self.authority_family
    }

    pub fn required_capability_families(self) -> &'static [ForgeQueryCapabilityFamily] {
        self.required_capability_families
    }

    pub fn required_config_sections(self) -> &'static [ForgeQueryConfigSectionFamily] {
        self.required_config_sections
    }

    pub fn reason(self) -> &'static str {
        self.reason
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ForgeQueryDeclarationRelationalTruthRoutingSupportStatus {
    Admitted,
    Unsupported,
    InvalidContext,
}

impl ForgeQueryDeclarationRelationalTruthRoutingSupportStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Admitted => "admitted",
            Self::Unsupported => "unsupported",
            Self::InvalidContext => "invalid_context",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryDeclarationRelationalRoutingSupportRow {
    truth_claim: ForgeQueryDeclarationRelationalTruthClaim,
    authority_family: ForgeQueryDeclarationRelationalAuthorityFamily,
    status: ForgeQueryDeclarationRelationalTruthRoutingSupportStatus,
    reason: &'static str,
}

impl ForgeQueryDeclarationRelationalRoutingSupportRow {
    pub(crate) fn new(
        truth_claim: ForgeQueryDeclarationRelationalTruthClaim,
        authority_family: ForgeQueryDeclarationRelationalAuthorityFamily,
        status: ForgeQueryDeclarationRelationalTruthRoutingSupportStatus,
        reason: &'static str,
    ) -> Self {
        Self {
            truth_claim,
            authority_family,
            status,
            reason,
        }
    }

    pub fn truth_claim(&self) -> ForgeQueryDeclarationRelationalTruthClaim {
        self.truth_claim
    }

    pub fn authority_family(&self) -> ForgeQueryDeclarationRelationalAuthorityFamily {
        self.authority_family
    }

    pub fn status(&self) -> ForgeQueryDeclarationRelationalTruthRoutingSupportStatus {
        self.status
    }

    pub fn reason(&self) -> &'static str {
        self.reason
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryDeclarationRelationalRoutingSupportReport<
    D: ForgeQueryDomainEntryMarker,
    I: ForgeQueryDeclarationInput<D>,
> {
    declaration_family_key: &'static str,
    rows: Vec<ForgeQueryDeclarationRelationalRoutingSupportRow>,
    support_digest: String,
    _marker: std::marker::PhantomData<(D, I)>,
}

impl<D: ForgeQueryDomainEntryMarker, I: ForgeQueryDeclarationInput<D>>
    ForgeQueryDeclarationRelationalRoutingSupportReport<D, I>
{
    pub(crate) fn new(
        declaration_family_key: &'static str,
        rows: Vec<ForgeQueryDeclarationRelationalRoutingSupportRow>,
        support_digest: String,
    ) -> Self {
        Self {
            declaration_family_key,
            rows,
            support_digest,
            _marker: std::marker::PhantomData,
        }
    }

    pub fn declaration_family_key(&self) -> &'static str {
        self.declaration_family_key
    }

    pub fn rows(&self) -> &[ForgeQueryDeclarationRelationalRoutingSupportRow] {
        &self.rows
    }

    pub fn support_digest(&self) -> &str {
        &self.support_digest
    }
}

pub(crate) fn derive_relational_routing_support_report<
    D: ForgeQueryDomainEntryMarker,
    C: ForgeQueryDomainOperatingContext<D>,
    I: ForgeQueryDeclarationInput<D>,
>(
    handle: &ForgeQueryAdmittedConfiguredDomainHandle<D, C>,
) -> ForgeQueryDeclarationRelationalRoutingSupportReport<D, I> {
    crate::application::forge_query_relational_routing_support_from_entry_readiness::<D, C, I>(
        handle,
    )
}
