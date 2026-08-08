# WORTH Query Workspace

This workspace owns the Query engine, its audience facades, and its explicit
cold certification package.

For architecture and usage, start with:

- [`docs/AI_README.md`](./crates/worth-query/docs/AI_README.md) for the authority
  map and current public conventions
- [Runtime-Installed Domains And Operations](./crates/worth-query/docs/domain-capabilities/runtime-installed-domains.md)
- [Conditional Installed Operations](./crates/worth-query/docs/domain-capabilities/conditional-installed-operations.md)
- [Installed Operation Re-Execution And Replay](./crates/worth-query/docs/domain-capabilities/installed-operation-reexecution-and-replay.md)
- [Typed Stops And Remediation Guidance](./crates/worth-query/docs/domain-capabilities/typed-stops-and-remediation-guidance.md)
- [Installed Operation Lineage And Promotion](./crates/worth-query/docs/domain-capabilities/installed-operation-lineage-and-promotion.md)

Use the smallest package that owns the change. Declaration work does not build
installation, the remaining engine, replay, or certification:

```text
cargo check --manifest-path workspaces/worth-query/Cargo.toml -p worth-query-declaration
cargo test --manifest-path workspaces/worth-query/Cargo.toml -p worth-query-declaration
```

Installation work does not build the remaining engine, replay, or
certification:

```text
cargo check --manifest-path workspaces/worth-query/Cargo.toml -p worth-query-installation
cargo test --manifest-path workspaces/worth-query/Cargo.toml -p worth-query-installation
```

For behavior still owned by the remaining engine, choose either a check or a
test command. Do not pre-run check and `--no-run` before the test:

```text
cargo check --manifest-path workspaces/worth-query/Cargo.toml -p worth-query --tests
cargo test --manifest-path workspaces/worth-query/Cargo.toml -p worth-query
```

Compiler and reconstruction certification are cold and must be selected only
when their boundary changes or at closeout:

```text
cargo test --manifest-path workspaces/worth-query/Cargo.toml -p worth-query-certification --test compile_certification
cargo test --manifest-path workspaces/worth-query/Cargo.toml -p worth-query-certification -p worth-query-replay
```

`make query-declaration-test`, `make query-installation-test`, `make
query-test`, and `make query-compiler-certification` are short forms of these
same Cargo commands. They add no runner, cache, or selection protocol.

The repository root remains an orchestrator and may consume Query through path
dependencies. It does not own Query package membership.
