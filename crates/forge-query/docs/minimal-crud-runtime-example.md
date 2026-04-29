# Minimal CRUD Runtime Example

## What This Is

This is a short end-to-end example of the smallest useful thing Forge Query
lets you do with the stabilized runtime surface:

- declare CRUD-shaped runtime surfaces for `Car` and `Person`
- expose rows that are ready to hand to a UI
- update one piece of data through the runtime
- observe that only the touched live surfaces and dependent computed surfaces
  wake up

The goal is not to show every feature. The goal is to show the bare minimum
that replaces a lot of ordinary app wiring.

## Why You Use This Shape

In a typical app, you often hand-build all of this separately:

- database reads
- backend endpoint code
- response DTOs
- cache keys and invalidation
- UI selectors

With Forge Query, ordinary product code starts from one runtime workspace and
declares the surfaces it needs once. You still have authoritative truth below
the runtime, but you stop hand-wiring the read model, patch routing, and
derived UI state yourself.

## Stable Entry Points

- `runtime.workspace(...)`
- `workspace.live_view(...)`
- `workspace.computed(...)`
- `workspace.insert(...)`
- `workspace.update(...)`
- `workspace.write(...)` as the lower-level compatibility path
- `workspace.read(...)`
- `workspace.observe(...)`
- `workspace.materialize(...)`

Useful evidence surfaces:

- `ForgeQueryWriteReceipt::affected_live_view_ids()`
- `ForgeQueryWriteReceipt::affected_derived_view_ids()`

## Core Mental Model

Think of the runtime in three layers:

- authoritative truth: `Car` and `Person`
- live views: the runtime-owned read/cache/subscription surfaces over that
  truth
- computed surfaces: UI-ready derived state built from those live views

That means you do not separately build:

- an API response cache for `people.table`
- a selector cache for `garage.people_rows`
- custom invalidation code for "person changed but car table did not"

The runtime already knows which surfaces depend on which aspects.

Informally, updates are O(touched surfaces), not O(all declared surfaces).
Unrelated live views and computeds stay asleep.

## How It Executes

1. Open a workspace.
2. Declare one live view for cars and one for people.
3. Declare one computed surface that turns those runtime surfaces into UI rows.
4. Insert or update authoritative truth through `workspace.insert(...)` or
   `workspace.update(...)`.
5. Read live rows with `read(...)`.
6. Materialize UI rows with `materialize(...)`.
7. Drain patch batches with `observe(...)`.
8. Use the write receipt to see which surfaces were actually touched.

## Small Example

```rust
use forge_query::facade::ForgeQueryLiveView;
use serde_json::Value;

let mut workspace = runtime.workspace("garage").unwrap();

let cars: ForgeQueryLiveView<Value> = workspace
    .live_view("garage.cars", |q| {
        q.from("Car")
            .select(["identity.id", "make.value", "model.value"])
            .order_by("make.value")
            .schema_basis("garage-cars")
    })
    .unwrap();

let people: ForgeQueryLiveView<Value> = workspace
    .live_view("garage.people", |q| {
        q.from("Person")
            .select(["identity.id", "name.value", "car_id.value"])
            .order_by("name.value")
            .schema_basis("garage-people")
    })
    .unwrap();

workspace
    .insert("Car", |car| {
        car.aspect("identity.id", "car-1")
            .aspect("make.value", "Honda")
            .aspect("model.value", "Civic")
    })
    .unwrap();

workspace
    .insert("Person", |person| {
        person
            .aspect("identity.id", "person-1")
            .aspect("name.value", "Ava")
            .aspect("car_id.value", "car-1")
    })
    .unwrap();

let car_rows = workspace.read(&cars);
let person_rows = workspace.read(&people);
```

This is already doing more than it looks like:

- the runtime owns the live read surface
- writes route through one authority path
- the same declared surfaces are ready for reads, observation, and UI-facing
  derivation

## Real Example

