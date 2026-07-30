use super::UiIntentSchema;

/// Typed payloads declare their stable schema independently of Rust layout.
pub trait UiIntentPayload: Send + 'static {
    const SCHEMA: UiIntentSchema;
}
