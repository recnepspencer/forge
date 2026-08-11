//! Public trust-boundary progression into the sealed boundary wrapper.

worth_proof::authority_marker!(WitnessAuthority);

struct WitnessPhase;
impl worth_proof::PhaseMarker for WitnessPhase {}

fn authority() -> worth_proof::AuthorityWitness<WitnessAuthority> {
    WitnessAuthority::witness()
}

pub(crate) fn no_assumption_basis() -> worth_proof::NoAssumptionBasis {
    worth_proof::NoAssumptionBasis
}

pub(crate) fn current_validity() -> worth_proof::CurrentValidity {
    worth_proof::CurrentValidity
}

pub(crate) fn stale_readable() -> worth_proof::StaleReadable {
    worth_proof::StaleReadable
}

pub(crate) fn rebind_required() -> worth_proof::RebindRequired {
    worth_proof::RebindRequired
}

pub(crate) fn authority_revalidation_required() -> worth_proof::AuthorityRevalidationRequired {
    worth_proof::AuthorityRevalidationRequired
}

pub(crate) fn boundary_bridged() -> worth_proof::BoundaryBridged<
    worth_proof::AuthorityRevalidationRequiredBasis<u8>,
> {
    let artifact = worth_proof::Artifact::<WitnessPhase, _, _, _>::with_current_basis(
        "payload",
        11_u8,
        authority(),
    );
    let (_, _, boundary) = artifact.bridge_trust_boundary().into_parts().into_parts();
    boundary
}

pub(crate) fn assumption_basis() -> worth_proof::AssumptionBasis<u8> {
    worth_proof::AssumptionBasis::new(11)
}

pub(crate) fn freshness_scoped_basis() -> worth_proof::FreshnessScopedBasis<
    worth_proof::CurrentValidity,
    worth_proof::AssumptionBasis<u8>,
> {
    let artifact = worth_proof::Artifact::<WitnessPhase, _, _, _>::with_current_basis(
        "payload",
        11_u8,
        authority(),
    );
    let (_, _, basis) = artifact.into_parts().into_parts();
    basis
}

pub(crate) struct WitnessSource;

impl worth_proof::FreshnessSource for WitnessSource {
    type Sample = u8;
    type Error = core::convert::Infallible;

    fn sample(&self) -> Result<Self::Sample, Self::Error> {
        Ok(13)
    }
}

pub(crate) struct WitnessPolicy;

impl worth_proof::FreshnessPolicy<WitnessSource, u8> for WitnessPolicy {
    fn classify(&self, _basis: &u8, _observed_at: &u8) -> worth_proof::FreshnessVerdict {
        worth_proof::FreshnessVerdict::Current
    }
}

pub(crate) fn freshness_sample() -> worth_proof::FreshnessSample<WitnessSource> {
    worth_proof::take_sample(&WitnessSource).expect("the witness source is infallible")
}

pub(crate) fn freshness_evaluation(
) -> worth_proof::FreshnessEvaluation<WitnessSource, WitnessPolicy, u8> {
    let evaluated = worth_proof::evaluate_freshness(7, &WitnessSource, &WitnessPolicy)
        .expect("the witness source is infallible");
    match evaluated {
        worth_proof::EvaluatedFreshness::Current(value) => value.into_basis(),
        worth_proof::EvaluatedFreshness::StaleReadable(value) => value.into_basis(),
        worth_proof::EvaluatedFreshness::RebindRequired(value) => value.into_basis(),
        worth_proof::EvaluatedFreshness::AuthorityRevalidationRequired(value) => {
            value.into_basis()
        }
    }
}

pub(crate) fn evaluated_freshness(
) -> worth_proof::EvaluatedFreshness<WitnessSource, WitnessPolicy, u8> {
    worth_proof::evaluate_freshness(7, &WitnessSource, &WitnessPolicy)
        .expect("the witness source is infallible")
}

pub(crate) fn freshness_verdict() -> worth_proof::FreshnessVerdict {
    worth_proof::FreshnessVerdict::Current
}
