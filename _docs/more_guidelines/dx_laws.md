# Developer Experience Laws for Serious Frameworks

1. Good DX is organized truth, not cute syntax. A beautiful API is not one that hides complexity; it is one that places complexity at the exact level where the caller must make a responsible decision. The common path should read like intent, the advanced path should expose the next lower layer, and the unsafe path should make its weakened guarantees explicit.

```ts
await users.rename(userId, {
  displayName: "Spencer",
});

const plan = users.rename
  .intent({ userId, displayName: "Spencer" })
  .compile();

await executor.run(plan);
```

2. API signatures must expose every boundary where caller responsibility changes. A call that crosses process, network, disk, queue, human, transaction, replica, or long-running compute boundaries must give the caller typed control over the concerns only the caller can own: deadline, cancellation, idempotency, retry tolerance, consistency requirement, scope, and artifact policy. Hiding these controls behind a friendly method name does not simplify the system; it merely moves correctness decisions to an invisible layer.

```ts
await client.execute(RenameUser.intent(input), {
  idempotencyKey,
  deadline: "3s",
  retry: RetryPolicy.exponential({ maxAttempts: 3 }),
  signal: abortController.signal,
  consistency: "read-your-writes",
  artifactPolicy: "audit",
});
```

3. Low-level APIs are not inferior; they are phase-inappropriate at high-level call sites. HTTP, SQL, queues, files, locks, and sockets belong in adapters, transports, executors, and drivers. Domain intent should not be forced to speak infrastructure, but infrastructure boundaries must remain reachable to the layer responsible for them.

```ts
const transport = HttpTransport.from(fetch);
const client = UserClient.using(transport);

await client.users.rename(userId, {
  displayName: "Spencer",
});
```

4. Expensive work must look expensive. A method that may trigger heavy computation, graph traversal, disk IO, distributed coordination, replica synchronization, or human workflow must not masquerade as a cheap property read. API shape must distinguish local observation, remote IO, long-running orchestration, and human-mediated waiting.

```ts
user.displayName;                 // local value
await user.profile.load();         // IO boundary
await user.rebuildSearchIndex();   // orchestration boundary
await approval.wait(request);      // human boundary
```

5. Long-running and unbounded work must return handles, streams, or sessions rather than pretending to be ordinary function calls. The API must expose progress, cancellation, checkpoints, partial results, warnings, recovery, and finalization as part of the normal surface.

```ts
const job = await indexes.rebuild({
  scope: Workspace(workspaceId),
  mode: "online",
});

for await (const event of job.events()) {
  renderProgress(event);
}

await job.cancel();
await job.recover();
```

6. Cost, scope, and consistency are part of readability. A friendly API that hides catastrophic cost, global effect, stale reads, replica drift, cache behavior, or large invalidation scope is not readable; it is deceptive. Before execution, serious operations must make their operational footprint inspectable.

```ts
const plan = query.compile();

plan.cost();
/*
{
  estimatedRows: 4_200_000,
  invalidationScope: "workspace",
  memoryClass: "high",
  consistency: "snapshot",
  parallelizable: true,
  externalReads: 3
}
*/
```

7. Global effects must require explicit scope. Any operation that can affect more than one entity, tenant, workspace, account, model, assembly, cache, index, or deployment target must force the caller to name the blast radius. Global scope should be impossible to request accidentally.

```ts
await reindex({
  scope: Workspace(workspaceId),
  target: UserSearchIndex,
});
```

Not:

```ts
await reindex();
```

8. Declarative definitions dominate scattered registration. If defining a resource, computation, route, permission, workflow, projection, table, or handler requires unrelated calls across separate registries, the API has leaked its wiring model. The declaration should describe the semantic unit; the framework should derive wiring, lifecycle, validation, scheduling, and discovery from that unit.

```ts
export const UserModule = defineModule({
  model: User,
  routes: UserRoutes,
  permissions: UserPermissions,
  forms: UserForms,
  workflows: UserWorkflows,
});
```

