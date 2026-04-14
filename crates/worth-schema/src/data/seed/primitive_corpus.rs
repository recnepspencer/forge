use forge_relational::facade::history::BranchId;
use forge_relational::facade::runtime::RelationalRuntime;
use serde::{Deserialize, Serialize};

use crate::data::authority::{
    RawWorthTopologyIntent, WorthMutationOrigin, WorthTopologyAuthority,
    WorthTopologyAuthorityError, VerifiedTopologyCommit,
};
use crate::data::entities::{WorthEntityKind, WorthTopologyEntityKind};
use crate::data::relations::{WorthRelationKind, WorthTopologyRelationKind};
use crate::data::seed::{created_ref, WorthTopologyCreateBatchBuilder};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum WorthMilestoneOnePrimitiveCase {
    WireOpen { half_edge_count: usize },
    WireClosed { half_edge_count: usize },
    WireBranch { branch_count: usize },
    SheetDisk { edge_count: usize },
    SheetPatch { face_count: usize },
    SolidShell { face_count: usize },
    NmtEdgeFan { face_count: usize },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum WorthMilestoneOnePrimitiveRole {
    Smallest,
    Generic,
    HostileAdmitted,
    OutOfClass,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum WorthMilestoneOnePrimitiveExpectedOutcome {
    Admit,
    Reject,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorthMilestoneOnePrimitiveScenario {
    pub family: String,
    pub role: WorthMilestoneOnePrimitiveRole,
    pub primitive: WorthMilestoneOnePrimitiveCase,
    pub expected_outcome: WorthMilestoneOnePrimitiveExpectedOutcome,
}

#[derive(Debug)]
pub enum WorthMilestoneOnePrimitiveAuthoringError {
    InvalidParameter {
        family: &'static str,
        parameter: usize,
        requirement: &'static str,
    },
    Authority(WorthTopologyAuthorityError),
}

impl std::fmt::Display for WorthMilestoneOnePrimitiveAuthoringError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidParameter {
                family,
                parameter,
                requirement,
            } => write!(
                f,
                "invalid `{family}` parameter `{parameter}`; expected {requirement}"
            ),
            Self::Authority(error) => write!(f, "{error:?}"),
        }
    }
}

impl std::error::Error for WorthMilestoneOnePrimitiveAuthoringError {}

impl From<WorthTopologyAuthorityError> for WorthMilestoneOnePrimitiveAuthoringError {
    fn from(value: WorthTopologyAuthorityError) -> Self {
        Self::Authority(value)
    }
}

pub fn build_milestone_one_primitive_intent(
    stem: &str,
    primitive: &WorthMilestoneOnePrimitiveCase,
) -> Result<RawWorthTopologyIntent, WorthMilestoneOnePrimitiveAuthoringError> {
    let mut graph = PrimitiveGraphAuthoring::new(stem);
    match primitive {
        WorthMilestoneOnePrimitiveCase::WireOpen { half_edge_count } => {
            if *half_edge_count < 1 {
                return Err(WorthMilestoneOnePrimitiveAuthoringError::InvalidParameter {
                    family: "WireOpen(n)",
                    parameter: *half_edge_count,
                    requirement: "n >= 1",
                });
            }
            graph.author_wire_open(*half_edge_count);
        }
        WorthMilestoneOnePrimitiveCase::WireClosed { half_edge_count } => {
            if *half_edge_count < 3 {
                return Err(WorthMilestoneOnePrimitiveAuthoringError::InvalidParameter {
                    family: "WireClosed(n)",
                    parameter: *half_edge_count,
                    requirement: "n >= 3",
                });
            }
            graph.author_wire_closed(*half_edge_count);
        }
        WorthMilestoneOnePrimitiveCase::WireBranch { branch_count } => {
            if *branch_count < 3 {
                return Err(WorthMilestoneOnePrimitiveAuthoringError::InvalidParameter {
                    family: "WireBranch(k)",
                    parameter: *branch_count,
                    requirement: "k >= 3",
                });
            }
            graph.author_wire_branch(*branch_count);
        }
        WorthMilestoneOnePrimitiveCase::SheetDisk { edge_count } => {
            if *edge_count < 3 {
                return Err(WorthMilestoneOnePrimitiveAuthoringError::InvalidParameter {
                    family: "SheetDisk(n)",
                    parameter: *edge_count,
                    requirement: "n >= 3",
                });
            }
            graph.author_sheet_disk(*edge_count);
        }
        WorthMilestoneOnePrimitiveCase::SheetPatch { face_count } => {
            if *face_count < 2 {
                return Err(WorthMilestoneOnePrimitiveAuthoringError::InvalidParameter {
                    family: "SheetPatch(f)",
                    parameter: *face_count,
                    requirement: "f >= 2",
                });
            }
            graph.author_sheet_patch(*face_count);
        }
        WorthMilestoneOnePrimitiveCase::SolidShell { face_count } => {
            if *face_count < 4 {
                return Err(WorthMilestoneOnePrimitiveAuthoringError::InvalidParameter {
                    family: "SolidShell(f)",
                    parameter: *face_count,
                    requirement: "f >= 4",
                });
            }
            graph.author_solid_shell(*face_count);
        }
        WorthMilestoneOnePrimitiveCase::NmtEdgeFan { face_count } => {
            if *face_count < 3 {
                return Err(WorthMilestoneOnePrimitiveAuthoringError::InvalidParameter {
                    family: "NmtEdgeFan(k)",
                    parameter: *face_count,
                    requirement: "k >= 3",
                });
            }
            graph.author_nmt_edge_fan(*face_count);
        }
    }

    Ok(graph.finish())
}

