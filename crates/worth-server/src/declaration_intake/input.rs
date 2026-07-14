use super::{WorthServerDirectDeclarationSource, WorthServerDirectViewShape};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthServerDirectDeclaration {
    source: WorthServerDirectDeclarationSource,
    view_shape: WorthServerDirectViewShape,
}

impl WorthServerDirectDeclaration {
    pub fn builder() -> WorthServerDirectDeclarationBuilder {
        WorthServerDirectDeclarationBuilder::default()
    }

    pub fn named_read(operation_name: impl Into<String>) -> Self {
        Self {
            source: WorthServerDirectDeclarationSource::named_read(operation_name),
            view_shape: WorthServerDirectViewShape::Detail,
        }
    }

    pub fn saved_query(saved_query_name: impl Into<String>) -> Self {
        Self {
            source: WorthServerDirectDeclarationSource::saved_query(saved_query_name),
            view_shape: WorthServerDirectViewShape::Detail,
        }
    }

    pub fn template(template_name: impl Into<String>) -> Self {
        Self {
            source: WorthServerDirectDeclarationSource::template(template_name),
            view_shape: WorthServerDirectViewShape::Detail,
        }
    }

    pub fn with_view_shape(mut self, view_shape: WorthServerDirectViewShape) -> Self {
        self.view_shape = view_shape;
        self
    }

    pub fn source(&self) -> &WorthServerDirectDeclarationSource {
        &self.source
    }

    pub fn view_shape(&self) -> WorthServerDirectViewShape {
        self.view_shape
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct WorthServerDirectDeclarationBuilder {
    source: Option<WorthServerDirectDeclarationSource>,
    view_shape: Option<WorthServerDirectViewShape>,
}

impl WorthServerDirectDeclarationBuilder {
    pub fn with_named_read(mut self, operation_name: impl Into<String>) -> Self {
        self.source = Some(WorthServerDirectDeclarationSource::named_read(
            operation_name,
        ));
        self
    }

    pub fn with_saved_query(mut self, saved_query_name: impl Into<String>) -> Self {
        self.source = Some(WorthServerDirectDeclarationSource::saved_query(
            saved_query_name,
        ));
        self
    }

    pub fn with_template(mut self, template_name: impl Into<String>) -> Self {
        self.source = Some(WorthServerDirectDeclarationSource::template(template_name));
        self
    }

    pub fn with_view_shape(mut self, view_shape: WorthServerDirectViewShape) -> Self {
        self.view_shape = Some(view_shape);
        self
    }

    pub fn build(self) -> Result<WorthServerDirectDeclaration, WorthServerDirectDeclarationError> {
        Ok(WorthServerDirectDeclaration {
            source: self
                .source
                .ok_or(WorthServerDirectDeclarationError::MissingSource)?,
            view_shape: self
                .view_shape
                .unwrap_or(WorthServerDirectViewShape::Detail),
        })
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthServerDirectDeclarationError {
    MissingSource,
}
