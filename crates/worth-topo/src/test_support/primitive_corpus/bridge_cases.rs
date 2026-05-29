use schema::facade::topology_authoring::MilestoneOnePrimitiveCase;

pub(crate) fn milestone_one_bridge_proof_cases() -> [MilestoneOnePrimitiveCase; 7] {
    [
        MilestoneOnePrimitiveCase::WireOpen { half_edge_count: 4 },
        MilestoneOnePrimitiveCase::WireClosed { half_edge_count: 4 },
        MilestoneOnePrimitiveCase::WireBranch { branch_count: 4 },
        MilestoneOnePrimitiveCase::SheetDisk { edge_count: 5 },
        MilestoneOnePrimitiveCase::SheetPatch { face_count: 4 },
        MilestoneOnePrimitiveCase::SolidShell { face_count: 6 },
        MilestoneOnePrimitiveCase::NmtEdgeFan { face_count: 4 },
    ]
}
