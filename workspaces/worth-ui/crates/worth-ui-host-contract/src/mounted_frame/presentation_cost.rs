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
    order_index_lookups: u64,
    order_index_node_touches: u64,
    order_index_rotations: u64,
    order_index_high_water: u64,
    logical_damage_regions: u64,
    logical_damage_pixels: u64,
    retained_command_scans: u64,
    retained_command_clones: u64,
    damage_index_probes: u64,
    damage_index_stored_records: u64,
    damage_index_high_water: u64,
    damage_region_command_checks: u64,
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
    pub order_index_lookups: u64,
    pub order_index_node_touches: u64,
    pub order_index_rotations: u64,
    pub order_index_high_water: u64,
    pub logical_damage_regions: u64,
    pub logical_damage_pixels: u64,
    pub retained_command_scans: u64,
    pub retained_command_clones: u64,
    pub damage_index_probes: u64,
    pub damage_index_stored_records: u64,
    pub damage_index_high_water: u64,
    pub damage_region_command_checks: u64,
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

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct UiMountedPresentationProductionCost {
    source_instances: u64,
    commands_considered: u64,
    command_index_lookups: u64,
    order_lookups: u64,
    retained_command_scans: u64,
    retained_command_clones: u64,
    projection_rows_materialized: u64,
}

#[doc(hidden)]
pub struct UiMountedPresentationProductionCostInput {
    pub source_instances: u64,
    pub commands_considered: u64,
    pub command_index_lookups: u64,
    pub order_lookups: u64,
    pub retained_command_scans: u64,
    pub retained_command_clones: u64,
    pub projection_rows_materialized: u64,
}

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
            order_index_lookups,
            order_index_node_touches,
            order_index_rotations,
            order_index_high_water,
            logical_damage_regions,
            logical_damage_pixels,
            retained_command_scans,
            retained_command_clones,
            damage_index_probes,
            damage_index_stored_records,
            damage_index_high_water,
            damage_region_command_checks,
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
            order_index_lookups,
            order_index_node_touches,
            order_index_rotations,
            order_index_high_water,
            logical_damage_regions,
            logical_damage_pixels,
            retained_command_scans,
            retained_command_clones,
            damage_index_probes,
            damage_index_stored_records,
            damage_index_high_water,
            damage_region_command_checks,
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
        order_index_lookups,
        order_index_node_touches,
        order_index_rotations,
        order_index_high_water,
        logical_damage_regions,
        logical_damage_pixels,
        retained_command_scans,
        retained_command_clones,
        damage_index_probes,
        damage_index_stored_records,
        damage_index_high_water,
        damage_region_command_checks,
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

impl UiMountedPresentationProductionCost {
    #[doc(hidden)]
    pub const fn from_runtime_mounting(input: UiMountedPresentationProductionCostInput) -> Self {
        Self {
            source_instances: input.source_instances,
            commands_considered: input.commands_considered,
            command_index_lookups: input.command_index_lookups,
            order_lookups: input.order_lookups,
            retained_command_scans: input.retained_command_scans,
            retained_command_clones: input.retained_command_clones,
            projection_rows_materialized: input.projection_rows_materialized,
        }
    }

    cost_accessors!(
        source_instances,
        commands_considered,
        command_index_lookups,
        order_lookups,
        retained_command_scans,
        retained_command_clones,
        projection_rows_materialized,
    );
}

fn checked(left: u64, right: u64) -> Result<u64, UiHostPresentationCostOverflow> {
    left.checked_add(right)
        .ok_or(UiHostPresentationCostOverflow)
}
