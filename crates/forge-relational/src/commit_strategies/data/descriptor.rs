use serde::{de::Error as DeError, Deserialize, Deserializer, Serialize};

use super::canonical_digest::commit_strategy_descriptor_digest;
use super::strategy_id::CommitStrategyId;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
pub struct CommitStrategySemanticName(String);

impl CommitStrategySemanticName {
    pub fn new(name: impl Into<String>) -> Self {
        Self(name.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl<'de> Deserialize<'de> for CommitStrategySemanticName {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserialize_non_empty_name(deserializer, "commit strategy semantic name")
            .map(CommitStrategySemanticName)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
pub struct CommitStrategyFamilyName(String);

impl CommitStrategyFamilyName {
    pub fn new(name: impl Into<String>) -> Self {
        Self(name.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl<'de> Deserialize<'de> for CommitStrategyFamilyName {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserialize_non_empty_name(deserializer, "commit strategy family name")
            .map(CommitStrategyFamilyName)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
pub struct StrategyInputSchemaName(String);

impl StrategyInputSchemaName {
    pub fn new(name: impl Into<String>) -> Self {
        Self(name.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl<'de> Deserialize<'de> for StrategyInputSchemaName {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserialize_non_empty_name(deserializer, "strategy input schema name")
            .map(StrategyInputSchemaName)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct StrategyInputSchemaVersion(pub u16);

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
pub struct StrategyOutputSchemaName(String);

impl StrategyOutputSchemaName {
    pub fn new(name: impl Into<String>) -> Self {
        Self(name.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl<'de> Deserialize<'de> for StrategyOutputSchemaName {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserialize_non_empty_name(deserializer, "strategy output schema name")
            .map(StrategyOutputSchemaName)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
pub struct StrategyIntentName(String);

impl StrategyIntentName {
    pub fn new(name: impl Into<String>) -> Self {
        Self(name.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl<'de> Deserialize<'de> for StrategyIntentName {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserialize_non_empty_name(deserializer, "strategy intent name").map(StrategyIntentName)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
pub struct PersistentArtifactName(String);

impl PersistentArtifactName {
    pub fn new(name: impl Into<String>) -> Self {
        Self(name.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl<'de> Deserialize<'de> for PersistentArtifactName {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserialize_non_empty_name(deserializer, "persistent artifact name")
            .map(PersistentArtifactName)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct CommitStrategyVersion {
    pub major: u16,
    pub minor: u16,
}

impl CommitStrategyVersion {
    pub const fn new(major: u16, minor: u16) -> Self {
        Self { major, minor }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum StrategyRequestCanonicalization {
    NativeCanonicalBytesV1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum StrategyReadScopeClass {
    ExplicitTargetsOnly,
    KindBoundedScan,
    PartitionBoundedScan,
    BoundedNeighborhood,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum StrategyReadLocalityClass {
    SinglePartition,
    PartitionBounded,
    CrossPartitionBounded,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum StrategyTraversalBasis {
    NoTraversal,
    AdjacencyBounded { max_depth: u16 },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum StrategyPacketContract {
    ProjectionOnly,
    PlannedPacketOnly,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum StrategyReadCostClass {
    ORequestedSurface,
    OPartitionBoundedSurface,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StrategyReadContract {
    pub scope_class: StrategyReadScopeClass,
    pub locality_class: StrategyReadLocalityClass,
    pub traversal_basis: StrategyTraversalBasis,
    pub packet_contract: StrategyPacketContract,
    pub cost_class: StrategyReadCostClass,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct CommitStrategyDescriptorDigest(pub [u8; 32]);

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct CommitStrategyDescriptor {
    id: CommitStrategyId,
    semantic_name: CommitStrategySemanticName,
    family_name: CommitStrategyFamilyName,
    version: CommitStrategyVersion,
    intent_name: StrategyIntentName,
    input_schema_name: StrategyInputSchemaName,
    input_schema_version: StrategyInputSchemaVersion,
    output_schema_name: StrategyOutputSchemaName,
    request_canonicalization: StrategyRequestCanonicalization,
    read_contract: StrategyReadContract,
    artifact_name: PersistentArtifactName,
    digest: CommitStrategyDescriptorDigest,
}

impl CommitStrategyDescriptor {
    pub fn new(
        id: CommitStrategyId,
        semantic_name: CommitStrategySemanticName,
        family_name: CommitStrategyFamilyName,
        version: CommitStrategyVersion,
        intent_name: StrategyIntentName,
        input_schema_name: StrategyInputSchemaName,
        input_schema_version: StrategyInputSchemaVersion,
        output_schema_name: StrategyOutputSchemaName,
        request_canonicalization: StrategyRequestCanonicalization,
        read_contract: StrategyReadContract,
        artifact_name: PersistentArtifactName,
    ) -> Self {
        let digest = commit_strategy_descriptor_digest(
            id,
            &semantic_name,
            &family_name,
            version,
            &intent_name,
            &input_schema_name,
            input_schema_version,
            &output_schema_name,
            request_canonicalization,
            &read_contract,
            &artifact_name,
        );
        Self {
            id,
            semantic_name,
            family_name,
            version,
            intent_name,
            input_schema_name,
            input_schema_version,
            output_schema_name,
            request_canonicalization,
            read_contract,
            artifact_name,
            digest,
        }
    }

    pub fn id(&self) -> CommitStrategyId {
        self.id
    }

    pub fn semantic_name(&self) -> &CommitStrategySemanticName {
        &self.semantic_name
    }

    pub fn family_name(&self) -> &CommitStrategyFamilyName {
        &self.family_name
    }

    pub fn version(&self) -> CommitStrategyVersion {
        self.version
    }

    pub fn intent_name(&self) -> &StrategyIntentName {
        &self.intent_name
    }

    pub fn input_schema_name(&self) -> &StrategyInputSchemaName {
        &self.input_schema_name
    }

    pub fn input_schema_version(&self) -> StrategyInputSchemaVersion {
        self.input_schema_version
    }

    pub fn output_schema_name(&self) -> &StrategyOutputSchemaName {
        &self.output_schema_name
    }

    pub fn request_canonicalization(&self) -> StrategyRequestCanonicalization {
        self.request_canonicalization
    }

    pub fn read_contract(&self) -> &StrategyReadContract {
        &self.read_contract
    }

    pub fn artifact_name(&self) -> &PersistentArtifactName {
        &self.artifact_name
    }

    pub fn digest(&self) -> CommitStrategyDescriptorDigest {
        self.digest
    }
}

impl<'de> Deserialize<'de> for CommitStrategyDescriptor {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct RawCommitStrategyDescriptor {
            id: CommitStrategyId,
            semantic_name: CommitStrategySemanticName,
            family_name: CommitStrategyFamilyName,
            version: CommitStrategyVersion,
            intent_name: StrategyIntentName,
            input_schema_name: StrategyInputSchemaName,
            input_schema_version: StrategyInputSchemaVersion,
            output_schema_name: StrategyOutputSchemaName,
            request_canonicalization: StrategyRequestCanonicalization,
            read_contract: StrategyReadContract,
            artifact_name: PersistentArtifactName,
            digest: CommitStrategyDescriptorDigest,
        }

        let raw = RawCommitStrategyDescriptor::deserialize(deserializer)?;
        let expected = commit_strategy_descriptor_digest(
            raw.id,
            &raw.semantic_name,
            &raw.family_name,
            raw.version,
            &raw.intent_name,
            &raw.input_schema_name,
            raw.input_schema_version,
            &raw.output_schema_name,
            raw.request_canonicalization,
            &raw.read_contract,
            &raw.artifact_name,
        );
        if raw.digest != expected {
            return Err(D::Error::custom(
                "commit strategy descriptor digest does not match descriptor contents",
            ));
        }
        Ok(Self {
            id: raw.id,
            semantic_name: raw.semantic_name,
            family_name: raw.family_name,
            version: raw.version,
            intent_name: raw.intent_name,
            input_schema_name: raw.input_schema_name,
            input_schema_version: raw.input_schema_version,
            output_schema_name: raw.output_schema_name,
            request_canonicalization: raw.request_canonicalization,
            read_contract: raw.read_contract,
            artifact_name: raw.artifact_name,
            digest: raw.digest,
        })
    }
}

fn deserialize_non_empty_name<'de, D>(
    deserializer: D,
    label: &'static str,
) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    let value = String::deserialize(deserializer)?;
    if value.trim().is_empty() {
        return Err(D::Error::custom(format!("{label} must not be empty")));
    }
    Ok(value)
}

#[cfg(test)]
mod tests {
    use super::{
        CommitStrategyDescriptor, CommitStrategyFamilyName, CommitStrategyId,
        CommitStrategySemanticName, CommitStrategyVersion, PersistentArtifactName,
        StrategyInputSchemaName, StrategyInputSchemaVersion, StrategyIntentName,
        StrategyOutputSchemaName, StrategyPacketContract, StrategyReadContract,
        StrategyReadCostClass, StrategyReadLocalityClass, StrategyReadScopeClass,
        StrategyRequestCanonicalization, StrategyTraversalBasis,
    };

    fn descriptor() -> CommitStrategyDescriptor {
        CommitStrategyDescriptor::new(
            CommitStrategyId(11),
            CommitStrategySemanticName::new("strategy.intent.reconcile"),
            CommitStrategyFamilyName::new("strategy.intent"),
            CommitStrategyVersion::new(1, 0),
            StrategyIntentName::new("reconcile.desired.state"),
            StrategyInputSchemaName::new("intent.input.v1"),
            StrategyInputSchemaVersion(1),
            StrategyOutputSchemaName::new("intent.output.v1"),
            StrategyRequestCanonicalization::NativeCanonicalBytesV1,
            StrategyReadContract {
                scope_class: StrategyReadScopeClass::ExplicitTargetsOnly,
                locality_class: StrategyReadLocalityClass::SinglePartition,
                traversal_basis: StrategyTraversalBasis::NoTraversal,
                packet_contract: StrategyPacketContract::ProjectionOnly,
                cost_class: StrategyReadCostClass::ORequestedSurface,
            },
            PersistentArtifactName::new("strategy.intent.reconcile"),
        )
    }

    #[test]
    fn descriptor_roundtrip_preserves_verified_digest() {
        let descriptor = descriptor();
        let bytes = serde_json::to_vec(&descriptor).expect("serialize descriptor");
        let roundtripped: CommitStrategyDescriptor =
            serde_json::from_slice(&bytes).expect("deserialize descriptor");

        assert_eq!(roundtripped.digest(), descriptor.digest());
        assert_eq!(
            roundtripped.request_canonicalization(),
            StrategyRequestCanonicalization::NativeCanonicalBytesV1
        );
    }

    #[test]
    fn descriptor_deserialization_rejects_forged_digest() {
        let mut value = serde_json::to_value(descriptor()).expect("descriptor value");
        value["semantic_name"] = serde_json::Value::String("strategy.intent.tampered".to_string());

        let error = serde_json::from_value::<CommitStrategyDescriptor>(value).unwrap_err();
        assert!(error
            .to_string()
            .contains("digest does not match descriptor contents"));
    }
}
