export function createGraphExportImportRuntime() {
  const values = new Map();
  const sourceIds = new Set();
  const callbackRecipes = new Map();
  const projectionRecipeReads = new Map();

  function cloneValue(value) {
    if (typeof globalThis.structuredClone === "function") {
      try {
        return globalThis.structuredClone(value);
      } catch {
        return value;
      }
    }
    if (Array.isArray(value)) {
      return value.slice();
    }
    if (value && typeof value === "object") {
      return { ...value };
    }
    return value;
  }

  function recomputeDerivedValues() {
    for (const [id, recipe] of callbackRecipes) {
      const result = recipe.callback();
      const value = result?.__WorthSignalCallbackCapture
        ? result.value
        : result;
      if (result?.__WorthSignalCallbackCapture) {
        recipe.reads = [...result.reads];
      }
      values.set(id, cloneValue(value));
    }
    for (const [id, sourceId] of projectionRecipeReads) {
      values.set(id, cloneValue(values.get(sourceId)));
    }
  }

  function exportDefinitions() {
    return {
      policy: { preset: "webDevelopment" },
      sources: [...sourceIds].map((id) => ({ id, initial: null })),
      recipes: [
        ...[...callbackRecipes.entries()].map(([id, recipe]) => ({
          id,
          reads: recipe.reads,
        })),
        ...[...projectionRecipeReads.entries()].map(([id, sourceId]) => ({
          id,
          reads: [sourceId],
        })),
      ],
      sourceFamilies: [],
      recipeFamilies: [],
      unavailableCallbacks: [],
    };
  }

  function exportRuntimeEnvelope() {
    return {
      values: Object.fromEntries(
        [...values.entries()].map(([id, value]) => [id, cloneValue(value)]),
      ),
      definitions: exportDefinitions(),
    };
  }

  function restoreRuntimeEnvelope(envelope) {
    values.clear();
    sourceIds.clear();
    callbackRecipes.clear();
    projectionRecipeReads.clear();
    for (const [id, value] of Object.entries(envelope?.values ?? {})) {
      values.set(id, cloneValue(value));
    }
    for (const source of envelope?.definitions?.sources ?? []) {
      if (typeof source?.id === "string" && source.id.length > 0) {
        sourceIds.add(source.id);
      }
    }
    for (const recipe of envelope?.definitions?.recipes ?? []) {
      if (typeof recipe?.id !== "string" || recipe.id.length === 0) {
        continue;
      }
      if (
        Array.isArray(recipe.reads) &&
        recipe.reads.length === 1 &&
        typeof recipe.reads[0] === "string" &&
        recipe.id.includes(".")
      ) {
        projectionRecipeReads.set(recipe.id, recipe.reads[0]);
        continue;
      }
      callbackRecipes.set(recipe.id, {
        callback() {
          return values.get(recipe.id);
        },
        reads: Array.isArray(recipe.reads) ? [...recipe.reads] : [],
      });
    }
  }

  return {
    input(id, initial) {
      sourceIds.add(id);
      values.set(id, cloneValue(initial));
      return {
        id,
        get() {
          return values.get(id);
        },
        peek() {
          return values.get(id);
        },
        free() {},
      };
    },
    computedSpec(id, spec) {
      if (spec?.expr?.kind === "read" && typeof spec.expr.id === "string") {
        projectionRecipeReads.set(id, spec.expr.id);
        values.set(id, cloneValue(values.get(spec.expr.id)));
      }
      return {
        id,
        get() {
          const sourceId = projectionRecipeReads.get(id);
          return sourceId ? values.get(sourceId) : values.get(id);
        },
        peek() {
          const sourceId = projectionRecipeReads.get(id);
          return sourceId ? values.get(sourceId) : values.get(id);
        },
        free() {},
      };
    },
    computedCallback(id, callback) {
      const result = callback();
      callbackRecipes.set(id, {
        callback,
        reads: result?.__WorthSignalCallbackCapture ? [...result.reads] : [],
      });
      values.set(
        id,
        cloneValue(
          result?.__WorthSignalCallbackCapture ? result.value : result,
        ),
      );
      return {
        id,
        get() {
          return values.get(id);
        },
        peek() {
          return values.get(id);
        },
        free() {},
      };
    },
    outputSpec(id, spec) {
      const sourceId = spec?.expr?.id;
      if (typeof sourceId === "string") {
        projectionRecipeReads.set(id, sourceId);
        values.set(id, cloneValue(values.get(sourceId)));
      }
      return {
        id,
        get() {
          const currentSourceId = projectionRecipeReads.get(id);
          return currentSourceId ? values.get(currentSourceId) : values.get(id);
        },
        peek() {
          const currentSourceId = projectionRecipeReads.get(id);
          return currentSourceId ? values.get(currentSourceId) : values.get(id);
        },
        free() {},
      };
    },
    read(target) {
      const id = typeof target === "string" ? target : target.id;
      return values.get(id);
    },
    watch() {
      return { free() {} };
    },
    effect() {
      return { free() {} };
    },
    transaction(callback) {
      callback({
        set(target, value) {
          values.set(target.id, cloneValue(value));
        },
        setWithAspects(target, value) {
          values.set(target.id, cloneValue(value));
        },
        setWithRegions(target, value) {
          values.set(target.id, cloneValue(value));
        },
        setWithRegionsAndAspects(target, value) {
          values.set(target.id, cloneValue(value));
        },
        free() {},
      });
      recomputeDerivedValues();
      return { committed: true };
    },
    batch(callback) {
      return this.transaction(callback);
    },
    nuke() {
      return true;
    },
    diagnostics() {
      return {
        subscribe() {
          return { free() {} };
        },
        why(id) {
          return { id, family: "why" };
        },
        health() {
          return null;
        },
        summaryNow() {
          return { profile: "WebDevelopment", active_node_count: values.size };
        },
        historyNow() {
          return {
            history: {
              profile: "WebDevelopment",
              traced_node_count: values.size,
              execution_record_count: values.size,
              latest_execution_record_id: values.size,
              reuse_origin_counts: {},
              nodes: [],
            },
            callbackNodes: [],
          };
        },
        latestObservation() {
          return null;
        },
        latestFlow() {
          return null;
        },
        performanceSummary() {
          return {};
        },
        latestFailure() {
          return null;
        },
        latestRollback() {
          return null;
        },
        latestInvalidationPlanningEstimate() {
          return null;
        },
        latestInvalidationTraceRecords() {
          return [];
        },
        recentHistory() {
          return [];
        },
      };
    },
    history() {
      return {
        replay_for(id) {
          return { id, family: "replay", frames: [{ id }] };
        },
        lineage_for(id) {
          return { id, family: "lineage" };
        },
        snapshot() {
          return { snapshot: { meta: { branch_id: 0 } } };
        },
        snapshot_wire() {
          return JSON.stringify({ snapshot: { meta: { branch_id: 0 } } });
        },
        snapshot_portable_wire() {
          return JSON.stringify({
            portable: true,
            snapshot: { meta: { branch_id: 0 } },
          });
        },
        free() {},
      };
    },
    specialist() {
      return {
        read_versions(ids) {
          return ids.map((id, index) => ({ id, version: index + 1 }));
        },
        free() {},
      };
    },
    adapters() {
      return {
        export_definitions() {
          return exportDefinitions();
        },
        export_runtime_envelope() {
          return exportRuntimeEnvelope();
        },
        export_runtime_envelope_wire() {
          return JSON.stringify(exportRuntimeEnvelope());
        },
        export_runtime_envelope_portable_wire() {
          return JSON.stringify({ portable: true, ...exportRuntimeEnvelope() });
        },
        replace_runtime_envelope(envelope) {
          restoreRuntimeEnvelope(envelope);
        },
        replace_runtime_envelope_wire(envelope) {
          restoreRuntimeEnvelope(JSON.parse(envelope));
        },
        replace_runtime_envelope_portable_wire(envelope) {
          restoreRuntimeEnvelope(JSON.parse(envelope));
        },
        runtime_proof_report() {
          return { kind: "proof" };
        },
        free() {},
      };
    },
    compatibilityApp() {
      return { family: "app" };
    },
    compatibilityRuntime() {
      return { family: "runtime" };
    },
    free() {},
  };
}
