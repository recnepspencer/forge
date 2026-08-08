# Gate 8.7 — Turn 2 Report

## Boundary reviewed (corrective)

F1: `redispatch_admitted_external_effect` previously took a caller-supplied
outbox and no recovery handle/authority, so R8.69 was caller discipline and
R8.30's linear lifecycle was bypassable. Fix reads outbox only from a live
handle binding and requires `WorthQueryRecoveryEffectAuthority` before
transport.

F2: Exactly-once was proved only for DisappearMidDispatch and Succeed-then-
retry — not under `CommitThenLoseResponse` indeterminacy.

F3: Arity compile-fail for `safe_retry` did not bind private construction of
`WorthQueryAdmittedExternalRedispatch`.

F4: Foreign-principal denial assertion accepted four causes.

## Slice built

Corrective closure of F1–F4 plus drop of ceremonial `_private` on the
redispatch proof type. Corrective edits were already present in the working
tree at turn start; this turn confirmed they match the brief and re-ran the
full standing set.

## Material files

| File | Role |
|---|---|
| `.../external_dispatch.rs` | `redispatch` takes handle+authority; `require_fresh_effect_authority` first; outbox from binding |
| `.../redispatch.rs` | private fields only; `pub(crate) mint`; no `_private` |
| `.../safe_retry.rs` | outbox equality retained — blocks cross-handle proof swap |
| `bank-server/.../recovery.rs` | no caller outbox clone |
| `phase8_safe_retry.rs` | `CommitThenLoseResponse` exactly-once; F4 pinned to `CapabilityGrantMissing` |
| `admitted_external_redispatch_constructor_is_private.{rs,stderr}` | R8.66 privacy |
| `redispatch_requires_handle_and_authority.{rs,stderr}` | R8.69 structural |

## Verification (by target)

| Target | Result |
|---|---|
| `cargo test -p bank-server --test ordinary_mutations` | **ok** — **88 passed** (was 87; **+1** = `lost_response_after_commit_safe_retry_emits_nothing_twice`) |
| `cargo test -p worth-query-certification --test application_aftermath_compile_fail` | **ok** (incl. constructor privacy + redispatch handle/authority) |
| `cargo test -p worth-query-certification --test compile_certification` | **ok** (14) |
| `cargo test -p worth-query-execution --lib` ×5 | **ok** each — 579 / 579 / 579 / 579 / 579 |
| `boundary-check --root .` | **ok** |
| `agent-context check` | **ok** |
| `check_workspace_rust_line_caps.sh dirty` | **PASS** |
| `RUSTFLAGS=-Dwarnings cargo check` (touched crates, both workspaces) | **ok** |
| `installed_operating_world` | **ok** (313) |
| `public_declarative_journeys` | **ok** (37) |
| `runtime_public_journeys` | **ok** (22) |
| `cargo test --workspace --no-fail-fast` (worth-query) | **ok** |
| `cargo test --workspace --no-fail-fast` (bank-world) | **ok** |

Pre-existing warning (untouched): unused `undo` field in `phase8_redo_support.rs`.

## Remaining

- **Q8.14** still deferred (typed `StoreCapabilityRequired`, not built).
- Ledger rows not marked in `_docs/` (brief forbids `_docs/` edits).
- No `#[allow(...)]` introduced.
