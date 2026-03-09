use serde_json::Value;

pub type ExtensionResult = Result<(), Box<dyn std::error::Error + Send + Sync>>;

pub trait FixturePreparationHook<FixtureData> {
    fn prepare_fixture(&self, _fixture: &mut FixtureData) -> ExtensionResult {
        Ok(())
    }
}

pub trait MutationEnricher<MutationData> {
    fn enrich_mutation(&self, _mutation: &mut MutationData) -> ExtensionResult {
        Ok(())
    }
}

pub trait ExecutionProfileAugmenter<ExecutionProfile> {
    fn augment_profile(&self, _profile: &mut ExecutionProfile) -> ExtensionResult {
        Ok(())
    }
}

pub trait PreRunCaptureHook<Runtime> {
    fn before_run(&self, _runtime: &Runtime) -> ExtensionResult {
        Ok(())
    }
}

pub trait PostRunCaptureHook<Runtime> {
    fn after_run(&self, _runtime: &Runtime) -> ExtensionResult {
        Ok(())
    }
}

pub trait RecordCollector<Runtime> {
    fn collect_record(
        &self,
        _runtime: &Runtime,
    ) -> Result<Option<Value>, Box<dyn std::error::Error + Send + Sync>> {
        Ok(None)
    }
}

pub trait EventProjector<EventRecord> {
    fn project_event(
        &self,
        _event: &EventRecord,
    ) -> Result<Option<Value>, Box<dyn std::error::Error + Send + Sync>> {
        Ok(None)
    }
}

pub trait ComparisonRule<Record> {
    fn compare(
        &self,
        _left: &Record,
        _right: &Record,
    ) -> Result<Option<Value>, Box<dyn std::error::Error + Send + Sync>> {
        Ok(None)
    }
}

pub trait RecordOracle<Record> {
    fn evaluate(
        &self,
        _record: &Record,
    ) -> Result<Option<Value>, Box<dyn std::error::Error + Send + Sync>> {
        Ok(None)
    }
}

pub trait EquivalenceOracle<Record> {
    fn compare_equivalence(
        &self,
        _left: &Record,
        _right: &Record,
    ) -> Result<Option<Value>, Box<dyn std::error::Error + Send + Sync>> {
        Ok(None)
    }
}

pub trait ComparisonRenderer<ComparisonRecord> {
    fn render(
        &self,
        _comparison: &ComparisonRecord,
    ) -> Result<Option<String>, Box<dyn std::error::Error + Send + Sync>> {
        Ok(None)
    }
}

pub trait ReplayEnricher<ReplayRecord> {
    fn enrich_replay(&self, _record: &mut ReplayRecord) -> ExtensionResult {
        Ok(())
    }
}

pub trait ExportSink<Record> {
    fn export(&self, _record: &Record) -> ExtensionResult {
        Ok(())
    }
}

pub struct ExtensionPipeline<FixtureData, MutationData, Profile, Runtime, ReplayRecord> {
    fixture_hooks: Vec<Box<dyn FixturePreparationHook<FixtureData>>>,
    mutation_enrichers: Vec<Box<dyn MutationEnricher<MutationData>>>,
    profile_augmenters: Vec<Box<dyn ExecutionProfileAugmenter<Profile>>>,
    pre_run_hooks: Vec<Box<dyn PreRunCaptureHook<Runtime>>>,
    post_run_hooks: Vec<Box<dyn PostRunCaptureHook<Runtime>>>,
    replay_enrichers: Vec<Box<dyn ReplayEnricher<ReplayRecord>>>,
}

impl<FixtureData, MutationData, Profile, Runtime, ReplayRecord>
    Default for ExtensionPipeline<FixtureData, MutationData, Profile, Runtime, ReplayRecord>
{
    fn default() -> Self {
        Self {
            fixture_hooks: Vec::new(),
            mutation_enrichers: Vec::new(),
            profile_augmenters: Vec::new(),
            pre_run_hooks: Vec::new(),
            post_run_hooks: Vec::new(),
            replay_enrichers: Vec::new(),
        }
    }
}

