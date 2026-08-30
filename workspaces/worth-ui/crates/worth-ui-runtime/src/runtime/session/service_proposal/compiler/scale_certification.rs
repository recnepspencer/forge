/// Proposal-compiler scale evidence for `RS-10`.
///
/// The 64 neighborhoods are sibling semantic surfaces of **one** application
/// generation, so they are genuinely capable of interfering: a plan that swept
/// the index instead of keying into its own neighborhood would visit the other
/// 63. Every returned count is read back from live compiler state; none is a
/// literal restated by the courtroom.
pub(crate) struct UiServiceProposalScaleEvidence {
    pub(crate) neighborhoods: u64,
    pub(crate) proposal_requirements_visited: u64,
    pub(crate) unrelated_neighborhoods_touched: u64,
    pub(crate) terminal_census_is_zero: bool,
}

const SCALE_NEIGHBORHOODS: u64 = 64;

pub(crate) fn proposal_scale_evidence() -> UiServiceProposalScaleEvidence {
    let mut compiler = super::UiServiceProposalCompiler::new();
    let support = all_service_support();
    let application = super::super::fixture_application_generation(1);
    let mut reservations = Vec::new();

    for index in 0..SCALE_NEIGHBORHOODS {
        let coherence =
            super::super::fixture_service_request_coherence_in(&application, 120 + index);
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
    let evidence = UiServiceProposalScaleEvidence {
        neighborhoods: compiler.live_neighborhood_count() as u64,
        proposal_requirements_visited: counters.proposal_requirements_visited(),
        unrelated_neighborhoods_touched: counters.unrelated_neighborhoods_touched(),
        terminal_census_is_zero: false,
    };
    while let Some(reservation) = reservations.pop() {
        compiler
            .shutdown_reservation(reservation)
            .expect("reservation releases at shutdown");
    }
    UiServiceProposalScaleEvidence {
        terminal_census_is_zero: compiler.census().is_zero()
            && compiler.live_neighborhood_count() == 0,
        ..evidence
    }
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
