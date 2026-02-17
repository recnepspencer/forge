---
description: Create a new module or feature directory following the Bento Box pattern and naming conventions
---

---
title: New Module (v2)
description: Create modules following the Provider Pattern and Firewall architecture
---

# New Module Workflow (v2)

## Step 1: Identify the Layer

Refer to `architecture.md` to ensure correct dependency direction:
- **math**: Pure numbers only.
- **core**: Shared traits (`GeometrySource`, `PolicyResult`).
- **geom**: Stateless solvers only.
- **topo**: Connectivity and Generational safety. No `f64` math.
- **kernel**: Policy, Feature Tree, and Orchestration.

## Step 2: The Template (Feature Module)

### `intent.rs` — The Agent API
```rust
//! High-level intent and serialization for <Feature>.
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct <Feature>Intent {
    pub name: String,
    pub parameters: <Feature>Params,
}