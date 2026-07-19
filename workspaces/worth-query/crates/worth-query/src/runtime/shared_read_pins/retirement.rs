use std::collections::BTreeMap;
use std::sync::Arc;

use super::WorthQuerySharedReadGenerationEntry;

pub(in crate::runtime) fn collect_retired_zero_pin_generations(
    generations: &mut BTreeMap<u64, Arc<WorthQuerySharedReadGenerationEntry>>,
) {
    let removable = generations
        .iter()
        .filter_map(|(ordinal, entry)| {
            (entry.is_retired() && entry.pin_count() == 0).then_some(*ordinal)
        })
        .collect::<Vec<_>>();
    for ordinal in removable {
        generations.remove(&ordinal);
    }
}
