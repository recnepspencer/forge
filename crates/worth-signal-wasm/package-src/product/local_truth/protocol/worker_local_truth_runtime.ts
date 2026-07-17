import { createLocalTruthAuthority } from "../authority/local_truth_authority.js";
import { createLocalTruthSignalProjection } from "../projection/signal_projection.js";
import { createWorkerSignalProjectionDriver } from "../projection/worker_signal_driver.js";
import { restoreDeclaredSchema } from "../schema/schema_declaration.js";
import { canonicalDigest } from "../support/canonical.js";

const MAX_REPLAY_RESULTS = 256;

export function createWorkerLocalTruthRuntime(runtime) {
  const registrations = new Map();
  return Object.freeze({
    async command(envelope) {
      requireEnvelope(envelope);
      if (envelope.operation === "create") {
        return createRegistration(runtime, registrations, envelope);
      }
      const registration = registrations.get(envelope.authorityId);
      if (!registration || registration.registrationId !== envelope.registrationId) {
        throw new TypeError("worker local truth authority registration is unavailable or foreign");
      }
      if (envelope.operation === "terminate") {
        admitSequence(registration, envelope);
        registrations.delete(envelope.authorityId);
        await registration.authority.terminate();
        return { posture: "success", value: null };
      }
      const replay = admitSequence(registration, envelope);
      if (replay) return replay;
      const method = registration.authority[envelope.operation];
      if (typeof method !== "function") {
        throw new TypeError(`unsupported worker local truth operation ${envelope.operation}`);
      }
      const result = await method.call(registration.authority, envelope.request);
      rememberResult(registration, envelope, result);
      return result;
    },
  });
}

async function createRegistration(runtime, registrations, envelope) {
  const existing = registrations.get(envelope.authorityId);
  if (existing) {
    if (
      existing.registrationId === envelope.registrationId
      && envelope.sequence === 0
      && existing.createDigest === commandDigest(envelope)
    ) {
      return existing.createResult;
    }
    throw new TypeError(`worker local truth authority ${envelope.authorityId} already exists`);
  }
  if (envelope.sequence !== 0) {
    throw new TypeError("worker local truth create command must use sequence zero");
  }
  const schema = restoreDeclaredSchema(envelope.request.schema);
  const projection = createLocalTruthSignalProjection({
    schema,
    bindings: envelope.request.bindings,
    driver: createWorkerSignalProjectionDriver(runtime),
  });
  const authority = createLocalTruthAuthority(
    { ...envelope.request, schema },
    {
      faultInjector: null,
      onInitialize: (branch, snapshot) => projection.initialize(branch, snapshot),
      onBranchFork: (branch, parent) => projection.fork(branch, parent),
      onCommitted: (commit, snapshot) => projection.project(commit, snapshot),
      projection,
      acceptSerializedBases: true,
    },
  );
  const inspection = await authority.inspect();
  const result = {
    inspection,
    initialDerivation: projection.posture("branch:main"),
  };
  registrations.set(envelope.authorityId, {
    registrationId: envelope.registrationId,
    authority,
    createDigest: commandDigest(envelope),
    createResult: result,
    nextSequence: 1,
    results: new Map(),
  });
  return result;
}

function requireEnvelope(envelope) {
  if (
    !envelope
    || typeof envelope.authorityId !== "string"
    || typeof envelope.registrationId !== "string"
    || typeof envelope.operation !== "string"
    || !Number.isSafeInteger(envelope.sequence)
    || envelope.sequence < 0
  ) {
    throw new TypeError("worker local truth command envelope is invalid");
  }
}

function admitSequence(registration, envelope) {
  const digest = commandDigest(envelope);
  if (envelope.sequence < registration.nextSequence) {
    const prior = registration.results.get(envelope.sequence);
    if (prior?.digest === digest) return prior.result;
    throw new TypeError("worker local truth command sequence was replayed with different content");
  }
  if (envelope.sequence !== registration.nextSequence) {
    throw new TypeError("worker local truth command sequence is out of order");
  }
  return null;
}

function rememberResult(registration, envelope, result) {
  registration.results.set(envelope.sequence, { digest: commandDigest(envelope), result });
  registration.nextSequence += 1;
  if (registration.results.size > MAX_REPLAY_RESULTS) {
    registration.results.delete(registration.nextSequence - MAX_REPLAY_RESULTS - 1);
  }
}

function commandDigest(envelope) {
  return canonicalDigest({
    authorityId: envelope.authorityId,
    registrationId: envelope.registrationId,
    sequence: envelope.sequence,
    operation: envelope.operation,
    request: envelope.request ?? null,
  });
}
