use topology::facade::{
    WorthTopologyRelationalInvariantOldPackResidueReport,
    WorthTopologyRelationalInvariantOldPackResidueRow,
    WorthTopologyRelationalInvariantOldPackResidueStatus,
};

fn main() {
    let _row = WorthTopologyRelationalInvariantOldPackResidueRow {
        source_path: String::new(),
        status: WorthTopologyRelationalInvariantOldPackResidueStatus::DeletedOrdinaryPath,
        owner: String::new(),
        blocker: String::new(),
        removal_trigger: String::new(),
        ordinary_path_count: 1,
        registration_count: 0,
        row_digest: String::new(),
    };
    let _report = WorthTopologyRelationalInvariantOldPackResidueReport {
        rows: Vec::new(),
        source_pack_registration_count: 0,
        ordinary_path_count: 1,
        report_digest: String::new(),
    };
}
