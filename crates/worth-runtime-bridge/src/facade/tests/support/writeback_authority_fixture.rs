use crate::adapter::{
    TruthWritebackAuthority, TruthWritebackAuthorityError, TruthWritebackRequest,
};
use crate::facade::BridgeWritebackOutcomeClass;

#[derive(Clone)]
pub(in crate::facade::tests) struct StaticWritebackAuthority;

impl TruthWritebackAuthority for StaticWritebackAuthority {
    fn execute_writeback(
        &self,
        request: TruthWritebackRequest,
    ) -> Result<crate::adapter::TruthWritebackReceipt, TruthWritebackAuthorityError> {
        Ok(crate::adapter::TruthWritebackReceipt::new(
            BridgeWritebackOutcomeClass::AuthoritativeCommit,
            &request,
        ))
    }
}
