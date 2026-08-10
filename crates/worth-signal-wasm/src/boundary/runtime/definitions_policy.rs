use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

use crate::boundary::serde::from_js;
use crate::recipe::model::{KeyedRecipeFamilySpec, KeyedSourceFamilySpec, RecipeSpec, SourceSpec};
use crate::runtime::policy::RuntimePolicySpec;

use super::super::types::SignalRuntime;

#[wasm_bindgen]
impl SignalRuntime {
    pub fn set_runtime_policy(&self, policy: JsValue) -> Result<(), JsValue> {
        let policy: RuntimePolicySpec = from_js(policy)?;
        self.core
            .borrow_mut()
            .set_runtime_policy(policy)
            .map_err(JsValue::from)
    }

    pub fn define_source(&self, spec: JsValue) -> Result<(), JsValue> {
        let spec: SourceSpec = from_js(spec)?;
        self.core
            .borrow_mut()
            .define_source(spec)
            .map_err(JsValue::from)
    }

    pub fn define_recipe(&self, spec: JsValue) -> Result<(), JsValue> {
        let spec: RecipeSpec = from_js(spec)?;
        self.core
            .borrow_mut()
            .define_recipe(spec)
            .map_err(JsValue::from)
    }

    pub fn define_source_family(&self, spec: JsValue) -> Result<(), JsValue> {
        let spec: KeyedSourceFamilySpec = from_js(spec)?;
        self.core
            .borrow_mut()
            .define_source_family(spec)
            .map_err(JsValue::from)
    }

    pub fn define_recipe_family(&self, spec: JsValue) -> Result<(), JsValue> {
        let spec: KeyedRecipeFamilySpec = from_js(spec)?;
        self.core
            .borrow_mut()
            .define_keyed_recipe_family(spec)
            .map_err(JsValue::from)
    }
}