pub fn seed_milestone_one_primitive(
    runtime: &mut RelationalRuntime,
    stem: &str,
    primitive: &WorthMilestoneOnePrimitiveCase,
) -> Result<VerifiedTopologyCommit, WorthMilestoneOnePrimitiveAuthoringError> {
    let intent = build_milestone_one_primitive_intent(stem, primitive)?;
    Ok(WorthTopologyAuthority::new(runtime).apply_topology_intent(intent)?)
}

pub fn seed_milestone_one_primitive_on_branch(
    runtime: &mut RelationalRuntime,
    stem: &str,
    primitive: &WorthMilestoneOnePrimitiveCase,
    branch_id: BranchId,
    mutation_origin: WorthMutationOrigin,
) -> Result<VerifiedTopologyCommit, WorthMilestoneOnePrimitiveAuthoringError> {
    let mut intent = build_milestone_one_primitive_intent(stem, primitive)?;
    intent.mutation_origin = mutation_origin;
    Ok(WorthTopologyAuthority::new(runtime).apply_topology_intent_on_branch(intent, branch_id)?)
}

pub fn milestone_one_default_primitive_corpus() -> Vec<WorthMilestoneOnePrimitiveScenario> {
    use WorthMilestoneOnePrimitiveCase as Case;
    use WorthMilestoneOnePrimitiveExpectedOutcome as Outcome;
    use WorthMilestoneOnePrimitiveRole as Role;

    vec![
        scenario("WireOpen(n)", Role::Smallest, Case::WireOpen { half_edge_count: 1 }, Outcome::Admit),
        scenario("WireOpen(n)", Role::Generic, Case::WireOpen { half_edge_count: 4 }, Outcome::Admit),
        scenario("WireOpen(n)", Role::HostileAdmitted, Case::WireOpen { half_edge_count: 8 }, Outcome::Admit),
        scenario("WireOpen(n)", Role::OutOfClass, Case::WireOpen { half_edge_count: 0 }, Outcome::Reject),
        scenario("WireClosed(n)", Role::Smallest, Case::WireClosed { half_edge_count: 3 }, Outcome::Admit),
        scenario("WireClosed(n)", Role::Generic, Case::WireClosed { half_edge_count: 4 }, Outcome::Admit),
        scenario("WireClosed(n)", Role::HostileAdmitted, Case::WireClosed { half_edge_count: 8 }, Outcome::Admit),
        scenario("WireClosed(n)", Role::OutOfClass, Case::WireClosed { half_edge_count: 2 }, Outcome::Reject),
        scenario("WireBranch(k)", Role::Smallest, Case::WireBranch { branch_count: 3 }, Outcome::Admit),
        scenario("WireBranch(k)", Role::Generic, Case::WireBranch { branch_count: 4 }, Outcome::Admit),
        scenario("WireBranch(k)", Role::HostileAdmitted, Case::WireBranch { branch_count: 8 }, Outcome::Admit),
        scenario("WireBranch(k)", Role::OutOfClass, Case::WireBranch { branch_count: 2 }, Outcome::Reject),
        scenario("SheetDisk(n)", Role::Smallest, Case::SheetDisk { edge_count: 3 }, Outcome::Admit),
        scenario("SheetDisk(n)", Role::Generic, Case::SheetDisk { edge_count: 5 }, Outcome::Admit),
        scenario("SheetDisk(n)", Role::HostileAdmitted, Case::SheetDisk { edge_count: 9 }, Outcome::Admit),
        scenario("SheetDisk(n)", Role::OutOfClass, Case::SheetDisk { edge_count: 2 }, Outcome::Reject),
        scenario("SheetPatch(f)", Role::Smallest, Case::SheetPatch { face_count: 2 }, Outcome::Admit),
        scenario("SheetPatch(f)", Role::Generic, Case::SheetPatch { face_count: 5 }, Outcome::Admit),
        scenario("SheetPatch(f)", Role::HostileAdmitted, Case::SheetPatch { face_count: 8 }, Outcome::Admit),
        scenario("SheetPatch(f)", Role::OutOfClass, Case::SheetPatch { face_count: 1 }, Outcome::Reject),
        scenario("SolidShell(f)", Role::Smallest, Case::SolidShell { face_count: 4 }, Outcome::Admit),
        scenario("SolidShell(f)", Role::Generic, Case::SolidShell { face_count: 6 }, Outcome::Admit),
        scenario("SolidShell(f)", Role::HostileAdmitted, Case::SolidShell { face_count: 9 }, Outcome::Admit),
        scenario("SolidShell(f)", Role::OutOfClass, Case::SolidShell { face_count: 3 }, Outcome::Reject),
        scenario("NmtEdgeFan(k)", Role::Smallest, Case::NmtEdgeFan { face_count: 3 }, Outcome::Admit),
        scenario("NmtEdgeFan(k)", Role::Generic, Case::NmtEdgeFan { face_count: 4 }, Outcome::Admit),
        scenario("NmtEdgeFan(k)", Role::HostileAdmitted, Case::NmtEdgeFan { face_count: 8 }, Outcome::Admit),
        scenario("NmtEdgeFan(k)", Role::OutOfClass, Case::NmtEdgeFan { face_count: 2 }, Outcome::Reject),
    ]
}

