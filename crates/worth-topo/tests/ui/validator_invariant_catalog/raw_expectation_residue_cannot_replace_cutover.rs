use topology::facade::{
    WorthTopologyOperatorCertificationCutoverCloseout,
    WorthTopologyOperatorCertificationOldExpectationResidueReport,
    WorthTopologyOperatorCertificationOldExpectationResidueRow,
};

fn main() {
    let residue = WorthTopologyOperatorCertificationOldExpectationResidueReport::from_rows([
        WorthTopologyOperatorCertificationOldExpectationResidueRow::capped_comparison(
            "old.rs",
            "validator-expectation-array",
            "worth-topo",
            "comparison only",
            "delete old row",
        ),
    ]);
    let _closeout: WorthTopologyOperatorCertificationCutoverCloseout = residue;
}
