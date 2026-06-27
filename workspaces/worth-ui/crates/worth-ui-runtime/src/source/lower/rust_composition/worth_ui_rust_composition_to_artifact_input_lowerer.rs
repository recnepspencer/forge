use crate::source::{
    WorthUiArtifactInput, WorthUiRustAuthoredArtifactInput,
    WorthUiRustAuthoredToArtifactInputLowerer, WorthUiRustCompositionInput,
    WorthUiRustCompositionMetrics, WorthUiRustCompositionReport,
};

#[derive(Clone, Debug, Default)]
pub(crate) struct WorthUiRustCompositionToArtifactInputLowerer;

impl WorthUiRustCompositionToArtifactInputLowerer {
    pub(crate) fn lower(composition: &WorthUiRustCompositionInput) -> WorthUiArtifactInput {
        Self::lower_with_report(composition).into_artifact_input()
    }

    pub(crate) fn lower_with_report(
        composition: &WorthUiRustCompositionInput,
    ) -> WorthUiRustCompositionReport {
        let metrics = metrics_for_composition(composition);
        let authored_input = WorthUiRustAuthoredArtifactInput::from_modules(
            composition
                .modules()
                .iter()
                .map(|module| module.authored_module().clone()),
        );
        let artifact_input = WorthUiRustAuthoredToArtifactInputLowerer::lower(&authored_input);
        WorthUiRustCompositionReport::new(artifact_input, metrics)
    }
}

fn metrics_for_composition(
    composition: &WorthUiRustCompositionInput,
) -> WorthUiRustCompositionMetrics {
    WorthUiRustCompositionMetrics::from_counts(
        composition.modules().len(),
        composition
            .modules()
            .iter()
            .map(|module| module.declaration_count())
            .sum(),
    )
}
