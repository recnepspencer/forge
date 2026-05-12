use crate::runtime::worker_host::{WorkerPortableGraphPublication, WorkerRuntimeShell};

use crate::recipe::model::{AspectSelectionSpec, RecipeReadSignalSpec};
use crate::runtime::tests::support::*;

#[test]
fn worker_runtime_shell_denies_callback_definition_envelope_publication() {
    let mut compatibility_runtime = RuntimeCore::new(RuntimePolicySpec::default()).unwrap();
    compatibility_runtime
        .define_source(SourceSpec {
            id: "counter".to_owned(),
            initial: SignalValue::Number(1.0),
            produces_aspects: None,
        })
        .unwrap();
    compatibility_runtime
        .define_web_computed_native_callback(
            "callbackDouble".to_owned(),
            Box::new(|| {
                Ok(compute_callbacks::ComputeCallbackInvocationResult {
                    value: SignalValue::Number(2.0),
                    captured_read_ids: vec!["counter".to_owned()],
                    captured_host_capability_reads: Vec::new(),
                    runtime_read_breadth: 1,
                    return_serialization_breadth: 1,
                })
            }),
        )
        .unwrap();

    let definition_envelope = compatibility_runtime.export_definitions().unwrap();
    let mut worker_shell = WorkerRuntimeShell::new(RuntimePolicySpec::default()).unwrap();
    let err = worker_shell
        .publish_definition_envelope(definition_envelope)
        .unwrap_err();

    assert_eq!(
        err.code,
        "workerRuntimePublicationRequiresPortableDefinitions"
    );
    assert!(err.message.contains("callbackDouble"));
}

#[test]
fn worker_portable_publication_rejects_unknown_reads_without_partial_sources() {
    let mut worker_shell = WorkerRuntimeShell::new(RuntimePolicySpec::default()).unwrap();

    let err = worker_shell
        .publish_graph(WorkerPortableGraphPublication {
            policy: RuntimePolicySpec::default(),
            sources: vec![SourceSpec {
                id: "base".to_owned(),
                initial: SignalValue::Number(1.0),
                produces_aspects: None,
            }],
            recipes: vec![RecipeSpec {
                id: "derived".to_owned(),
                reads: vec![RecipeReadSpec::LegacyId("missing".to_owned())],
                expr: read("missing"),
                when: None,
                identity: None,
                produces_aspects: None,
            }],
            output_ids: Vec::new(),
        })
        .unwrap_err();

    assert!(err.message.contains("missing"));
    assert!(worker_shell.read_value("base").is_err());
    assert!(worker_shell.read_value("derived").is_err());
}

#[test]
fn worker_portable_publication_rejects_later_recipe_family_reads_without_partial_sources() {
    let mut worker_shell = WorkerRuntimeShell::new(RuntimePolicySpec::default()).unwrap();

    let err = worker_shell
        .publish_definition_envelope(crate::runtime::adapters::RuntimeDefinitionEnvelope {
            policy: RuntimePolicySpec::default(),
            sources: vec![SourceSpec {
                id: "base".to_owned(),
                initial: SignalValue::Number(1.0),
                produces_aspects: None,
            }],
            recipes: Vec::new(),
            source_families: Vec::new(),
            recipe_families: vec![
                KeyedRecipeFamilySpec {
                    family_id: "derivedFamily".to_owned(),
                    reads: vec![RecipeFamilyReadSpec::Keyed {
                        family_id: "laterFamily".to_owned(),
                        scope: None,
                        aspects: Default::default(),
                    }],
                    expr: read("base"),
                    when: None,
                    identity: None,
                    produces_aspects: None,
                },
                KeyedRecipeFamilySpec {
                    family_id: "laterFamily".to_owned(),
                    reads: Vec::new(),
                    expr: number(2.0),
                    when: None,
                    identity: None,
                    produces_aspects: None,
                },
            ],
            worker_public_output_ids: Vec::new(),
            unavailable_callbacks: Vec::new(),
        })
        .unwrap_err();

    assert!(err.message.contains("laterFamily"));
    assert!(worker_shell.read_value("base").is_err());
}

#[test]
fn worker_portable_publication_rejects_invalid_recipe_aspect_without_partial_sources() {
    let mut worker_shell = WorkerRuntimeShell::new(RuntimePolicySpec::default()).unwrap();

    let err = worker_shell
        .publish_graph(WorkerPortableGraphPublication {
            policy: RuntimePolicySpec::default(),
            sources: vec![SourceSpec {
                id: "base".to_owned(),
                initial: SignalValue::Number(1.0),
                produces_aspects: None,
            }],
            recipes: vec![RecipeSpec {
                id: "derived".to_owned(),
                reads: vec![RecipeReadSpec::Signal(RecipeReadSignalSpec {
                    id: "base".to_owned(),
                    scope: None,
                    aspects: AspectSelectionSpec {
                        aspect: Some(255),
                        aspects: None,
                    },
                })],
                expr: read("base"),
                when: None,
                identity: None,
                produces_aspects: None,
            }],
            output_ids: Vec::new(),
        })
        .unwrap_err();

    assert!(err.message.contains("out of range"));
    assert!(worker_shell.read_value("base").is_err());
    assert!(worker_shell.read_value("derived").is_err());
}