fn scenario(
    family: &str,
    role: WorthMilestoneOnePrimitiveRole,
    primitive: WorthMilestoneOnePrimitiveCase,
    expected_outcome: WorthMilestoneOnePrimitiveExpectedOutcome,
) -> WorthMilestoneOnePrimitiveScenario {
    WorthMilestoneOnePrimitiveScenario {
        family: family.to_string(),
        role,
        primitive,
        expected_outcome,
    }
}

struct PrimitiveGraphAuthoring {
    stem: String,
    builder: WorthTopologyCreateBatchBuilder,
}

impl PrimitiveGraphAuthoring {
    fn new(stem: &str) -> Self {
        Self {
            stem: stem.to_string(),
            builder: WorthTopologyCreateBatchBuilder::new(),
        }
    }

    fn finish(self) -> RawWorthTopologyIntent {
        self.builder.finish(WorthMutationOrigin::Seed)
    }

    fn key(&self, suffix: impl AsRef<str>) -> String {
        format!("{}.{}", self.stem, suffix.as_ref())
    }

    fn add_named_entity(&mut self, key: &str, kind: WorthTopologyEntityKind) {
        self.builder
            .push_topology_entity(key.to_string(), WorthEntityKind::Topology(kind));
        self.builder.push_persistent_name_for(key.to_string());
    }

    fn relate(
        &mut self,
        create_key: String,
        kind: WorthTopologyRelationKind,
        source: &str,
        target: &str,
    ) {
        self.builder.push_relation(
            create_key,
            WorthRelationKind::Topology(kind),
            created_ref(source.to_string()),
            created_ref(target.to_string()),
        );
    }