```rust
use forge_query::facade::{
    ForgeQueryDerivedViewHandle, ForgeQueryLiveView, ForgeQueryWorkspace,
};
use serde_json::Value;

struct CreateCarInput<'a> {
    id: &'a str,
    make: &'a str,
    model: &'a str,
}

struct CreatePersonInput<'a> {
    id: &'a str,
    name: &'a str,
    car_id: &'a str,
}

fn create_car(
    workspace: &mut ForgeQueryWorkspace,
    input: CreateCarInput<'_>,
) {
    workspace
        .insert("Car", |car| {
            car.aspect("identity.id", input.id)
                .aspect("make.value", input.make)
                .aspect("model.value", input.model)
        })
        .unwrap();
}

fn create_person(
    workspace: &mut ForgeQueryWorkspace,
    input: CreatePersonInput<'_>,
) {
    workspace
        .insert("Person", |person| {
            person
                .aspect("identity.id", input.id)
                .aspect("name.value", input.name)
                .aspect("car_id.value", input.car_id)
        })
        .unwrap();
}

let mut workspace = runtime.workspace("garage").unwrap();

let cars: ForgeQueryLiveView<Value> = workspace
    .live_view("garage.cars", |q| {
        q.from("Car")
            .select(["identity.id", "make.value", "model.value"])
            .order_by("make.value")
            .schema_basis("garage-cars")
    })
    .unwrap();

let people: ForgeQueryLiveView<Value> = workspace
    .live_view("garage.people", |q| {
        q.from("Person")
            .select(["identity.id", "name.value", "car_id.value"])
            .order_by("name.value")
            .schema_basis("garage-people")
    })
    .unwrap();

let people_rows: ForgeQueryDerivedViewHandle<Value> = workspace
    .computed(
        "garage.people_rows",
        |c| {
            c.depends_on_live(&people)
                .depends_on_live(&cars)
                .reads([
                    "name.value",
                    "car_id.value",
                    "make.value",
                    "model.value",
                ])
                .produces(["ui.person_row"])
        },
        PersonGarageRowsMaintainer,
    )
    .unwrap();

create_car(
    &mut workspace,
    CreateCarInput {
        id: "car-1",
        make: "Honda",
        model: "Civic",
    },
);

create_person(
    &mut workspace,
    CreatePersonInput {
        id: "person-1",
        name: "Ava",
        car_id: "car-1",
    },
);

let ui_rows = workspace.materialize(&people_rows);

let rename_receipt = workspace
    .update("person-1", |person| {
        person.aspect("name.value", "Ava Chen")
    })
    .unwrap();

let person_patches = workspace.observe(&people);
let car_patches = workspace.observe(&cars);
let refreshed_ui_rows = workspace.materialize(&people_rows);

assert_eq!(rename_receipt.affected_live_view_ids(), &["garage.people".to_string()]);
assert_eq!(
    rename_receipt.affected_derived_view_ids(),
    &["garage.people_rows".to_string()]
);
assert!(car_patches.live_patches.is_empty());
```

What this shows:

- `garage.cars` is your car CRUD read surface
- `garage.people` is your person CRUD read surface
- `garage.people_rows` is the UI-ready surface
- renaming a person wakes `garage.people` and `garage.people_rows`
- it does not wake `garage.cars`, because car data was not touched

That is the important runtime property. You are not refetching the whole app.
You are only routing work through the touched surface graph.

## How It Relates To Other Features

- [Workspace Overview](./workspace-overview.md) explains the public runtime
  facade this example lives on.
- [Live Views](./live-views.md) explains the retained read surfaces.
- [Computed](./computed.md) explains the UI-ready derived surface.
- [Reads, Observe, and Materialization](./reads-observe-materialize.md)
  explains the three consumption paths used here.

## Inspection And Debugging

When this shape does not behave the way you expect, check:

- `affected_live_view_ids()` on the write receipt
- `affected_derived_view_ids()` on the write receipt
- `workspace.observe(&view)` to see which live patches actually drained
- `workspace.materialize(&computed)` to see current UI rows

If an unrelated surface wakes up, that usually means the declared read aspects
were broader than you intended.

## Anti-Patterns

- Building separate hand-written cache invalidation on top of the runtime.
- Treating live views as one-shot queries instead of retained runtime surfaces.
- Putting UI row shaping into ad hoc frontend selectors when it belongs in a
  computed runtime surface.
- Declaring overly broad read aspects and then expecting narrow touched-only
  updates.

## Current Limits

- This example is about the stable synchronous runtime surface.
- It is a replacement for a lot of ordinary app-layer wiring, not a claim that
  the workspace owns the underlying truth engine.
- Async resources, temporal execution, durable restart, and store-backed
  replay remain future neighbors rather than part of this minimal example.

## Related Docs

- [Workspace Overview](./workspace-overview.md)
- [Live Views](./live-views.md)
- [Computed](./computed.md)
- [Reads, Observe, and Materialization](./reads-observe-materialize.md)
