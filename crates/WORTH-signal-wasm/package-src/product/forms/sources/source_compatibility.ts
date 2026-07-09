import { FormDeclarationError } from "../form_errors.js";
import { cloneFormValue, stableValueDigest } from "../values/value_paths.js";
import { readSourceDraftMigration, readSourceSchemaVersion } from "./form_sources.js";

const SOURCE_COMPATIBILITY_POSTURES = new Set([
  "notDeclared",
  "current",
  "compatible",
  "migrated",
  "unavailable",
]);

export function createSourceCompatibilityStore(sourceDeclaration) {
  let nextArtifactId = 1;
  let draftSchemaVersion = null;
  let lastUnavailableKey = null;
  let latestArtifact = null;
  const history = [];
  return Object.freeze({
    reconcile(rawSource, currentDraft) {
      const currentSchemaVersion = schemaVersion(rawSource);
      const draftPresent = draftExists(currentDraft);
      const previousDraftSchemaVersion = draftPresent ? draftSchemaVersion : null;
      if (!sourceDeclaresSchema(sourceDeclaration)) {
        latestArtifact = null;
        return Object.freeze({
          draft: currentDraft,
          report: sourceCompatibilityReport({
            posture: "notDeclared",
            currentSchemaVersion,
            draftSchemaVersion: previousDraftSchemaVersion,
            artifact: null,
            history,
          }),
        });
      }
      if (!draftPresent || previousDraftSchemaVersion === null || previousDraftSchemaVersion === currentSchemaVersion) {
        latestArtifact = null;
        return Object.freeze({
          draft: currentDraft,
          report: sourceCompatibilityReport({
            posture: "current",
            currentSchemaVersion,
            draftSchemaVersion: previousDraftSchemaVersion,
            artifact: null,
            history,
          }),
        });
      }
      const resolution = resolveSchemaDrift(
        sourceDeclaration,
        currentDraft,
        rawSource,
        previousDraftSchemaVersion,
        currentSchemaVersion,
      );
      if (resolution.kind === "unavailable") {
        if (lastUnavailableKey !== resolution.resolutionKey) {
          latestArtifact = pushArtifact(
            nextArtifactId++,
            "unavailable",
            previousDraftSchemaVersion,
            currentSchemaVersion,
            currentDraft,
            currentDraft,
            resolution.reason,
            resolution.resolutionKey,
          );
          lastUnavailableKey = resolution.resolutionKey;
        }
        return Object.freeze({
          draft: currentDraft,
          report: sourceCompatibilityReport({
            posture: "unavailable",
            currentSchemaVersion,
            draftSchemaVersion: previousDraftSchemaVersion,
            artifact: latestArtifact,
            history,
          }),
        });
      }
      const nextDraft = resolution.kind === "migrated"
        ? resolution.draft
        : currentDraft;
      latestArtifact = pushArtifact(
        nextArtifactId++,
        resolution.kind,
        previousDraftSchemaVersion,
        currentSchemaVersion,
        currentDraft,
        nextDraft,
        resolution.reason,
        resolution.resolutionKey,
      );
      draftSchemaVersion = currentSchemaVersion;
      lastUnavailableKey = null;
      return Object.freeze({
        draft: nextDraft,
        report: sourceCompatibilityReport({
          posture: resolution.kind,
          currentSchemaVersion,
          draftSchemaVersion,
          artifact: latestArtifact,
          history,
        }),
      });
    },
    noteDraft(nextDraft, rawSource) {
      if (!sourceDeclaresSchema(sourceDeclaration)) {
        return;
      }
      if (!draftExists(nextDraft)) {
        draftSchemaVersion = null;
        lastUnavailableKey = null;
        latestArtifact = null;
        return;
      }
      if (draftSchemaVersion === null) {
        draftSchemaVersion = schemaVersion(rawSource);
        lastUnavailableKey = null;
      }
    },
    history() {
      return Object.freeze([...history]);
    },
  });

  function schemaVersion(rawSource) {
    return readSourceSchemaVersion(sourceDeclaration) ?? stableValueDigest(rawSource);
  }

  function pushArtifact(
    artifactId,
    posture,
    previousSchemaVersion,
    currentSchemaVersion,
    previousDraft,
    nextDraft,
    reason,
    resolutionKey,
  ) {
    const artifact = Object.freeze({
      kind: "sourceCompatibility",
      artifactId,
      posture,
      previousSchemaVersion,
      currentSchemaVersion,
      previousDraftDigest: stableValueDigest(previousDraft),
      nextDraftDigest: stableValueDigest(nextDraft),
      nextDraft: posture === "migrated" ? cloneFormValue(nextDraft) : null,
      reason,
      resolutionKey,
      compatibilityDigest: stableValueDigest({
        posture,
        previousSchemaVersion,
        currentSchemaVersion,
        previousDraftDigest: stableValueDigest(previousDraft),
        nextDraftDigest: stableValueDigest(nextDraft),
        reason,
      }),
    });
    history.push(artifact);
    return artifact;
  }
}

