use worth_ui::facade::app::WorthUiPreparedApplicationGenerationIdentity;
use worth_ui::facade::source::WorthUiCandidateCompositionBasis;

fn require_prepared_generation(_: &WorthUiPreparedApplicationGenerationIdentity) {}

fn promote(candidate: &WorthUiCandidateCompositionBasis) {
    require_prepared_generation(candidate);
}

fn main() {}
