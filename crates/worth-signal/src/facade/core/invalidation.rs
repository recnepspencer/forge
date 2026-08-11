use std::ops::DerefMut;

use crate::data::aspect::Aspect;
use crate::data::error::SignalError;
use crate::data::graph::SignalGraph;
use crate::data::handle::NodeId;
use crate::data::output::ChangedRegion;

pub fn mark_dirty(
    graph: impl DerefMut<Target = SignalGraph>,
    source: NodeId,
    changed_aspect: Aspect,
) -> Result<(), SignalError> {
    #[cfg(any(test, doctest))]
    {
        crate::logic::invalidation::mark_dirty(graph, source, changed_aspect)
    }
    #[cfg(not(any(test, doctest)))]
    {
        let _ = crate::logic::invalidation::mark_dirty_batch(
            graph,
            &crate::data::proof::DirtyBatch::singleton(
                source,
                changed_aspect,
                Vec::<ChangedRegion>::new(),
            ),
        )?;
        Ok(())
    }
}

pub fn mark_changed(
    graph: impl DerefMut<Target = SignalGraph>,
    source: NodeId,
    changed_aspect: Aspect,
) -> Result<(), SignalError> {
    mark_dirty(graph, source, changed_aspect)
}

pub fn mark_dirty_with_regions(
    graph: impl DerefMut<Target = SignalGraph>,
    source: NodeId,
    changed_aspect: Aspect,
    changed_regions: &[ChangedRegion],
) -> Result<(), SignalError> {
    #[cfg(any(test, doctest))]
    {
        crate::logic::invalidation::mark_dirty_with_regions(
            graph,
            source,
            changed_aspect,
            changed_regions,
        )
    }
    #[cfg(not(any(test, doctest)))]
    {
        let _ = crate::logic::invalidation::mark_dirty_batch(
            graph,
            &crate::data::proof::DirtyBatch::singleton(
                source,
                changed_aspect,
                changed_regions.to_vec(),
            ),
        )?;
        Ok(())
    }
}

pub fn mark_changed_with_regions(
    graph: impl DerefMut<Target = SignalGraph>,
    source: NodeId,
    changed_aspect: Aspect,
    changed_regions: &[ChangedRegion],
) -> Result<(), SignalError> {
    mark_dirty_with_regions(graph, source, changed_aspect, changed_regions)
}