9. Co-location owns meaning; explicit registration owns membership. Domain files may live together because they share semantic ownership, but the set of modules entering the application graph must be explicit. Magic discovery through file-system conventions, bundler behavior, reflection, or naming tricks must be inspectable and replaceable, not the only path.

```ts
app.register(UserModule);
app.register(BillingModule);
app.register(EquipmentModule);

const graph = app.compile();

graph.validate();
graph.explain();
graph.missingRegistrations();
graph.circularDependencies();
```

10. Definition functions are semantic and inference boundaries. A raw object describes shape; a `defineX()` function captures meaning, preserves literals, validates structure, assigns identity, and returns a typed artifact the rest of the framework can reason about. If a definition function does not improve inference, validation, identity, or inspection, it is ceremony.

```ts
const routes = defineRoutes({
  detail: get("/users/:userId"),
  update: post("/users/:userId", { body: UpdateUser }),
});
```

11. Object specs encode shape; builders encode progression. If the user is defining a resource, schema, route map, table, permission model, or workflow, the API should show the whole structure at once. If the user is accumulating constraints, proofs, phases, or ordered transformations, a builder chain is appropriate. Using the wrong surface either hides structure or fakes progression.

```ts
const form = defineForm({
  fields: {
    name: text().required(),
    role: select(["admin", "operator"]),
  },
  submit: operation("user.rename"),
});
```

```rust
let plan = Query::from("users")
    .filter(active.eq(true))
    .select([name, email])
    .authorize(can_read_users)
    .lower();
```

12. Canonical definitions should produce default projections, not imprison every projection. “Define truth once” means the model can derive common create, edit, read, table, form, filter, API, and workflow views. It does not mean every view must mirror the canonical model forever. Derived artifacts must support local semantic overrides without forking the source of truth.

```ts
const UserTable = User.deriveTable({
  include: ["displayName", "role"],
  override: {
    displayName: column.text({
      label: "Name",
      searchable: true,
    }),
  },
});
```

13. Names, namespaces, versions, and identities are compatibility contracts. A name is not just a label for humans; it is a stable reference used by clients, queues, persisted workflows, audit records, generated SDKs, migrations, policies, and diagnostic tools. Any reusable definition must carry enough identity to survive refactors and version evolution.

```ts
const RenameUser = defineOperation({
  namespace: Identity.User,
  name: "rename",
  version: 2,
});
```

14. The type system is part of the user interface. Valid next actions should appear in autocomplete; invalid actions should be unrepresentable, unreachable, or visibly discouraged. Type inference, typed references, generated clients, phantom tags, branded IDs, and proof-bearing wrappers are not academic machinery; they are how the API teaches correct usage at the call site.

```ts
app.users.rename(...)
app.operations.user.rename.intent(...)
User.operations.rename
User.fields.displayName
User.views.profile
```

15. Stringly-typed names and loose flags are acceptable only at declaration boundaries or where the domain itself is primitive. After declaration, names should become typed references. Positional booleans, anonymous modes, and generic bags are defects when they encode policy, authority, lifecycle, or execution strategy; they are fine when they express an obvious binary domain value.

```ts
router.to(routes.userDetail, { userId });

deleteUser(userId, { soft: true });
field.visible(true);
cache.enabled(false);
```

Not:

```ts
router.push(`/users/${userId}`);
updateUser(userId, body, true, false);
```

16. Configuration and context must mirror the semantic boundaries of the system. A flat bag of options is not a DX surface; it is an implementation leak. Context that affects correctness — actor, tenant, workspace, clock, locale, request identity, trace identity, environment, safety mode, and artifact policy — must be explicit, typed, and injectable. Context that only affects plumbing may be ambient.

```ts
await app.execute(intent, {
  actor,
  tenant,
  workspace,
  clock,
  locale,
  requestId,
  traceId,
  environment: Production,
  mode: StrictExecution,
  artifactPolicy: FullAudit,
});
```

