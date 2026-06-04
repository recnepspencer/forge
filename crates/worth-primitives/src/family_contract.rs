#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PrimitiveConstructionFamilyKey {
    SimplexSolid,
    Orthotope,
    RegularPrism,
    RegularPyramid,
    WireBody,
    ShellWithHole,
}

impl PrimitiveConstructionFamilyKey {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::SimplexSolid => "simplex_solid",
            Self::Orthotope => "orthotope",
            Self::RegularPrism => "regular_prism",
            Self::RegularPyramid => "regular_pyramid",
            Self::WireBody => "wire_body",
            Self::ShellWithHole => "shell_with_hole",
        }
    }

    pub fn topology_birth_class(self) -> &'static str {
        match self {
            Self::WireBody => "planar_wire_body",
            Self::ShellWithHole => "planar_shell_with_hole_body",
            _ => "solid_body",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PrimitiveWitnessTopologySummary {
    vertex_count: usize,
    edge_count: usize,
    loop_count: usize,
    wire_count: usize,
    face_count: usize,
    shell_count: usize,
    body_count: usize,
}

impl PrimitiveWitnessTopologySummary {
    pub fn new(
        vertex_count: usize,
        edge_count: usize,
        loop_count: usize,
        wire_count: usize,
        face_count: usize,
        shell_count: usize,
        body_count: usize,
    ) -> Self {
        Self {
            vertex_count,
            edge_count,
            loop_count,
            wire_count,
            face_count,
            shell_count,
            body_count,
        }
    }

    pub fn vertex_count(self) -> usize { self.vertex_count }
    pub fn edge_count(self) -> usize { self.edge_count }
    pub fn loop_count(self) -> usize { self.loop_count }
    pub fn wire_count(self) -> usize { self.wire_count }
    pub fn face_count(self) -> usize { self.face_count }
    pub fn shell_count(self) -> usize { self.shell_count }
    pub fn body_count(self) -> usize { self.body_count }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PrimitiveConstructionTopologyContract {
    vertex_count: usize,
    edge_count: usize,
    loop_count: usize,
    wire_count: usize,
    face_count: usize,
    shell_count: usize,
    body_count: usize,
}

impl PrimitiveConstructionTopologyContract {
    pub fn from_summary(summary: PrimitiveWitnessTopologySummary) -> Self {
        Self {
            vertex_count: summary.vertex_count(),
            edge_count: summary.edge_count(),
            loop_count: summary.loop_count(),
            wire_count: summary.wire_count(),
            face_count: summary.face_count(),
            shell_count: summary.shell_count(),
            body_count: summary.body_count(),
        }
    }

    pub fn vertex_count(self) -> usize { self.vertex_count }
    pub fn edge_count(self) -> usize { self.edge_count }
    pub fn loop_count(self) -> usize { self.loop_count }
    pub fn wire_count(self) -> usize { self.wire_count }
    pub fn face_count(self) -> usize { self.face_count }
    pub fn shell_count(self) -> usize { self.shell_count }
    pub fn body_count(self) -> usize { self.body_count }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PrimitiveWitnessSupportSummary {
    support_plane_count: usize,
}

impl PrimitiveWitnessSupportSummary {
    pub fn new(support_plane_count: usize) -> Self {
        Self { support_plane_count }
    }

    pub fn support_plane_count(self) -> usize {
        self.support_plane_count
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PrimitiveConstructionSupportContract {
    support_plane_count: usize,
}

impl PrimitiveConstructionSupportContract {
    pub fn from_summary(summary: PrimitiveWitnessSupportSummary) -> Self {
        Self {
            support_plane_count: summary.support_plane_count(),
        }
    }

    pub fn support_plane_count(self) -> usize {
        self.support_plane_count
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum PrimitiveWitnessDescriptor {
    SimplexSolid,
    Orthotope,
    RegularPrism { side_count: u32 },
    RegularPyramid { side_count: u32 },
    WireBody { edge_count: u32 },
    ShellWithHole {
        outer_loop_edge_count: u32,
        hole_loop_edge_counts: Vec<u32>,
    },
}

impl PrimitiveWitnessDescriptor {
    pub fn family(&self) -> PrimitiveConstructionFamilyKey {
        match self {
            Self::SimplexSolid => PrimitiveConstructionFamilyKey::SimplexSolid,
            Self::Orthotope => PrimitiveConstructionFamilyKey::Orthotope,
            Self::RegularPrism { .. } => PrimitiveConstructionFamilyKey::RegularPrism,
            Self::RegularPyramid { .. } => PrimitiveConstructionFamilyKey::RegularPyramid,
            Self::WireBody { .. } => PrimitiveConstructionFamilyKey::WireBody,
            Self::ShellWithHole { .. } => PrimitiveConstructionFamilyKey::ShellWithHole,
        }
    }

    pub fn topology_summary(&self) -> PrimitiveWitnessTopologySummary {
        match self {
            Self::SimplexSolid => PrimitiveWitnessTopologySummary::new(4, 6, 4, 0, 4, 1, 1),
            Self::Orthotope => PrimitiveWitnessTopologySummary::new(8, 12, 6, 0, 6, 1, 1),
            Self::RegularPrism { side_count } => {
                let sides = *side_count as usize;
                PrimitiveWitnessTopologySummary::new(sides * 2, sides * 3, sides + 2, 0, sides + 2, 1, 1)
            }
            Self::RegularPyramid { side_count } => {
                let sides = *side_count as usize;
                PrimitiveWitnessTopologySummary::new(sides + 1, sides * 2, sides + 1, 0, sides + 1, 1, 1)
            }
            Self::WireBody { edge_count } => {
                let edges = *edge_count as usize;
                PrimitiveWitnessTopologySummary::new(edges, edges, 1, 1, 0, 0, 1)
            }
            Self::ShellWithHole {
                outer_loop_edge_count,
                hole_loop_edge_counts,
            } => {
                let edge_count = *outer_loop_edge_count as usize
                    + hole_loop_edge_counts.iter().map(|count| *count as usize).sum::<usize>();
                PrimitiveWitnessTopologySummary::new(
                    edge_count,
                    edge_count,
                    1 + hole_loop_edge_counts.len(),
                    0,
                    1,
                    1,
                    1,
                )
            }
        }
    }

    pub fn support_summary(&self) -> PrimitiveWitnessSupportSummary {
        match self {
            Self::SimplexSolid => PrimitiveWitnessSupportSummary::new(4),
            Self::Orthotope => PrimitiveWitnessSupportSummary::new(6),
            Self::RegularPrism { side_count } => {
                PrimitiveWitnessSupportSummary::new(*side_count as usize + 2)
            }
            Self::RegularPyramid { side_count } => {
                PrimitiveWitnessSupportSummary::new(*side_count as usize + 1)
            }
            Self::WireBody { .. } | Self::ShellWithHole { .. } => {
                PrimitiveWitnessSupportSummary::new(1)
            }
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PrimitiveConstructionFamilyContract {
    family: PrimitiveConstructionFamilyKey,
    topology_birth_class: &'static str,
}

impl PrimitiveConstructionFamilyContract {
    pub fn family(self) -> PrimitiveConstructionFamilyKey { self.family }
    pub fn topology_birth_class(self) -> &'static str { self.topology_birth_class }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PrimitiveConstructionBirthSynopsisContract {
    family_contract: PrimitiveConstructionFamilyContract,
    topology_contract: PrimitiveConstructionTopologyContract,
    support_contract: PrimitiveConstructionSupportContract,
}

impl PrimitiveConstructionBirthSynopsisContract {
    pub fn family(self) -> PrimitiveConstructionFamilyKey {
        self.family_contract.family()
    }

    pub fn topology_birth_class(self) -> &'static str {
        self.family_contract.topology_birth_class()
    }

    pub fn topology_contract(self) -> PrimitiveConstructionTopologyContract {
        self.topology_contract
    }

    pub fn support_contract(self) -> PrimitiveConstructionSupportContract {
        self.support_contract
    }
}

pub struct PrimitiveConstructionFamilyContractRegistry;

impl PrimitiveConstructionFamilyContractRegistry {
    pub fn contract_for(descriptor: &PrimitiveWitnessDescriptor) -> PrimitiveConstructionBirthSynopsisContract {
        let topology_summary = descriptor.topology_summary();
        let support_summary = descriptor.support_summary();
        PrimitiveConstructionBirthSynopsisContract {
            family_contract: PrimitiveConstructionFamilyContract {
                family: descriptor.family(),
                topology_birth_class: descriptor.family().topology_birth_class(),
            },
            topology_contract: PrimitiveConstructionTopologyContract::from_summary(
                topology_summary,
            ),
            support_contract: PrimitiveConstructionSupportContract::from_summary(
                support_summary,
            ),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{
        PrimitiveConstructionFamilyContractRegistry, PrimitiveConstructionFamilyKey,
        PrimitiveWitnessDescriptor,
    };

    #[test]
    fn shell_with_hole_contract_is_derived_from_descriptor_not_manual_tuple() {
        let contract = PrimitiveConstructionFamilyContractRegistry::contract_for(
            &PrimitiveWitnessDescriptor::ShellWithHole {
                outer_loop_edge_count: 8,
                hole_loop_edge_counts: vec![3, 5],
            },
        );

        assert_eq!(contract.family(), PrimitiveConstructionFamilyKey::ShellWithHole);
        assert_eq!(contract.topology_birth_class(), "planar_shell_with_hole_body");
        assert_eq!(contract.topology_contract().vertex_count(), 16);
        assert_eq!(contract.topology_contract().loop_count(), 3);
        assert_eq!(contract.support_contract().support_plane_count(), 1);
    }

    #[test]
    fn regular_prism_contract_derives_support_and_face_counts_from_side_count() {
        let contract = PrimitiveConstructionFamilyContractRegistry::contract_for(
            &PrimitiveWitnessDescriptor::RegularPrism { side_count: 7 },
        );

        assert_eq!(contract.topology_contract().vertex_count(), 14);
        assert_eq!(contract.topology_contract().edge_count(), 21);
        assert_eq!(contract.topology_contract().face_count(), 9);
        assert_eq!(contract.support_contract().support_plane_count(), 9);
    }
}