    fn add_container_context(&mut self, prefix: &str) -> ContainerContext {
        let model = self.key(format!("{prefix}.model"));
        let body = self.key(format!("{prefix}.body"));
        let lump = self.key(format!("{prefix}.lump"));
        let region = self.key(format!("{prefix}.region"));
        let shell = self.key(format!("{prefix}.shell"));

        self.add_named_entity(&model, WorthTopologyEntityKind::Model);
        self.add_named_entity(&body, WorthTopologyEntityKind::Body);
        self.add_named_entity(&lump, WorthTopologyEntityKind::Lump);
        self.add_named_entity(&region, WorthTopologyEntityKind::Region);
        self.add_named_entity(&shell, WorthTopologyEntityKind::Shell);

        self.relate(
            format!("{}.owns_body", model),
            WorthTopologyRelationKind::ModelOwnsBody,
            &model,
            &body,
        );
        self.relate(
            format!("{}.owns_lump", body),
            WorthTopologyRelationKind::BodyOwnsLump,
            &body,
            &lump,
        );
        self.relate(
            format!("{}.owns_region", lump),
            WorthTopologyRelationKind::LumpOwnsRegion,
            &lump,
            &region,
        );
        self.relate(
            format!("{}.owns_shell", region),
            WorthTopologyRelationKind::RegionOwnsShell,
            &region,
            &shell,
        );

        ContainerContext { shell }
    }

    fn add_shell_context(&mut self, prefix: &str) -> ShellContext {
        let container = self.add_container_context(prefix);
        let face = self.key(format!("{prefix}.face"));
        let loop_key = self.key(format!("{prefix}.loop"));
        let wire = self.key(format!("{prefix}.wire"));
        self.add_named_entity(&face, WorthTopologyEntityKind::Face);
        self.add_named_entity(&loop_key, WorthTopologyEntityKind::Loop);

        ShellContext {
            shell: container.shell,
            face,
            loop_key,
            wire,
        }
    }

    fn link_half_edge(
        &mut self,
        half_edge: &str,
        loop_key: &str,
        wire: &str,
        edge: &str,
        start: &str,
        end: &str,
        next: &str,
        prev: &str,
        radial: &str,
    ) {
        self.relate(
            format!("{half_edge}.loop"),
            WorthTopologyRelationKind::LoopOwnsHalfEdge,
            loop_key,
            half_edge,
        );
        self.relate(
            format!("{half_edge}.wire"),
            WorthTopologyRelationKind::WireOwnsHalfEdge,
            wire,
            half_edge,
        );
        self.relate(
            format!("{half_edge}.next"),
            WorthTopologyRelationKind::HalfEdgeNext,
            half_edge,
            next,
        );
        self.relate(
            format!("{half_edge}.prev"),
            WorthTopologyRelationKind::HalfEdgePrev,
            half_edge,
            prev,
        );
        self.relate(
            format!("{half_edge}.radial"),
            WorthTopologyRelationKind::HalfEdgeRadialNext,
            half_edge,
            radial,
        );
        self.relate(
            format!("{half_edge}.edge"),
            WorthTopologyRelationKind::HalfEdgeUsesEdge,
            half_edge,
            edge,
        );
        self.relate(
            format!("{half_edge}.start"),
            WorthTopologyRelationKind::HalfEdgeStartsAtVertex,
            half_edge,
            start,
        );
        self.relate(
            format!("{half_edge}.end"),
            WorthTopologyRelationKind::HalfEdgeEndsAtVertex,
            half_edge,
            end,
        );
    }

    fn author_wire_open(&mut self, half_edge_count: usize) {
        let ctx = self.add_shell_context("wire_open");
        self.add_named_entity(&ctx.wire, WorthTopologyEntityKind::Wire);
        self.relate(
            format!("{}.owns_face", ctx.shell),
            WorthTopologyRelationKind::ShellOwnsFace,
            &ctx.shell,
            &ctx.face,
        );
        self.relate(
            format!("{}.outer_loop", ctx.face),
            WorthTopologyRelationKind::FaceOuterLoop,
            &ctx.face,
            &ctx.loop_key,
        );

        for index in 0..=half_edge_count {
            self.add_named_entity(
                &self.key(format!("wire_open.vertex.{index}")),
                WorthTopologyEntityKind::Vertex,
            );
        }
        for index in 0..half_edge_count {
            let edge = self.key(format!("wire_open.edge.{index}"));
            let half_edge = self.key(format!("wire_open.half_edge.{index}"));
            let start = self.key(format!("wire_open.vertex.{index}"));
            let end = self.key(format!("wire_open.vertex.{}", index + 1));
            self.add_named_entity(&edge, WorthTopologyEntityKind::Edge);
            self.add_named_entity(&half_edge, WorthTopologyEntityKind::HalfEdge);
            self.link_half_edge(
                &half_edge,
                &ctx.loop_key,
                &ctx.wire,
                &edge,
                &start,
                &end,
                &half_edge,
                &half_edge,
                &half_edge,
            );
        }
    }