17. Semantic intent must be a first-class object. The caller should express what they mean, not pass loose primitives that force the framework to infer the operation after the fact. A serious intent object can be validated, authorized, planned, simulated, queued, retried, audited, serialized, migrated, replayed, and explained.

```ts
const intent = RenameUser.intent({
  userId,
  displayName: "Spencer",
});

await app.execute(intent);
await app.simulate(intent, TestScenario);
await queue.enqueue(intent, { idempotencyKey });
```

18. Domain intent may be portable across execution substrates, but substrate semantics must remain explicit. The same intent may run locally, remotely, through a queue, in simulation, in replay, or as part of a workflow. Portability is valuable only if each substrate exposes its own deadlines, retries, ordering, durability, consistency, and failure modes.

```ts
await local.execute(intent);
await remote.execute(intent, { deadline: "2s" });
await queue.enqueue(intent, { idempotencyKey });
await simulator.run(intent, TestScenario);
```

19. Friendly APIs must lower into inspectable plans before execution. The readable call site is the authoring surface; the lowered plan is the accountability surface. The executor must receive resolved strategy, policy, locality, artifact policy, concurrency footprint, and effect declarations rather than re-deciding them during execution.

```ts
const plan = RenameUser.compile(input);

plan.requiredCapabilities;
plan.invalidations;
plan.effects;
plan.executionStrategy;
plan.concurrency;
plan.explain();

await executor.run(plan);
```

20. Pipelines must expose proof-carrying phase progression. If the system moves from raw input to parsed, validated, eligible, planned, lowered, executed, and enveloped states, each phase output should carry exactly the proof that phase established. APIs that accept weaker types than the proof chain guarantees force downstream code to defensively re-prove facts that should already be structural.

```rust
let validated = RawInput::new(input).validate()?;
let planned = validated.plan()?;
let lowered = planned.lower()?;
let executed = lowered.execute()?;
```

21. Locality, concurrency, and structural footprint must be visible before execution. Developers should not manually reason about every lock, but the API must expose the read set, write set, invalidation set, conflict set, and parallel-admission proof carried by the plan. Parallel safety must be a planned property, not speculative runtime hope.

```ts
const plan = RenameUser.plan(input);

plan.concurrency;
/*
{
  reads: ["user:123"],
  writes: ["user:123.displayName"],
  invalidates: ["profile:123"],
  conflictsWith: ["user.delete", "user.rename"],
  canRunInParallel: true
}
*/
```

22. Policy must be named, declarative, and lowered before execution. Permission checks embedded across business logic are unreadable and unauditable. The framework should pre-solve applicability, produce a policy plan, and pass that plan to execution rather than interleaving rule discovery with mutation.

```ts
const EnterQms = capability("platform.app.qms.enter", {
  grants: [
    role("platform-admin"),
    assignment("qms-user"),
  ],
});

const policyPlan = await PolicyPlanner.resolve({
  actor,
  intent,
  context,
});

await Executor.run(loweredPlan, policyPlan);
```

23. Failure topology is part of the public API. A high-stakes framework cannot collapse invalid input, rejected policy, dependency failure, conflict, timeout, cancellation, partial execution, external uncertainty, and internal defects into the same error shape. Expected business failures should be typed values; exceptions should be reserved for programmer defects or unrecoverable runtime failures.

```ts
type ExecutionResult<T> =
  | Succeeded<T>
  | Rejected<PolicyTrace>
  | Invalid<ValidationReport>
  | Conflicted<ConflictSet>
  | Cancelled<CancellationContext>
  | TimedOut<DeadlineContext>
  | Failed<FailureTopology>
  | Indeterminate<RecoveryHandle>;
```

24. Binary outcomes are insufficient for serious workflows. Validation, policy, invariant checks, numerical operations, planning, and human workflows often need success, advisory, violation, partial success, and indeterminate states. Treating everything as true/false destroys the context higher-level workflows need to adapt safely.

