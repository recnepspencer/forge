use super::patch_payload::retained_materialized_row_from_scalar_values;
use super::*;

#[derive(Clone, Debug, PartialEq, Default)]
pub struct WorthQueryDerivedViewMaterialization {
    rows: Vec<WorthQueryRetainedMaterializedRow>,
    published: bool,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct WorthQueryRetainedUpstreamInputs {
    live_rows: BTreeMap<WorthQueryLiveArtifactTarget, Vec<WorthQueryEntity>>,
    computed_rows:
        BTreeMap<WorthQueryDerivedMaterializationTarget, Vec<WorthQueryRetainedMaterializedRow>>,
}

impl WorthQueryRetainedUpstreamInputs {
    pub(in crate::runtime) fn new(
        live_rows: impl IntoIterator<Item = (WorthQueryLiveArtifactTarget, Vec<WorthQueryEntity>)>,
        computed_rows: impl IntoIterator<
            Item = (
                WorthQueryDerivedMaterializationTarget,
                Vec<WorthQueryRetainedMaterializedRow>,
            ),
        >,
    ) -> Self {
        Self {
            live_rows: live_rows.into_iter().collect(),
            computed_rows: computed_rows.into_iter().collect(),
        }
    }

    pub(in crate::runtime) fn from_retained_computed_rows(
        live_rows: impl IntoIterator<Item = (WorthQueryLiveArtifactTarget, Vec<WorthQueryEntity>)>,
        computed_rows: impl IntoIterator<
            Item = (
                WorthQueryDerivedMaterializationTarget,
                Vec<WorthQueryRetainedMaterializedRow>,
            ),
        >,
    ) -> Self {
        Self::new(live_rows, computed_rows)
    }

    pub fn live_rows_for<T>(&self, view: &WorthQueryLiveView<T>) -> Option<&[WorthQueryEntity]> {
        self.live_rows
            .get(
                &WorthQueryLiveArtifactTarget::from_subscription_installation(
                    view.subscription_installation(),
                ),
            )
            .map(Vec::as_slice)
    }

    fn live_rows_by_name(&self, view_name: &str) -> Option<&[WorthQueryEntity]> {
        self.live_rows
            .get(&WorthQueryLiveArtifactTarget::from_view_name(view_name))
            .map(Vec::as_slice)
    }

    pub fn declared_live_rows_for<T>(
        &self,
        declaration: &WorthQueryDerivedView,
        view: &WorthQueryLiveView<T>,
    ) -> Option<&[WorthQueryEntity]> {
        self.declared_live_rows_by_name(declaration, view.name())
    }

    pub fn declared_live_row_sets<'a>(
        &'a self,
        declaration: &'a WorthQueryDerivedView,
    ) -> impl Iterator<Item = &'a [WorthQueryEntity]> + 'a {
        declaration
            .upstream_live_views()
            .iter()
            .filter_map(|view_name| self.live_rows_by_name(view_name))
    }

    fn declared_live_rows_by_name(
        &self,
        declaration: &WorthQueryDerivedView,
        view_name: &str,
    ) -> Option<&[WorthQueryEntity]> {
        declaration
            .upstream_live_views()
            .iter()
            .any(|declared| declared == view_name)
            .then(|| self.live_rows_by_name(view_name))
            .flatten()
    }

    pub fn retained_computed_rows_for<T>(
        &self,
        view: &WorthQueryDerivedViewHandle<T>,
    ) -> &[WorthQueryRetainedMaterializedRow] {
        self.retained_computed_rows_by_name(view.name())
    }

    fn retained_computed_rows_by_name(
        &self,
        view_name: &str,
    ) -> &[WorthQueryRetainedMaterializedRow] {
        self.computed_rows
            .get(&WorthQueryDerivedMaterializationTarget::new(view_name))
            .map(Vec::as_slice)
            .unwrap_or_default()
    }

    pub fn declared_retained_computed_rows_for<T>(
        &self,
        declaration: &WorthQueryDerivedView,
        view: &WorthQueryDerivedViewHandle<T>,
    ) -> &[WorthQueryRetainedMaterializedRow] {
        self.declared_retained_computed_rows_by_name(declaration, view.name())
    }

