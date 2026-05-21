import { createApiFactory } from "../api/api_namespace.js";
import {
  forbidOpaqueIdOption,
  requireAuthoringOptions,
  requireOptionalDebugName,
} from "../authoring_option_validation.js";
import { freezeObject } from "../graph_support.js";
import { PRIVATE_AUTHORING_ID } from "../symbols.js";
import { createWorkerFirstScopedNamespace } from "./worker_first_authoring_namespace.js";
import { createWorkerFirstAsyncInputHandle } from "./worker_first_async_input.js";
import { createWorkerFirstAsyncLinkedHandle } from "./worker_first_async_linked.js";
import {
  createWorkerFirstAsyncRecipeHandle,
  normalizeWorkerFirstAsyncRecipeOptions,
} from "./worker_first_async_recipe.js";
import { runWorkerFirstAsyncTransaction } from "./worker_first_async_transaction.js";
import { createWorkerFirstFormFactory } from "./worker_first_form_factory.js";
import { createWorkerFirstResourceNamespace } from "./worker_first_resource_namespace.js";
import { createWorkerFirstExplicitSpecNamespace } from "./worker_first_explicit_spec_namespace.js";
import { createWorkerFirstRootImportedGraph } from "./worker_first_root_imported_graph.js";
import { createRootHistoryFacade } from "./worker_first_root_history.js";
import { createWorkerFirstRootSession } from "./worker_first_root_session.js";
import {
  createRootAdaptersFacade,
  createRootDiagnosticsFacade,
  createRootSpecialistFacade,
  readRootSignalValue,
} from "./worker_first_root_cached_facades.js";

export function createWorkerFirstCallableSignals(request) {
  const rootSession = createWorkerFirstRootSession(request);
  return createWorkerFirstCallableSignalsAfterBootstrap(rootSession, request);
}

