use super::super::support::*;

#[test]
fn aspect_native_mutation_builders_reject_empty_or_duplicate_authoring() {
    let mut workspace = stateful_bridge_task_runtime()
        .workspace("tasks.aspect-errors")
        .expect("task runtime should open a named workspace");

    let empty = workspace
        .insert("Task", |task| task)
        .expect_err("empty aspect mutation should fail closed");
    match empty {
        ForgeQueryRuntimeError::Workspace(error) => {
            assert!(error.to_string().contains("at least one aspect"));
        }
        other => panic!("expected workspace authoring error, got {other:?}"),
    }

    let duplicate = workspace
        .insert("Task", |task| {
            task.aspect("title.value", "Buy milk")
                .aspect("title.value", "Buy oat milk")
        })
        .expect_err("duplicate aspect paths should fail closed");
    match duplicate {
        ForgeQueryRuntimeError::Workspace(error) => {
            assert!(error.to_string().contains("may only be declared once"));
        }
        other => panic!("expected workspace authoring error, got {other:?}"),
    }

    let duplicate_clear = workspace
        .update(test_entity_identity("entity:1:1:1"), |task| {
            task.clear("title.value").aspect("title.value", "Buy milk")
        })
        .expect_err("clear and set of the same aspect should fail closed");
    match duplicate_clear {
        ForgeQueryRuntimeError::Workspace(error) => {
            assert!(error.to_string().contains("may only be declared once"));
        }
        other => panic!("expected workspace authoring error, got {other:?}"),
    }

    let empty_batch = workspace
        .batch(|batch| batch)
        .expect_err("empty mutation batch should fail closed");
    match empty_batch {
        ForgeQueryRuntimeError::Workspace(error) => {
            assert!(error.to_string().contains("at least one operation"));
        }
        other => panic!("expected workspace authoring error, got {other:?}"),
    }
}
