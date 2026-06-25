#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WorthUiLiveViewAdmissionCounters {
    binding_count: usize,
    denial_count: usize,
    state_fact_lookup_count: usize,
    source_reparse_count: usize,
    renderer_parse_count: usize,
}

impl WorthUiLiveViewAdmissionCounters {
    pub(crate) fn new(binding_count: usize, denial_count: usize) -> Self {
        Self {
            binding_count,
            denial_count,
            state_fact_lookup_count: binding_count,
            source_reparse_count: 0,
            renderer_parse_count: 0,
        }
    }

    pub fn binding_count(self) -> usize {
        self.binding_count
    }

    pub fn denial_count(self) -> usize {
        self.denial_count
    }

    pub fn state_fact_lookup_count(self) -> usize {
        self.state_fact_lookup_count
    }

    pub fn source_reparse_count(self) -> usize {
        self.source_reparse_count
    }

    pub fn renderer_parse_count(self) -> usize {
        self.renderer_parse_count
    }
}