export function sourceCompatibilityBlockers(report) {
  if (report.posture !== "unavailable") {
    return Object.freeze([]);
  }
  return Object.freeze([Object.freeze({
    kind: "schema:drift",
    reason: report.reason ?? "source schema changed and draft migration is unavailable",
    schemaVersion: report.currentSchemaVersion,
    previousSchemaVersion: report.draftSchemaVersion,
  })]);
}

function sourceCompatibilityReport({ posture, currentSchemaVersion, draftSchemaVersion, artifact, history }) {
  if (!SOURCE_COMPATIBILITY_POSTURES.has(posture)) {
    throw new FormDeclarationError("source compatibility posture is not supported", { posture });
  }
  const compatibleDrifts = history.filter((entry) => entry.posture === "compatible").length;
  const migrations = history.filter((entry) => entry.posture === "migrated").length;
  const unavailableDrifts = history.filter((entry) => entry.posture === "unavailable").length;
  return Object.freeze({
    posture,
    currentSchemaVersion,
    draftSchemaVersion,
    reason: artifact?.reason ?? null,
    artifact,
    counters: Object.freeze({
      costBasis: "sourceSchemaCompatibilityDerivedScan",
      incrementalStatus: "notIncremental",
      schemaReads: posture === "notDeclared" ? 0 : 1,
      migrations,
      compatibleDrifts,
      unavailableDrifts,
      historyArtifacts: history.length,
    }),
  });
}

function sourceDeclaresSchema(sourceDeclaration) {
  return readSourceSchemaVersion(sourceDeclaration) !== null;
}

function draftExists(draft) {
  return stableValueDigest(draft) !== "{}";
}

function resolveSchemaDrift(sourceDeclaration, currentDraft, rawSource, previousSchemaVersion, currentSchemaVersion) {
  const migrateDraft = readSourceDraftMigration(sourceDeclaration);
  const resolutionKey = stableValueDigest({
    previousSchemaVersion,
    currentSchemaVersion,
    currentDraft,
    rawSource,
  });
  if (migrateDraft === null) {
    return Object.freeze({
      kind: "unavailable",
      reason: "source schema changed and no draft migration policy is declared",
      resolutionKey,
    });
  }
  const result = migrateDraft(cloneFormValue(currentDraft), {
    previousSchemaVersion,
    currentSchemaVersion,
    source: cloneFormValue(rawSource),
  });
  if (result == null || result === true || result.kind === "compatible") {
    return Object.freeze({
      kind: "compatible",
      reason: typeof result?.reason === "string" ? result.reason : "draft remains compatible across source schema drift",
      resolutionKey,
    });
  }
  if (result.kind === "migrated") {
    return Object.freeze({
      kind: "migrated",
      draft: cloneFormValue(result.draft),
      reason: typeof result.reason === "string" ? result.reason : "draft was migrated to the current source schema",
      resolutionKey,
    });
  }
  if (result.kind === "unavailable") {
    return Object.freeze({
      kind: "unavailable",
      reason: typeof result.reason === "string" ? result.reason : "draft migration is unavailable for the current source schema",
      resolutionKey,
    });
  }
  throw new FormDeclarationError("source draft migration resolver returned an unsupported posture", {
    result,
  });
}
