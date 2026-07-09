use worth_query::facade::{
    WorthQueryDeclarationEntryOrchestrationExposureLevel,
    WorthQueryDeclarationEntryOrchestrationVerb,
    WorthQueryDeclarationEntryOrchestrationVerbCeiling,
    WorthQueryDeclarationEntryOrchestrationVerbFamily,
};

fn main() {
    let _ = WorthQueryDeclarationEntryOrchestrationVerb {
        public_name: "orchestrate_declaration_entry_debug",
        family: WorthQueryDeclarationEntryOrchestrationVerbFamily::GenericDeclarationEntry,
        exposure_level: WorthQueryDeclarationEntryOrchestrationExposureLevel::Ordinary,
        ceiling: WorthQueryDeclarationEntryOrchestrationVerbCeiling::Envelope,
        canonical_base_name: "orchestrate_declaration_entry",
    };
}
