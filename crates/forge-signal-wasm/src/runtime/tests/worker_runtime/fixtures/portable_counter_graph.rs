use crate::runtime::worker_host::WorkerPortableGraphPublication;

use crate::runtime::tests::support::*;

pub(in crate::runtime::tests::worker_runtime) fn portable_counter_publication(
) -> WorkerPortableGraphPublication {
    WorkerPortableGraphPublication {
        policy: RuntimePolicySpec::default(),
        sources: vec![SourceSpec {
            id: "counter".to_owned(),
            initial: SignalValue::Number(1.0),
            produces_aspects: None,
        }],
        recipes: vec![RecipeSpec {
            id: "doubleCounter".to_owned(),
            reads: vec![RecipeReadSpec::LegacyId("counter".to_owned())],
            expr: Expr::Sum {
                args: vec![read("counter"), read("counter")],
            },
            when: None,
            identity: Some(IdentitySpec::Exact),
            produces_aspects: None,
        }],
    }
}

pub(in crate::runtime::tests::worker_runtime) fn define_portable_counter_graph(
    runtime: &mut RuntimeCore,
) {
    runtime
        .define_source(SourceSpec {
            id: "counter".to_owned(),
            initial: SignalValue::Number(1.0),
            produces_aspects: None,
        })
        .unwrap();
    runtime
        .define_recipe(RecipeSpec {
            id: "doubleCounter".to_owned(),
            reads: vec![RecipeReadSpec::LegacyId("counter".to_owned())],
            expr: Expr::Sum {
                args: vec![read("counter"), read("counter")],
            },
            when: None,
            identity: Some(IdentitySpec::Exact),
            produces_aspects: None,
        })
        .unwrap();
}
