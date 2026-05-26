use forge_query::facade::{
    ForgeQueryDeclarationEntryOrchestrationExposureLevel,
    ForgeQueryDeclarationEntryOrchestrationVerb,
    ForgeQueryDeclarationEntryOrchestrationVerbCeiling,
    ForgeQueryDeclarationEntryOrchestrationVerbFamily,
};

fn main() {
    let _ = ForgeQueryDeclarationEntryOrchestrationVerb {
        public_name: "orchestrate_declaration_entry_debug",
        family: ForgeQueryDeclarationEntryOrchestrationVerbFamily::GenericDeclarationEntry,
        exposure_level: ForgeQueryDeclarationEntryOrchestrationExposureLevel::Ordinary,
        ceiling: ForgeQueryDeclarationEntryOrchestrationVerbCeiling::Envelope,
        canonical_base_name: "orchestrate_declaration_entry",
    };
}
