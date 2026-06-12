use super::{ForgeServerDirectDeclarationSource, ForgeServerDirectViewShape};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeServerDirectDeclaration {
    source: ForgeServerDirectDeclarationSource,
    view_shape: ForgeServerDirectViewShape,
}

impl ForgeServerDirectDeclaration {
    pub fn builder() -> ForgeServerDirectDeclarationBuilder {
        ForgeServerDirectDeclarationBuilder::default()
    }

    pub fn named_read(operation_name: impl Into<String>) -> Self {
        Self {
            source: ForgeServerDirectDeclarationSource::named_read(operation_name),
            view_shape: ForgeServerDirectViewShape::Detail,
        }
    }

    pub fn saved_query(saved_query_name: impl Into<String>) -> Self {
        Self {
            source: ForgeServerDirectDeclarationSource::saved_query(saved_query_name),
            view_shape: ForgeServerDirectViewShape::Detail,
        }
    }

    pub fn template(template_name: impl Into<String>) -> Self {
        Self {
            source: ForgeServerDirectDeclarationSource::template(template_name),
            view_shape: ForgeServerDirectViewShape::Detail,
        }
    }

    pub fn with_view_shape(mut self, view_shape: ForgeServerDirectViewShape) -> Self {
        self.view_shape = view_shape;
        self
    }

    pub fn source(&self) -> &ForgeServerDirectDeclarationSource {
        &self.source
    }

    pub fn view_shape(&self) -> ForgeServerDirectViewShape {
        self.view_shape
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct ForgeServerDirectDeclarationBuilder {
    source: Option<ForgeServerDirectDeclarationSource>,
    view_shape: Option<ForgeServerDirectViewShape>,
}

impl ForgeServerDirectDeclarationBuilder {
    pub fn with_named_read(mut self, operation_name: impl Into<String>) -> Self {
        self.source = Some(ForgeServerDirectDeclarationSource::named_read(
            operation_name,
        ));
        self
    }

    pub fn with_saved_query(mut self, saved_query_name: impl Into<String>) -> Self {
        self.source = Some(ForgeServerDirectDeclarationSource::saved_query(
            saved_query_name,
        ));
        self
    }

    pub fn with_template(mut self, template_name: impl Into<String>) -> Self {
        self.source = Some(ForgeServerDirectDeclarationSource::template(template_name));
        self
    }

    pub fn with_view_shape(mut self, view_shape: ForgeServerDirectViewShape) -> Self {
        self.view_shape = Some(view_shape);
        self
    }

    pub fn build(self) -> Result<ForgeServerDirectDeclaration, ForgeServerDirectDeclarationError> {
        Ok(ForgeServerDirectDeclaration {
            source: self
                .source
                .ok_or(ForgeServerDirectDeclarationError::MissingSource)?,
            view_shape: self
                .view_shape
                .unwrap_or(ForgeServerDirectViewShape::Detail),
        })
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ForgeServerDirectDeclarationError {
    MissingSource,
}
