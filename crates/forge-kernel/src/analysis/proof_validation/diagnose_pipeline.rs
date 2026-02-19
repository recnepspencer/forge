//! Mid-pipeline diagnostic runner for boolean operations.
//!
//! DOMAIN: Non-fatal validation that captures structural health of an arena
//! at intermediate pipeline stages. Unlike `run_checkpoint` which propagates
//! errors, this module collects diagnostics without aborting the caller.
//!
//! DEPENDENCIES: `forge-topo` (arena, validate), `forge-core` (KernelError)

use forge_topo::arena::TopologyArena;
use forge_topo::validate::{validate_topology, ValidationLevel};
use serde::{Deserialize, Serialize};

/// Pipeline stage where a diagnostic was captured.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PipelineStage {
    /// After faces are copied into the result arena.
    PostCopy,
    /// After twin stitching (success or failure).
    PostStitch,
    /// After degenerate topology cleanup.
    PostCleanup,
    /// After coplanar merge and redundant vertex removal.
    PostPostprocess,
}

impl std::fmt::Display for PipelineStage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PipelineStage::PostCopy => write!(f, "PostCopy"),
            PipelineStage::PostStitch => write!(f, "PostStitch"),
            PipelineStage::PostCleanup => write!(f, "PostCleanup"),
            PipelineStage::PostPostprocess => write!(f, "PostPostprocess"),
        }
    }
}

/// Structured diagnostic captured at one pipeline stage.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PipelineDiagnostic {
    stage: PipelineStage,
    structural_ok: bool,
    structural_errors: Vec<String>,
    face_count: usize,
    half_edge_count: usize,
    vertex_count: usize,
    unpaired_twins: usize,
}

impl PipelineDiagnostic {
    /// The pipeline stage this diagnostic was captured at.
    pub fn stage(&self) -> PipelineStage {
        self.stage
    }

    /// Whether structural validation passed.
    pub fn structural_ok(&self) -> bool {
        self.structural_ok
    }

    /// Error messages from structural validation (empty if passed).
    pub fn structural_errors(&self) -> &[String] {
        &self.structural_errors
    }

    /// Number of faces in the arena at this stage.
    pub fn face_count(&self) -> usize {
        self.face_count
    }

    /// Number of halfedges in the arena at this stage.
    pub fn half_edge_count(&self) -> usize {
        self.half_edge_count
    }

    /// Number of vertices in the arena at this stage.
    pub fn vertex_count(&self) -> usize {
        self.vertex_count
    }

    /// Number of halfedges whose twin pointer is self (unpaired).
    pub fn unpaired_twins(&self) -> usize {
        self.unpaired_twins
    }

    /// Whether this diagnostic indicates a healthy state (structural ok + no unpaired).
    pub fn is_healthy(&self) -> bool {
        self.structural_ok && self.unpaired_twins == 0
    }
}

impl std::fmt::Display for PipelineDiagnostic {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let status = if self.is_healthy() { "HEALTHY" } else { "DEFECTIVE" };
        write!(
            f,
            "[{}] {} — F={} HE={} V={} unpaired={}",
            self.stage, status,
            self.face_count, self.half_edge_count, self.vertex_count, self.unpaired_twins
        )?;
        for err in &self.structural_errors {
            write!(f, "\n  ✗ {}", err)?;
        }
        Ok(())
    }
}

/// Run non-fatal structural diagnostics on an arena at a given pipeline stage.
///
/// Unlike `run_checkpoint`, this function never returns `Err`. It captures
/// all validation failures as data within the `PipelineDiagnostic` struct,
/// allowing the caller to log and continue.
pub fn diagnose_arena(
    arena: &TopologyArena,
    stage: PipelineStage,
) -> PipelineDiagnostic {
    let face_count = arena.face_count();
    let half_edge_count = arena.half_edge_count();
    let vertex_count = arena.vertex_count();

    let unpaired_twins = count_unpaired_twins(arena);

    let (structural_ok, structural_errors) = match validate_topology(arena, ValidationLevel::Full) {
        Ok(()) => (true, Vec::new()),
        Err(e) => (false, vec![format!("{e}")]),
    };

    PipelineDiagnostic {
        stage,
        structural_ok,
        structural_errors,
        face_count,
        half_edge_count,
        vertex_count,
        unpaired_twins,
    }
}

/// Count halfedges whose twin pointer points to themselves (unpaired).
fn count_unpaired_twins(arena: &TopologyArena) -> usize {
    arena.iter_half_edges()
        .filter(|(id, data)| *id == data.twin())
        .count()
}
