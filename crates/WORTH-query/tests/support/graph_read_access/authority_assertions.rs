use worth_query::facade::runtime::WorthQueryGraphReadAccessAuthorityDenial;

pub fn assert_authority_denial_before_buffers(denial: &WorthQueryGraphReadAccessAuthorityDenial) {
    assert_eq!(denial.counters().authority_denial_count(), 1);
    assert_eq!(denial.counters().adjacency_buffer_build_count(), 0);
    assert_eq!(denial.counters().frontier_buffer_build_count(), 0);
    assert_eq!(denial.counters().visited_buffer_build_count(), 0);
    assert_eq!(denial.counters().result_buffer_build_count(), 0);
}
