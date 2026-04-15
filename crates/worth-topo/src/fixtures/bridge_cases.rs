use worth_schema::facade::WorthMilestoneOnePrimitiveCase;

pub(crate) fn milestone_one_bridge_proof_cases() -> [WorthMilestoneOnePrimitiveCase; 7] {
    [
        WorthMilestoneOnePrimitiveCase::WireOpen { half_edge_count: 4 },
        WorthMilestoneOnePrimitiveCase::WireClosed { half_edge_count: 4 },
        WorthMilestoneOnePrimitiveCase::WireBranch { branch_count: 4 },
        WorthMilestoneOnePrimitiveCase::SheetDisk { edge_count: 5 },
        WorthMilestoneOnePrimitiveCase::SheetPatch { face_count: 4 },
        WorthMilestoneOnePrimitiveCase::SolidShell { face_count: 6 },
        WorthMilestoneOnePrimitiveCase::NmtEdgeFan { face_count: 4 },
    ]
}
