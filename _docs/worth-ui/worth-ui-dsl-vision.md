# Worth UI DSL Vision

## Purpose

This document defines the intended shape of the Worth UI authoring language.

It is not a grammar file, parser spec, or implementation checklist.

Its job is to lock the architectural direction before the DSL grows enough to
become its own source of folklore.

The central question is:

```text
What kind of language must the Worth UI DSL be so that authored meaning lowers
honestly into canonical declaration artifacts, runtime graph truth, lowered
execution plans, mounted receipts, and bounded rebind?
```

## Thesis

The better idea is:

```text
do not design the DSL around components, modifiers, styles, or widgets
design it around runtime-declarable semantic lanes
```

CSS, Flutter, and SwiftUI each demonstrate a version of the same trap:

```text
the authoring surface becomes powerful by letting local syntax smuggle meaning
that the runtime cannot fully classify
```

Worth UI is explicitly trying to prevent that.

The authored meaning must lower into canonical declaration artifacts. The
runtime graph must be runtime truth. Measurement must be runtime-owned. Mounted
receipts must be host output. Host events must be observations rather than
semantic decisions.

The DSL must reinforce that architecture instead of sneaking around it.

## Core DSL Rule

The DSL is not a component language.

The DSL is not a style language.

The DSL is not a render language.

The DSL is a semantic authoring language for canonical runtime declarations.

The syntax should make the author declare:

```text
structure
identity
participation
layout policy
appearance role
content projection
state/query binding
intent
services
diagnostic posture
```

But each of those must lower into separate semantic lanes and aspect contracts,
not one blob called a component.

## Non-Goals

The DSL must not become:

- a SwiftUI-style modifier chain language
- a CSS-style selector and cascade system
- a Flutter-style view-builder/runtime-tree authority surface
- an AutoLayout or Flexbox clone with ambient parent magic
- a renderer-shaped convenience syntax that outruns runtime admission

If a piece of syntax cannot be lowered into canonical runtime artifacts with
declared identity, aspect contract, graph touch, and bounded invalidation, it
does not belong in the ordinary DSL path yet.

## Semantic Lane Model

The DSL should be organized around semantic lanes rather than component-local
omnibus objects.

The important distinction is:

```text
declaration family says what category of UI thing exists
semantic lane says which slice of meaning is being authored
aspect contract says which exact semantic slices are published, consumed,
invalidated, preserved, or denied
```

The ordinary authoring lanes are:

```text
structure
identity
participation
layout
appearance
content
query-binding
intent
services
diagnostics
motion
operability
```

The author may touch several lanes for one control or region, but those lanes
must remain separable in the lowered artifact.

## Reject Component-Local Meaning Blobs

Avoid this style:

```text
Button("Save")
  .padding(12)
  .background(primary)
  .disabled(!canSave)
  .onClick(save)
  .popover(...)
```

That shape mixes:

```text
structure
layout
appearance
operability
intent
portal service
state dependency
```

into one local chain.

Modifier order becomes meaning.

Hidden environment becomes meaning.

The component becomes a mini-runtime.

Worth UI should prefer authority-lane blocks:

```text
control save_button {
  identity: action("save_step")

  content {
    text: "Save"
  }

  layout {
    width: fill
    height: hug
    min_height: 32
  }

  appearance {
    role: primary_action
    state_axes: [operability, focus, validation]
  }

  operability {
    from: selected_step.update_readiness
  }

  intent {
    submit: update_step
    payload: selected_step.update_payload
  }
}
```

That is less cute but more honest. It gives the compiler and runtime separable
facts instead of one local chain with hidden semantics.

## Reject CSS-Style Selectors and Cascade

CSS's original architectural sin is not colors, padding, or reuse.

It is:

```text
ambient selection plus cascading override authority
```

Do not design Worth UI authoring like this:

```text
.inspector button.primary:hover {
  background: blue;
}
```

That means style can find structure from the outside and mutate appearance
based on selector reach.

It creates spooky action at a distance.

Worth UI should prefer admitted appearance projection over semantic roles and
aspect coverage:

