use crate::PhysicalIntegrityCloseoutDenial;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IntegrityOwnedCloseoutFileEvidence {
    file_name: String,
    line_count: u32,
    line_cap: u32,
}

impl IntegrityOwnedCloseoutFileEvidence {
    pub fn checked(
        file_name: impl Into<String>,
        line_count: u32,
        line_cap: u32,
    ) -> Result<Self, PhysicalIntegrityCloseoutDenial> {
        let file_name = file_name.into();
        if file_name.trim().is_empty() || line_count == 0 {
            return Err(PhysicalIntegrityCloseoutDenial::MissingOwnedCloseoutFile);
        }
        if line_count > line_cap {
            return Err(
                PhysicalIntegrityCloseoutDenial::IntegrityOwnedCloseoutFileOverBudget(file_name),
            );
        }
        Ok(Self {
            file_name,
            line_count,
            line_cap,
        })
    }

    pub fn file_name(&self) -> &str {
        &self.file_name
    }

    pub const fn line_count(&self) -> u32 {
        self.line_count
    }

    pub const fn line_cap(&self) -> u32 {
        self.line_cap
    }
}
