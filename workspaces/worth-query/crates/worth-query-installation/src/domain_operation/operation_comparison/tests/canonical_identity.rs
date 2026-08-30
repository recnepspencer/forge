use worth_foundational::facade::{
    AbsenceLaw, AspectContract, AspectContractRevision, AspectEvolutionPolicy, AspectIdentity,
    AspectKey, AspectMask, CanonicalFieldPath, FieldDeclaration, FieldKey, FieldRequirement,
    ProjectionMask, ScalarAspectType, StructAspectShape,
};

use crate::domain_operation::*;

use super::{operation, semantics};

#[test]
fn replay_comparator_family_and_noise_are_canonical_identity_dimensions() {
    let comparator = |family| {
        WorthQueryOperationReplayComparatorContract::new(family)
            .expect("test comparator family is portable")
    };
    let mut exact_semantics = semantics(1, "Entity");
    exact_semantics.replay = WorthQueryOperationReplayContract::CertReplayable {
        comparator: comparator("worth.tests.replay.exact"),
    };
    let mut alternate_semantics = semantics(1, "Entity");
    alternate_semantics.replay = WorthQueryOperationReplayContract::CertReplayable {
        comparator: comparator("worth.tests.replay.alternate"),
    };
    let mut strict_semantics = semantics(1, "Entity");
    strict_semantics.replay = WorthQueryOperationReplayContract::CertReplayableWithNoise {
        comparator: comparator("worth.tests.replay.exact"),
        noise: WorthQueryOperationReplayNoiseContract {
            diagnostic_warnings: false,
        },
    };
    let mut noisy_semantics = semantics(1, "Entity");
    noisy_semantics.replay = WorthQueryOperationReplayContract::CertReplayableWithNoise {
        comparator: comparator("worth.tests.replay.exact"),
        noise: WorthQueryOperationReplayNoiseContract {
            diagnostic_warnings: true,
        },
    };

    let exact = operation("inspect", 1, exact_semantics);
    let alternate = operation("inspect", 1, alternate_semantics);
    let strict = operation("inspect", 1, strict_semantics);
    let noisy = operation("inspect", 1, noisy_semantics);

    assert_ne!(exact.canonical_identity(), alternate.canonical_identity());
    assert_ne!(strict.canonical_identity(), noisy.canonical_identity());
}

#[test]
fn replay_identity_frames_owned_family_away_from_variant_and_noise() {
    for (injected_family, diagnostic_warnings) in
        [("strict:foo", false), ("diagnostic-noise:foo", true)]
    {
        let mut injected = semantics(1, "Entity");
        injected.replay = WorthQueryOperationReplayContract::CertReplayable {
            comparator: WorthQueryOperationReplayComparatorContract::new(injected_family)
                .expect("delimiter-bearing family remains portable"),
        };
        let mut separated = semantics(1, "Entity");
        separated.replay = WorthQueryOperationReplayContract::CertReplayableWithNoise {
            comparator: WorthQueryOperationReplayComparatorContract::new("foo")
                .expect("test comparator family is portable"),
            noise: WorthQueryOperationReplayNoiseContract {
                diagnostic_warnings,
            },
        };

        assert_ne!(
            operation("inspect", 1, injected).canonical_identity(),
            operation("inspect", 1, separated).canonical_identity()
        );
    }
}

#[test]
fn graph_read_identity_frames_mask_paths_without_delimiter_collisions() {
    let contract = delimiter_contract();
    let single_pipe = WorthQueryOperationNativeProjectionContract::new(
        contract.clone(),
        AspectMask::<ProjectionMask>::new([CanonicalFieldPath::single(
            FieldKey::new("a|b").unwrap(),
        )]),
    )
    .unwrap();
    let separate_paths = WorthQueryOperationNativeProjectionContract::new(
        contract,
        AspectMask::<ProjectionMask>::new([
            CanonicalFieldPath::single(FieldKey::new("a").unwrap()),
            CanonicalFieldPath::single(FieldKey::new("b").unwrap()),
        ]),
    )
    .unwrap();

    let mut left = semantics(1, "Entity");
    left.graph_reads = graph_read(single_pipe);
    let mut right = semantics(1, "Entity");
    right.graph_reads = graph_read(separate_paths);
    let left = operation("inspect", 1, left);
    let right = operation("inspect", 1, right);

    assert_ne!(left.canonical_identity(), right.canonical_identity());
}

fn graph_read(
    projection: WorthQueryOperationNativeProjectionContract,
) -> WorthQueryOperationGraphReadContract {
    WorthQueryOperationGraphReadContract::DeclaredDomain {
        roles: vec![WorthQueryDomainOperationGraphReadRole {
            role: "model".into(),
            participation: WorthQueryOperationGraphParticipation::PrimaryLogicalGraph,
            access: WorthQueryOperationGraphAccess::Project,
            semantic_reads: vec![projection],
        }],
    }
}

fn delimiter_contract() -> AspectContract {
    let fields = ["a|b", "a", "b"].map(|key| {
        FieldDeclaration::new(
            FieldKey::new(key).unwrap(),
            ScalarAspectType::String,
            FieldRequirement::Required,
            AbsenceLaw::Required,
            AspectEvolutionPolicy::ExplicitBreakRequired,
        )
        .unwrap()
    });
    AspectContract::struct_aspect(
        AspectKey::new("delimiter-test").unwrap(),
        AspectIdentity(1603),
        AspectContractRevision(1),
        StructAspectShape::new(fields).unwrap(),
    )
}
