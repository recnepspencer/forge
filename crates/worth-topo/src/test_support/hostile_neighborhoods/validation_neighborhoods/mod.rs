mod non_manifold_shell;
mod primitives;
mod seeded_and_closed_shell;
mod sheet_neighborhoods;
mod tetrahedral_shell;
mod wire_neighborhoods;

pub(crate) use non_manifold_shell::open_shell_nmt_fan_view;
pub(crate) use primitives::{edge, entity, half_edge, half_edge_with_links, vertex};
pub(crate) use seeded_and_closed_shell::{base_seeded_view, closed_shell_view};
pub(crate) use sheet_neighborhoods::{open_sheet_patch_view, single_face_sheet_disk_view};
pub(crate) use tetrahedral_shell::tetrahedral_closed_shell_view;
pub(crate) use wire_neighborhoods::{connected_wire_branch_view, open_wire_chain_view};