```ts
type Decision<T> =
  | Allow<T>
  | Advise<T, AdvisoryContext>
  | Deny<ViolationContext>
  | Partial<T, WarningSet>
  | Indeterminate<MissingAuthority>;
```

25. Every boundary result must be a self-describing envelope. A consumer that has never seen the producer’s internals should be able to reconstruct what happened from the returned artifact: primary result, structured warnings, typed errors, decision trace, effects, invalidations, boundary metadata, integrity markers, performance counters, and recovery handles according to artifact policy.

```ts
const result = await app.execute(intent);

result.value;
result.warnings;
result.trace;
result.effects;
result.invalidations;
result.integrity;
result.performance;
result.boundaries;
result.recovery;
```

26. Recovery handles are mandatory for indeterminate outcomes. Distributed systems, external APIs, file systems, queues, long-running jobs, numerical solvers, and human workflows can produce states where the framework cannot honestly say success or failure yet. The API must represent uncertainty as a recoverable object, not hide it in logs or throw a generic exception.

```ts
const result = await execute(intent);

if (result.isIndeterminate()) {
  await result.recovery.inspect();
  await result.recovery.retry();
  await result.recovery.compensate();
}
```

27. Overrides and human approvals are governed state transitions, not side channels. High-stakes business logic must represent review, signatures, escalation, deferral, rejection, emergency override, and guarantee weakening as first-class workflow states with authority, scope, reason, expiration, and audit policy.

```ts
const overridden = plan.override({
  reason: "Emergency production release",
  authority: EmergencyOverrideWitness,
  scope: Equipment(equipmentId),
  expiresAt: clock.now().plusHours(2),
});

await approvals.request({
  approver: QualityManager,
  reason: result.approvalReason,
  pendingPlan: result.plan,
});
```

28. Read contracts and write contracts are duals. A framework that treats mutations as planned operations but treats reads as casual getters has split its correctness model in half. Every mutation should declare what it invalidates; every projection, query, subscription, and derived value should declare what it consumes.

```text
Write path:
Intent -> Validate -> Plan -> Execute -> Envelope

Read path:
Query -> Dependency Declaration -> Snapshot -> Projection -> Subscription -> Invalidation
```

29. Derived reads must declare dependency, consistency, and invalidation semantics. Cache invalidation should be computed from structural dependencies, not hand-maintained by every caller. Subscriptions must expose whether they observe identity changes, structural changes, value changes, stale projections, speculative state, or committed truth.

```ts
const UserProfile = defineProjection({
  input: UserId,
  reads: [User.fields.displayName, User.fields.role],
  compute: ({ user }) => ({
    displayName: user.displayName,
    role: user.role,
  }),
});

app.query(UserProfile, { userId }).subscribe({
  select: profile => profile.displayName,
  invalidation: "structural",
  consistency: "committed",
  onChange: render,
});
```

30. Authoritative, derived, cached, stale, speculative, pending, and committed state must be different API objects. A system that exposes all state through the same shape forces callers to infer truth status from convention. Optimistic updates, previews, drafts, projections, materializations, and reconciled truth must remain visibly distinct.

```ts
const tx = await app.optimistic(RenameUser.intent(input), {
  preview: RenameUserPreview,
  rollback: "automatic",
  reconcile: "server-authoritative",
});

tx.preview;
tx.pending;
tx.committed;
tx.rollback();
```

31. Streams and subscriptions must expose partial materialization, backpressure, overflow, and lifecycle. Any API that produces work faster than another system can consume it must let the caller declare buffering, overflow behavior, concurrency policy, cancellation, and disposal. Fire-and-forget event APIs are production incident generators.

```ts
events.subscribe(UserRenamed, {
  buffer: 1_000,
  overflow: "drop-oldest",
  backpressure: "pause-producer",
});

for await (const page of query.stream({ batchSize: 500 })) {
  process(page);
}
```

32. Every abstraction must expose an explanation surface at its own semantic level. Explanation is not logging. If the user wrote an intent, explain the intent. If the framework compiled a plan, explain the plan. If execution produced effects, explain the effects. Hidden orchestration is acceptable only when it is explainable orchestration.

