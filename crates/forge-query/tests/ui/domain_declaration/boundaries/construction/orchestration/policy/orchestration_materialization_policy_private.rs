use forge_foundational::facade::FoundationalBoundaryEvidenceMaterializationProfile;
use forge_query::facade::{
    ForgeQueryDeclarationEntryOrchestrationCostPosture,
    ForgeQueryDeclarationEntryOrchestrationMaterializationGate,
    ForgeQueryDeclarationEntryOrchestrationMaterializationPolicy,
    ForgeQueryDeclarationEntryOrchestrationMaterializationTier,
};

fn main() {
    let _ = ForgeQueryDeclarationEntryOrchestrationMaterializationPolicy {
        foundational_evidence_profile:
            FoundationalBoundaryEvidenceMaterializationProfile::FullDescriptiveRichness,
        receipt_tier: ForgeQueryDeclarationEntryOrchestrationMaterializationTier::SupportReady,
        envelope_tier: ForgeQueryDeclarationEntryOrchestrationMaterializationTier::FullDescriptive,
        support_rich_publication_admitted: true,
        diagnostic_rich_publication_admitted: true,
        cost_posture: ForgeQueryDeclarationEntryOrchestrationCostPosture::ExplicitlyRich,
        materialization_gate:
            ForgeQueryDeclarationEntryOrchestrationMaterializationGate::AdmittedByDefault,
    };
}