    fn author_wire_closed(&mut self, half_edge_count: usize) {
        let ctx = self.add_shell_context("wire_closed");
        self.add_named_entity(&ctx.wire, WorthTopologyEntityKind::Wire);
        self.relate(
            format!("{}.owns_face", ctx.shell),
            WorthTopologyRelationKind::ShellOwnsFace,
            &ctx.shell,
            &ctx.face,
        );
        self.relate(
            format!("{}.outer_loop", ctx.face),
            WorthTopologyRelationKind::FaceOuterLoop,
            &ctx.face,
            &ctx.loop_key,
        );

        for index in 0..half_edge_count {
            self.add_named_entity(
                &self.key(format!("wire_closed.vertex.{index}")),
                WorthTopologyEntityKind::Vertex,
            );
        }
        for index in 0..half_edge_count {
            let edge = self.key(format!("wire_closed.edge.{index}"));
            let half_edge = self.key(format!("wire_closed.half_edge.{index}"));
            let next = self.key(format!("wire_closed.half_edge.{}", (index + 1) % half_edge_count));
            let prev = self.key(format!(
                "wire_closed.half_edge.{}",
                (index + half_edge_count - 1) % half_edge_count
            ));
            let start = self.key(format!("wire_closed.vertex.{index}"));
            let end = self.key(format!(
                "wire_closed.vertex.{}",
                (index + 1) % half_edge_count
            ));
            self.add_named_entity(&edge, WorthTopologyEntityKind::Edge);
            self.add_named_entity(&half_edge, WorthTopologyEntityKind::HalfEdge);
            self.link_half_edge(
                &half_edge,
                &ctx.loop_key,
                &ctx.wire,
                &edge,
                &start,
                &end,
                &next,
                &prev,
                &half_edge,
            );
        }
    }

    fn author_wire_branch(&mut self, branch_count: usize) {
        let ctx = self.add_shell_context("wire_branch");
        self.add_named_entity(&ctx.wire, WorthTopologyEntityKind::Wire);
        self.relate(
            format!("{}.owns_face", ctx.shell),
            WorthTopologyRelationKind::ShellOwnsFace,
            &ctx.shell,
            &ctx.face,
        );
        self.relate(
            format!("{}.outer_loop", ctx.face),
            WorthTopologyRelationKind::FaceOuterLoop,
            &ctx.face,
            &ctx.loop_key,
        );

        let center = self.key("wire_branch.vertex.center");
        self.add_named_entity(&center, WorthTopologyEntityKind::Vertex);
        for index in 0..branch_count {
            let edge = self.key(format!("wire_branch.edge.{index}"));
            let half_edge = self.key(format!("wire_branch.half_edge.{index}"));
            let leaf = self.key(format!("wire_branch.vertex.leaf.{index}"));
            self.add_named_entity(&leaf, WorthTopologyEntityKind::Vertex);
            self.add_named_entity(&edge, WorthTopologyEntityKind::Edge);
            self.add_named_entity(&half_edge, WorthTopologyEntityKind::HalfEdge);
            self.link_half_edge(
                &half_edge,
                &ctx.loop_key,
                &ctx.wire,
                &edge,
                &center,
                &leaf,
                &half_edge,
                &half_edge,
                &half_edge,
            );
        }
    }

    fn author_sheet_disk(&mut self, edge_count: usize) {
        let ctx = self.add_shell_context("sheet_disk");
        self.add_named_entity(&ctx.wire, WorthTopologyEntityKind::Wire);
        self.relate(
            format!("{}.owns_face", ctx.shell),
            WorthTopologyRelationKind::ShellOwnsFace,
            &ctx.shell,
            &ctx.face,
        );
        self.relate(
            format!("{}.outer_loop", ctx.face),
            WorthTopologyRelationKind::FaceOuterLoop,
            &ctx.face,
            &ctx.loop_key,
        );

        for index in 0..edge_count {
            self.add_named_entity(
                &self.key(format!("sheet_disk.vertex.{index}")),
                WorthTopologyEntityKind::Vertex,
            );
        }
        for index in 0..edge_count {
            let edge = self.key(format!("sheet_disk.edge.{index}"));
            let half_edge = self.key(format!("sheet_disk.half_edge.{index}"));
            let next = self.key(format!("sheet_disk.half_edge.{}", (index + 1) % edge_count));
            let prev = self.key(format!(
                "sheet_disk.half_edge.{}",
                (index + edge_count - 1) % edge_count
            ));
            let start = self.key(format!("sheet_disk.vertex.{index}"));
            let end = self.key(format!(
                "sheet_disk.vertex.{}",
                (index + 1) % edge_count
            ));
            self.add_named_entity(&edge, WorthTopologyEntityKind::Edge);
            self.add_named_entity(&half_edge, WorthTopologyEntityKind::HalfEdge);
            self.link_half_edge(
                &half_edge,
                &ctx.loop_key,
                &ctx.wire,
                &edge,
                &start,
                &end,
                &next,
                &prev,
                &half_edge,
            );
        }
    }

