use crate::diagnostics::{
    FoundationalDiagnosticArtifactKind, FoundationalDiagnosticDeliveryClass,
    FoundationalDiagnosticMaterializationDenial, FoundationalExplanationBundleArtifactKind,
};
use crate::profiles::FoundationalProfileSet;

use super::admission::admit_common_materialization;
use super::inputs::FoundationalDiagnosticExplanationInput;
use super::plan::FoundationalDiagnosticMaterializationPlan;

pub fn plan_diagnostic_explanation_bundle(
    input: FoundationalDiagnosticExplanationInput,
    profile: FoundationalProfileSet,
    delivery_class: FoundationalDiagnosticDeliveryClass,
) -> Result<
    FoundationalDiagnosticMaterializationPlan<FoundationalExplanationBundleArtifactKind>,
    FoundationalDiagnosticMaterializationDenial,
> {
    admit_common_materialization(
        input.availability,
        &input.partiality,
        &input.assembly_debts,
        delivery_class,
        FoundationalDiagnosticArtifactKind::ExplanationBundle,
    )?;

    Ok(
        FoundationalDiagnosticMaterializationPlan::from_explanation_input(
            input,
            profile,
            delivery_class,
        ),
    )
}
