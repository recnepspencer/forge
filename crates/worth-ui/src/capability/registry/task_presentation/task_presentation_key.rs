use super::TaskPresentationDescriptor;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TaskPresentationKey {
    projection_basis: String,
}

impl TaskPresentationKey {
    pub(crate) fn from_descriptor(descriptor: &TaskPresentationDescriptor) -> Self {
        Self::new(format!(
            "{}|{}|{}|{}|{}|{}|{}",
            length_prefixed(descriptor.id().as_str()),
            descriptor.family().digest_basis(),
            descriptor
                .lifecycle_posture()
                .map(|posture| posture.digest_basis())
                .unwrap_or("none"),
            descriptor
                .cancellation_posture()
                .map(|posture| posture.digest_basis())
                .unwrap_or("none"),
            descriptor
                .failure_posture()
                .map(|posture| posture.digest_basis())
                .unwrap_or("none"),
            descriptor
                .projection_eligibility()
                .map(|eligibility| eligibility.digest_basis())
                .unwrap_or("none"),
            descriptor
                .runtime_authority_posture()
                .map(|posture| posture.digest_basis())
                .unwrap_or("none")
        ))
    }

    fn new(projection_basis: impl Into<String>) -> Self {
        Self {
            projection_basis: projection_basis.into(),
        }
    }

    pub fn projection_basis(&self) -> &str {
        &self.projection_basis
    }
}

fn length_prefixed(value: &str) -> String {
    format!("{}:{value}", value.len())
}
