use std::sync::Arc;

/// Provider-owned payload mechanics admitted into a Query-managed artifact.
///
/// Query never exposes this resource or downcasts it. The provider supplies
/// only the canonical semantic projection, retained-byte accounting, and the
/// destruction operation required by the installed artifact contract.
pub trait WorthQueryArtifactProviderResource: Send + 'static {
    const PROVIDER_FAMILY: &'static str;

    fn canonical_semantic_projection(&self) -> Vec<u8>;

    fn retained_bytes(&self) -> usize;

    fn dispose(self);
}

pub(crate) trait WorthQueryErasedArtifactProviderResource: Send {
    fn dispose(self: Box<Self>);
}

struct TypedArtifactProviderResource<R: WorthQueryArtifactProviderResource> {
    resource: Option<R>,
}

impl<R: WorthQueryArtifactProviderResource> WorthQueryErasedArtifactProviderResource
    for TypedArtifactProviderResource<R>
{
    fn dispose(mut self: Box<Self>) {
        self.resource
            .take()
            .expect("registered provider resource is disposed exactly once")
            .dispose();
    }
}

pub(crate) struct WorthQueryPreparedArtifactResource {
    pub(crate) provider_family: &'static str,
    pub(crate) semantic_projection: WorthQueryArtifactSemanticProjection,
    pub(crate) retained_bytes: usize,
    pub(crate) resource: Box<dyn WorthQueryErasedArtifactProviderResource>,
}

impl WorthQueryPreparedArtifactResource {
    pub(crate) fn prepare<R: WorthQueryArtifactProviderResource>(resource: R) -> Self {
        let semantic_projection =
            WorthQueryArtifactSemanticProjection::new(resource.canonical_semantic_projection());
        let retained_bytes = resource.retained_bytes();
        Self {
            provider_family: R::PROVIDER_FAMILY,
            semantic_projection,
            retained_bytes,
            resource: Box::new(TypedArtifactProviderResource {
                resource: Some(resource),
            }),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryArtifactSemanticProjection {
    bytes: Arc<[u8]>,
}

impl WorthQueryArtifactSemanticProjection {
    pub(crate) fn new(bytes: Vec<u8>) -> Self {
        Self {
            bytes: Arc::from(bytes),
        }
    }

    pub fn bytes(&self) -> &[u8] {
        &self.bytes
    }

    pub(crate) fn canonical_identity(&self) -> String {
        crate::identity::hash_parts(&[
            "worth_query_artifact_semantic_projection_v1".into(),
            format!("bytes:{}", encode_hex(&self.bytes)),
        ])
    }
}

fn encode_hex(bytes: &[u8]) -> String {
    const DIGITS: &[u8; 16] = b"0123456789abcdef";
    let mut encoded = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        encoded.push(DIGITS[(byte >> 4) as usize] as char);
        encoded.push(DIGITS[(byte & 0x0f) as usize] as char);
    }
    encoded
}