```text
appearance role primary_action {
  applies_to: control where interaction.kind = submit

  covers [
    appearance.background,
    appearance.foreground,
    appearance.radius,
    appearance.opacity
  ]

  states {
    default {
      background: token(action.primary.background)
      foreground: token(action.primary.foreground)
    }

    disabled {
      background: token(action.disabled.background)
      foreground: token(action.disabled.foreground)
      opacity: 0.62
    }

    focused {
      outline: token(focus.ring)
    }
  }
}
```

The key difference is:

```text
CSS selector = external pattern reaches into tree
Worth role = admitted appearance projection over declared semantic aspects
```

No specificity wars.

No cascade.

No "last rule wins" ambiguity.

No random parent selector changing child semantics from the outside.

## Reject View-Builder Authority

Flutter and SwiftUI both make the authored tree feel like the runtime tree.
That is the pit.

Worth UI needs these layers to stay asymmetric:

```text
DSL source tree != canonical declaration artifact
canonical declaration artifact != runtime UI graph
runtime UI graph != lowered execution plan
lowered execution plan != mounted receipt graph
mounted receipt graph != host widget tree
```

Those layers have different authority, identity, lifecycle, invalidation, and
proof obligations.

The DSL may support fragments, templates, and sugar, but they must be lowering
constructs rather than runtime authority objects.

The execution plan is also a lowering product, not an authoring surface. DSL
syntax may declare semantic requirements that affect a plan, but it may not
name runtime handles, choose internal lane strategy, author host contacts, or
make egui-specific mechanics part of canonical UI meaning. Source spans and
provenance remain available for plan inspection; ordinary frame execution does
not reopen DSL source.

Equivalent sugar, fragments, and non-semantic source ordering must converge on
the same executable meaning. A source change that affects capability support,
Query binding, host requirements, lane policy, or another execution-bearing
contract must remain visible to plan equivalence even when its rendered output
happens to look unchanged.

Bad:

```text
component InspectorField(...) {
  local state
  callbacks
  layout
  query read
  popover
}
```

Better:

```text
fragment inspector_field(field) lowers_to control {
  identity: field.logical_identity
  content: field.label
  projection: field.projected_value
  intent: edit_field(field.identity)
}
```

A fragment may expand into canonical declarations. It must not silently own
state, callbacks, portal behavior, layout truth, or runtime identity unless it
declares those lanes explicitly.

## Layout Must Be a Small Algebra

Do not build:

```text
random modifiers
arbitrary constraints everywhere
implicit flex behavior
parent-specific child magic
geometry-reader hacks
```

Build a small algebra of explicit layout operators:

```text
stack
row
grid
split
mosaic
overlay
scroll
flow
portal_anchor
```

Each operator must declare:

```text
child participation
measure-pass requirements
sizing modes allowed
overflow behavior
scroll ownership
invalidation aspects
allocation receipt shape
```

Example:

```text
layout inspector_body {
  operator: stack

  participation {
    children: layout_participating
    hidden_children: excluded
  }

  sizing {
    width: fill
    height: hug
    gap: token(space.3)
  }

  overflow {
    block_axis: scroll_owned
    inline_axis: deny_overflow
  }
}
```

The important constraint is that `stack`, `row`, `grid`, `mosaic`, and friends
are not helper names. They are admitted layout strategies with runtime-owned
measurement and allocation receipts.

## Appearance Is Aspect Coverage, Not Random Properties

Do not start the DSL from arbitrary visual properties.

Start from appearance aspects and coverage.

Every visual rule should answer:

```text
which appearance aspects does this cover?
which state axes does it vary by?
which role owns this projection?
what happens if coverage is missing?
```

So instead of beginning from:

```text
background: red
border-radius: 8
```

think in terms of:

```text
appearance {
  role: danger_action
  covers: [
    appearance.background,
    appearance.foreground,
    appearance.radius
  ]

  state_axes: [operability, focus, validation]
}
```

Eventually the system can still expose radius, border, opacity, and similar
visual outcomes. But those belong inside admitted aspect projections, not as
ambient override properties with arbitrary reach.

## No Ambient Environment Fog

CSS has cascade.

SwiftUI has `Environment`.

Flutter has inherited widgets and ambient context.

All of them can become fog if they are allowed to hide dependency authority.

Worth UI should allow environment-like values only as typed capability inputs:

```text
theme: ThemeCapability
density: DensityCapability
locale: LocaleCapability
motion_policy: MotionCapability
host_text_metrics: HostMeasurementCapability
query_basis: QueryBasisCapability
```

