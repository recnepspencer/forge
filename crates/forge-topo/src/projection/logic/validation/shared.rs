use forge_core::KernelError;

pub(crate) fn vf(validator: &str, detail: String) -> KernelError {
    KernelError::TopologyViolation {
        err: forge_core::TopologyError::ValidatorFailure {
            validator: validator.to_string(),
            detail,
        },
        context: None,
    }
}
