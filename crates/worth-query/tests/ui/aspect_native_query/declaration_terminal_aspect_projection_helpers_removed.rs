use worth_query::facade::foundation::{WorthQueryDeclarationAspectPublication, WorthQueryGroupedAspectParticipationSummary};

fn assert_public_declaration_terminal_helpers_removed(
    publication: &WorthQueryDeclarationAspectPublication,
    grouped: &WorthQueryGroupedAspectParticipationSummary,
) {
    let _ = publication.terminal_present_projections();
    let _ = publication.terminal_widened_projections();
    let _ = publication.terminal_elided_projections();
    let _ = publication.terminal_masked_projections();

    let _ = grouped.terminal_present_any_projections();
    let _ = grouped.terminal_present_all_projections();
    let _ = grouped.terminal_masked_any_projections();
    let _ = grouped.terminal_conflicting_any_projections();
}

fn main() {}
