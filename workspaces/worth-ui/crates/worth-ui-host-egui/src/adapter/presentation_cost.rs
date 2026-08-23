use worth_ui_host_contract::{
    UiHostPresentationCostInput, UiHostPresentationCostReport, UiHostSurfacePresentationDenial,
    UiMountedPresentationWorkView,
};

pub(super) fn for_work(
    work: UiMountedPresentationWorkView<'_>,
) -> Result<UiHostPresentationCostReport, UiHostSurfacePresentationDenial> {
    let (presented_surfaces, rows, bytes, delta_rows, draw_mutations, order_mutations, damage) =
        match work {
            UiMountedPresentationWorkView::Initial(initial) => {
                let rows = initial
                    .commands()
                    .len()
                    .checked_add(initial.order().len())
                    .and_then(|value| value.checked_add(initial.damage().len()))
                    .ok_or(UiHostSurfacePresentationDenial::CapacityExceeded)?;
                (
                    1,
                    rows,
                    std::mem::size_of_val(initial.commands()),
                    0,
                    0,
                    0,
                    initial.damage().len(),
                )
            }
            UiMountedPresentationWorkView::Delta(delta) => {
                let rows = delta
                    .changes()
                    .len()
                    .checked_add(delta.nodes().len())
                    .and_then(|value| value.checked_add(delta.order().len()))
                    .and_then(|value| value.checked_add(delta.damage().len()))
                    .ok_or(UiHostSurfacePresentationDenial::CapacityExceeded)?;
                (
                    1,
                    rows,
                    std::mem::size_of_val(delta.changes()) + std::mem::size_of_val(delta.nodes()),
                    rows,
                    delta.changes().len(),
                    delta.order().len(),
                    delta.damage().len(),
                )
            }
            UiMountedPresentationWorkView::Reconstruction(work) => {
                let rows = work
                    .commands()
                    .len()
                    .checked_add(work.order().len())
                    .and_then(|value| value.checked_add(work.damage().len()))
                    .ok_or(UiHostSurfacePresentationDenial::CapacityExceeded)?;
                (
                    1,
                    rows,
                    std::mem::size_of_val(work.commands()),
                    rows,
                    work.commands().len(),
                    work.order().len(),
                    work.damage().len(),
                )
            }
            UiMountedPresentationWorkView::Unchanged(_) => (0, 0, 0, 0, 0, 0, 0),
        };
    Ok(UiHostPresentationCostReport::from_adapter(
        UiHostPresentationCostInput {
            presented_surfaces,
            translated_rows: u64::try_from(rows)
                .map_err(|_| UiHostSurfacePresentationDenial::CapacityExceeded)?,
            translated_bytes: u64::try_from(bytes)
                .map_err(|_| UiHostSurfacePresentationDenial::CapacityExceeded)?,
            native_resource_cache_hits: 0,
            native_resource_cache_misses: 0,
            asynchronous_handoffs: 0,
            delta_rows_carried: u64::try_from(delta_rows)
                .map_err(|_| UiHostSurfacePresentationDenial::CapacityExceeded)?,
            draw_list_mutations: u64::try_from(draw_mutations)
                .map_err(|_| UiHostSurfacePresentationDenial::CapacityExceeded)?,
            order_mutations: u64::try_from(order_mutations)
                .map_err(|_| UiHostSurfacePresentationDenial::CapacityExceeded)?,
            logical_damage_regions: u64::try_from(damage)
                .map_err(|_| UiHostSurfacePresentationDenial::CapacityExceeded)?,
            ..Default::default()
        },
    ))
}
