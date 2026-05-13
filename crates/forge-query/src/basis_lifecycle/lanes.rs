pub trait BasisOperationLane: Clone + core::fmt::Debug + Eq + PartialEq {
    fn lane_name() -> &'static str;
}

macro_rules! lane_witness {
    ($name:ident, $lane:literal) => {
        #[derive(Clone, Debug, Eq, PartialEq)]
        pub struct $name {
            _sealed: (),
        }

        impl $name {
            pub(crate) fn new() -> Self {
                Self { _sealed: () }
            }
        }

        impl BasisOperationLane for $name {
            fn lane_name() -> &'static str {
                $lane
            }
        }
    };
}

lane_witness!(ObservationLaneWitness, "observation");
lane_witness!(MutationPreparationLaneWitness, "mutation_preparation");
lane_witness!(ReplayLaneWitness, "replay");
lane_witness!(InspectionLaneWitness, "inspection");
lane_witness!(MaterializationLaneWitness, "materialization");
lane_witness!(
    SubscriptionDeclarationLaneWitness,
    "subscription_declaration"
);
lane_witness!(SubscriptionActivationLaneWitness, "subscription_activation");
lane_witness!(PreviewCloseoutLaneWitness, "preview_closeout");
lane_witness!(CertificationLaneWitness, "certification");
