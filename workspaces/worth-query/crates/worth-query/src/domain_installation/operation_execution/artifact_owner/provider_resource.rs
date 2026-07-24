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
    fn canonical_semantic_projection(&self) -> Vec<u8>;

    fn retained_bytes(&self) -> usize;

    fn dispose(self: Box<Self>);
}

struct TypedArtifactProviderResource<R: WorthQueryArtifactProviderResource> {
    resource: Option<R>,
}

impl<R: WorthQueryArtifactProviderResource> WorthQueryErasedArtifactProviderResource
    for TypedArtifactProviderResource<R>
{
    fn canonical_semantic_projection(&self) -> Vec<u8> {
        self.resource
            .as_ref()
            .expect("prepared provider resource remains present")
            .canonical_semantic_projection()
    }

    fn retained_bytes(&self) -> usize {
        self.resource
            .as_ref()
            .expect("prepared provider resource remains present")
            .retained_bytes()
    }

    fn dispose(mut self: Box<Self>) {
        self.resource
            .take()
            .expect("registered provider resource is disposed exactly once")
            .dispose();
    }
}

impl<R: WorthQueryArtifactProviderResource> Drop for TypedArtifactProviderResource<R> {
    fn drop(&mut self) {
        if let Some(resource) = self.resource.take() {
            resource.dispose();
        }
    }
}

pub(crate) struct WorthQueryGuardedArtifactResource {
    pub(crate) provider_family: &'static str,
    resource: Option<Box<dyn WorthQueryErasedArtifactProviderResource>>,
}

impl WorthQueryGuardedArtifactResource {
    pub(crate) fn new<R: WorthQueryArtifactProviderResource>(resource: R) -> Self {
        let resource: Box<dyn WorthQueryErasedArtifactProviderResource> =
            Box::new(TypedArtifactProviderResource {
                resource: Some(resource),
            });
        Self {
            provider_family: R::PROVIDER_FAMILY,
            resource: Some(resource),
        }
    }

    pub(crate) fn prepare(mut self) -> WorthQueryPreparedArtifactResource {
        let resource = self
            .resource
            .take()
            .expect("guarded artifact retains exactly one provider resource");
        let semantic_projection =
            WorthQueryArtifactSemanticProjection::new(resource.canonical_semantic_projection());
        let retained_bytes = resource.retained_bytes();
        WorthQueryPreparedArtifactResource {
            provider_family: self.provider_family,
            semantic_projection: Some(semantic_projection),
            retained_bytes,
            resource: Some(resource),
        }
    }
}

impl Drop for WorthQueryGuardedArtifactResource {
    fn drop(&mut self) {
        if let Some(resource) = self.resource.take() {
            resource.dispose();
        }
    }
}

pub(crate) struct WorthQueryPreparedArtifactResource {
    pub(crate) provider_family: &'static str,
    semantic_projection: Option<WorthQueryArtifactSemanticProjection>,
    pub(crate) retained_bytes: usize,
    resource: Option<Box<dyn WorthQueryErasedArtifactProviderResource>>,
}

impl WorthQueryPreparedArtifactResource {
    pub(crate) fn semantic_projection(&self) -> &WorthQueryArtifactSemanticProjection {
        self.semantic_projection
            .as_ref()
            .expect("prepared artifact retains its semantic projection")
    }

    pub(crate) fn into_owner_parts(
        mut self,
    ) -> (
        WorthQueryArtifactSemanticProjection,
        usize,
        Box<dyn WorthQueryErasedArtifactProviderResource>,
    ) {
        (
            self.semantic_projection
                .take()
                .expect("prepared artifact transfers one semantic projection"),
            self.retained_bytes,
            self.resource
                .take()
                .expect("prepared artifact transfers one provider resource"),
        )
    }
}

impl Drop for WorthQueryPreparedArtifactResource {
    fn drop(&mut self) {
        if let Some(resource) = self.resource.take() {
            resource.dispose();
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
