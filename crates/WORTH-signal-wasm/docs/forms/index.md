# Forms Overview

Forms are the product surface for runtime-owned draft state, semantic dirty
truth, patch planning, validation, action planning, and adjacent form
artifacts.

Use `signals.form(...)` when you want the runtime to own:

- source vs draft vs effective value layering
- field handles and repeated-item identity
- semantic dirty checks and patch plans
- validation artifacts and visible messages
- readiness and later action planning

Start in this order:

1. [Your First Form](./getting-started/your-first-form.md)
2. [Choosing A Form Source](./getting-started/choosing-a-form-source.md)
3. [Source Truth, Draft, And Effective Values](./state/source-truth-draft-and-effective-values.md)
4. [Dirty State](./changes/dirty-state.md)
5. [Validation Overview](./validation/validation-overview.md)
6. [Field And Control Availability](./availability/field-and-control-availability.md)
7. [Action Overview](./actions/action-overview.md)
8. [Layout Overview](./layout/layout-overview.md)
9. [Input Adapter Overview](./inputs/input-adapter-overview.md)
10. [Focus, Touch, And Visited State](./interaction/focus-touch-and-visited-state.md)
11. [Entry Bootstrap](./lifecycle/entry-bootstrap.md)
12. [Attachments](./media/attachments.md)
13. [Resource Line Source](./resource-backed/resource-line-source.md)
14. [Collaboration Overview](./collaboration/collaboration-overview.md)
15. [Route Authority Handoff](./route-coupling/route-authority-handoff.md)
16. [Diagnostics Summary](./diagnostics/diagnostics-summary.md)
17. [Verification Packages](./verification/verification-packages.md)

Fast lookup:

- "I need one normal form."  
  [Your First Form](./getting-started/your-first-form.md)
- "I need to know where form truth lives."  
  [Source Truth, Draft, And Effective Values](./state/source-truth-draft-and-effective-values.md)
- "I need to patch a complex edit form."  
  [Patching Complex Edit Forms](./changes/patching-complex-edit-forms.md)
- "I need to disable submit if unchanged."  
  [Unchanged Forms And Submit Readiness](./changes/unchanged-forms-and-submit-readiness.md)
- "I need async validation or draft migration."  
  [Async Validation](./validation/async-validation.md) and
  [Source Compatibility And Draft Migration](./validation/source-compatibility-and-draft-migration.md)
- "I need to disable a field, gate submit, or require approval."  
  [Availability And Permissions](./availability/README.md)
- "I need multi-step behavior or submit execution."  
  [Steps And Multi-Step Forms](./steps/README.md) and
  [Actions And Submit](./actions/README.md)
- "I need label size, layout hints, or accessibility reads."  
  [Layout And Accessibility](./layout/README.md)
- "I need to wire a dropdown, search box, or external control honestly."  
  [Inputs And Controls](./inputs/README.md)
- "I need focus state, offline blockers, or host facts."  
  [Interaction And Host Facts](./interaction/README.md)
- "I need entry readiness, exit state, or handoff visibility."  
  [Lifecycle](./lifecycle/README.md)
- "I need attachment/media state or transfer readback."  
  [Attachments And Media](./media/README.md)
- "I need resource-backed draft truth and execution."  
  [Resource-Backed Forms](./resource-backed/README.md)
- "I need shared editing, locks, or reviewer posture."  
  [Collaboration](./collaboration/README.md)
- "I need route-coupled draft handoff or freeze/discard/defer behavior."  
  [Route-Coupled Forms](./route-coupling/README.md)
- "I need to inspect what changed or prove the public surfaces agree."  
  [Diagnostics And History](./diagnostics/README.md) and
  [Verification](./verification/README.md)

Related package entrypoints:

- [start_here.md](../start_here.md)
- [Feature Index](../learn/feature-index.md)