And every usage should become a consumed fact and consumed aspect.

Bad:

```text
use whatever theme/context is nearby
```

Better:

```text
appearance {
  role: primary_action
  consumes: theme.roles.action.primary
}
```

Now hot rebind can know what depended on the theme instead of treating
environment usage as invisible magic.

## Expressions Must Be Pure and Aspect-Tracked

The DSL needs expressions, but not arbitrary execution.

Allowed:

```text
visible when selected_step.type == Approval
disabled when selected_step.update_readiness != ready
options from selected_step.approver_policies
```

Not allowed:

```text
run Rust code during render
mutate state during expression evaluation
call async service from field visibility
close over random app state
```

Every expression should lower to:

```text
consumed projection facts
consumed aspects
result type
invalidating facts
diagnostic source span
```

That is what makes runtime rebind honest.

## Authoring Shape

The DSL should be organized by semantic lanes rather than component-local
modifiers.

Example shape:

```text
page workflow_editor {
  route "/workflows/:workflow_id"

  bindings {
    selected_step: query workflow.selected_step view inspector_detail
  }

  structure {
    mosaic shell {
      left: region step_list fixed(280)
      center: region graph_canvas fill
      right: region inspector fixed(360)
    }
  }

  region inspector {
    bind selected_step

    layout {
      operator: stack
      gap: token(space.3)
      overflow-y: scroll_owned
    }

    control title {
      identity: field("title")
      kind: text_input

      content {
        value: selected_step.title
      }

      layout {
        width: fill
        height: hug
      }

      intent {
        edit: update_field("title")
      }
    }

    control approver_policy {
      identity: field("approver_policy")
      kind: dropdown

      content {
        value: selected_step.approver_policy
        options: selected_step.approver_policies
      }

      service {
        portal: dropdown anchored
        focus: contained
      }

      intent {
        select: update_field("approver_policy")
      }
    }

    when selected_step.type == Approval {
      control escalation_days {
        identity: field("escalation_days")
        kind: number_input

        motion {
          enter: fade_slide
          exit: preserve_then_fade
        }

        intent {
          edit: update_field("escalation_days")
        }
      }
    }

    control save {
      identity: action("save")
      kind: button

      content {
        text: "Save"
      }

      appearance {
        role: primary_action
      }

      operability {
        from: selected_step.update_readiness
      }

      intent {
        submit: update_step
        payload: selected_step.update_payload
      }
    }
  }
}
```

The important point is not this exact syntax.

The important point is that no single construct secretly owns structure,
appearance, operability, intent, services, and runtime semantics as one local
blob.

## Lowering Test

Every DSL construct should be evaluated by this test:

```text
can it lower into:
  source span
  stable identity
  declaration family
  aspect contract
  graph touch descriptor
  index contributions
  consumed facts
  support requirements
  host/lane support requirements
  canonical execution-plan contribution
  diagnostic contract
?
```

If the answer is no, the feature is not ready for the ordinary DSL path.

That should be the architectural gate.

## Sugar Rule

Do not make the DSL nice first.

Make it hard to lie first.

The first syntax can be a little verbose. That is fine.

Once the lowering model is honest, sugar can be added safely.

If the project starts with cute modifier chains, CSS-ish selectors, or
SwiftUI-ish view builders, future work will spend its time dragging hidden
meaning back into the runtime graph.

That is the wrong direction.

## Relationship To The Roadmap

This DSL vision is not a later polish document. It must co-develop with the
roadmap phases that define source lowering, declaration artifacts, aspect
contracts, graph truth, measurement/allocation, Query binding, intent, and
services.

At minimum:

- Milestone 2 must not define canonical source/lowering in a way that conflicts
  with this DSL model
- Milestone 3.2 must carry declaration artifacts and aspect contracts that can
  honestly support this authoring shape
- Milestone 3.3 through 3.12 must close the runtime lanes this DSL relies on
- Milestone 3.9 must lower admitted semantic lanes into host-neutral execution
  plans without exposing handles or executor strategy in the DSL
- later product milestones must consume this runtime-backed authoring model
  instead of smuggling semantics back into widget-local abstractions

The DSL and the runtime are not separate projects.

The DSL is the semantic source boundary for the same runtime architecture.