```ts
const trace = await app.explain(() =>
  app.users.rename(userId, "Spencer")
);

trace.show({
  phases: true,
  permissions: true,
  invalidations: true,
  derivedWrites: true,
  externalEffects: true,
});
```

33. Diagnostic artifacts must be structured, policy-controlled, and separate from domain truth. Logs, traces, metrics, audit entries, decision records, debug trees, and AI-readable inspection data should be derived from typed operation envelopes, but their materialization lifecycle should not pollute the hot path or the domain result.

```ts
const result = await app.execute(intent, {
  artifactPolicy: "minimal",
});

result.value;        // domain truth

const forensic = await result.materializeDiagnostics({
  include: ["trace", "cost", "effects", "boundaries"],
});

forensic.toDiagnosticJson();
forensic.toMermaid();
forensic.toTraceSpans();
```

34. Business logic must be testable before execution. A validation rule should not require a database, container, queue, server, network, or full runtime unless it truly depends on that boundary. Tests should target definitions, plans, effects, errors, policy decisions, and traces — not only final outputs after execution.

```ts
const validation = RenameUserInput.validate(input, {
  actor,
  clock: fixedClock,
  policy: fakePolicy,
});

const plan = RenameUser.compile(input);

expect(validation).toBeValid();
expect(plan.effects).toContain(UserRenamed);
expect(plan.requiredCapabilities).toContain(UserCapabilities.Rename);
```

35. Simulation and replay are first-class DX surfaces. Simulation lets developers ask what would happen without committing it; replay lets them reproduce what did happen using the same inputs, clock, actor, permissions, configuration, versioned definitions, and external observations that existed at the original decision boundary.

```ts
const simulation = await app.simulate(intent, {
  actor,
  clock: fixedClock,
  permissions: testPermissions,
});

const replay = await app.replay(decisionId);

expect(replay.result).toEqual(original.result);
expect(replay.trace).toMatchDecisionTrace(original.trace);
```

36. Public definitions require migration, compatibility, and versioned interpretation. A definition system that cannot express migration is a snapshot format, not a framework contract. Serialized plans, audit records, queued jobs, persisted workflows, materialized views, generated SDKs, and old clients must remain interpretable after code changes.

```ts
const RenameUser = defineOperation("user.rename", {
  version: 2,
  input: RenameUserInputV2,
  compatibleWith: [RenameUserV1],
  deprecatedFields: ["fullName"],
});

RenameUser.migrateFrom(v1, RenameUserV2Migration);

const interpreted = registry
  .forVersion(record.operation, record.version)
  .interpret(record);
```

37. Physical, financial, geometric, temporal, and numerical APIs must encode units, precision, tolerance, rounding, and reference frames. A beautiful API prevents silent semantic corruption by making the important quantities explicit at the type or call-site level.

```ts
part.move(Vector3.mm(10, 20, 30), {
  frame: CoordinateFrame.World,
});

surface.equals(otherSurface, {
  tolerance: Length.microns(5),
  topology: "same-boundary",
});
```

38. Framework-owned lifecycle must be visible at the API surface. Computations, subscriptions, observers, cache entries, projections, jobs, workflows, sessions, and optimistic transactions must be registered, tracked, cancellable, and disposable through framework-owned handles. If a consumer can create a managed resource the framework cannot see, cleanup is a heuristic.

```ts
const subscription = app.subscriptions.create({
  source: UserViews.profile(userId),
  observer: renderProfile,
});

await app.lifecycle.dispose(subscription);
```

39. Developer experience includes authoring, reading, debugging, testing, migration, recovery, and operation. A framework is not pleasant to use if it is easy to write code and impossible to diagnose, evolve, replay, govern, or scale. The final test of DX is whether a junior developer can read the call site, a senior developer can extend the mechanism, an operator can diagnose production behavior, and the runtime can still prove what happened.