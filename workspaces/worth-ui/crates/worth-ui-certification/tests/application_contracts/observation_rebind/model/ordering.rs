#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
enum ObservationOwner {
    AuthoredSource,
    Query,
    Measurement,
    Viewport,
    ScrollExtent,
    PortalAnchor,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum DuplicateLaw {
    Reject,
    SuppressEquivalent,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum LossLaw {
    Lossless,
    LatestValue,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum ResetLaw {
    ExplicitOnly,
    Unsupported,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct FamilyLaw {
    owner: ObservationOwner,
    duplicate: DuplicateLaw,
    loss: LossLaw,
    reset: ResetLaw,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct ModelObservation {
    owner: ObservationOwner,
    owner_order: u64,
}

fn family_law(owner: ObservationOwner) -> FamilyLaw {
    match owner {
        ObservationOwner::AuthoredSource => FamilyLaw {
            owner,
            duplicate: DuplicateLaw::SuppressEquivalent,
            loss: LossLaw::Lossless,
            reset: ResetLaw::ExplicitOnly,
        },
        ObservationOwner::Query => FamilyLaw {
            owner,
            duplicate: DuplicateLaw::Reject,
            loss: LossLaw::Lossless,
            reset: ResetLaw::ExplicitOnly,
        },
        ObservationOwner::Measurement => FamilyLaw {
            owner,
            duplicate: DuplicateLaw::SuppressEquivalent,
            loss: LossLaw::Lossless,
            reset: ResetLaw::Unsupported,
        },
        ObservationOwner::Viewport => FamilyLaw {
            owner,
            duplicate: DuplicateLaw::SuppressEquivalent,
            loss: LossLaw::LatestValue,
            reset: ResetLaw::Unsupported,
        },
        ObservationOwner::ScrollExtent | ObservationOwner::PortalAnchor => FamilyLaw {
            owner,
            duplicate: DuplicateLaw::Reject,
            loss: LossLaw::Lossless,
            reset: ResetLaw::Unsupported,
        },
    }
}

fn canonical_order(mut observations: Vec<ModelObservation>) -> Vec<ModelObservation> {
    observations.sort_by_key(|observation| (observation.owner, observation.owner_order));
    observations
}

#[test]
fn framework_owner_rank_precedes_owner_issued_order() {
    let observations = vec![
        ModelObservation {
            owner: ObservationOwner::PortalAnchor,
            owner_order: 1,
        },
        ModelObservation {
            owner: ObservationOwner::AuthoredSource,
            owner_order: 90,
        },
        ModelObservation {
            owner: ObservationOwner::Query,
            owner_order: 2,
        },
        ModelObservation {
            owner: ObservationOwner::Query,
            owner_order: 1,
        },
    ];
    assert_eq!(
        canonical_order(observations),
        vec![
            ModelObservation {
                owner: ObservationOwner::AuthoredSource,
                owner_order: 90,
            },
            ModelObservation {
                owner: ObservationOwner::Query,
                owner_order: 1,
            },
            ModelObservation {
                owner: ObservationOwner::Query,
                owner_order: 2,
            },
            ModelObservation {
                owner: ObservationOwner::PortalAnchor,
                owner_order: 1,
            },
        ]
    );
}

#[test]
fn duplicate_loss_and_reset_laws_remain_owner_specific() {
    let owners = [
        ObservationOwner::AuthoredSource,
        ObservationOwner::Query,
        ObservationOwner::Measurement,
        ObservationOwner::Viewport,
        ObservationOwner::ScrollExtent,
        ObservationOwner::PortalAnchor,
    ];
    let laws = owners.map(family_law);
    assert_eq!(laws[0].reset, ResetLaw::ExplicitOnly);
    assert_eq!(laws[1].reset, ResetLaw::ExplicitOnly);
    assert_eq!(laws[2].reset, ResetLaw::Unsupported);
    assert_eq!(laws[3].loss, LossLaw::LatestValue);
    assert_eq!(laws[4].duplicate, DuplicateLaw::Reject);
    assert_eq!(laws[5].duplicate, DuplicateLaw::Reject);
    assert_ne!(laws[0], laws[1]);
    assert_ne!(laws[2], laws[3]);
}
