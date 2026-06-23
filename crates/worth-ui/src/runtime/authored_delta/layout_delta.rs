use crate::source::{
    WorthUiLayoutTopologyChild, WorthUiLayoutTopologyNode, WorthUiPageLayoutTopology,
};

pub(super) fn layout_topology_changed(
    active: Option<&WorthUiPageLayoutTopology>,
    candidate: Option<&WorthUiPageLayoutTopology>,
) -> bool {
    match (active, candidate) {
        (Some(active), Some(candidate)) => {
            node_topology_changed(active.root(), candidate.root())
                || active.page_name() != candidate.page_name()
                || active.layout_name() != candidate.layout_name()
                || active.dynamic_template() != candidate.dynamic_template()
        }
        _ => active != candidate,
    }
}

pub(super) fn layout_gap_changed(
    active: Option<&WorthUiPageLayoutTopology>,
    candidate: Option<&WorthUiPageLayoutTopology>,
) -> bool {
    match (active, candidate) {
        (Some(active), Some(candidate)) => node_gap_changed(active.root(), candidate.root()),
        _ => active != candidate,
    }
}

pub(super) fn layout_padding_changed(
    active: Option<&WorthUiPageLayoutTopology>,
    candidate: Option<&WorthUiPageLayoutTopology>,
) -> bool {
    match (active, candidate) {
        (Some(active), Some(candidate)) => node_padding_changed(active.root(), candidate.root()),
        _ => active != candidate,
    }
}

fn node_topology_changed(
    active: &WorthUiLayoutTopologyNode,
    candidate: &WorthUiLayoutTopologyNode,
) -> bool {
    if active.axis() != candidate.axis()
        || active.dimension() != candidate.dimension()
        || active.sizing() != candidate.sizing()
        || active.scroll_owner() != candidate.scroll_owner()
        || active.resizable() != candidate.resizable()
        || active.restorable() != candidate.restorable()
        || active.children().len() != candidate.children().len()
    {
        return true;
    }

    active
        .children()
        .iter()
        .zip(candidate.children())
        .any(child_topology_changed)
}

fn child_topology_changed(
    (active, candidate): (&WorthUiLayoutTopologyChild, &WorthUiLayoutTopologyChild),
) -> bool {
    match (active, candidate) {
        (
            WorthUiLayoutTopologyChild::Region(active),
            WorthUiLayoutTopologyChild::Region(candidate),
        ) => node_topology_changed(active, candidate),
        (WorthUiLayoutTopologyChild::Slot(active), WorthUiLayoutTopologyChild::Slot(candidate)) => {
            active.slot_name() != candidate.slot_name()
        }
        _ => true,
    }
}

fn node_gap_changed(
    active: &WorthUiLayoutTopologyNode,
    candidate: &WorthUiLayoutTopologyNode,
) -> bool {
    if active.gap() != candidate.gap() || active.children().len() != candidate.children().len() {
        return true;
    }

    active
        .children()
        .iter()
        .zip(candidate.children())
        .any(|(active, candidate)| match (active, candidate) {
            (
                WorthUiLayoutTopologyChild::Region(active),
                WorthUiLayoutTopologyChild::Region(candidate),
            ) => node_gap_changed(active, candidate),
            (WorthUiLayoutTopologyChild::Slot(_), WorthUiLayoutTopologyChild::Slot(_)) => false,
            _ => true,
        })
}

fn node_padding_changed(
    active: &WorthUiLayoutTopologyNode,
    candidate: &WorthUiLayoutTopologyNode,
) -> bool {
    if active.padding() != candidate.padding()
        || active.children().len() != candidate.children().len()
    {
        return true;
    }

    active
        .children()
        .iter()
        .zip(candidate.children())
        .any(|(active, candidate)| match (active, candidate) {
            (
                WorthUiLayoutTopologyChild::Region(active),
                WorthUiLayoutTopologyChild::Region(candidate),
            ) => node_padding_changed(active, candidate),
            (WorthUiLayoutTopologyChild::Slot(_), WorthUiLayoutTopologyChild::Slot(_)) => false,
            _ => true,
        })
}
