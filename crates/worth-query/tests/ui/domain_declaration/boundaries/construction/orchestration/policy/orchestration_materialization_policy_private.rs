use worth_foundational::facade::FoundationalBoundaryEvidenceMaterializationProfile;
use worth_query::facade::foundation::{WorthQueryDeclarationEntryOrchestrationCostPosture, WorthQueryDeclarationEntryOrchestrationMaterializationGate, WorthQueryDeclarationEntryOrchestrationMaterializationPolicy, WorthQueryDeclarationEntryOrchestrationMaterializationTier};

fn main() {
    let _ = WorthQueryDeclarationEntryOrchestrationMaterializationPolicy {
        foundational_evidence_profile:
            FoundationalBoundaryEvidenceMaterializationProfile::FullDescriptiveRichness,
        receipt_tier: WorthQueryDeclarationEntryOrchestrationMaterializationTier::SupportReady,
        envelope_tier: WorthQueryDeclarationEntryOrchestrationMaterializationTier::FullDescriptive,
        support_rich_publication_admitted: true,
        diagnostic_rich_publication_admitted: true,
        cost_posture: WorthQueryDeclarationEntryOrchestrationCostPosture::ExplicitlyRich,
        materialization_gate:
            WorthQueryDeclarationEntryOrchestrationMaterializationGate::AdmittedByDefault,
    };
}
