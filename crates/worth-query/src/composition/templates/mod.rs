mod binding_set;
mod descriptor;
mod instantiation;
mod slot;
#[cfg(test)]
mod test_support;

pub use binding_set::TemplateBindingSet;
pub use descriptor::QueryTemplateDescriptor;
pub(crate) use instantiation::instantiate_template;
pub use instantiation::TemplateInstantiationArtifact;
pub use slot::{TemplateParameterSlot, TemplateParameterSlotKind};
