#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiIntentConsequenceContract {
    mounted_posture: bool,
    query_collection_change: Option<worth_ui_query_binding::WorthUiQueryViewIdentity>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct UiResolvedIntentConsequenceContract {
    mounted_posture: bool,
    query_collection_change: Option<worth_ui_query_binding::WorthUiQueryViewIdentity>,
}

impl UiIntentConsequenceContract {
    pub const fn none() -> Self {
        Self {
            mounted_posture: false,
            query_collection_change: None,
        }
    }

    pub const fn mounted_posture() -> Self {
        Self {
            mounted_posture: true,
            query_collection_change: None,
        }
    }

    pub fn query_collection_change(
        query: worth_ui_query_binding::WorthUiQueryViewIdentity,
    ) -> Self {
        Self {
            mounted_posture: false,
            query_collection_change: Some(query),
        }
    }

    pub fn mounted_posture_and_query(
        query: worth_ui_query_binding::WorthUiQueryViewIdentity,
    ) -> Self {
        Self {
            mounted_posture: true,
            query_collection_change: Some(query),
        }
    }

    pub(crate) fn into_dsl(self) -> worth_ui_dsl::WorthUiIntentConsequenceContractSpec {
        match (self.mounted_posture, self.query_collection_change) {
            (false, None) => worth_ui_dsl::WorthUiIntentConsequenceContractSpec::none(),
            (true, None) => worth_ui_dsl::WorthUiIntentConsequenceContractSpec::mounted_posture(),
            (false, Some(query)) => {
                worth_ui_dsl::WorthUiIntentConsequenceContractSpec::query_collection_change(
                    query.as_str(),
                )
            }
            (true, Some(query)) => {
                worth_ui_dsl::WorthUiIntentConsequenceContractSpec::mounted_posture_and_query(
                    query.as_str(),
                )
            }
        }
    }
}

pub(crate) fn resolve_consequence_contract(
    declaration: &str,
    authored: &worth_ui_dsl::WorthUiIntentConsequenceContractSpec,
    definition: &crate::capability::IntentDefinitionDescriptor,
    query: &worth_ui_query_binding::WorthUiQueryBindingPlan,
) -> Result<UiResolvedIntentConsequenceContract, super::UiIntentCatalogPreparationDenial> {
    let query_collection_change = authored
        .query_collection_change_identity()
        .map(|identity| resolve_query(declaration, identity, definition, query))
        .transpose()?;
    Ok(UiResolvedIntentConsequenceContract {
        mounted_posture: authored.includes_mounted_posture(),
        query_collection_change,
    })
}

fn resolve_query(
    declaration: &str,
    identity: &str,
    definition: &crate::capability::IntentDefinitionDescriptor,
    query: &worth_ui_query_binding::WorthUiQueryBindingPlan,
) -> Result<worth_ui_query_binding::WorthUiQueryViewIdentity, super::UiIntentCatalogPreparationDenial>
{
    let families = definition.product_consequence_families();
    if !families.permits_query_collection_change() && !families.permits_query_projection() {
        return Err(
            super::UiIntentCatalogPreparationDenial::ConsequenceFamilyNotPermitted {
                declaration: declaration.into(),
                family:
                    crate::capability::UiIntentProductConsequenceFamilies::QUERY_COLLECTION_CHANGE,
            },
        );
    }
    let resolved = if families.permits_query_projection() {
        query
            .projection_identities()
            .into_iter()
            .find(|candidate| candidate.as_str() == identity)
    } else {
        query
            .definitions()
            .into_iter()
            .find(|candidate| candidate.identity().as_str() == identity)
            .map(|definition| definition.identity().clone())
    };
    resolved.ok_or_else(
        || super::UiIntentCatalogPreparationDenial::UnknownConsequenceQuery {
            declaration: declaration.into(),
            query: identity.into(),
        },
    )
}

impl UiResolvedIntentConsequenceContract {
    pub(crate) const fn includes_mounted_posture(&self) -> bool {
        self.mounted_posture
    }

    pub(crate) const fn query_collection_change(
        &self,
    ) -> Option<&worth_ui_query_binding::WorthUiQueryViewIdentity> {
        self.query_collection_change.as_ref()
    }
}

#[cfg(test)]
#[path = "consequence_contract_tests.rs"]
mod tests;
