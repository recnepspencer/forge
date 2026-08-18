use super::handles::SupplyChainSemanticHandles;
use super::program::CompiledSupplyChainProgram;
use worth_relational::facade::history::RelationalCommitReceipt;
use worth_relational::facade::runtime::RelationalInitialSchemaInstallationReceipt;
use worth_relational::facade::runtime::RelationalRuntime;
use worth_relational::facade::transactions::CommitResult;

pub(crate) struct ProductionSeededSupplyChainWorld {
    pub(crate) runtime: RelationalRuntime,
    pub(crate) program: CompiledSupplyChainProgram,
    pub(crate) handles: SupplyChainSemanticHandles,
    pub(crate) commit: RelationalCommitReceipt,
    pub(crate) commit_result: CommitResult,
    pub(crate) schema_receipt: RelationalInitialSchemaInstallationReceipt,
}
