//! Adversarial tests for merge eligibility certification.
//!
//! DOMAIN: Tests that the boundary certification → merge gating pipeline
//! works correctly under adversarial inputs. Split into two sections:
//!
//! 1. **Certifier adversarial** — exercises forge-geom boundary_cert with
//!    pathological geometry (D2/D6/D7 regressions).
//! 2. **Kernel integration** — exercises the full pipeline through
//!    `certify_merge_boundary` with real `TopologyArena` + `GeometryState`,
//!    and validates trace propagation + geometry cleanup.
//!
//! DEPENDENCIES: forge-geom (boundary_cert), forge-topo, GeometryState, ModelingContext.


use crate::geom_facade::*;
use std::sync::{Mutex, OnceLock};


mod certifier;
mod integration;
mod sprint3;
mod regressions;
mod gate_policy;