async function createWorkerFirstCallableSignalsAfterBootstrap(rootSession, request) {
  await rootSession.ready();
  const trackedGraphs = new Set();
  let diagnostics = null;
  let history = null;
  let adapters = null;
  let specialist = null;
  let form = null;
  let resource = null;
  let api = null;
  let spec = null;
  let rootNamespace = null;
  void request;

  const namespace = () => {
    rootNamespace ??= createWorkerFirstScopedNamespace(rootSession);
    return rootNamespace;
  };

  const callableSignals = {
    host: rootSession.hostSurface(),
    get spec() {
      spec ??= createWorkerFirstExplicitSpecNamespace(rootSession);
      return spec;
    },
    get form() {
      form ??= createWorkerFirstFormFactory(callableSignals);
      return form;
    },
    get resource() {
      resource ??= createWorkerFirstResourceNamespace(rootSession);
      return resource;
    },
    get api() {
      api ??= createApiFactory(callableSignals);
      return api;
    },
    scope(localScopeId) {
      return namespace().scope(localScopeId);
    },
    controller(definitionOrBuilder) {
      return namespace().controller(definitionOrBuilder);
    },
    publicInput(handle, options) {
      return namespace().publicInput(handle, options);
    },
    input(initial, options) {
      void initial;
      void options;
      throwWorkerFirstCallableUnavailable("signals.input");
    },
    async inputAsync(initial, options) {
      const normalizedOptions = normalizeWorkerFirstInputOptions(options);
      const id =
        normalizedOptions?.[PRIVATE_AUTHORING_ID]
        ?? rootSession.nextGeneratedStandaloneSignalId("input");
      await rootSession.createStandaloneInput(id, initial, normalizedOptions);
      return createWorkerFirstAsyncInputHandle(
        rootSession,
        id,
        normalizedOptions ? requireOptionalDebugName("input", normalizedOptions) : null,
      );
    },
    linked(sourceOrDefinition, options) {
      void sourceOrDefinition;
      void options;
      throwWorkerFirstCallableUnavailable("signals.linked");
    },
    async linkedAsync(sourceOrDefinition, options) {
      return createWorkerFirstAsyncLinkedHandle(
        rootSession,
        rootSession.nextGeneratedStandaloneSignalId("input"),
        sourceOrDefinition,
        options,
      );
    },
    computedSpec(id, spec, options) {
      return callableSignals.spec.computed(id, spec, options);
    },
    computed(specOrCompute, options) {
      void specOrCompute;
      void options;
      throwWorkerFirstCallableUnavailable("signals.computed");
    },
    async computedAsync(specOrCompute, options) {
      const normalizedOptions = normalizeWorkerFirstAsyncRecipeOptions("computed", options);
      return createWorkerFirstAsyncRecipeHandle(
        rootSession,
        "computed",
        rootSession.nextGeneratedStandaloneSignalId("computed"),
        specOrCompute,
        normalizedOptions,
      );
    },
    outputSpec(id, spec, options) {
      return callableSignals.spec.output(id, spec, options);
    },
    output(specOrCompute, options) {
      void specOrCompute;
      void options;
      throwWorkerFirstCallableUnavailable("signals.output");
    },
    async outputAsync(specOrCompute, options) {
      const normalizedOptions = normalizeWorkerFirstAsyncRecipeOptions("output", options);
      return createWorkerFirstAsyncRecipeHandle(
        rootSession,
        "output",
        rootSession.nextGeneratedStandaloneSignalId("output"),
        specOrCompute,
        normalizedOptions,
      );
    },
    outputCallback(id, compute, options) {
      void id;
      void compute;
      void options;
      throwWorkerFirstCallableUnavailable("signals.outputCallback");
    },
    graph(id, definitionOrBuilder) {
      return namespace().graph(id, definitionOrBuilder);
    },
    importGraph(definition, snapshot) {
      const graph = createWorkerFirstRootImportedGraph(rootSession, { definition, snapshot });
      trackedGraphs.add(graph);
      return freezeObject({
        ...graph,
        async terminate() {
          trackedGraphs.delete(graph);
          await graph.terminate();
        },
      });
    },
    read(target) {
      return readRootSignalValue(rootSession, target);
    },
    transaction(callback) {
      void callback;
      throwWorkerFirstCallableUnavailable("signals.transaction");
    },
    batch(callback) {
      void callback;
      throwWorkerFirstCallableUnavailable("signals.batch");
    },
    transactionAsync(callback) {
      return runWorkerFirstAsyncTransaction(
        rootSession,
        callback,
        "signals.transactionAsync",
      );
    },
    batchAsync(callback) {
      return runWorkerFirstAsyncTransaction(
        rootSession,
        callback,
        "signals.batchAsync",
      );
    },
    watch(target, callback) {
      return rootSession.watch(target, callback);
    },
    effect(target, callback) {
      return rootSession.effect(target, callback);
    },
    nuke(handle) {
      return rootSession.nuke(handle);
    },
    diagnostics() {
      diagnostics ??= createRootDiagnosticsFacade(rootSession);
      return diagnostics;
    },
    history() {
      history ??= createRootHistoryFacade(rootSession);
      return history;
    },
    specialist() {
      specialist ??= createRootSpecialistFacade(rootSession);
      return specialist;
    },
    adapters() {
      adapters ??= createRootAdaptersFacade(rootSession);
      return adapters;
    },
    compatibilityApp() {
      throwWorkerFirstCallableUnavailable("signals.compatibilityApp");
    },
    compatibilityRuntime() {
      throwWorkerFirstCallableUnavailable("signals.compatibilityRuntime");
    },
    free() {
      terminateTrackedGraphs(trackedGraphs);
      void rootSession.terminate();
    },
    [Symbol.dispose]() {
      terminateTrackedGraphs(trackedGraphs);
      void rootSession.terminate();
    },
  };

  return freezeObject(callableSignals);
}

function createUnavailableNamespace(operation) {
  return new Proxy(
    freezeObject({}),
    {
      get(_target, property) {
        void property;
        throwWorkerFirstCallableUnavailable(operation);
      },
      has() {
        return false;
      },
      ownKeys() {
        return [];
      },
      getOwnPropertyDescriptor() {
        return undefined;
      },
    },
  );
}

function terminateTrackedGraphs(trackedGraphs) {
  for (const graph of trackedGraphs) {
    trackedGraphs.delete(graph);
    void graph.terminate();
  }
}

function normalizeWorkerFirstInputOptions(options) {
  if (options === undefined) {
    return undefined;
  }
  const normalized = requireAuthoringOptions("input", options);
  forbidOpaqueIdOption("input", normalized);
  return normalized;
}

function throwWorkerFirstCallableUnavailable(operation) {
  const error = new Error(
    `${operation} is unavailable on the current worker-first callable surface; use deployment: "mainThreadCompatibility" for authoring and root runtime operations beyond imported graph hydration`,
  );
  error.name = "WorkerFirstCallableSurfaceUnavailable";
  error.code = "workerFirstCallableSurfaceUnavailable";
  error.compatibilityRecovery = Object.freeze({
    deployment: "mainThreadCompatibility",
    message:
      'Retry with deployment: "mainThreadCompatibility" to use the full callable root surface.',
  });
  throw error;
}
