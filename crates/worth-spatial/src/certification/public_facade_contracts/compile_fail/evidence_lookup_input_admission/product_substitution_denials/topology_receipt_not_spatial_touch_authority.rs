use topology::facade::DerivedInvalidationMilestoneElevenProductReceiptRef;
use worth_spatial::facade::evidence_lookup_input_admission::EvidenceLookupInputAdmissionRequest;

fn main() {
    fn needs_spatial_touch_authority(receipt: &DerivedInvalidationMilestoneElevenProductReceiptRef) {
        let _ = EvidenceLookupInputAdmissionRequest::from_spatial_touch_authority(receipt);
    }
}
