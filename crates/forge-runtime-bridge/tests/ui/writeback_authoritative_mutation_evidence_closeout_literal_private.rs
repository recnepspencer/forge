use forge_runtime_bridge::facade::BridgeAuthoritativeMutationEvidenceCloseout;

fn main() {
    let _ = BridgeAuthoritativeMutationEvidenceCloseout {
        support_digest: String::new(),
        safe_to_build_now: Vec::new(),
        must_not_assume_yet: Vec::new(),
        required_verification_commands: Vec::new(),
        closeout_digest: String::new(),
    };
}
