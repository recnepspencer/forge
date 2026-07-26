# Application Inspection

## What This Feature Is

Application inspection explains prepared or active Worth UI state without
letting the caller mutate, execute, publish, or reconstruct that state. You
submit a typed query and receive a generation-bound receipt plus explicit
support and relevance posture.

## Why You Use It

- Explain which declarations and graph facts were admitted.
- Inspect a running generation without borrowing its mutable owners.
- Ask for compact evidence first and expand selected references when needed.
- Distinguish supported, diagnostic-only, deferred, unsupported, and
  wrong-world requests.

## Stable Entry Points

- `worth_ui::facade::inspection::UiInspectionQuery`
- `UiInspectionTarget`
- `UiInspectionScope`
- `UiEvidenceRichness`
- `UiEvidenceBudget`
- `WorthUiApp::inspect(...)`
- `WorthUiActiveApplicationSession::inspect(...)`
- `WorthUiApp::inspection_support_report_for(...)`
- `WorthUiApp::expand_evidence_ref(...)`

## Core Mental Model

Inspection is a read-only projection. The prepared app or active session still
owns the real graph, plan, mounting, observation, and publication state.
Receipts carry identities and evidence references so results can be explained,
but those values cannot be promoted back into operational authority.

Active inspection also carries the exact application-generation identity. Use
it to reject or label stale results after replacement.

## How It Executes

```text
typed target + scope + relevance + richness + budget
-> support and relevance admission
-> indexed read-only projection
-> compact receipt
-> optional bounded evidence expansion
```

A scope can be supported, diagnostic-only, deferred, unsupported, or valid only
in another world. Check support posture rather than treating an empty result as
success.

## Small Example

```rust
use worth_ui::facade::app::WorthUi;
use worth_ui::facade::inspection::{
    UiInspectionQuery, UiInspectionScope, UiInspectionTarget,
};

let app = WorthUi::app().freeze()?;
let query = UiInspectionQuery::new(
    UiInspectionTarget::product_root(),
    UiInspectionScope::graph(),
);
let receipt = app.inspect(query);
```

This asks the prepared application for a graph-scoped summary. It does not
launch or mutate the application.

## Real Example

```rust
let query = UiInspectionQuery::new(target, scope)
    .with_richness(richness)
    .with_budget(budget)
    .with_relevance(relevance);

let support = app.inspection_support_report_for(&query);
let receipt = app.inspect(query);
if let Some(reference) = selected_evidence_ref(&receipt) {
    let detail = app.expand_evidence_ref(reference, requested_richness);
    present_detail(detail);
}
present_support(support);
```

The support report tells the UI whether the request is available, deferred, or
unsupported. Evidence expansion is bounded and still read-only.

## How It Relates To Other Features

- Inspect `WorthUiApp` before launch for prepared truth.
- Inspect `WorthUiActiveApplicationSession` for generation-bound active truth.
- Query-specific inspection can cite the exact Query attempt or settled
  projection without copying it into UI-owned state.

## Inspection And Debugging

Compare the active inspection receipt’s generation identity with the current
session generation before displaying long-lived results. Surface explicit
relevance and support outcomes; do not collapse them into a generic “no data.”

## Anti-Patterns

- Importing runtime storage, planning, mounting, or publication modules.
- Treating evidence identities or digests as constructors.
- Using inspection output to drive operational mutation.
- Requesting rich evidence globally when a compact reference is enough.

## Current Limits

Not every future scope is admitted. Deferred and diagnostic-only rows are
intentional public truth, not incomplete success responses.

## Related Docs

- [Application lifecycle](./application-lifecycle.md)
- [Runtime subsystems](./runtime-subsystems.md)
- [Query-backed UI views](./query-binding.md)