impl<FixtureData, MutationData, Profile, Runtime, ReplayRecord>
    ExtensionPipeline<FixtureData, MutationData, Profile, Runtime, ReplayRecord>
{
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_fixture_hook(
        mut self,
        hook: impl FixturePreparationHook<FixtureData> + 'static,
    ) -> Self {
        self.fixture_hooks.push(Box::new(hook));
        self
    }

    pub fn with_mutation_enricher(
        mut self,
        enricher: impl MutationEnricher<MutationData> + 'static,
    ) -> Self {
        self.mutation_enrichers.push(Box::new(enricher));
        self
    }

    pub fn with_profile_augmenter(
        mut self,
        augmenter: impl ExecutionProfileAugmenter<Profile> + 'static,
    ) -> Self {
        self.profile_augmenters.push(Box::new(augmenter));
        self
    }

    pub fn with_pre_run_hook(
        mut self,
        hook: impl PreRunCaptureHook<Runtime> + 'static,
    ) -> Self {
        self.pre_run_hooks.push(Box::new(hook));
        self
    }

    pub fn with_post_run_hook(
        mut self,
        hook: impl PostRunCaptureHook<Runtime> + 'static,
    ) -> Self {
        self.post_run_hooks.push(Box::new(hook));
        self
    }

    pub fn with_replay_enricher(
        mut self,
        enricher: impl ReplayEnricher<ReplayRecord> + 'static,
    ) -> Self {
        self.replay_enrichers.push(Box::new(enricher));
        self
    }

    pub fn prepare_fixture(&self, fixture: &mut FixtureData) -> ExtensionResult {
        for hook in &self.fixture_hooks {
            hook.prepare_fixture(fixture)?;
        }
        Ok(())
    }

    pub fn enrich_mutation(&self, mutation: &mut MutationData) -> ExtensionResult {
        for enricher in &self.mutation_enrichers {
            enricher.enrich_mutation(mutation)?;
        }
        Ok(())
    }

    pub fn augment_profile(&self, profile: &mut Profile) -> ExtensionResult {
        for augmenter in &self.profile_augmenters {
            augmenter.augment_profile(profile)?;
        }
        Ok(())
    }

    pub fn before_run(&self, runtime: &Runtime) -> ExtensionResult {
        for hook in &self.pre_run_hooks {
            hook.before_run(runtime)?;
        }
        Ok(())
    }

    pub fn after_run(&self, runtime: &Runtime) -> ExtensionResult {
        for hook in &self.post_run_hooks {
            hook.after_run(runtime)?;
        }
        Ok(())
    }

    pub fn enrich_replay(&self, replay: &mut ReplayRecord) -> ExtensionResult {
        for enricher in &self.replay_enrichers {
            enricher.enrich_replay(replay)?;
        }
        Ok(())
    }
}

pub struct CollectorSuite<Runtime, Record, EventRecord> {
    record_collectors: Vec<Box<dyn RecordCollector<Runtime>>>,
    event_projectors: Vec<Box<dyn EventProjector<EventRecord>>>,
    export_sinks: Vec<Box<dyn ExportSink<Record>>>,
}

impl<Runtime, Record, EventRecord> Default for CollectorSuite<Runtime, Record, EventRecord> {
    fn default() -> Self {
        Self {
            record_collectors: Vec::new(),
            event_projectors: Vec::new(),
            export_sinks: Vec::new(),
        }
    }
}

