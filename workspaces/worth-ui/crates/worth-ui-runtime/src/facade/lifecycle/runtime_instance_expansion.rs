use std::collections::BTreeMap;

use crate::declaration::UiDeclarationGraphHandoff;
use crate::graph::UiRuntimeInstanceBasisAdmission;

/// Expand one semantic declaration handoff into the runtime-keyed occurrences
/// explicitly admitted for preparation. Declarations without runtime bases
/// remain single declaration-keyed occurrences.
pub(super) fn expand_runtime_instance_handoffs(
    handoffs: &[UiDeclarationGraphHandoff],
    runtime_bases: &[UiRuntimeInstanceBasisAdmission],
) -> Vec<UiDeclarationGraphHandoff> {
    let occurrence_counts =
        runtime_bases
            .iter()
            .fold(BTreeMap::<u64, usize>::new(), |mut counts, admission| {
                *counts
                    .entry(admission.declaration_identity().digest().raw())
                    .or_default() += 1;
                counts
            });

    handoffs
        .iter()
        .flat_map(|handoff| {
            let occurrences = occurrence_counts
                .get(&handoff.identity().digest().raw())
                .copied()
                .unwrap_or(1);
            std::iter::repeat_n(handoff.clone(), occurrences)
        })
        .collect()
}
