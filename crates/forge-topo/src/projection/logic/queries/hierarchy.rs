use crate::projection::data::{
    ProjectedBodyId, ProjectedFaceId, ProjectedLoopId, ProjectedLumpId, ProjectedRegionId,
    ProjectedShellId, ProjectedTopology,
};

pub fn body_lumps(topology: &ProjectedTopology, body: ProjectedBodyId) -> Vec<ProjectedLumpId> {
    topology.body(body).lumps.clone()
}

pub fn lump_body(topology: &ProjectedTopology, lump: ProjectedLumpId) -> ProjectedBodyId {
    topology.lump(lump).body
}

pub fn lump_regions(topology: &ProjectedTopology, lump: ProjectedLumpId) -> Vec<ProjectedRegionId> {
    topology.lump(lump).regions.clone()
}

pub fn region_lump(topology: &ProjectedTopology, region: ProjectedRegionId) -> ProjectedLumpId {
    topology.region(region).lump
}

pub fn region_shells(
    topology: &ProjectedTopology,
    region: ProjectedRegionId,
) -> Vec<ProjectedShellId> {
    topology.region(region).shells.clone()
}

pub fn shell_region(topology: &ProjectedTopology, shell: ProjectedShellId) -> ProjectedRegionId {
    topology.shell(shell).region
}

pub fn face_shell(topology: &ProjectedTopology, face: ProjectedFaceId) -> ProjectedShellId {
    topology.face(face).shell
}

pub fn face_outer_loop(topology: &ProjectedTopology, face: ProjectedFaceId) -> ProjectedLoopId {
    topology.face(face).outer_loop
}

pub fn face_inner_loops(
    topology: &ProjectedTopology,
    face: ProjectedFaceId,
) -> Vec<ProjectedLoopId> {
    topology.face(face).inner_loops.clone()
}

pub fn loop_face(topology: &ProjectedTopology, loop_id: ProjectedLoopId) -> ProjectedFaceId {
    topology.loop_data(loop_id).face
}
