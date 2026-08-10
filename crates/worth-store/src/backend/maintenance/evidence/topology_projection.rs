pub(super) fn project_scheduler_topology(
    work_class_count: u64,
    locality_bucket_count: u64,
) -> crate::Milestone11SchedulerTopologyReport {
    crate::Milestone11SchedulerTopologyReport {
        queue_family_count: work_class_count,
        locality_bucket_count,
        has_restart_recovered_intake_lane: true,
        has_foreground_reservation_pool: true,
        has_background_reservation_pool: true,
    }
}