    pub fn declared_retained_computed_row_sets<'a>(
        &'a self,
        declaration: &'a WorthQueryDerivedView,
    ) -> impl Iterator<Item = &'a [WorthQueryRetainedMaterializedRow]> + 'a {
        declaration
            .upstream_derived_views()
            .iter()
            .map(|view_name| self.retained_computed_rows_by_name(view_name))
    }

    fn declared_retained_computed_rows_by_name(
        &self,
        declaration: &WorthQueryDerivedView,
        view_name: &str,
    ) -> &[WorthQueryRetainedMaterializedRow] {
        if declaration
            .upstream_derived_views()
            .iter()
            .any(|declared| declared == view_name)
        {
            self.retained_computed_rows_by_name(view_name)
        } else {
            &[]
        }
    }

    pub fn single_retained_computed_row_for<T>(
        &self,
        view: &WorthQueryDerivedViewHandle<T>,
    ) -> Result<&WorthQueryRetainedMaterializedRow, WorthQueryRuntimeError> {
        self.single_retained_computed_row_by_name(view.name())
    }

    pub fn single_declared_retained_computed_row_for<T>(
        &self,
        declaration: &WorthQueryDerivedView,
        view: &WorthQueryDerivedViewHandle<T>,
    ) -> Result<&WorthQueryRetainedMaterializedRow, WorthQueryRuntimeError> {
        self.single_declared_retained_computed_row_by_name(declaration, view.name())
    }

    fn single_declared_retained_computed_row_by_name(
        &self,
        declaration: &WorthQueryDerivedView,
        view_name: &str,
    ) -> Result<&WorthQueryRetainedMaterializedRow, WorthQueryRuntimeError> {
        if !declaration
            .upstream_derived_views()
            .iter()
            .any(|declared| declared == view_name)
        {
            return Err(WorthQueryRuntimeError::RetainedRowDecode {
                view_name: view_name.to_string(),
                stage: "retained-upstream",
                message: "retained computed row was not declared as an upstream".to_string(),
            });
        }
        self.single_retained_computed_row_by_name(view_name)
    }

    fn single_retained_computed_row_by_name(
        &self,
        view_name: &str,
    ) -> Result<&WorthQueryRetainedMaterializedRow, WorthQueryRuntimeError> {
        match self.retained_computed_rows_by_name(view_name) {
            [] => Err(WorthQueryRuntimeError::RetainedRowDecode {
                view_name: view_name.to_string(),
                stage: "retained-upstream",
                message: "expected one retained row, found none".to_string(),
            }),
            [row] => Ok(row),
            rows => Err(WorthQueryRuntimeError::RetainedRowDecode {
                view_name: view_name.to_string(),
                stage: "retained-upstream",
                message: format!("expected one retained row, found {}", rows.len()),
            }),
        }
    }
}

impl WorthQueryDerivedViewMaterialization {
    pub(in crate::runtime) fn retained_rows(&self) -> &[WorthQueryRetainedMaterializedRow] {
        &self.rows
    }

    pub fn is_published(&self) -> bool {
        self.published
    }

    pub(in crate::runtime) fn replace_retained_rows(
        &mut self,
        rows: impl IntoIterator<Item = WorthQueryRetainedMaterializedRow>,
    ) {
        self.rows = rows.into_iter().collect();
        self.published = true;
    }

    pub(in crate::runtime) fn push_retained_row(&mut self, row: WorthQueryRetainedMaterializedRow) {
        self.rows.push(row);
        self.published = true;
    }

    pub fn replace_retained_scalar_row(
        &mut self,
        scalar_values: impl IntoIterator<Item = (WorthQueryRetainedFieldPath, AspectValue)>,
    ) -> Result<(), String> {
        let row = retained_materialized_row_from_scalar_values(scalar_values)?;
        self.replace_retained_rows([row]);
        Ok(())
    }

    pub fn push_retained_scalar_row(
        &mut self,
        scalar_values: impl IntoIterator<Item = (WorthQueryRetainedFieldPath, AspectValue)>,
    ) -> Result<(), String> {
        let row = retained_materialized_row_from_scalar_values(scalar_values)?;
        self.push_retained_row(row);
        Ok(())
    }
}
