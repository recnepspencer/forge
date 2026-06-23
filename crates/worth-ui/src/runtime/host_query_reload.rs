use crate::runtime::{
    WorthUiAdmittedRuntimeChangeEvidence, WorthUiClassifiedRuntimeChange,
    WorthUiQueryRuntimeFactLoweringInput, WorthUiQueryRuntimeFactLoweringReceipt,
    WorthUiRuntimeChangeAdmissionDenial, WorthUiRuntimeHost, WorthUiRuntimeInstanceWitness,
};

impl WorthUiRuntimeHost {
    pub fn admit_query_runtime_fact_lowering(
        &self,
        input: WorthUiQueryRuntimeFactLoweringInput,
    ) -> WorthUiQueryRuntimeFactLoweringReceipt {
        WorthUiQueryRuntimeFactLoweringReceipt::lower(input)
    }

    pub fn admit_query_runtime_change(
        &self,
        receipt: &WorthUiQueryRuntimeFactLoweringReceipt,
    ) -> Result<WorthUiAdmittedRuntimeChangeEvidence, WorthUiRuntimeChangeAdmissionDenial> {
        let runtime_instance = WorthUiRuntimeInstanceWitness::from_raw(self.instance_id().raw());
        let classified =
            WorthUiClassifiedRuntimeChange::from_query_lowering_receipt(runtime_instance, receipt);
        WorthUiAdmittedRuntimeChangeEvidence::admit(classified, runtime_instance)
    }
}
