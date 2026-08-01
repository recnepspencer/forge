use worth_ui::facade::observation::UiObservationFamily;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub(super) enum Cause {
    Interaction,
    Source,
    Query,
    Viewport,
}

pub(super) const CANONICAL_OBSERVATIONS: [UiObservationFamily; 3] = [
    UiObservationFamily::AuthoredSource,
    UiObservationFamily::HostViewport,
    UiObservationFamily::Query,
];

pub(super) fn permutations() -> Vec<[Cause; 4]> {
    let causes = [
        Cause::Interaction,
        Cause::Source,
        Cause::Query,
        Cause::Viewport,
    ];
    let mut permutations = Vec::with_capacity(24);
    for first in causes {
        for second in causes {
            for third in causes {
                for fourth in causes {
                    let candidate = [first, second, third, fourth];
                    if all_distinct(candidate) {
                        permutations.push(candidate);
                    }
                }
            }
        }
    }
    permutations
}

fn all_distinct(candidate: [Cause; 4]) -> bool {
    (0..candidate.len())
        .all(|left| ((left + 1)..candidate.len()).all(|right| candidate[left] != candidate[right]))
}
