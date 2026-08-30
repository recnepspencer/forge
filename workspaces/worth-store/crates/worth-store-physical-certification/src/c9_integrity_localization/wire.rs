use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub(crate) enum RootWireRole {
    Producer,
    ArtifactEditor,
    Recovery,
    OfflineVerifier,
    ParentOracle,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub(crate) struct RootWireIdentity {
    protocol: String,
    version: u16,
    role: RootWireRole,
    scenario_identity: [u8; 32],
    run_identity: [u8; 32],
    identity: [u8; 32],
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum RootWireDenial {
    MissingScenarioIdentity,
    MissingRunIdentity,
    IdentityEncoding,
    ProtocolSubstitution,
    RoleSubstitution,
    ScenarioSubstitution,
    RunSubstitution,
    IdentitySubstitution,
}

const ROOT_PROTOCOL: &str = "store.physical.c9-root-localization";
const ROOT_PROTOCOL_VERSION: u16 = 1;

impl RootWireIdentity {
    pub(crate) fn bind(
        role: RootWireRole,
        scenario_identity: [u8; 32],
        run_identity: [u8; 32],
    ) -> Result<Self, RootWireDenial> {
        if scenario_identity == [0; 32] {
            return Err(RootWireDenial::MissingScenarioIdentity);
        }
        if run_identity == [0; 32] {
            return Err(RootWireDenial::MissingRunIdentity);
        }
        let identity = digest(&(
            ROOT_PROTOCOL,
            ROOT_PROTOCOL_VERSION,
            role,
            scenario_identity,
            run_identity,
        ))?;
        Ok(Self {
            protocol: ROOT_PROTOCOL.to_owned(),
            version: ROOT_PROTOCOL_VERSION,
            role,
            scenario_identity,
            run_identity,
            identity,
        })
    }

    pub(crate) fn require_binding(
        &self,
        expected_role: RootWireRole,
        expected_scenario: [u8; 32],
        expected_run: [u8; 32],
    ) -> Result<(), RootWireDenial> {
        if self.protocol != ROOT_PROTOCOL || self.version != ROOT_PROTOCOL_VERSION {
            return Err(RootWireDenial::ProtocolSubstitution);
        }
        if self.role != expected_role {
            return Err(RootWireDenial::RoleSubstitution);
        }
        if self.scenario_identity != expected_scenario {
            return Err(RootWireDenial::ScenarioSubstitution);
        }
        if self.run_identity != expected_run {
            return Err(RootWireDenial::RunSubstitution);
        }
        let expected_identity = digest(&(
            &self.protocol,
            self.version,
            self.role,
            self.scenario_identity,
            self.run_identity,
        ))?;
        if self.identity != expected_identity {
            return Err(RootWireDenial::IdentitySubstitution);
        }
        Ok(())
    }

    pub(crate) fn protocol(&self) -> &str {
        &self.protocol
    }
    pub(crate) const fn version(&self) -> u16 {
        self.version
    }
    pub(crate) const fn role(&self) -> RootWireRole {
        self.role
    }
    pub(crate) const fn scenario_identity(&self) -> [u8; 32] {
        self.scenario_identity
    }
    pub(crate) const fn run_identity(&self) -> [u8; 32] {
        self.run_identity
    }
    pub(crate) const fn identity(&self) -> [u8; 32] {
        self.identity
    }
}

fn digest(value: &impl Serialize) -> Result<[u8; 32], RootWireDenial> {
    bincode::serialize(value)
        .map(|bytes| Sha256::digest(bytes).into())
        .map_err(|_| RootWireDenial::IdentityEncoding)
}
