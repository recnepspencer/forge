#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WorthUiEffectiveViewportParticipationCounters {
    row_count: usize,
    clipped_row_count: usize,
    governing_boundary_application_count: usize,
    source_reparse_count: usize,
    renderer_parse_count: usize,
}

impl WorthUiEffectiveViewportParticipationCounters {
    pub(super) fn new(
        row_count: usize,
        clipped_row_count: usize,
        governing_boundary_application_count: usize,
    ) -> Self {
        Self {
            row_count,
            clipped_row_count,
            governing_boundary_application_count,
            source_reparse_count: 0,
            renderer_parse_count: 0,
        }
    }

    pub fn row_count(self) -> usize {
        self.row_count
    }

    pub fn clipped_row_count(self) -> usize {
        self.clipped_row_count
    }

    pub fn governing_boundary_application_count(self) -> usize {
        self.governing_boundary_application_count
    }

    pub fn source_reparse_count(self) -> usize {
        self.source_reparse_count
    }

    pub fn renderer_parse_count(self) -> usize {
        self.renderer_parse_count
    }
}
