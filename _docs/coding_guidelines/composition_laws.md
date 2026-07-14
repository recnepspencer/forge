# File Composition Laws

1. API compliance is not absolution. A file can call the correct framework APIs and still be structurally corrupt if its functions hide classification, validation, transformation, mutation, diagnostics, and result construction inside one body. The standard is not “did it use the framework correctly?” The standard is “can the reader predict the semantic sequence without interpreting every line?” Correct API calls inside a god function do not redeem it; they only make the collapse look sanctioned.

2. A file is a semantic compilation unit, not an editing workspace. A file exists to express one named responsibility, and its contents must be predictable from its filename and folder context. If a reader must scan the whole file to discover what the file owns, the file has failed as a boundary. A file that accepts every nearby helper, branch, fixture, formatter, adapter, and policy check becomes a private universe. Private universes become unreviewable, unowned, and eventually untouchable.

3. A file must let the reader predict its logic hierarchy. A reader should be able to descend from the file name to the public entry point, from the entry point to orchestration, from orchestration to semantic steps, and from semantic steps to local mechanics. If every function appears to be a peer, the file has hidden its structure. A bag of functions is not composition; it is unnamed topology. The reader must discover by archaeology what the author refused to name.

4. God functions form before god files. First the step is unnamed, then the branch is tolerated, then the helper is hidden, then the file becomes a private runtime. A function that loads state, validates policy, classifies cases, computes values, mutates storage, emits effects, maps errors, and formats responses has already collapsed a subsystem, even if the file is still short. Waiting until the file is large enough to be obviously bad means the fracture lines have already been buried.

5. A nontrivial function body must be dominated by named semantic steps. Inline mechanics are allowed only when they are local, obvious, and below the function’s abstraction level. If most of the body consists of raw conditions, object construction, data reshaping, nested branching, storage mutation, effect emission, and error mapping, the function is doing unnamed work. Unnamed work becomes unreadable work; unreadable work becomes unowned work.

6. A function may orchestrate multiple responsibilities only by naming them; it may not inline them. Orchestration is allowed to be broad, but its body must read like a table of contents for the operation. If the body reads like the whole book, the function is a compressed subsystem. A good orchestrator exposes sequence; a bad orchestrator hides workflow inside mechanics.

```ts
async function correctTrade(input: CorrectTradeInput) {
  const state = await loadTradeCorrectionState(input);
  const correction = calculateTradeCorrection(state);
  const eligibility = validateCorrectionEligibility(state, correction);

  if (eligibility.isViolation()) {
    return eligibility.toResult();
  }

  const plan = buildTradeCorrectionPlan(state, correction);
  const result = await applyTradeCorrectionPlan(plan);

  return buildTradeCorrectionResult(result);
}
```

7. Inline meaning is hidden policy. When a condition, calculation, branch, transformation, or predicate encodes a domain rule, invariant, authority decision, lifecycle distinction, or numerical assumption, it must be named. A business rule without a name is policy smuggled through syntax. A rule written only as syntax must be rediscovered by every reader and re-litigated at every edit.

8. A function must not mix abstraction levels unless that mixture is the named responsibility. A use-case function may coordinate validation, planning, persistence, effects, and result building, but it must not inline the mechanics of all of them. A low-level function may manipulate storage rows, but it must not decide domain policy. Abstraction-level mixing creates god functions before it creates god files.

9. Branching must be named when it represents domain classification. A function with multiple policy branches, lifecycle branches, error branches, strategy branches, or state-transition branches must extract the classification step from the execution step. If a reader cannot name the branch set without reading every condition, the function is hiding a decision table. Unnamed branches become decision fog.

```ts
const correctionCase = classifyTradeCorrectionCase(state, input);

switch (correctionCase.kind) {
  case "within_limit":
    return applyStandardCorrection(correctionCase);
  case "requires_approval":
    return requestCorrectionApproval(correctionCase);
  case "blocked":
    return rejectCorrection(correctionCase);
}
```

