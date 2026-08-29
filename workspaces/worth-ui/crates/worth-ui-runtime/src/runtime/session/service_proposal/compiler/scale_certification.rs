pub(crate) fn proposal_scale_evidence() -> (u64, u64, bool) {
    let mut compiler = super::UiServiceProposalCompiler::new();
    let support = all_service_support();
    let mut reservations = Vec::new();

    for index in 0_u64..64 {
        let coherence = super::super::fixture_service_request_coherence(120 + index);
        let candidate = six_family_candidate(1_000 + index, index + 1, &coherence);
        let preflighted = compiler
            .preflight(candidate, &coherence, support)
            .expect("scale proposal preflights");
        let reservation = match compiler
            .reserve(preflighted)
            .expect("scale proposal reserves")
        {
            super::UiServiceProposalReservationOutcome::Reserved(reservation) => reservation,
            super::UiServiceProposalReservationOutcome::Coalesced { .. } => {
                panic!("independent service neighborhoods cannot coalesce")
            }
        };
        reservations.push(reservation);
    }
    let counters = compiler.occupancy_work_counters();
    let visited = counters.proposal_requirements_visited();
    let unrelated = counters.unrelated_neighborhoods_touched();
    while let Some(reservation) = reservations.pop() {
        compiler
            .shutdown_reservation(reservation)
            .expect("reservation releases at shutdown");
    }
    (visited, unrelated, compiler.census().is_zero())
}

fn six_family_candidate(
    identity: u64,
    scope: u64,
    coherence: &super::super::UiServiceRequestCoherence,
) -> super::UiServiceProposalCandidate {
    let families = crate::capability::UiRuntimeServiceFamily::ALL
        .into_iter()
        .map(|family| super::UiServiceFamilyProposal::recorded_fixture(family, scope, 1, 1, 1))
        .collect();
    super::UiServiceProposalCandidate::for_test(
        identity,
        super::UiServiceProposalDemand::recorded_fixture(
            super::super::fixture_service_family_participation(6),
            6,
            6,
            6,
        ),
        coherence.clone(),
        families,
    )
}

fn all_service_support() -> crate::capability::UiRuntimeServiceSupport {
    crate::capability::UiRuntimeServiceFamily::ALL
        .into_iter()
        .fold(
            crate::capability::UiRuntimeServiceSupport::none_installed(),
            crate::capability::UiRuntimeServiceSupport::with_installed,
        )
}