impl<Runtime, Record, EventRecord> CollectorSuite<Runtime, Record, EventRecord> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_record_collector(
        mut self,
        collector: impl RecordCollector<Runtime> + 'static,
    ) -> Self {
        self.record_collectors.push(Box::new(collector));
        self
    }

    pub fn with_event_projector(
        mut self,
        projector: impl EventProjector<EventRecord> + 'static,
    ) -> Self {
        self.event_projectors.push(Box::new(projector));
        self
    }

    pub fn with_export_sink(mut self, sink: impl ExportSink<Record> + 'static) -> Self {
        self.export_sinks.push(Box::new(sink));
        self
    }

    pub fn collect_records(
        &self,
        runtime: &Runtime,
    ) -> Result<Vec<Value>, Box<dyn std::error::Error + Send + Sync>> {
        let mut values = Vec::new();
        for collector in &self.record_collectors {
            if let Some(value) = collector.collect_record(runtime)? {
                values.push(value);
            }
        }
        Ok(values)
    }

    pub fn project_event(
        &self,
        event: &EventRecord,
    ) -> Result<Vec<Value>, Box<dyn std::error::Error + Send + Sync>> {
        let mut values = Vec::new();
        for projector in &self.event_projectors {
            if let Some(value) = projector.project_event(event)? {
                values.push(value);
            }
        }
        Ok(values)
    }

    pub fn export_record(&self, record: &Record) -> ExtensionResult {
        for sink in &self.export_sinks {
            sink.export(record)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::sync::{Arc, Mutex};

    use serde_json::{json, Value};

    use super::{
        CollectorSuite, EventProjector, ExecutionProfileAugmenter, ExportSink, ExtensionPipeline,
        FixturePreparationHook, MutationEnricher, RecordCollector,
    };

    struct FixtureHook;
    impl FixturePreparationHook<String> for FixtureHook {
        fn prepare_fixture(&self, fixture: &mut String) -> super::ExtensionResult {
            fixture.push_str("-prepared");
            Ok(())
        }
    }

    struct MutationHook;
    impl MutationEnricher<String> for MutationHook {
        fn enrich_mutation(&self, mutation: &mut String) -> super::ExtensionResult {
            mutation.push_str("-enriched");
            Ok(())
        }
    }

    struct ProfileHook;
    impl ExecutionProfileAugmenter<String> for ProfileHook {
        fn augment_profile(&self, profile: &mut String) -> super::ExtensionResult {
            profile.push_str("-augmented");
            Ok(())
        }
    }

    struct RuntimeCollector;
    impl RecordCollector<String> for RuntimeCollector {
        fn collect_record(
            &self,
            runtime: &String,
        ) -> Result<Option<Value>, Box<dyn std::error::Error + Send + Sync>> {
            Ok(Some(json!({ "runtime": runtime })))
        }
    }

    struct Projector;
    impl EventProjector<String> for Projector {
        fn project_event(
            &self,
            event: &String,
        ) -> Result<Option<Value>, Box<dyn std::error::Error + Send + Sync>> {
            Ok(Some(json!({ "event": event })))
        }
    }

    struct Sink(Arc<Mutex<Vec<String>>>);
    impl ExportSink<String> for Sink {
        fn export(&self, record: &String) -> super::ExtensionResult {
            self.0.lock().unwrap().push(record.clone());
            Ok(())
        }
    }

    #[test]
    fn extension_pipeline_applies_hooks_in_order() {
        let pipeline = ExtensionPipeline::<String, String, String, String, String>::new()
            .with_fixture_hook(FixtureHook)
            .with_mutation_enricher(MutationHook)
            .with_profile_augmenter(ProfileHook);

        let mut fixture = "fixture".to_string();
        let mut mutation = "mutation".to_string();
        let mut profile = "profile".to_string();
        pipeline.prepare_fixture(&mut fixture).unwrap();
        pipeline.enrich_mutation(&mut mutation).unwrap();
        pipeline.augment_profile(&mut profile).unwrap();

        assert_eq!(fixture, "fixture-prepared");
        assert_eq!(mutation, "mutation-enriched");
        assert_eq!(profile, "profile-augmented");
    }

    #[test]
    fn collector_suite_collects_projects_and_exports() {
        let exported = Arc::new(Mutex::new(Vec::new()));
        let suite = CollectorSuite::<String, String, String>::new()
            .with_record_collector(RuntimeCollector)
            .with_event_projector(Projector)
            .with_export_sink(Sink(exported.clone()));

        let collected = suite.collect_records(&"runtime".to_string()).unwrap();
        let projected = suite.project_event(&"event".to_string()).unwrap();
        suite.export_record(&"record".to_string()).unwrap();

        assert_eq!(collected.len(), 1);
        assert_eq!(projected.len(), 1);
        assert_eq!(exported.lock().unwrap().len(), 1);
    }
}
