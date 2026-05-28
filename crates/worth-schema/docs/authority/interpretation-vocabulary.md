# Interpretation Vocabulary

## What This Feature Is

This feature covers the published topology interpretation vocabulary in
`worth-schema`.

## Why You Use It

Use this when you need stable names for interpreted topology shape, especially
for wires and shells.

## Stable Entry Points

- `WireInterpretationClass`
- `ShellInterpretationClass`
- `WireInterpretationRecord`
- `ShellInterpretationRecord`
- `TopologyInterpretationRecordSet`
- `CertifiedTopologyInterpretation`

## Core Mental Model

These types name interpreted topology structure.

For example:

- a wire may be an open chain, closed cycle, connected branch, or disconnected
- a shell may be an open sheet, closed solid, open non-manifold, or closed
  non-manifold

The record types carry the interpreted facts about one wire or shell. The
record set groups them.

## How It Executes

This page is about the vocabulary, not the public runtime pipeline that
produces it.

## Small Example

```rust
use worth_schema::facade::platform::authority::{
    ShellInterpretationClass, WireInterpretationClass,
};

let wire = WireInterpretationClass::ClosedCycle;
let shell = ShellInterpretationClass::ClosedSolid;
```

## Real Example

```rust
use forge_relational::facade::identity::EntityId;
use worth_schema::facade::platform::authority::{
    TopologyInterpretationRecordSet,
    WireInterpretationClass,
    WireInterpretationRecord,
};

let wire_record = WireInterpretationRecord {
    wire_id: EntityId(1),
    class: WireInterpretationClass::OpenChain,
    connected_component_count: 1,
    terminal_vertex_ids: Vec::new(),
    branch_vertex_ids: Vec::new(),
};

let record_set = TopologyInterpretationRecordSet {
    wires: vec![wire_record],
    shells: Vec::new(),
};
```

## How It Relates To Other Features

- Use [Geometry Binding Vocabulary](./geometry-binding-vocabulary.md) for the
  geometry-side classification names that often sit beside these records.
- Use [Precision Fallbacks](./precision-fallbacks.md) when interpretation also
  needs precision escalation or fallback context.

## Inspection And Debugging

If interpretation output is hard to reason about:

- inspect the class first
- then inspect the topology ids and counts that explain why that class was
  chosen

## Anti-Patterns

- Do not flatten these interpretation classes into generic strings.
- Do not treat the existence of these record types as a promise about which
  runtime surface will produce them.

## Current Limits

- This page documents the vocabulary, not the runtime lane that produces it.

## Related Docs

- [Authority](./README.md)
- [Geometry Binding Vocabulary](./geometry-binding-vocabulary.md)
- [Precision Fallbacks](./precision-fallbacks.md)
