# Grouped Authoring

## What This Feature Is

Grouped authoring is the Query-owned surface for turning a meaningful
multi-member declaration into one typed grouped artifact and one typed grouped
orchestration lane.

Use it when the operation you mean is not "run these declarations one by one,"
but "treat these declarations as one neighborhood-shaped unit with shared
intent and shared group posture."

The current shipped slice is intentionally narrow:

- one grouped declaration input type
- one grouped declaration artifact
- one grouped orchestration lane
- one grouped recovery lane
- one geometry helper front door for local-neighborhood authoring from active
  face selection

## Why You Use It

- keep grouped intent explicit instead of passing a raw `Vec<I>`
- preserve both group-level and member-level truth
- bind grouped work to the admitted handle and operating world that admitted it
- keep grouped wrong-world and wrong-handle posture typed instead of flattening
  it into generic failure
- let grouped helper calls lower onto the same canonical grouped boundary every
  time

## Stable Entry Points

Core grouped types:

- `ForgeQueryGroupedDeclarationInput<D, I>`
- `ForgeQueryGroupedDeclarationArtifact<D, I>`
- `ForgeQueryGroupedDeclarationMember<D, I>`
- `ForgeQueryGroupedDeclarationChecked<D, I>`
- `ForgeQueryGroupedDeclarationStop`
- `ForgeQueryGroupedDeclarationStopKind`
- `ForgeQueryGroupedSemantics`
- `ForgeQueryGroupedOrdering`
- `ForgeQueryGroupedOrchestration<D, I>`
- `ForgeQueryGroupedEnvelopeMember<D, I>`
- `ForgeQueryGroupedMemberOrchestrationStop<D, I>`
- `ForgeQueryGroupedOrchestrationAlignmentStop<D, I>`
- `ForgeQueryGroupedOrchestrationStop<D, I>`
- `ForgeQueryGroupedOrchestrationChecked<D, I>`
- `ForgeQueryGroupedOrchestrationProof<D, I>`
- `ForgeQueryGroupedOrchestrationTranscript<D, I>`

Geometry helper entry points:

- `local_neighborhood_for_active_face_selection(...)`
- `declare_local_neighborhood_for_active_face_selection(...)`
- `declare_local_neighborhood_for_active_face_selection_checked(...)`
- `orchestrate_local_neighborhood_for_active_face_selection(...)`
- `orchestrate_local_neighborhood_for_active_face_selection_outcome(...)`
- `orchestrate_local_neighborhood_for_active_face_selection_checked(...)`
- `orchestrate_local_neighborhood_for_active_face_selection_proof(...)`

Grouped recovery entry points:

- `recover_from_grouped_orchestration_checked(...)`
- `recover_from_grouped_orchestration_proof(...)`

Grouped input builders:

- `ForgeQueryGroupedDeclarationInput::local_neighborhood(...)`
- `with_member(...)`
- `with_members(...)`
- `with_shared_rationale(...)`
- `with_ordering(...)`

## Core Mental Model

Think of grouped authoring as "one typed group first, then one grouped run."

The important boundary is:

- single-declaration helpers still express one declaration
- grouped authoring expresses one retained group with member declarations inside

That means grouped authoring is not a loop helper.

You first build one `ForgeQueryGroupedDeclarationInput`. Query then admits each
member through the normal declaration path and freezes the successful result as
one `ForgeQueryGroupedDeclarationArtifact`.

That grouped artifact retains:

- the handle identity that admitted the group
- the operating-context identity that admitted the group
- the shared grouped posture
- the grouped semantics and ordering
- the optional shared rationale
- the retained member declarations

Grouped orchestration then lowers those retained member declarations through
the existing declaration-entry envelope lane while keeping grouped alignment
truth visible.

If the group was admitted on the wrong world or wrong handle, the grouped lane
stops before member lowering. If a member stops later, the grouped lane keeps
that stop positioned on the member rather than flattening it into vague partial
success.

## How It Executes

The current local-neighborhood lifecycle is:

1. start from an admitted configured handle
2. choose the geometry helper facade with `geometry_helpers()`
3. build one grouped input with
   `local_neighborhood_for_active_face_selection(...)`
4. optionally add more members, a shared rationale, or explicit ordering
5. declare the grouped artifact
6. orchestrate the grouped artifact through the grouped orchestration lane
7. inspect:
   - grouped alignment posture
   - grouped member envelopes
   - grouped ordinary or recovery posture

Grouped declaration admission can stop before orchestration with one
`ForgeQueryGroupedDeclarationStopKind`:

- `Deferred`
- `Unsupported`
- `InvalidContext`
- `Canonicalization`

Grouped orchestration can then:

- bind successfully and return grouped member envelopes
- stop at `WrongWorld`
- stop at `WrongHandle`
- stop at one member-level declaration-entry orchestration outcome

## Small Example

