use worth_ui::facade::inspection::{
    UiEvidenceBudget, UiEvidenceRichness, UiInspectionScope, UiInspectionTarget,
};

fn match_target(target: UiInspectionTarget) -> &'static str {
    match target {
        UiInspectionTarget::ProductRoot => "product-root",
    }
}

fn match_scope(scope: UiInspectionScope) -> &'static str {
    match scope {
        UiInspectionScope::Graph => "graph",
    }
}

fn match_budget(budget: UiEvidenceBudget) -> &'static str {
    match budget {
        UiEvidenceBudget::Bounded { richness } => match_richness(richness),
        UiEvidenceBudget::Exhaustive => "exhaustive",
    }
}

fn match_richness(richness: UiEvidenceRichness) -> &'static str {
    match richness {
        UiEvidenceRichness::Summary => "summary",
        UiEvidenceRichness::Full => "full",
    }
}

fn main() {
    let _ = match_target(UiInspectionTarget::product_root());
    let _ = match_scope(UiInspectionScope::graph());
    let _ = match_budget(UiEvidenceBudget::bounded(UiEvidenceRichness::summary()));
}
