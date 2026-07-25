use super::{require_text, PhysicalWorkRunProvenanceDenial};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PhysicalWorkRerunEvidence {
    program: Box<str>,
    arguments: Box<[Box<str>]>,
}

impl PhysicalWorkRerunEvidence {
    pub fn new(
        program: impl Into<Box<str>>,
        arguments: impl IntoIterator<Item = impl Into<Box<str>>>,
    ) -> Result<Self, PhysicalWorkRunProvenanceDenial> {
        let program = program.into();
        require_text(&program, PhysicalWorkRunProvenanceDenial::EmptyRerunProgram)?;
        let arguments = arguments
            .into_iter()
            .map(Into::into)
            .collect::<Vec<Box<str>>>();
        for argument in &arguments {
            require_text(
                argument,
                PhysicalWorkRunProvenanceDenial::EmptyRerunArgument,
            )?;
        }
        Ok(Self {
            program,
            arguments: arguments.into_boxed_slice(),
        })
    }

    pub fn program(&self) -> &str {
        &self.program
    }

    pub const fn arguments(&self) -> &[Box<str>] {
        &self.arguments
    }
}
