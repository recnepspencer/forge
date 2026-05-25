# Domain Capabilities

`forge-query` domain capabilities let downstream domains contribute semantic
runtime posture through Query-owned public surfaces while Query keeps canonical
artifact authority.

This docs tree is organized by capability area so you can start from the kind
of domain work you are trying to do:

- [Platform Entry](./platform-entry.md)
- [Configured Domain Handles](./configured-domain-handles.md)
- [Canonical Domain Declarations](./canonical-domain-declarations.md)
- [Declaration Family Taxonomy](./declaration-family-taxonomy.md)
- [Declaration Family Capability Matrix](./declaration-family-capability-matrix.md)
- [Declaration Legality](./declaration-legality.md)
- [Declaration Progression](./declaration-progression.md)
- [Declaration Foundational Evidence](./declaration-foundational-evidence.md)
- `admission/`
  - [Advisory And Violation Contributions](./admission/advisory-and-violation-contributions.md)
  - [Declaration Vs Admitted-Plan Targets](./admission/declaration-vs-admitted-plan-targets.md)
- `support/`
  - [Declaration-Scoped Support And Traceability](./support/declaration-scoped-support-and-traceability.md)
  - [Admission-Local Support Reports](./support/admission-local-support-reports.md)
  - [Lower-Runtime Support And Boundary Traceability](./support/lower-runtime-support-and-boundary-traceability.md)
- `invariants/`
  - [Registering Domain Invariants Through Query](./invariants/registering-domain-invariants-through-query.md)
  - [Capability Gaps And Invariant Denials](./invariants/capability-gaps-and-invariant-denials.md)
- `workflow/`
  - [Preview Inspection And Mutation Planning](./workflow/preview-inspection-and-mutation-planning.md)
  - [Runtime-Preflight Workflow Contributions](./workflow/runtime-preflight-workflow-contributions.md)
  - [Workflow Lanes: Common, Checked, Proof, And Raw](./workflow/workflow-lanes-common-checked-proof-raw.md)
- `continuity/`
  - [Continuity Contributions And Authoritative Successors](./continuity/continuity-contributions-and-authoritative-successors.md)
  - [Continuity Vs Correspondence](./continuity/continuity-vs-correspondence.md)
- `aftermath/`
  - [Projection Contract Consumption](./aftermath/projection-contract-consumption.md)
  - [Aftermath Review, Support, Eligibility, And Materialization](./aftermath/aftermath-review-support-eligibility-and-materialization.md)
- `explanation/`
  - [Lower-Runtime Explanation Contributions](./explanation/lower-runtime-explanation-contributions.md)
  - [Cross-Runtime Fallback Vs Store-Backed Replay Gap](./explanation/cross-runtime-fallback-vs-store-backed-replay-gap.md)
- `certification/`
  - [Certification Surface And Closeout Bundle](./certification/certification-surface-and-closeout-bundle.md)
  - [Goldens, Boundaries, And Hostile Certification](./certification/goldens-boundaries-and-hostile-certification.md)

Use these docs when you are building domain-specific behavior on top of the
public Query runtime, especially when your domain needs typed admission,
support, workflow, continuity, projection aftermath, or explanation artifacts
without rebuilding a pseudo-Query layer locally.

Start with [Platform Entry](./platform-entry.md) when you need the typed
facade-first domain front door where the downstream domain supplies its own
marker type rather than relying on separate string-authored contribution
surfaces.

Move next to [Configured Domain Handles](./configured-domain-handles.md) when
you need an admitted operating world, then to
[Canonical Domain Declarations](./canonical-domain-declarations.md) when that
admitted world needs to express declaration-local meaning through one retained
Query-owned declaration artifact. Use
[Declaration Family Taxonomy](./declaration-family-taxonomy.md) when you need
to understand how Query classifies downstream declaration families and carries
that classification forward without owning the family nouns themselves. Use
[Declaration Family Capability Matrix](./declaration-family-capability-matrix.md)
when you need family-scoped support reports, checked family admission, or
structural witness surfaces on canonical declarations. Use
[Declaration Legality](./declaration-legality.md) when you need to review an
already admitted canonical declaration for structural legality inside one
admitted operating world. Use
[Declaration Progression](./declaration-progression.md) when you need to carry
that legality-cleared declaration into a proof-bearing admitted progression or
one typed deferred/denied/stale/rebind/failed outcome. Use
[Declaration Foundational Evidence](./declaration-foundational-evidence.md)
when you need to describe retained legality or progression truth through shared
foundational provenance, support, receipt, and attachment-bundle artifacts.

## Declaration Pipeline Surface Map

The declaration-side public lane is handle-centered and progresses in this
order:

- configured-handle admission:
  - `with_operating_context(...)`
  - `validate()`
  - `admit()`
- family support and declaration authoring:
  - `family_support::<F>()`
  - `family_support_checked::<F>()`
  - `declare(...)`
  - `declare_checked(...)`
  - `declare_with_version(...)`
- legality review:
  - `review_legality(...)`
  - `review_legality_checked(...)`
  - `declare_and_review(...)`
- proof-bearing progression:
  - `declaration_progression_recipe(...)`
  - `progress_declaration(...)`
  - `progress_declaration_checked(...)`
  - `progress_declaration_recipe(...)`
  - `progress_declaration_recipe_checked(...)`
  - `declare_review_and_progress(...)`
- foundational description:
  - `describe_foundational(...)`
  - `describe_foundational_checked(...)`
  - `describe_foundational_with_profile(...)`

The main retained public artifacts introduced along that path are:

- `ForgeQueryCanonicalDeclarationArtifact`
- `ForgeQueryDeclarationLegalityEvidence`
- `ForgeQueryAdmittedDeclarationProgression`
- `ForgeQueryDeclarationFoundationalEvidence`

The main checked and denied families are:

- `ForgeQueryDeclaredFamilyChecked`
- `ForgeQueryDeclarationAdmissionError`
- `ForgeQueryDeclarationLegalityChecked`
- `ForgeQueryDeclarationLegalityDenial`
- `ForgeQueryDeclarationAdmissionOrLegalityError`
- `ForgeQueryDeclarationProgressionChecked`
- `ForgeQueryDeclarationProgressionTerminalError`
- `ForgeQueryDeclarationEntryProgressionError`
- `ForgeQueryDeclarationFoundationalEvidenceChecked`
- `ForgeQueryDeclarationFoundationalEvidenceDenial`

Start here if:

- you need ordinary Query-facing invariants: [Registering Domain Invariants Through Query](./invariants/registering-domain-invariants-through-query.md)
- you need declaration-preview workflow planning: [Preview Inspection And Mutation Planning](./workflow/preview-inspection-and-mutation-planning.md)
- you need successor truth across topology changes: [Continuity Contributions And Authoritative Successors](./continuity/continuity-contributions-and-authoritative-successors.md)
- you need lower-runtime causal explanation: [Lower-Runtime Explanation Contributions](./explanation/lower-runtime-explanation-contributions.md)
- you need projection aftermath contracts: [Projection Contract Consumption](./aftermath/projection-contract-consumption.md)
- you need to audit the proof surface itself: [Certification Surface And Closeout Bundle](./certification/certification-surface-and-closeout-bundle.md)
