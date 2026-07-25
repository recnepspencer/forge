use super::{require_text, PhysicalWorkRunProvenanceDenial};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PhysicalWorkPlatformEvidence {
    operating_system: Box<str>,
    architecture: Box<str>,
    family: Box<str>,
    pointer_width: u8,
    endian: Box<str>,
}

impl PhysicalWorkPlatformEvidence {
    pub fn current() -> Self {
        Self {
            operating_system: std::env::consts::OS.into(),
            architecture: std::env::consts::ARCH.into(),
            family: std::env::consts::FAMILY.into(),
            pointer_width: usize::BITS as u8,
            endian: if cfg!(target_endian = "little") {
                "little".into()
            } else {
                "big".into()
            },
        }
    }

    pub fn new(
        operating_system: impl Into<Box<str>>,
        architecture: impl Into<Box<str>>,
        family: impl Into<Box<str>>,
        pointer_width: u8,
        endian: impl Into<Box<str>>,
    ) -> Result<Self, PhysicalWorkRunProvenanceDenial> {
        let evidence = Self {
            operating_system: operating_system.into(),
            architecture: architecture.into(),
            family: family.into(),
            pointer_width,
            endian: endian.into(),
        };
        for field in [
            evidence.operating_system.as_ref(),
            evidence.architecture.as_ref(),
            evidence.family.as_ref(),
            evidence.endian.as_ref(),
        ] {
            require_text(field, PhysicalWorkRunProvenanceDenial::EmptyPlatformField)?;
        }
        Ok(evidence)
    }

    pub fn operating_system(&self) -> &str {
        &self.operating_system
    }

    pub fn architecture(&self) -> &str {
        &self.architecture
    }

    pub fn family(&self) -> &str {
        &self.family
    }

    pub const fn pointer_width(&self) -> u8 {
        self.pointer_width
    }

    pub fn endian(&self) -> &str {
        &self.endian
    }
}
