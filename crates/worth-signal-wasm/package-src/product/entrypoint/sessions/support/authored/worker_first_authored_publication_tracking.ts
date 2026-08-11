/**
 * Per-id eager publication readiness for worker-first authored signals.
 * Settle must reject when any tracked publication fails — never resolve-away.
 */

export function createAuthoredPublicationTracker() {
  const pendingPublications = new Set();
  const publicationById = new Map();

  function trackPendingPublication(ids, publication, onSettled) {
    const idList = Array.isArray(ids) ? ids : [ids];
    const tracked = Promise.resolve(publication).then(
      () => {
        onSettled("ready");
      },
      (error) => {
        onSettled("failed", error);
        throw error;
      },
    ).finally(() => {
      pendingPublications.delete(tracked);
      for (const id of idList) {
        if (publicationById.get(id) === tracked) {
          publicationById.delete(id);
        }
      }
    });
    for (const id of idList) {
      publicationById.set(id, tracked);
    }
    pendingPublications.add(tracked);
    return tracked;
  }

  async function settlePendingPublications() {
    while (pendingPublications.size > 0) {
      await Promise.all([...pendingPublications]);
    }
  }

  async function awaitPublication(id) {
    const tracked = publicationById.get(id);
    if (tracked) {
      await tracked;
    }
  }

  function isPublicationPending(id) {
    return publicationById.has(id);
  }

  return Object.freeze({
    trackPendingPublication,
    settlePendingPublications,
    awaitPublication,
    isPublicationPending,
  });
}

export function markAuthoredPublicationReady(state) {
  if (state && state.publicationState === "pending") {
    state.publicationState = "ready";
  }
}

export function markAuthoredPublicationFailed(state, message) {
  if (!state) {
    return;
  }
  state.publicationState = "failed";
  if (state.invalidatedMessage === null) {
    state.invalidatedMessage = message;
  }
}

export function isAuthoredPublicationReady(state) {
  return state != null
    && state.invalidatedMessage === null
    && state.publicationState === "ready";
}
