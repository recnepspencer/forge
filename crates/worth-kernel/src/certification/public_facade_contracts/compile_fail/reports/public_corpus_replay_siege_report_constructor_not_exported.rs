use worth_kernel::facade::{
    PrimitiveConstructionCorpusAuthoringOrderRow, PrimitiveConstructionCorpusRejectionWitnessRow,
    PrimitiveConstructionCorpusReplaySiegeReport,
};

fn main() {
    let _ = PrimitiveConstructionCorpusReplaySiegeReport::new(
        vec![],
        0,
        0,
        Vec::<PrimitiveConstructionCorpusAuthoringOrderRow>::new(),
        Vec::<PrimitiveConstructionCorpusRejectionWitnessRow>::new(),
    );
}