    fn author_sheet_patch(&mut self, face_count: usize) {
        let ctx = self.add_container_context("sheet_patch");
        let center = self.key("sheet_patch.vertex.center");
        self.add_named_entity(&center, WorthTopologyEntityKind::Vertex);

        for index in 0..=face_count {
            self.add_named_entity(
                &self.key(format!("sheet_patch.vertex.boundary.{index}")),
                WorthTopologyEntityKind::Vertex,
            );
            self.add_named_entity(
                &self.key(format!("sheet_patch.edge.spoke.{index}")),
                WorthTopologyEntityKind::Edge,
            );
        }
        for index in 0..face_count {
            self.add_named_entity(
                &self.key(format!("sheet_patch.edge.boundary.{index}")),
                WorthTopologyEntityKind::Edge,
            );
        }

        for index in 0..face_count {
            let face = self.key(format!("sheet_patch.face.{index}"));
            let loop_key = self.key(format!("sheet_patch.loop.{index}"));
            let wire = self.key(format!("sheet_patch.wire.{index}"));
            let half_edge_a = self.key(format!("sheet_patch.half_edge.{index}.a"));
            let half_edge_b = self.key(format!("sheet_patch.half_edge.{index}.b"));
            let half_edge_c = self.key(format!("sheet_patch.half_edge.{index}.c"));
            let boundary_start = self.key(format!("sheet_patch.vertex.boundary.{index}"));
            let boundary_end = self.key(format!("sheet_patch.vertex.boundary.{}", index + 1));
            let spoke_start_edge = self.key(format!("sheet_patch.edge.spoke.{index}"));
            let spoke_end_edge = self.key(format!("sheet_patch.edge.spoke.{}", index + 1));
            let boundary_edge = self.key(format!("sheet_patch.edge.boundary.{index}"));
            let radial_prev = if index == 0 {
                half_edge_a.clone()
            } else {
                self.key(format!("sheet_patch.half_edge.{}.c", index - 1))
            };
            let radial_next = if index + 1 == face_count {
                half_edge_c.clone()
            } else {
                self.key(format!("sheet_patch.half_edge.{}.a", index + 1))
            };

            self.add_named_entity(&face, WorthTopologyEntityKind::Face);
            self.add_named_entity(&loop_key, WorthTopologyEntityKind::Loop);
            self.add_named_entity(&wire, WorthTopologyEntityKind::Wire);
            self.relate(
                format!("{}.owns_face.{index}", ctx.shell),
                WorthTopologyRelationKind::ShellOwnsFace,
                &ctx.shell,
                &face,
            );
            self.relate(
                format!("{}.outer_loop", face),
                WorthTopologyRelationKind::FaceOuterLoop,
                &face,
                &loop_key,
            );

            self.add_named_entity(&half_edge_a, WorthTopologyEntityKind::HalfEdge);
            self.add_named_entity(&half_edge_b, WorthTopologyEntityKind::HalfEdge);
            self.add_named_entity(&half_edge_c, WorthTopologyEntityKind::HalfEdge);
            self.link_half_edge(
                &half_edge_a,
                &loop_key,
                &wire,
                &spoke_start_edge,
                &center,
                &boundary_start,
                &half_edge_b,
                &half_edge_c,
                &radial_prev,
            );
            self.link_half_edge(
                &half_edge_b,
                &loop_key,
                &wire,
                &boundary_edge,
                &boundary_start,
                &boundary_end,
                &half_edge_c,
                &half_edge_a,
                &half_edge_b,
            );
            self.link_half_edge(
                &half_edge_c,
                &loop_key,
                &wire,
                &spoke_end_edge,
                &boundary_end,
                &center,
                &half_edge_a,
                &half_edge_b,
                &radial_next,
            );
        }
    }

