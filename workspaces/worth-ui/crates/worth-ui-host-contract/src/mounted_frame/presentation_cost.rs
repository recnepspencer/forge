#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct UiHostPresentationCostReport {
    presented_surfaces: u64,
    translated_rows: u64,
    translated_bytes: u64,
    native_resource_cache_hits: u64,
    native_resource_cache_misses: u64,
    asynchronous_handoffs: u64,
    delta_rows_carried: u64,
    draw_list_mutations: u64,
    order_mutations: u64,
    logical_damage_regions: u64,
    logical_damage_pixels: u64,
    damage_index_probes: u64,
    intersecting_commands: u64,
    replayed_commands: u64,
    cleared_pixels: u64,
    rendered_pixels: u64,
    presented_pixels: u64,
    gpu_writes: u64,
    render_passes: u64,
    surface_copies: u64,
    surface_acquisitions: u64,
    queue_submissions: u64,
    presents: u64,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct UiHostPresentationCostInput {
    pub presented_surfaces: u64,
    pub translated_rows: u64,
    pub translated_bytes: u64,
    pub native_resource_cache_hits: u64,
    pub native_resource_cache_misses: u64,
    pub asynchronous_handoffs: u64,
    pub delta_rows_carried: u64,
    pub draw_list_mutations: u64,
    pub order_mutations: u64,
    pub logical_damage_regions: u64,
    pub logical_damage_pixels: u64,
    pub damage_index_probes: u64,
    pub intersecting_commands: u64,
    pub replayed_commands: u64,
    pub cleared_pixels: u64,
    pub rendered_pixels: u64,
    pub presented_pixels: u64,
    pub gpu_writes: u64,
    pub render_passes: u64,
    pub surface_copies: u64,
    pub surface_acquisitions: u64,
    pub queue_submissions: u64,
    pub presents: u64,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiHostPresentationCostOverflow;

macro_rules! cost_fields {
    ($source:ident, $($field:ident),+ $(,)?) => {
        Self { $($field: $source.$field),+ }
    };
}

macro_rules! checked_cost_fields {
    ($left:ident, $right:ident, $($field:ident),+ $(,)?) => {
        Ok(Self { $($field: checked($left.$field, $right.$field)?),+ })
    };
}

macro_rules! cost_accessors {
    ($($field:ident),+ $(,)?) => {
        $(pub const fn $field(self) -> u64 { self.$field })+
    };
}

impl UiHostPresentationCostReport {
    pub const fn from_adapter(input: UiHostPresentationCostInput) -> Self {
        cost_fields!(
            input,
            presented_surfaces,
            translated_rows,
            translated_bytes,
            native_resource_cache_hits,
            native_resource_cache_misses,
            asynchronous_handoffs,
            delta_rows_carried,
            draw_list_mutations,
            order_mutations,
            logical_damage_regions,
            logical_damage_pixels,
            damage_index_probes,
            intersecting_commands,
            replayed_commands,
            cleared_pixels,
            rendered_pixels,
            presented_pixels,
            gpu_writes,
            render_passes,
            surface_copies,
            surface_acquisitions,
            queue_submissions,
            presents,
        )
    }

    pub fn checked_add(self, other: Self) -> Result<Self, UiHostPresentationCostOverflow> {
        checked_cost_fields!(
            self,
            other,
            presented_surfaces,
            translated_rows,
            translated_bytes,
            native_resource_cache_hits,
            native_resource_cache_misses,
            asynchronous_handoffs,
            delta_rows_carried,
            draw_list_mutations,
            order_mutations,
            logical_damage_regions,
            logical_damage_pixels,
            damage_index_probes,
            intersecting_commands,
            replayed_commands,
            cleared_pixels,
            rendered_pixels,
            presented_pixels,
            gpu_writes,
            render_passes,
            surface_copies,
            surface_acquisitions,
            queue_submissions,
            presents,
        )
    }

    cost_accessors!(
        presented_surfaces,
        translated_rows,
        translated_bytes,
        native_resource_cache_hits,
        native_resource_cache_misses,
        asynchronous_handoffs,
        delta_rows_carried,
        draw_list_mutations,
        order_mutations,
        logical_damage_regions,
        logical_damage_pixels,
        damage_index_probes,
        intersecting_commands,
        replayed_commands,
        cleared_pixels,
        rendered_pixels,
        presented_pixels,
        gpu_writes,
        render_passes,
        surface_copies,
        surface_acquisitions,
        queue_submissions,
        presents,
    );
}

fn checked(left: u64, right: u64) -> Result<u64, UiHostPresentationCostOverflow> {
    left.checked_add(right)
        .ok_or(UiHostPresentationCostOverflow)
}
