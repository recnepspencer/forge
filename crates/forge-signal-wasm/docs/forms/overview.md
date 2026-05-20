# Forms Overview

This page is the feature router for the shipped `signals.form(...)` surface.

Use it when your question is "which forms feature owns this problem?" Start
here, then jump to the canonical feature page.

## Default Path

For ordinary form work, the normal lane is:

1. declare a form with `signals.form(...)`
2. choose the source authority with `signals.form.source.*(...)` when needed
3. declare fields with `field(...)`, `repeated(...)`, or `evidence(...)`
4. add validation, availability, admission, steps, and actions where the form
   needs them
5. read runtime truth from the controller: `effective()`, `dirty()`,
   `readiness()`, `validation()`, `actions()`, and `diagnosticsSummary()`

## Feature Map

- form identity, field loci, repeated items, attachment evidence fields, input
  adapters, source authority, draft truth, and effective projection:
  [Form Kernel And Fields](./form-kernel-and-fields.md)
- semantic dirty truth, patch planning, empty-patch posture, raw input slots,
  and submit readiness:
  [Dirty, Patch, And Readiness](./dirty-patch-and-readiness.md)
- validation artifacts, parse failures, visible messages, and readiness
  blockers:
  [Validation And Messages](./validation-and-messages.md)
- dynamic availability, admission gates, and controller-local steps:
  [Availability, Admission, And Steps](./availability-admission-and-steps.md)
- action planning, submit planning, recovery actions, and execution posture:
  [Actions And Submit](./actions-and-submit.md)
- async validation, action lifecycle, server rejection mapping, and
  canonicalization:
  [Async Lifecycle And Canonicalization](./async-lifecycle-and-canonicalization.md)
- host facts, interaction facts, accessibility artifacts, layout hints, and
  measurement:
  [Host, Interaction, Accessibility, And Layout](./host-interaction-accessibility-and-layout.md)
- attachment/media/handoff/exit presentation lanes and visible settlement
  posture:
  [Presentation And External Lanes](./presentation-and-external-lanes.md)
- resource-line forms, merge/drift posture, resource effect lowering, reset,
  replay, and restore:
  [Resource-Line Forms](./resource-line-forms.md)
- multi-actor locks, leases, branch-per-actor posture, comments, and advisory
  presence:
  [Collaboration](./collaboration.md)
- diagnostics summary, full diagnostics, retained histories, and verification
  packages:
  [Diagnostics, History, And Verification](./diagnostics-history-and-verification.md)

## Fast Decisions

- "I need one form with ordinary local state."
  [Form Kernel And Fields](./form-kernel-and-fields.md)
- "I need to know whether this form is actually changed."
  [Dirty, Patch, And Readiness](./dirty-patch-and-readiness.md)
- "I need validators, messages, or submit blockers."
  [Validation And Messages](./validation-and-messages.md)
- "I need role, lock, approval, or dynamic enablement rules."
  [Availability, Admission, And Steps](./availability-admission-and-steps.md)
- "I need save, submit, approve, route, or custom button actions."
  [Actions And Submit](./actions-and-submit.md)
- "I need async validation or server canonicalization."
  [Async Lifecycle And Canonicalization](./async-lifecycle-and-canonicalization.md)
- "I need resource-backed submit, merge, drift, rollback, replay, or restore."
  [Resource-Line Forms](./resource-line-forms.md)
- "I need to explain why the form is blocked or reconstruct what happened."
  [Diagnostics, History, And Verification](./diagnostics-history-and-verification.md)

## Task-First Companion

- top-level product entrypoint:
  [start_here.md](../start_here.md)
- all forms feature pages in one index:
  [Feature Index](../learn/feature-index.md)

## Lower-Level References

Once you already know the feature, use the public type surface for exact
boundary detail:

- [App Surface Overview](../app-surface/overview.md)
- [Diagnostics And History](../app-surface/diagnostics-and-history.md)
- [Host Capabilities](../app-surface/host-capabilities.md)
- [Resource Line Reference](../api-reference/resource-line.md)