    fn author_solid_shell(&mut self, face_count: usize) {
        let ctx = self.add_container_context("solid_shell");
        let base_edge_count = face_count - 1;
        let apex = self.key("solid_shell.vertex.apex");
        self.add_named_entity(&apex, WorthTopologyEntityKind::Vertex);

        for index in 0..base_edge_count {
            self.add_named_entity(
                &self.key(format!("solid_shell.vertex.base.{index}")),
                WorthTopologyEntityKind::Vertex,
            );
            self.add_named_entity(
                &self.key(format!("solid_shell.edge.base.{index}")),
                WorthTopologyEntityKind::Edge,
            );
            self.add_named_entity(
                &self.key(format!("solid_shell.edge.side.{index}")),
                WorthTopologyEntityKind::Edge,
            );
        }

        for index in 0..face_count {
            let face = self.key(format!("solid_shell.face.{index}"));
            let loop_key = self.key(format!("solid_shell.loop.{index}"));
            let wire = self.key(format!("solid_shell.wire.{index}"));
            self.add_named_entity(&face, WorthTopologyEntityKind::Face);
            self.add_named_entity(&loop_key, WorthTopologyEntityKind::Loop);
            self.add_named_entity(&wire, WorthTopologyEntityKind::Wire);
            self.relate(
                format!("{}.owns_face.{index}", ctx.shell),
                WorthTopologyRelationKind::ShellOwnsFace,
                &ctx.shell,
                &face,
            );
            self.relate(
                format!("{}.outer_loop", face),
                WorthTopologyRelationKind::FaceOuterLoop,
                &face,
                &loop_key,
            );
        }

        for index in 0..base_edge_count {
            let loop_key = self.key(format!("solid_shell.loop.{index}"));
            let wire = self.key(format!("solid_shell.wire.{index}"));
            let half_edge_a = self.key(format!("solid_shell.half_edge.{index}.a"));
            let half_edge_b = self.key(format!("solid_shell.half_edge.{index}.b"));
            let half_edge_c = self.key(format!("solid_shell.half_edge.{index}.c"));
            let base_start = self.key(format!("solid_shell.vertex.base.{index}"));
            let base_end = self.key(format!(
                "solid_shell.vertex.base.{}",
                (index + 1) % base_edge_count
            ));
            let side_start_edge = self.key(format!("solid_shell.edge.side.{index}"));
            let side_end_edge =
                self.key(format!("solid_shell.edge.side.{}", (index + 1) % base_edge_count));
            let base_edge = self.key(format!("solid_shell.edge.base.{index}"));
            let radial_prev = self.key(format!(
                "solid_shell.half_edge.{}.c",
                (index + base_edge_count - 1) % base_edge_count
            ));
            let radial_next =
                self.key(format!("solid_shell.half_edge.{}.a", (index + 1) % base_edge_count));

            self.add_named_entity(&half_edge_a, WorthTopologyEntityKind::HalfEdge);
            self.add_named_entity(&half_edge_b, WorthTopologyEntityKind::HalfEdge);
            self.add_named_entity(&half_edge_c, WorthTopologyEntityKind::HalfEdge);
            self.link_half_edge(
                &half_edge_a,
                &loop_key,
                &wire,
                &side_start_edge,
                &apex,
                &base_start,
                &half_edge_b,
                &half_edge_c,
                &radial_prev,
            );
            self.link_half_edge(
                &half_edge_b,
                &loop_key,
                &wire,
                &base_edge,
                &base_start,
                &base_end,
                &half_edge_c,
                &half_edge_a,
                &self.key(format!("solid_shell.base_half_edge.{index}")),
            );
            self.link_half_edge(
                &half_edge_c,
                &loop_key,
                &wire,
                &side_end_edge,
                &base_end,
                &apex,
                &half_edge_a,
                &half_edge_b,
                &radial_next,
            );
        }

        let base_loop = self.key(format!("solid_shell.loop.{base_edge_count}"));
        let base_wire = self.key(format!("solid_shell.wire.{base_edge_count}"));
        for index in 0..base_edge_count {
            let half_edge = self.key(format!("solid_shell.base_half_edge.{index}"));
            let next = self.key(format!(
                "solid_shell.base_half_edge.{}",
                (index + base_edge_count - 1) % base_edge_count
            ));
            let prev =
                self.key(format!("solid_shell.base_half_edge.{}", (index + 1) % base_edge_count));
            let start = self.key(format!(
                "solid_shell.vertex.base.{}",
                (index + 1) % base_edge_count
            ));
            let end = self.key(format!("solid_shell.vertex.base.{index}"));
            let edge = self.key(format!("solid_shell.edge.base.{index}"));
            let radial = self.key(format!("solid_shell.half_edge.{index}.b"));

            self.add_named_entity(&half_edge, WorthTopologyEntityKind::HalfEdge);
            self.link_half_edge(
                &half_edge,
                &base_loop,
                &base_wire,
                &edge,
                &start,
                &end,
                &next,
                &prev,
                &radial,
            );
        }
    }