```rust
let declaration = handle
    .geometry_helpers()
    .declare_local_neighborhood_for_active_face_selection(
        handle
            .geometry_helpers()
            .local_neighborhood_for_active_face_selection(
                geometry_session.select_active_face("face-a")?,
            )
            .with_member(geometry_session.select_active_face("face-b")?)
            .with_shared_rationale("split the local neighborhood"),
    )?;

let outcome = handle
    .geometry_helpers()
    .orchestrate_local_neighborhood_for_active_face_selection_outcome(declaration);

match outcome {
    ForgeQueryOrdinaryOutcome::Bound(grouped) => {
        let _ = grouped.group_digest();
        let _ = grouped.member_envelopes();
    }
    other => {
        let _ = handle.recover_from_outcome(&other);
    }
}
```

Use this when you want the compact public lane and a family-native grouped
entry point.

## Real Example

```rust
let declaration = handle
    .geometry_helpers()
    .declare_local_neighborhood_for_active_face_selection(
        handle
            .geometry_helpers()
            .local_neighborhood_for_active_face_selection(
                geometry_session.select_active_face("seed-face")?,
            )
            .with_member(geometry_session.select_active_face("neighbor-a")?)
            .with_member(geometry_session.select_active_face("neighbor-b")?)
            .with_shared_rationale("preserve a stable local cut around the seed face"),
    )?;

let proof = handle
    .geometry_helpers()
    .orchestrate_local_neighborhood_for_active_face_selection_proof(declaration);

match proof.outcome() {
    ForgeQueryGroupedOrchestrationChecked::Bound(grouped) => {
        let _ = grouped.declaration().grouped_posture();
        let _ = grouped.declaration().shared_rationale();
        let _ = grouped.member_envelopes()[0].envelope().declaration_digest();
    }
    _ => {
        let brief = handle
            .recover_from_grouped_orchestration_proof(proof)
            .expect("grouped non-success should yield grouped recovery");
        let _ = brief.stop_family();
        let _ = brief.recommended_action();
    }
}
```

What this example is showing:

- the group is declared once and retained as one artifact
- grouped proof can still expose the checked grouped stop lane
- grouped recovery stays on the shared recovery boundary
- member-level truth remains inspectable after grouped success

## How It Relates To Other Features

- [Family Helpers](./family-helpers.md) own the ergonomic geometry front door
  for grouped local-neighborhood authoring.
- [Configured Domain Handles](./configured-domain-handles.md) own the admitted
  handle that grouped declaration and grouped orchestration remain bound to.
- [Ordinary Outcomes](./ordinary-outcomes.md) own the compact grouped outcome
  lane when you choose `..._outcome(...)`.
- [Recovery Boundary](./recovery-boundary.md) owns grouped repair guidance
  through `recover_from_grouped_orchestration_checked(...)` and
  `recover_from_grouped_orchestration_proof(...)`.
- [Canonical Domain Declarations](./canonical-domain-declarations.md) own the
  member declaration artifacts carried inside the grouped declaration artifact.
- [Declaration Family Taxonomy](./declaration-family-taxonomy.md) owns the
  grouped posture classification that grouped authoring consumes.

Use grouped authoring when the group itself is part of the meaning. Use single
family helpers when each declaration stands alone.

## Inspection And Debugging

Useful grouped declaration accessors:

- `group_digest()`
- `handle_identity_digest()`
- `operating_context_identity_digest()`
- `declaration_family_key()`
- `grouped_posture()`
- `semantics()`
- `ordering()`
- `shared_rationale()`
- `members()`

Useful grouped orchestration accessors:

- `orchestration_digest()`
- `declaration()`
- `member_envelopes()`
- `member_index()`
- `envelope()`
- `member_outcome()`
- `reason()`

Useful grouped recovery accessors:

- `recover_from_grouped_orchestration_checked(...)`
- `recover_from_grouped_orchestration_proof(...)`
- `stop_family()`
- `stop_kind()`
- `authority_surface()`
- `recommended_action()`

## Anti-Patterns

- using grouped authoring as a prettier `Vec<I>` batch helper
- assuming grouped success means every member can no longer be inspected
- rebuilding grouped meaning from helper names instead of one grouped artifact
- treating grouped wrong-world or wrong-handle posture as ordinary member
  failure
- expecting grouped authoring to imply grouped contribution composition or
  grouped continuation execution
- teaching grouped recovery as if it were separate from the shared recovery
  boundary

## Current Limits

- the current shipped grouped semantics are `LocalNeighborhood` only
- the current grouped front door is geometry-only
- grouped declaration admission and grouped orchestration are shipped, but
  grouped contribution composition is not
- grouped route-plan, receipt, envelope, relational-routing, bridge-routing,
  and signal-compatibility families are not shipped as standalone grouped
  public surfaces
- grouped recovery currently focuses on grouped orchestration stops, not a
  broader grouped support matrix

## Related Docs

- [Family Helpers](./family-helpers.md)
- [Configured Domain Handles](./configured-domain-handles.md)
- [Ordinary Outcomes](./ordinary-outcomes.md)
- [Recovery Boundary](./recovery-boundary.md)
- [Canonical Domain Declarations](./canonical-domain-declarations.md)
- [Declaration Family Taxonomy](./declaration-family-taxonomy.md)
- [Domain Capabilities](./README.md)
