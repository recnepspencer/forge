use forge_query::facade::{
    ForgeQueryGraphObligationOperatingWorldSelector, ForgeQueryGraphObligationRegistration,
    ForgeQueryGraphScopedCustomInvariantRegistration,
};

use crate::runtime_support::milestone_one_invariant_registrations;
use crate::validator_invariant_catalog::{
    WorthTopologyLegalityCatalog, WorthTopologyLegalityCatalogError,
    WorthTopologyLegalityFamilyRecord,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthTopologyGraphScopedCustomInvariantRegistrationProjectionRow {
    worth_family_identity_digest: String,
    custom_rule_id: String,
    execution_point: String,
    graph_obligation_registration_digest: String,
    row_digest: String,
}

impl WorthTopologyGraphScopedCustomInvariantRegistrationProjectionRow {
    fn from_invariant_family(
        family: &WorthTopologyLegalityFamilyRecord,
    ) -> Result<Self, WorthTopologyLegalityCatalogError> {
        let custom_invariant = current_custom_invariant_for_family(family)?;
        let graph_obligation = graph_obligation_for_family(family, &custom_invariant)?;
        let execution_point = custom_invariant
            .execution_point()
            .diagnostic_label()
            .to_string();
        let custom_rule_id = custom_invariant.rule_id().as_str().to_string();
        let row_digest = [
            "worth-topo-graph-scoped-custom-invariant-registration-projection-row-v1",
            family.identity().identity_digest(),
            custom_rule_id.as_str(),
            execution_point.as_str(),
            graph_obligation.registration_digest(),
        ]
        .join("|");
        Ok(Self {
            worth_family_identity_digest: family.identity().identity_digest().to_string(),
            custom_rule_id,
            execution_point,
            graph_obligation_registration_digest: graph_obligation
                .registration_digest()
                .to_string(),
            row_digest,
        })
    }

    pub fn worth_family_identity_digest(&self) -> &str {
        &self.worth_family_identity_digest
    }

    pub fn custom_rule_id(&self) -> &str {
        &self.custom_rule_id
    }

    pub fn execution_point(&self) -> &str {
        &self.execution_point
    }

    pub fn graph_obligation_registration_digest(&self) -> &str {
        &self.graph_obligation_registration_digest
    }

    pub fn row_digest(&self) -> &str {
        &self.row_digest
    }
}

pub(in crate::validator_invariant_catalog::relational_invariant_catalog) fn graph_scoped_custom_invariant_projection_rows_from_catalog(
    catalog: &WorthTopologyLegalityCatalog,
) -> Result<
    Vec<WorthTopologyGraphScopedCustomInvariantRegistrationProjectionRow>,
    WorthTopologyLegalityCatalogError,
> {
    invariant_family_records(catalog)
        .map(
            WorthTopologyGraphScopedCustomInvariantRegistrationProjectionRow::from_invariant_family,
        )
        .collect()
}

pub(in crate::validator_invariant_catalog::relational_invariant_catalog) fn graph_scoped_custom_invariant_registrations_from_catalog(
    catalog: &WorthTopologyLegalityCatalog,
) -> Result<Vec<ForgeQueryGraphScopedCustomInvariantRegistration>, WorthTopologyLegalityCatalogError>
{
    invariant_family_records(catalog)
        .map(graph_scoped_custom_invariant_registration_for_family)
        .collect()
}

pub(in crate::validator_invariant_catalog::relational_invariant_catalog) fn graph_obligation_registrations_from_catalog(
    catalog: &WorthTopologyLegalityCatalog,
) -> Result<Vec<ForgeQueryGraphObligationRegistration>, WorthTopologyLegalityCatalogError> {
    invariant_family_records(catalog)
        .map(|family| {
            let custom_invariant = current_custom_invariant_for_family(family)?;
            graph_obligation_for_family(family, &custom_invariant)
        })
        .collect()
}

fn invariant_family_records(
    catalog: &WorthTopologyLegalityCatalog,
) -> impl Iterator<Item = &WorthTopologyLegalityFamilyRecord> {
    catalog
        .records()
        .iter()
        .filter(|family| matches!(family, WorthTopologyLegalityFamilyRecord::Invariant(_)))
}

fn graph_scoped_custom_invariant_registration_for_family(
    family: &WorthTopologyLegalityFamilyRecord,
) -> Result<ForgeQueryGraphScopedCustomInvariantRegistration, WorthTopologyLegalityCatalogError> {
    let custom_invariant = current_custom_invariant_for_family(family)?;
    let touch_selector = family
        .touched_applicability()
        .query_touch_selector()
        .map_err(|error| WorthTopologyLegalityCatalogError::QueryRegistration(error.to_string()))?;
    Ok(ForgeQueryGraphScopedCustomInvariantRegistration::new(
        custom_invariant,
        touch_selector,
        ForgeQueryGraphObligationOperatingWorldSelector::any_committed_authority(),
    )
    .with_support_posture(family.query_support_posture().clone()))
}

fn current_custom_invariant_for_family(
    family: &WorthTopologyLegalityFamilyRecord,
) -> Result<
    forge_relational::facade::runtime::CustomInvariantRegistration,
    WorthTopologyLegalityCatalogError,
> {
    milestone_one_invariant_registrations()
        .map_err(|error| {
            WorthTopologyLegalityCatalogError::InvariantRegistration(format!("{error:?}"))
        })?
        .into_iter()
        .find(|registration| {
            current_custom_invariant_family_name(registration) == family.identity().name()
        })
        .ok_or_else(|| {
            WorthTopologyLegalityCatalogError::InvariantRegistration(format!(
                "missing current custom invariant registration for `{}`",
                family.identity().name()
            ))
        })
}

fn graph_obligation_for_family(
    family: &WorthTopologyLegalityFamilyRecord,
    custom_invariant: &forge_relational::facade::runtime::CustomInvariantRegistration,
) -> Result<ForgeQueryGraphObligationRegistration, WorthTopologyLegalityCatalogError> {
    Ok(ForgeQueryGraphObligationRegistration::custom_invariant(
        custom_invariant,
        family
            .touched_applicability()
            .query_touch_selector()
            .map_err(|error| {
                WorthTopologyLegalityCatalogError::QueryRegistration(error.to_string())
            })?,
        ForgeQueryGraphObligationOperatingWorldSelector::any_committed_authority(),
    )
    .with_support_posture(family.query_support_posture().clone()))
}

fn current_custom_invariant_family_name(
    registration: &forge_relational::facade::runtime::CustomInvariantRegistration,
) -> String {
    format!(
        "{}.{}",
        registration.rule_id().as_str(),
        registration.execution_point().diagnostic_label()
    )
}