10. Each semantic check must produce a named fact for downstream code. If later code depends on a condition established earlier, that condition must become a named value, proof object, result variant, or typed wrapper. Re-reading raw booleans and inline predicates across a function creates local proof debt. If the code relies on a proof the name does not reveal, the proof has been hidden from the reader.

11. Helper functions are subordinate structure, not a dumping ground. A helper is valid only when its name and location reveal the parent responsibility it serves. A helper without a parent responsibility is an orphan. Orphan helpers become helper swamps; helper swamps become private APIs no one designed.

12. Helper placement must follow semantic radius. Logic stays inline only while it is obvious and local; moves to a private helper when it names one semantic step; moves to a child module when a group of helpers forms a sub-responsibility; moves to a sibling module when it becomes a peer responsibility; and moves to shared infrastructure only when multiple callers depend on the same concept for the same semantic reason. Moving code to `helpers` or `utils` without naming its responsibility is not decomposition — it is semantic exile. Code exiled to helpers returns as coupling.

13. Extraction must increase predictability, not merely reduce length. A helper named `processData`, `handleLogic`, `runChecks`, `doStuff`, or `doValidation` has moved complexity without naming it. Valid extraction gives hidden responsibility a name that predicts its inputs, outputs, failure mode, and reason to change. Smaller code with worse names is not cleaner; it is fog cut into pieces.

14. Sections are an admission of failure, not boundaries. Comment headers such as `// validation`, `// helpers`, `// storage`, `// formatting`, and `// business logic` do not create decomposition. They stand as evidence that the file contains multiple unnamed responsibilities. If a section can be given a responsibility-predicting name, it must become a function, child module, or sibling file.

15. Comments must explain intent, not compensate for missing structure. A comment may explain why a decision exists. It must not carry what the structure refused to say. A comment that explains what a large block does is often a confession that the block lacks a name. Code must carry structural meaning through files, functions, types, and values. Comments are not boundaries.

16. File order must descend from meaning to mechanics. Public capabilities and primary orchestration should appear before subordinate semantic steps; semantic steps should appear before mechanical helpers. Local types and constants should live near the level they explain. A file ordered by authoring chronology forces every reader to reconstruct the logic hierarchy manually. That is the archaeology tax.

17. A function or variable name must predict semantic role, not implementation category. Names like `data`, `result`, `config`, `manager`, `processor`, `handler`, `context`, `state`, `item`, and `value` are valid only when their scope is so small that no meaning is lost. Every vague name transfers proof from the author to the reader. If the reader must search upward to remember what `result` means, the name is too weak.

18. Framework names are namespace context, not semantic content. Do not name values, functions, files, or modules after the framework unless the construct exists to model, configure, or operate the framework itself. Inside WORTH, nearly everything is WORTH-related; `WORTHData`, `WORTHManager`, `WORTHProcessor`, and `WORTHResult` predict nothing. Repeating the universe is not naming the thing. The name must identify the specific responsibility: `dependencyGraph`, `checkpointRecoveryPlan`, `projectionInvalidationIndex`, `runtimeExecutionEnvelope`, `capabilityResolutionTrace`, or `loweredExecutionPlan`.

19. Names must default to meaning, not brevity. The normal name is the name that preserves role, origin, phase, authority, scope, and guarantee at the point of use. Long names are often the cheapest names because they carry proof forward. Short names are not the baseline; they are a narrow local compression valid only when the meaning is visually undeniable and cannot drift. A name is too long only when its words stop improving prediction. A name is too short the moment the reader must re-derive what it means. Typing is paid once. Rereading is paid forever.

20. Generic verbs cannot carry semantic meaning alone. `process`, `handle`, `run`, `do`, `manage`, `apply`, `execute`, `build`, `make`, `create`, `update`, `get`, `set`, `check`, `compute`, `calculate`, and `validate` are incomplete names unless paired with the domain responsibility and phase they represent. A generic verb without a domain object is motion without meaning. `validate()` is a bucket. `validateRadialEdgeRingClosure()` is a responsibility.

