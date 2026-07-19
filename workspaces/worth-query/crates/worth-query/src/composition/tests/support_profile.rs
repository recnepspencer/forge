use crate::composition::{QueryCompositionTemporalAsyncPosture, ScopeFamily, TemplateFamily};

#[test]
fn runtime_backed_composition_support_profile_makes_future_surface_posture_explicit() {
    let profile = crate::composition::runtime_backed_query_composition_support_profile();

    assert_eq!(
        profile
            .scope_temporal_async_postures()
            .iter()
            .find(|(family, _)| *family == ScopeFamily::BasisAwareScope)
            .map(|(_, posture)| *posture),
        Some(QueryCompositionTemporalAsyncPosture::FuturePreserving)
    );
    assert_eq!(
        profile
            .scope_temporal_async_postures()
            .iter()
            .find(|(family, _)| *family == ScopeFamily::PredicateScope)
            .map(|(_, posture)| *posture),
        Some(QueryCompositionTemporalAsyncPosture::OrdinaryOnly)
    );
    assert_eq!(
        profile
            .template_temporal_async_postures()
            .iter()
            .find(|(family, _)| *family == TemplateFamily::GroupedCollectionTemplate)
            .map(|(_, posture)| *posture),
        Some(QueryCompositionTemporalAsyncPosture::FuturePreserving)
    );
}
