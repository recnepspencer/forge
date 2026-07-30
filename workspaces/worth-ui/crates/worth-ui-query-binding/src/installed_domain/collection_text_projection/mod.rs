mod definition;
mod executor;
mod operation;

pub(crate) use definition::collection_text_projection_definition;
pub(crate) use executor::WorthUiCollectionTextProjectionExecutor;
#[cfg(any(test, feature = "certification-construction"))]
pub(crate) use executor::WorthUiPartialCollectionTextProjectionExecutor;
pub use operation::{WorthUiCollectionTextProjection, WorthUiCollectionTextProjectionFamily};

pub(crate) const LOWERING_FAMILY: &str = "worth-ui-collection-text-projection-v1";
