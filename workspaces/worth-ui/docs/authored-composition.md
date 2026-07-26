# Authored Composition

## What This Feature Is

Authored composition lets you define the same Worth UI application from `.wui`
files or typed Rust values. Both routes are compiled by `worth-ui-dsl` into one
sealed semantic package before the runtime prepares an application.

## Why You Use It

- Keep product composition hot-reloadable in files.
- Generate composition in Rust without serializing it through text or JSON.
- Preserve one set of identities, diagnostics, and runtime behavior across
  both authoring styles.

## Stable Entry Points

- `worth_ui_dsl::WorthUiDslCompiler`
- `worth_ui_dsl::WorthUiRustAuthoredArtifactInput`
- `worth_ui_dsl::WorthUiRustAuthoredArtifactInputModule`
- `worth_ui::facade::source::WorthUiFilesystemSourceProvider`
- `worth_ui::facade::source::WorthUiFilesystemSourceWatcher`
- `WorthUiApplicationBuilder::with_rust_authored_input(...)`
- `WorthUiApplicationBuilder::with_candidate_submission(...)`

The source facade owns transport and settlement. Authored nodes, spans, compile
reports, and semantic packages come from `worth-ui-dsl`.

## Core Mental Model

Files and Rust are two ways to author meaning, not two runtime formats.
Compilation freezes imports, declarations, ordering, provenance, and semantic
identity into one package. Runtime ingress attaches source revision and
settlement evidence, then hands the complete candidate to application
preparation.

Malformed syntax stops in the DSL. A syntactically valid package may still be
denied later when the active capability snapshot cannot admit its runtime
requirements. Neither denial changes the current application.

## How It Executes

```text
filesystem snapshot -> DSL compiler ----\
                                         -> sealed semantic package
typed Rust input ----> DSL compiler -----/
-> candidate submission
-> WorthUi::app().with_candidate_submission(...)
-> freeze()
```

After `freeze`, frames use the active semantic package and runtime indexes.
They do not reread or reparse the authoring source.

## Small Example

```rust
use worth_ui::facade::app::WorthUi;
use worth_ui_dsl::WorthUiRustAuthoredArtifactInput;

let app = WorthUi::app()
    .with_rust_authored_input(WorthUiRustAuthoredArtifactInput::default())
    .freeze()?;
```

This is the smallest honest Rust-authored application. Real applications add
typed modules and body atoms before passing the input to the builder.

## Real Example

```rust
use worth_ui::facade::app::WorthUi;
use worth_ui::facade::source::WorthUiFilesystemSourceProvider;

let snapshot = WorthUiFilesystemSourceProvider::new(workspace_root).read()?;
let capability_app = WorthUi::app().freeze()?;
let submission =
    snapshot.lower_to_candidate_submission(capability_app.capabilities())?;
let app = WorthUi::app()
    .with_candidate_submission(submission)
    .freeze()?;
let session = app.launch()?;
```

`lower_to_candidate_submission` crosses the DSL compilation boundary and
returns the complete watched candidate submission. The capability-only
preparation supplies the exact admission snapshot for lowering; it does not
launch or become a second active application. Application code must not split
out declarations, forge a semantic package, or prepare runtime state from
loose source parts. Production certification proves equivalent file and Rust
definitions reach equivalent prepared generations.

## How It Relates To Other Features

- Register capabilities and Query views on the same application builder before
  `freeze`.
- Use the filesystem watcher for replacement candidates, then pass the settled
  submission to the session-owned replacement path.
- Use [Application inspection](./inspection.md) for provenance and admitted
  meaning after preparation.

## Inspection And Debugging

DSL failures return `WorthUiDslCompileReport` with typed diagnostic identities,
spans, and stop classes. Runtime preparation failures are separate typed
denials. Keep that distinction when presenting errors to users.

## Anti-Patterns

- Parsing `.wui` files inside frame or host-adapter code.
- Converting typed Rust authoring through JSON or source text.
- Extracting declarations from a candidate and rebuilding a runtime input.
- Treating a digest as permission to skip semantic comparison or lowering.

## Current Limits

`WorthUiDslSupportPosture` remains conservative. A type name in the DSL crate
does not promise that every future language family is admitted today. Follow
the current compiler diagnostics and capability support rows.

## Related Docs

- [Worth UI architecture](./architecture.md)
- [Application lifecycle](./application-lifecycle.md)
- [Milestone 3.10.1 migration](./migration-3.10.1.md)
