use std::num::NonZeroUsize;

use worth_query_admission::facade::graph_read_access::WorthQueryGraphReadBudget;

const DEFAULT_INLINE_INDEX_BYTES: usize = 5_120;
const DEFAULT_RESULT_BYTES_PER_ROOT: usize = 2_048;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryApplicationQueryResourceProfileDenial {
    ZeroInlineIndexBytes,
    ZeroResultBytesPerRoot,
    ZeroIntermediateSetSize,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthQueryApplicationQueryResourceProfile {
    maximum_inline_index_bytes: NonZeroUsize,
    maximum_result_bytes_per_root: NonZeroUsize,
    maximum_intermediate_set_size: NonZeroUsize,
}

impl WorthQueryApplicationQueryResourceProfile {
    pub fn bounded(
        maximum_inline_index_bytes: usize,
        maximum_result_bytes_per_root: usize,
        maximum_intermediate_set_size: usize,
    ) -> Result<Self, WorthQueryApplicationQueryResourceProfileDenial> {
        Ok(Self {
            maximum_inline_index_bytes: NonZeroUsize::new(maximum_inline_index_bytes)
                .ok_or(WorthQueryApplicationQueryResourceProfileDenial::ZeroInlineIndexBytes)?,
            maximum_result_bytes_per_root: NonZeroUsize::new(maximum_result_bytes_per_root)
                .ok_or(WorthQueryApplicationQueryResourceProfileDenial::ZeroResultBytesPerRoot)?,
            maximum_intermediate_set_size: NonZeroUsize::new(maximum_intermediate_set_size)
                .ok_or(WorthQueryApplicationQueryResourceProfileDenial::ZeroIntermediateSetSize)?,
        })
    }

    pub const fn maximum_inline_index_bytes(self) -> NonZeroUsize {
        self.maximum_inline_index_bytes
    }

    pub const fn maximum_result_bytes_per_root(self) -> NonZeroUsize {
        self.maximum_result_bytes_per_root
    }

    pub const fn maximum_intermediate_set_size(self) -> NonZeroUsize {
        self.maximum_intermediate_set_size
    }

    pub(crate) fn admission_budget(
        self,
        maximum_result_count: NonZeroUsize,
        request_maximum_work: NonZeroUsize,
    ) -> WorthQueryGraphReadBudget {
        WorthQueryGraphReadBudget::bounded(
            self.maximum_inline_index_bytes.get(),
            self.maximum_result_bytes_per_root
                .get()
                .saturating_mul(maximum_result_count.get()),
            self.maximum_intermediate_set_size
                .get()
                .min(request_maximum_work.get()),
        )
    }
}

impl Default for WorthQueryApplicationQueryResourceProfile {
    fn default() -> Self {
        Self {
            maximum_inline_index_bytes: NonZeroUsize::new(DEFAULT_INLINE_INDEX_BYTES)
                .expect("default inline index bytes are non-zero"),
            maximum_result_bytes_per_root: NonZeroUsize::new(DEFAULT_RESULT_BYTES_PER_ROOT)
                .expect("default result bytes are non-zero"),
            maximum_intermediate_set_size: NonZeroUsize::MAX,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::WorthQueryApplicationQueryResourceProfile;
    use std::num::NonZeroUsize;

    #[test]
    fn request_limits_only_narrow_the_installed_profile() {
        let profile =
            WorthQueryApplicationQueryResourceProfile::bounded(10_000, 2_000, 100).unwrap();
        let broad_request = profile.admission_budget(
            NonZeroUsize::new(3).unwrap(),
            NonZeroUsize::new(500).unwrap(),
        );
        assert_eq!(broad_request.max_inline_index_bytes(), 10_000);
        assert_eq!(broad_request.max_inline_result_bytes(), 6_000);
        assert_eq!(broad_request.max_inline_intermediate_set_size(), 100);

        let narrow_request = profile.admission_budget(
            NonZeroUsize::new(1).unwrap(),
            NonZeroUsize::new(40).unwrap(),
        );
        assert_eq!(narrow_request.max_inline_index_bytes(), 10_000);
        assert_eq!(narrow_request.max_inline_result_bytes(), 2_000);
        assert_eq!(narrow_request.max_inline_intermediate_set_size(), 40);
    }
}
