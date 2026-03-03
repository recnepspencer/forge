# Test Harness — Future Work

Items blocked on kernel features that don't exist yet.

## Phase 2 — Blocked on Boolean Stabilization

- [ ] **`builders/seeders/booleans.rs`** — Boolean test scenarios
  - `seed_overlapping_pair()` — two cubes sharing volume
  - `seed_face_touching_pair()` — coplanar contact
  - `seed_contained_pair()` — B fully inside A
  - `seed_coplanar_nightmare()` — opposing normals on shared face
  - `seed_csg_chain()` — A ∪ B → result ∩ C

- [ ] **`builders/scenes.rs`** — Fluent multi-solid composition
  - `SceneBuilder::new().cube(...).cube(...).build()` → `Vec<SolidEnvelope>`
  - Useful when operations consume multiple solids

- [ ] **Extend `selectors.rs`** — queries across multiple solids
  - `select_pair(&a, &b).shared_faces(tol)` for boolean pre-checks

## Phase 3 — Blocked on Fillets / NURBS

- [ ] **Extended entity selectors** — surface type queries

  ```rust
  select(&env).faces().where_surface_type(SurfaceKind::Cylinder).all();
  ```

- [ ] **Advanced seeders** — `seed_machined_block()`, `seed_sheet_metal_bend()`

## Performance Follow-Up

- [ ] **DecisionSink verbosity gating** — skip allocation for low-verbosity runs
- [ ] **Background thread post-processing** — offload DecisionLog summary computation



Areas to Watch
While the architecture is incredibly solid, here are a few things to keep in mind as the workspace scales:

Selector Query Performance: Right now, methods like where_normal_near and where_length_above iterate over all candidates and perform geometric computations on the fly. This is perfectly fine for integration tests on primitives, but if you start running these selectors on highly complex scenes (e.g., thousands of faces), you might need to introduce bounding-box early-outs or spatial indexing (like a BVH) to keep the test suite fast.

Error Context in Chains: In chains.rs, if a structural assertion fails mid-chain, the panic drops a step count and the error ("Structural invariant violation at step {}: {:?}", self.step_count, e)). You might want to automatically trigger the OBJ dump here as well, saving the specific intermediate state that broke, rather than just panicking out.

Lineage Test Brittleness: Your observability tests (lineage.rs) are currently ignored (Phase 1.2), but once active, asserting strictly on chronological event logs can sometimes make tests brittle if internal algorithms are optimized to batch or reorder non-dependent entity creations.
