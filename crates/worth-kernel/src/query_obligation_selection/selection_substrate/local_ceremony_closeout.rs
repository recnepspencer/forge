use forge_query::facade::consumer_kit::ForgeQueryGraphObligationLocalCeremonyAudit;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum QuerySelectionForbiddenAuthorityKind {
    LocalSelectorConstruction,
    PrivateSupportMatrixConstruction,
    LocalGraphWalk,
    SourceGrepAudit,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct QuerySelectionLocalCeremonyCloseout {
    evaluated_source_count: usize,
    clean: bool,
    audit_digest: String,
    forbidden_authority_kinds: Vec<QuerySelectionForbiddenAuthorityKind>,
    query_owned_selection_substrate: bool,
}

impl QuerySelectionLocalCeremonyCloseout {
    pub(super) fn from_audit(audit: &ForgeQueryGraphObligationLocalCeremonyAudit) -> Self {
        let mut forbidden_authority_kinds = audit
            .findings()
            .iter()
            .filter_map(|finding| forbidden_authority_kind_for_pattern(finding.pattern()))
            .collect::<Vec<_>>();
        forbidden_authority_kinds.sort();
        forbidden_authority_kinds.dedup();
        Self {
            evaluated_source_count: audit.evaluated_source_count(),
            clean: audit.is_clean(),
            audit_digest: audit.audit_digest().to_string(),
            forbidden_authority_kinds,
            query_owned_selection_substrate: audit.is_evaluated()
                && audit.is_clean()
                && audit
                    .audited_source_labels()
                    .iter()
                    .all(|label| label.contains("query_obligation_selection/selection_substrate")),
        }
    }

    pub const fn evaluated_source_count(&self) -> usize {
        self.evaluated_source_count
    }

    pub const fn is_clean(&self) -> bool {
        self.clean
    }

    pub fn audit_digest(&self) -> &str {
        &self.audit_digest
    }

    pub fn forbidden_authority_kinds(&self) -> &[QuerySelectionForbiddenAuthorityKind] {
        &self.forbidden_authority_kinds
    }

    pub fn rejected_forbidden_authority_count(&self) -> usize {
        self.forbidden_authority_kinds.len()
    }

    pub const fn is_query_owned_selection_substrate(&self) -> bool {
        self.query_owned_selection_substrate
    }
}

fn forbidden_authority_kind_for_pattern(
    pattern: &str,
) -> Option<QuerySelectionForbiddenAuthorityKind> {
    match pattern {
        "ForgeQueryGraphObligationRegistration::"
        | "ForgeQueryGraphObligationRegistrationCatalog::from_registrations"
        | "ForgeQueryGraphObligationIndex::from_catalog"
        | "ForgeQueryGraphTouchSelector::"
        | "select_graph_obligations_for_touch" => {
            Some(QuerySelectionForbiddenAuthorityKind::LocalSelectorConstruction)
        }
        "ForgeQueryGraphObligationSupportMatrixRow::new" => {
            Some(QuerySelectionForbiddenAuthorityKind::PrivateSupportMatrixConstruction)
        }
        "phase_chain"
        | "local_legality_graph"
        | "InvariantPack"
        | "invariant_pack"
        | "manual_precheck"
        | "manual_pre_check"
        | "private_validator"
        | "validator_dispatch" => Some(QuerySelectionForbiddenAuthorityKind::LocalGraphWalk),
        _ => None,
    }
}
