use forge_core::KernelError;

use crate::projection::data::{
    ProjectedBodyId, ProjectedFaceId, ProjectedLoopId, ProjectedLumpId, ProjectedRegionId,
    ProjectedShellId, ProjectedTopology,
};

use super::super::shared::vf;

pub fn validate_projected_hierarchy(topology: &ProjectedTopology) -> Result<(), KernelError> {
    for (lump_index, lump) in topology.lumps().iter().enumerate() {
        let body = lump.body;
        let body_data = body_checked(topology, body, "projected_hierarchy", "lump", lump_index as u32)?;
        if !body_data.lumps.contains(&ProjectedLumpId::new(lump_index as u32)) {
            return Err(vf(
                "projected_hierarchy",
                format!(
                    "Lump {} parent body {} does not list it as a child",
                    lump_index,
                    body.raw()
                ),
            ));
        }
    }

    for (region_index, region) in topology.regions().iter().enumerate() {
        let lump = region.lump;
        let lump_data =
            lump_checked(topology, lump, "projected_hierarchy", "region", region_index as u32)?;
        if !lump_data.regions.contains(&ProjectedRegionId::new(region_index as u32)) {
            return Err(vf(
                "projected_hierarchy",
                format!(
                    "Region {} parent lump {} does not list it as a child",
                    region_index,
                    lump.raw()
                ),
            ));
        }
    }

    for (shell_index, shell) in topology.shells().iter().enumerate() {
        let region = shell.region;
        let region_data =
            region_checked(topology, region, "projected_hierarchy", "shell", shell_index as u32)?;
        if !region_data.shells.contains(&ProjectedShellId::new(shell_index as u32)) {
            return Err(vf(
                "projected_hierarchy",
                format!(
                    "Shell {} parent region {} does not list it as a child",
                    shell_index,
                    region.raw()
                ),
            ));
        }
    }

    for (face_index, face) in topology.faces().iter().enumerate() {
        let shell = face.shell;
        let shell_data =
            shell_checked(topology, shell, "projected_hierarchy", "face", face_index as u32)?;
        if !shell_data.faces.contains(&ProjectedFaceId::new(face_index as u32)) {
            return Err(vf(
                "projected_hierarchy",
                format!(
                    "Face {} parent shell {} does not list it as a child",
                    face_index,
                    shell.raw()
                ),
            ));
        }
    }

    for (loop_index, loop_data) in topology.loops().iter().enumerate() {
        let face = loop_data.face;
        let face_data =
            face_checked(topology, face, "projected_hierarchy", "loop", loop_index as u32)?;
        let loop_id = ProjectedLoopId::new(loop_index as u32);
        if face_data.outer_loop != loop_id && !face_data.inner_loops.contains(&loop_id) {
            return Err(vf(
                "projected_hierarchy",
                format!(
                    "Loop {} parent face {} does not list it as outer or inner",
                    loop_index,
                    face.raw()
                ),
            ));
        }
    }

    Ok(())
}

fn body_checked<'a>(
    topology: &'a ProjectedTopology,
    id: ProjectedBodyId,
    validator: &str,
    child_kind: &str,
    child_index: u32,
) -> Result<&'a crate::projection::data::ProjectedBodyData, KernelError> {
    topology.bodies().get(id.index()).ok_or_else(|| {
        vf(
            validator,
            format!(
                "{} {} references missing body {}",
                child_kind,
                child_index,
                id.raw()
            ),
        )
    })
}

fn lump_checked<'a>(
    topology: &'a ProjectedTopology,
    id: ProjectedLumpId,
    validator: &str,
    child_kind: &str,
    child_index: u32,
) -> Result<&'a crate::projection::data::ProjectedLumpData, KernelError> {
    topology.lumps().get(id.index()).ok_or_else(|| {
        vf(
            validator,
            format!(
                "{} {} references missing lump {}",
                child_kind,
                child_index,
                id.raw()
            ),
        )
    })
}

fn region_checked<'a>(
    topology: &'a ProjectedTopology,
    id: ProjectedRegionId,
    validator: &str,
    child_kind: &str,
    child_index: u32,
) -> Result<&'a crate::projection::data::ProjectedRegionData, KernelError> {
    topology.regions().get(id.index()).ok_or_else(|| {
        vf(
            validator,
            format!(
                "{} {} references missing region {}",
                child_kind,
                child_index,
                id.raw()
            ),
        )
    })
}

fn shell_checked<'a>(
    topology: &'a ProjectedTopology,
    id: ProjectedShellId,
    validator: &str,
    child_kind: &str,
    child_index: u32,
) -> Result<&'a crate::projection::data::ProjectedShellData, KernelError> {
    topology.shells().get(id.index()).ok_or_else(|| {
        vf(
            validator,
            format!(
                "{} {} references missing shell {}",
                child_kind,
                child_index,
                id.raw()
            ),
        )
    })
}

fn face_checked<'a>(
    topology: &'a ProjectedTopology,
    id: ProjectedFaceId,
    validator: &str,
    child_kind: &str,
    child_index: u32,
) -> Result<&'a crate::projection::data::ProjectedFaceData, KernelError> {
    topology.faces().get(id.index()).ok_or_else(|| {
        vf(
            validator,
            format!(
                "{} {} references missing face {}",
                child_kind,
                child_index,
                id.raw()
            ),
        )
    })
}
