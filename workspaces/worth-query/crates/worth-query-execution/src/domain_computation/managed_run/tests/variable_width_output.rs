use super::*;

struct VariableWidthProvider;

struct VariableWidthExecution;

impl WorthQueryGraphProviderExecution for VariableWidthExecution {
    fn advance(
        &mut self,
        step: &mut WorthQueryGraphProviderStep,
    ) -> Result<WorthQueryGraphProviderStepDisposition, WorthQueryGraphProviderFailure> {
        step.perform_work_unit(|| Ok(()))?;
        step.emit_projection_chunk(variable_width_material(8_192))
            .map_err(step_failure)?;
        WorthQueryGraphProviderStepDisposition::complete("variable-width")
            .map_err(WorthQueryGraphProviderFailure::new)
    }

    fn dispose(&mut self) -> Result<(), WorthQueryGraphProviderFailure> {
        Ok(())
    }
}

impl WorthQueryGraphParticipationProvider<ManagedGraph> for VariableWidthProvider {
    type Execution = VariableWidthExecution;

    fn execution_resource_support(
        &self,
    ) -> worth_query_admission::facade::resource_admission::WorthQueryExecutionResourceSupport {
        crate::domain_computation::provider_session::execution_resource_support("variable-width", 8)
    }

    fn begin(
        &self,
        _call: &WorthQueryGraphProviderCall,
    ) -> Result<Self::Execution, WorthQueryGraphProviderFailure> {
        Ok(VariableWidthExecution)
    }
}

#[test]
fn one_variable_width_row_cannot_escape_the_retained_memory_ceiling() {
    let (running, graph) = managed_graph_run_with_provider(
        WorthQueryOperationGraphAccess::Project,
        VariableWidthProvider,
    );
    let active = running
        .begin_graph_execution(
            &graph,
            WorthQueryManagedGraphCallRequest::new(
                WorthQueryGraphProviderCallKind::Project,
                "variable-width",
            ),
        )
        .expect("variable-width provider should reach its governed step");
    let terminal = match active.advance() {
        WorthQueryDirectGraphStepOutcome::Failed(terminal) => terminal,
        _ => panic!("variable-width row escaped the retained-memory ceiling"),
    };
    assert_eq!(terminal.provider_work().completed_work_units(), 1);
    assert!(terminal.provider_work().retained_bytes() > 4_096);
    assert_eq!(terminal.provider_work().queue_state_mutation_count(), 0);
    terminal
        .cleanup()
        .expect("over-budget projection must preserve cleanup authority");
}

fn variable_width_material(capacity: usize) -> WorthQueryGraphReadMaterial {
    let mut value = String::with_capacity(capacity);
    value.push('x');
    let path = CanonicalFieldPath::single(FieldKey::new("value").unwrap());
    WorthQueryGraphReadMaterial::new([WorthQueryGraphReadRow::from_native_fields(
        "variable-width-entity",
        [(path, AspectValue::String(InternedString::from(value)))]
            .into_iter()
            .collect(),
    )
    .unwrap()])
}

fn step_failure(
    denial: crate::domain_computation::WorthQueryGraphProviderStepDenial,
) -> WorthQueryGraphProviderFailure {
    WorthQueryGraphProviderFailure::new(denial.detail())
}