    fn author_nmt_edge_fan(&mut self, face_count: usize) {
        let ctx = self.add_container_context("nmt_edge_fan");
        let shared_edge = self.key("nmt_edge_fan.edge.shared");
        let v1 = self.key("nmt_edge_fan.vertex.1");
        let v2 = self.key("nmt_edge_fan.vertex.2");
        self.add_named_entity(&shared_edge, WorthTopologyEntityKind::Edge);
        self.add_named_entity(&v1, WorthTopologyEntityKind::Vertex);
        self.add_named_entity(&v2, WorthTopologyEntityKind::Vertex);

        for index in 0..face_count {
            let face = self.key(format!("nmt_edge_fan.face.{index}"));
            let loop_key = self.key(format!("nmt_edge_fan.loop.{index}"));
            let wire = self.key(format!("nmt_edge_fan.wire.{index}"));
            let third = self.key(format!("nmt_edge_fan.vertex.third.{index}"));
            let shared = self.key(format!("nmt_edge_fan.shared_half_edge.{index}"));
            let side_a = self.key(format!("nmt_edge_fan.side_a_half_edge.{index}"));
            let side_b = self.key(format!("nmt_edge_fan.side_b_half_edge.{index}"));
            let edge_a = self.key(format!("nmt_edge_fan.edge.a.{index}"));
            let edge_b = self.key(format!("nmt_edge_fan.edge.b.{index}"));
            let shared_radial =
                self.key(format!("nmt_edge_fan.shared_half_edge.{}", (index + 1) % face_count));

            self.add_named_entity(&face, WorthTopologyEntityKind::Face);
            self.add_named_entity(&loop_key, WorthTopologyEntityKind::Loop);
            self.add_named_entity(&wire, WorthTopologyEntityKind::Wire);
            self.add_named_entity(&third, WorthTopologyEntityKind::Vertex);
            self.add_named_entity(&edge_a, WorthTopologyEntityKind::Edge);
            self.add_named_entity(&edge_b, WorthTopologyEntityKind::Edge);
            self.add_named_entity(&shared, WorthTopologyEntityKind::HalfEdge);
            self.add_named_entity(&side_a, WorthTopologyEntityKind::HalfEdge);
            self.add_named_entity(&side_b, WorthTopologyEntityKind::HalfEdge);

            self.relate(
                format!("{}.owns_face.{index}", ctx.shell),
                WorthTopologyRelationKind::ShellOwnsFace,
                &ctx.shell,
                &face,
            );
            self.relate(
                format!("{}.outer_loop", face),
                WorthTopologyRelationKind::FaceOuterLoop,
                &face,
                &loop_key,
            );

            let (shared_start, shared_end, side_a_start, side_a_end, side_b_start, side_b_end) =
                if index == 0 {
                    (&v1, &v2, &v2, &third, &third, &v1)
                } else {
                    (&v2, &v1, &v1, &third, &third, &v2)
                };

            self.link_half_edge(&shared, &loop_key, &wire, &shared_edge, shared_start, shared_end, &side_a, &side_b, &shared_radial);
            self.link_half_edge(&side_a, &loop_key, &wire, &edge_a, side_a_start, side_a_end, &side_b, &shared, &side_a);
            self.link_half_edge(&side_b, &loop_key, &wire, &edge_b, side_b_start, side_b_end, &shared, &side_a, &side_b);
        }
    }
}

struct ShellContext {
    shell: String,
    face: String,
    loop_key: String,
    wire: String,
}

struct ContainerContext {
    shell: String,
}
