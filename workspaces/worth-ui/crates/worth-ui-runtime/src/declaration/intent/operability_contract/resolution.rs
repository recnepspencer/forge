use crate::capability::UiIntentPayloadFieldKind;

use super::{
    UiIntentOperabilityDependencyAxis, UiResolvedIntentMutabilitySource,
    UiResolvedIntentOperabilityContract, UiResolvedIntentPolicySource,
    UiResolvedIntentReadinessSource,
};

pub(crate) fn resolve_operability_contract(
    declaration: &str,
    spec: &worth_ui_dsl::WorthUiIntentOperabilityContractSpec,
    interaction: crate::capability::UiSemanticInteractionFamily,
    query: &worth_ui_query_binding::WorthUiQueryBindingPlan,
    application_facts: &crate::declaration::UiIntentApplicationFactPlan,
) -> Result<UiResolvedIntentOperabilityContract, crate::declaration::UiIntentCatalogPreparationDenial>
{
    let mutability = resolve_mutability(
        declaration,
        spec.mutability(),
        interaction,
        query,
        application_facts,
    )?;
    let readiness = resolve_readiness(
        declaration,
        spec.readiness(),
        interaction,
        query,
        application_facts,
    )?;
    let policy = UiResolvedIntentPolicySource {
        slot: resolve_boolean_fact(
            declaration,
            UiIntentOperabilityDependencyAxis::Policy,
            spec.policy().application_fact(),
            application_facts,
        )?,
    };
    Ok(UiResolvedIntentOperabilityContract {
        identity: spec.identity().into(),
        mutability,
        readiness,
        policy,
    })
}

fn resolve_mutability(
    declaration: &str,
    source: &worth_ui_dsl::WorthUiIntentMutabilitySourceSpec,
    interaction: crate::capability::UiSemanticInteractionFamily,
    query: &worth_ui_query_binding::WorthUiQueryBindingPlan,
    application_facts: &crate::declaration::UiIntentApplicationFactPlan,
) -> Result<UiResolvedIntentMutabilitySource, crate::declaration::UiIntentCatalogPreparationDenial>
{
    if let Some(fact) = source.application_fact() {
        return Ok(UiResolvedIntentMutabilitySource::ApplicationBoolean(
            resolve_boolean_fact(
                declaration,
                UiIntentOperabilityDependencyAxis::Mutability,
                fact,
                application_facts,
            )?,
        ));
    }
    if let Some(projection) = source.projection() {
        let (identity, slot) = resolve_projection(
            declaration,
            UiIntentOperabilityDependencyAxis::Mutability,
            projection,
            query,
        )?;
        return Ok(UiResolvedIntentMutabilitySource::ProjectionReadonly { identity, slot });
    }
    require_draft_source(
        declaration,
        UiIntentOperabilityDependencyAxis::Mutability,
        interaction,
    )?;
    Ok(UiResolvedIntentMutabilitySource::CommittedDraft)
}

fn resolve_readiness(
    declaration: &str,
    source: &worth_ui_dsl::WorthUiIntentReadinessSourceSpec,
    interaction: crate::capability::UiSemanticInteractionFamily,
    query: &worth_ui_query_binding::WorthUiQueryBindingPlan,
    application_facts: &crate::declaration::UiIntentApplicationFactPlan,
) -> Result<UiResolvedIntentReadinessSource, crate::declaration::UiIntentCatalogPreparationDenial> {
    if let Some(fact) = source.application_fact() {
        return Ok(UiResolvedIntentReadinessSource::ApplicationBoolean(
            resolve_boolean_fact(
                declaration,
                UiIntentOperabilityDependencyAxis::Readiness,
                fact,
                application_facts,
            )?,
        ));
    }
    if let Some(projection) = source.projection_identity() {
        let (identity, slot) = resolve_projection(
            declaration,
            UiIntentOperabilityDependencyAxis::Readiness,
            projection,
            query,
        )?;
        return Ok(UiResolvedIntentReadinessSource::Projection { identity, slot });
    }
    require_draft_source(
        declaration,
        UiIntentOperabilityDependencyAxis::Readiness,
        interaction,
    )?;
    Ok(UiResolvedIntentReadinessSource::CommittedDraft)
}

fn resolve_boolean_fact(
    declaration: &str,
    axis: UiIntentOperabilityDependencyAxis,
    identity: &str,
    application_facts: &crate::declaration::UiIntentApplicationFactPlan,
) -> Result<
    crate::declaration::UiIntentApplicationFactSlot,
    crate::declaration::UiIntentCatalogPreparationDenial,
> {
    let fact = application_facts.get(identity).ok_or_else(|| {
        crate::declaration::UiIntentCatalogPreparationDenial::UnknownOperabilityApplicationFact {
            declaration: declaration.into(),
            axis,
            fact: identity.into(),
        }
    })?;
    if fact.kind() != UiIntentPayloadFieldKind::Boolean {
        return Err(
            crate::declaration::UiIntentCatalogPreparationDenial::
                OperabilityApplicationFactKindMismatch {
                    declaration: declaration.into(),
                    axis,
                    fact: identity.into(),
                    observed: fact.kind(),
                },
        );
    }
    Ok(fact.slot())
}

fn resolve_projection(
    declaration: &str,
    axis: UiIntentOperabilityDependencyAxis,
    authored: &str,
    query: &worth_ui_query_binding::WorthUiQueryBindingPlan,
) -> Result<
    (
        worth_ui_query_binding::WorthUiQueryViewIdentity,
        worth_ui_query_binding::UiProjectionInputSlot,
    ),
    crate::declaration::UiIntentCatalogPreparationDenial,
> {
    let identity =
        worth_ui_query_binding::WorthUiQueryViewIdentity::new(authored).map_err(|_| {
            crate::declaration::UiIntentCatalogPreparationDenial::
            InvalidOperabilityProjectionIdentity {
                declaration: declaration.into(),
                axis,
                projection: authored.into(),
            }
        })?;
    let slot = query.projection_input_slot(&identity).ok_or_else(|| {
        crate::declaration::UiIntentCatalogPreparationDenial::UnknownOperabilityProjection {
            declaration: declaration.into(),
            axis,
            projection: authored.into(),
        }
    })?;
    Ok((identity, slot))
}

fn require_draft_source(
    declaration: &str,
    axis: UiIntentOperabilityDependencyAxis,
    interaction: crate::capability::UiSemanticInteractionFamily,
) -> Result<(), crate::declaration::UiIntentCatalogPreparationDenial> {
    if interaction != crate::capability::UiSemanticInteractionFamily::EditCommit {
        return Err(
            crate::declaration::UiIntentCatalogPreparationDenial::OperabilityDraftSourceMismatch {
                declaration: declaration.into(),
                axis,
                interaction,
            },
        );
    }
    Ok(())
}
