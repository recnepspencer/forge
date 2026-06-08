use super::ForgeServerBindAddress;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeServerConfig {
    bind_address: ForgeServerBindAddress,
}

impl ForgeServerConfig {
    pub fn builder() -> ForgeServerConfigBuilder {
        ForgeServerConfigBuilder::default()
    }

    pub fn bind_address(&self) -> ForgeServerBindAddress {
        self.bind_address
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct ForgeServerConfigBuilder {
    bind_address: Option<ForgeServerBindAddress>,
}

impl ForgeServerConfigBuilder {
    pub fn with_bind_address(mut self, bind_address: ForgeServerBindAddress) -> Self {
        self.bind_address = Some(bind_address);
        self
    }

    pub fn build(self) -> Result<ForgeServerConfig, ForgeServerConfigError> {
        let bind_address = self
            .bind_address
            .ok_or(ForgeServerConfigError::MissingBindAddress)?;
        Ok(ForgeServerConfig { bind_address })
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ForgeServerConfigError {
    MissingBindAddress,
}
