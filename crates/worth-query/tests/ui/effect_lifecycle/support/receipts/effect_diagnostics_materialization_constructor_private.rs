use worth_query::facade::foundation::EffectDiagnosticsMaterialization;

fn main() {
    let _ = EffectDiagnosticsMaterialization {
        receipt_digest: String::new(),
        envelope_digest: String::new(),
        detail_sections: Vec::new(),
        diagnostics_digest: String::new(),
    };
}
