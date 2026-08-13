import { buildActiveImportContext } from "./sessions/support/worker_first_root_import_context.js";

/**
 * Admit a portable runtime envelope into the shared worker-first root session.
 */
export function beginWorkerFirstRootExactImport(deps, definition, snapshot, controller) {
  const portableWire = snapshot?.runtimeEnvelope?.runtimeEnvelopePortableWire;
  if (typeof portableWire !== "string") {
    throw new TypeError(
      "worker-first root importGraph(...) requires a snapshot.runtimeEnvelope artifact returned by adapters.exportRuntimeEnvelope()",
    );
  }
  deps.invalidateActiveImport(
    "worker-first imported graph was superseded by a newer root importGraph() call",
  );
  deps.authoredRuntime.invalidate(
    "worker-first imported graph importGraph(...) replaced the worker-owned runtime",
  );
  deps.setActiveImportController(controller);
  const importPromise = deps.importChain.then(async () => {
    deps.requireActive("importGraph");
    await deps.authoredRuntime.settlePendingPublications();
    await deps.ready();
    await deps.observations.clearContext(deps.bridge);
    deps.requireControllerActive(controller, "importGraph");
    const report = await deps.bridge.admitWorkerRuntimeEnvelopeImportPortableWire(portableWire);
    if (report?.importOutcome !== "Admitted") {
      throw new TypeError(
        `worker-first root importGraph(...) could not admit the portable runtime envelope: ${report?.importOutcome ?? "Unknown"}`,
      );
    }
    await deps.hostCapabilities.replayCurrentIngress();
    const activeImportContext = await buildActiveImportContext(
      deps.bridge,
      definition,
      snapshot,
    );
    deps.setActiveImportContext(activeImportContext);
    await deps.observations.replaceContext(deps.bridge, activeImportContext);
    await deps.publishDiagnosticsChanged();
    deps.requireControllerActive(controller, "importGraph");
  });
  deps.setImportChain(importPromise.catch(() => {}));
  return importPromise;
}
