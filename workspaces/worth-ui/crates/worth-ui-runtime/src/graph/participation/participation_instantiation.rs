use crate::graph::{UiGraphNodeInstantiationEntry, UiGraphParticipationPosture};

pub(crate) fn materialize_graph_participation_posture(
    entry: &UiGraphNodeInstantiationEntry,
) -> UiGraphParticipationPosture {
    entry.participation_seed().posture()
}
