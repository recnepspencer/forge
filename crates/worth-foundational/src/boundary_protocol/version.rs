use serde::{Deserialize, Deserializer, Serialize};

/// Exact version produced by one boundary artifact.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(transparent)]
pub struct BoundaryProtocolVersion(u32);

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BoundaryProtocolVersionDenial {
    Zero,
}

impl BoundaryProtocolVersion {
    /// Declares one positive protocol version.
    ///
    /// # Panics
    ///
    /// Panics when `value` is zero.
    ///
    /// ```compile_fail
    /// use worth_foundational::facade::BoundaryProtocolVersion;
    ///
    /// const INVALID: BoundaryProtocolVersion = BoundaryProtocolVersion::new(0);
    /// ```
    pub const fn new(value: u32) -> Self {
        if value == 0 {
            panic!("boundary protocol version must be positive");
        }
        Self(value)
    }

    pub const fn try_new(value: u32) -> Result<Self, BoundaryProtocolVersionDenial> {
        if value == 0 {
            return Err(BoundaryProtocolVersionDenial::Zero);
        }
        Ok(Self(value))
    }

    pub const fn get(self) -> u32 {
        self.0
    }
}

impl std::fmt::Display for BoundaryProtocolVersion {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(formatter)
    }
}

impl<'de> Deserialize<'de> for BoundaryProtocolVersion {
    fn deserialize<DeserializerType>(
        deserializer: DeserializerType,
    ) -> Result<Self, DeserializerType::Error>
    where
        DeserializerType: Deserializer<'de>,
    {
        let value = u32::deserialize(deserializer)?;
        Self::try_new(value).map_err(|BoundaryProtocolVersionDenial::Zero| {
            serde::de::Error::custom("boundary protocol version must be positive")
        })
    }
}

#[cfg(test)]
mod tests {
    use super::BoundaryProtocolVersion;

    #[test]
    fn zero_cannot_enter_through_json() {
        assert!(serde_json::from_str::<BoundaryProtocolVersion>("0").is_err());
        assert_eq!(
            serde_json::from_str::<BoundaryProtocolVersion>("2")
                .unwrap()
                .get(),
            2
        );
    }
}
