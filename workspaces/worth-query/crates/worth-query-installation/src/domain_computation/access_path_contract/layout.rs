use worth_foundational::facade::AspectContract;

use super::WorthQueryArtifactFieldSlicePosture;

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct WorthQueryArtifactNativeLayoutIdentity(String);

impl WorthQueryArtifactNativeLayoutIdentity {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct WorthQueryArtifactNativeLayoutVersion(u32);

impl WorthQueryArtifactNativeLayoutVersion {
    pub const fn new(value: u32) -> Self {
        Self(value)
    }

    pub const fn get(self) -> u32 {
        self.0
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthQueryArtifactNativeAlignment(usize);

impl WorthQueryArtifactNativeAlignment {
    pub const fn new(bytes: usize) -> Self {
        Self(bytes)
    }

    pub const fn bytes(self) -> usize {
        self.0
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryArtifactNativeFieldContract {
    aspect: AspectContract,
    field_slice: WorthQueryArtifactFieldSlicePosture,
}

impl WorthQueryArtifactNativeFieldContract {
    pub fn new(aspect: AspectContract, field_slice: WorthQueryArtifactFieldSlicePosture) -> Self {
        Self {
            aspect,
            field_slice,
        }
    }

    pub fn aspect(&self) -> &AspectContract {
        &self.aspect
    }

    pub const fn field_slice(&self) -> WorthQueryArtifactFieldSlicePosture {
        self.field_slice
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryArtifactNativeLayoutContract {
    identity: WorthQueryArtifactNativeLayoutIdentity,
    version: WorthQueryArtifactNativeLayoutVersion,
    alignment: WorthQueryArtifactNativeAlignment,
    fields: Vec<WorthQueryArtifactNativeFieldContract>,
}

impl WorthQueryArtifactNativeLayoutContract {
    pub fn new(
        identity: WorthQueryArtifactNativeLayoutIdentity,
        version: WorthQueryArtifactNativeLayoutVersion,
        alignment: WorthQueryArtifactNativeAlignment,
        fields: impl IntoIterator<Item = WorthQueryArtifactNativeFieldContract>,
    ) -> Self {
        Self {
            identity,
            version,
            alignment,
            fields: fields.into_iter().collect(),
        }
    }

    pub fn identity(&self) -> &WorthQueryArtifactNativeLayoutIdentity {
        &self.identity
    }

    pub const fn version(&self) -> WorthQueryArtifactNativeLayoutVersion {
        self.version
    }

    pub const fn alignment(&self) -> WorthQueryArtifactNativeAlignment {
        self.alignment
    }

    pub fn fields(&self) -> &[WorthQueryArtifactNativeFieldContract] {
        &self.fields
    }

    pub fn reference(&self) -> WorthQueryArtifactNativeLayoutReference {
        WorthQueryArtifactNativeLayoutReference {
            identity: self.identity.clone(),
            version: self.version,
            alignment: self.alignment,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryArtifactNativeLayoutReference {
    identity: WorthQueryArtifactNativeLayoutIdentity,
    version: WorthQueryArtifactNativeLayoutVersion,
    alignment: WorthQueryArtifactNativeAlignment,
}

impl WorthQueryArtifactNativeLayoutReference {
    pub fn identity(&self) -> &WorthQueryArtifactNativeLayoutIdentity {
        &self.identity
    }

    pub const fn version(&self) -> WorthQueryArtifactNativeLayoutVersion {
        self.version
    }

    pub const fn alignment(&self) -> WorthQueryArtifactNativeAlignment {
        self.alignment
    }
}
