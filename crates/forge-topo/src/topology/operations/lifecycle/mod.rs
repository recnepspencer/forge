//! Body, component, lump, and shell lifecycle operators.
//!
//! DOMAIN: Creation, destruction, splitting, merging, extraction,
//! and rehoming of top-level topological containers.
//!
//! OPERATORS (from operators-list.md §B):
//! - B1: CreateBody, DestroyBody, CloneBody, DetachBody, AttachBody, SplitBody, MergeBodies
//! - B2: CreateComponent, DestroyComponent, SplitComponent, MergeComponents, ExtractComponent, ReattachComponent
//! - B3: CreateLump, DestroyLump, SplitLump, MergeLumps, ExtractLump, InsertLump, RehomeLump
//! - B4: CreateShell, DestroyShell, SplitShell, MergeShells, ExtractShell, InsertShell, PromoteShell, DemoteShell, RehomeShell
//!
//! DEPENDENCIES: `euler` (primitives), `arena` (entity storage)

pub mod solid;
pub mod lump;
pub mod shell;