21. Names must preserve phase, authority, and truth status when those distinctions matter. A value that is raw, parsed, validated, eligible, planned, lowered, executed, cached, derived, speculative, committed, rejected, or indeterminate must not be named as if those states are interchangeable. `input`, `state`, `plan`, and `result` are too weak when the phase carries meaning. A name must carry the proof or truth status the code relies on.

22. Similar names must encode real distinctions. If two values or functions differ only by `new`, `old`, `temp`, `data2`, `result2`, `final`, `updated`, or `current`, the code has not named the distinction. Prefer names that encode semantic role: `previousCalibrationRecord`, `candidateCalibrationRecord`, `committedCalibrationRecord`, `rejectedCalibrationRecord`, `rawUserInput`, `validatedUserInput`, and `loweredExecutionPlan`. Numbered and temporal names become semantic drift as soon as the function changes.

23. A function name must advertise its abstraction level. A high-level function should name the domain step it performs, not the mechanics it uses. A low-level function should name the precise mechanic it owns, not the workflow it serves. A function named `correctTrade` may orchestrate correction; a function named `calculateRiskDelta` must not load accounts, mutate balances, or emit audit records. When a function’s body exceeds the abstraction level promised by its name, the name has become a lie.

24. Local types, constants, and enums must serve the file’s named responsibility. A local type is valid when it clarifies the file’s logic hierarchy. If a local type becomes useful outside the file’s responsibility, promote it to the narrowest named module that owns the concept. If it remains local only because no one wanted to name the concept, the file is hiding structure. Anonymous structure is still structure.

25. Public API files may aggregate; they must not implement. A façade, index, barrel, or module export file may be broad because it presents a public surface, but its responsibility is aggregation. It must not become a place for business logic, workflow branching, storage mutation, or diagnostic formatting. A façade that implements behavior becomes a mask over topology, not a boundary.

26. Copy-paste similarity requires a shared lifecycle test. Near-duplicate functions must not be unified merely because they look alike. They should be unified only when they share semantic authority, lifecycle, failure behavior, and test strategy. False unification creates shared failure where the domain did not require shared fate. False duplication creates drift. The deciding factor is shared responsibility, not visual similarity.

27. Tests obey the same composition laws as production. A test file should verify one responsibility, scenario family, invariant family, or behavior surface. Test files named `actions`, `world`, `helpers`, `misc`, `new_tests`, or `milestone_3_tests` create test fog. If a test failure cannot identify the responsibility that broke, the test file is too broad.

28. Setup, action, assertion, and fixture code must not collapse into one blob. Test composition should make it clear what world is being built, what behavior is being exercised, and what responsibility is being asserted. Shared setup is valid only when it preserves the reader’s ability to predict the behavior under test. Test convenience that hides responsibility creates false confidence.

29. File size is a symptom, not the disease. Large files are suspicious because responsibility boundaries usually appear before the line count explodes. A file becomes dangerous when its sections would need different names to be predicted. Line limits catch decay; they do not define cohesion. Small files can still be rotten. A 700-line generated table may be valid; a 180-line function that loads, validates, mutates, emits, and formats may already be a god function.

30. A file should be deletable with its responsibility. If removing a behavior requires preserving half the file because unrelated responsibilities live beside it, the file was not cohesive. Deletion resistance is evidence that the file became a storage location instead of a responsibility boundary. A file that cannot die with its responsibility never truly owned that responsibility.

31. A file should be reviewable as one idea. A reviewer should be able to understand the file’s purpose, failure surface, test surface, and dependency surface without switching between unrelated mental models. If reviewing the file requires simultaneously reasoning about policy, persistence, transport, formatting, diagnostics, retries, effect emission, and response shaping, the file has exceeded its semantic scope. Code that cannot be reviewed as one idea is not one idea.

32. The final test of file composition is whether the next correct edit is obvious. A well-composed file tells the reader where to add code, where not to add code, what to test, what can break, and what can be deleted. A poorly composed file makes the convenient edit easier than the correct edit. When the convenient edit is easier than the correct edit, entropy has become the default workflow. That is how god functions and god files form without anyone explicitly choosing them.