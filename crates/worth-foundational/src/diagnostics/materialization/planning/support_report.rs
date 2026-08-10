use crate::diagnostics::{
    FoundationalDiagnosticArtifactKind, FoundationalDiagnosticDeliveryClass,
    FoundationalDiagnosticMaterializationDenial, FoundationalSupportReportArtifactKind,
};
use crate::profiles::FoundationalProfileSet;

use super::admission::admit_common_materialization;
use super::inputs::FoundationalDiagnosticSupportInput;
use super::plan::FoundationalDiagnosticMaterializationPlan;
use super::support_claim::admit_support_claim;

pub fn plan_diagnostic_support_report(
    input: FoundationalDiagnosticSupportInput,
    profile: FoundationalProfileSet,
    delivery_class: FoundationalDiagnosticDeliveryClass,
) -> Result<
    FoundationalDiagnosticMaterializationPlan<FoundationalSupportReportArtifactKind>,
    FoundationalDiagnosticMaterializationDenial,
> {
    admit_common_materialization(
        input.availability,
        &input.partiality,
        &input.assembly_debts,
        delivery_class,
        FoundationalDiagnosticArtifactKind::SupportReport,
    )?;
    admit_support_claim(&input, profile)?;

    Ok(
        FoundationalDiagnosticMaterializationPlan::from_support_input(
            input,
            profile,
            delivery_class,
        ),
    )
}
