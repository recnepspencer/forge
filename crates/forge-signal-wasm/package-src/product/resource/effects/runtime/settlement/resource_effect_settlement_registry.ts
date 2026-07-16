function createResourceEffectSettlementRegistry() {
  const responses = new Map();
  const responseByEffect = new Map();
  const terminals = new Map();

  return Object.freeze({
    begin(effectId, settlement) {
      const identity = settlementIdentity(effectId, settlement);
      const recorded = responses.get(identity.responseId);
      if (recorded !== undefined) {
        requireSameResponse(recorded, identity);
        if (recorded.retryable && recorded.checkpoint !== null) {
          recorded.retryable = false;
          return Object.freeze({
            kind: "resume",
            token: recorded.identity,
            checkpoint: recorded.checkpoint,
          });
        }
        return Object.freeze({
          kind: "duplicate",
          receipt: duplicateReceipt(recorded.receipt, identity),
        });
      }
      const existingResponseId = responseByEffect.get(effectId);
      if (existingResponseId !== undefined) {
        const existing = requireResponse(responses, existingResponseId);
        requireSameOutcome(existing.identity, identity);
        return Object.freeze({
          kind: "duplicate",
          receipt: duplicateReceipt(existing.receipt, identity),
        });
      }
      const terminal = terminals.get(effectId);
      if (terminal !== undefined) {
        requireSameOutcome(terminal.identity, identity);
        return Object.freeze({
          kind: "duplicate",
          receipt: duplicateReceipt(terminal.receipt, identity),
        });
      }
      const token = Object.freeze({ ...identity, effectId });
      responses.set(identity.responseId, {
        identity: token,
        receipt: null,
        checkpoint: null,
        retryable: false,
      });
      responseByEffect.set(effectId, identity.responseId);
      return Object.freeze({ kind: "admitted", token });
    },
    record(token, receipt) {
      const response = requireResponse(responses, token.responseId);
      response.receipt = receipt;
      return receipt;
    },
    checkpoint(token, checkpoint) {
      const response = requireResponse(responses, token.responseId);
      response.checkpoint = checkpoint;
      response.retryable = false;
      return checkpoint;
    },
    terminal(token, receipt) {
      const response = requireResponse(responses, token.responseId);
      response.receipt = receipt;
      response.retryable = false;
      terminals.set(token.effectId, Object.freeze({ identity: token, receipt }));
      return receipt;
    },
    terminalCancellation(effectId, receipt) {
      const terminal = terminals.get(effectId);
      if (terminal !== undefined) {
        return terminal.receipt;
      }
      const recordedResponseId = responseByEffect.get(effectId);
      const recorded = recordedResponseId === undefined
        ? undefined
        : responses.get(recordedResponseId);
      const identity = recorded?.identity ?? Object.freeze({
        effectId,
        outcome: "rejected",
        responseId: `${effectId}:dependencyCancellation`,
      });
      if (recorded !== undefined) {
        recorded.receipt = receipt;
      } else {
        responses.set(identity.responseId, {
          identity,
          receipt,
          checkpoint: null,
          retryable: false,
        });
        responseByEffect.set(effectId, identity.responseId);
      }
      terminals.set(effectId, Object.freeze({ identity, receipt }));
      return receipt;
    },
    cancel(token) {
      const response = responses.get(token.responseId);
      if (response?.checkpoint !== null) {
        response.retryable = true;
      } else if (response?.receipt === null) {
        responses.delete(token.responseId);
        responseByEffect.delete(token.effectId);
      }
    },
  });
}

function settlementIdentity(effectId, settlement) {
  const outcome = settlement.kind === "rejected" ? "rejected" : "confirmed";
  const responseId = settlement.responseId ?? `${effectId}:${outcome}`;
  if (typeof responseId !== "string" || responseId.length === 0) {
    throw settlementDenial(
      "invalidResponseIdentity",
      effectId,
      "resource effect settlement responseId must be a non-empty string",
    );
  }
  return Object.freeze({ effectId, outcome, responseId });
}

function requireSameResponse(recorded, candidate) {
  if (
    recorded.identity.effectId === candidate.effectId
    && recorded.identity.outcome === candidate.outcome
  ) {
    return;
  }
  throw settlementDenial(
    "responseIdentityConflict",
    candidate.effectId,
    `resource effect response ${candidate.responseId} is already bound to ${recorded.identity.effectId}:${recorded.identity.outcome}`,
  );
}

function requireSameOutcome(recorded, candidate) {
  if (recorded.outcome === candidate.outcome) {
    return;
  }
  throw settlementDenial(
    "terminalOutcomeConflict",
    candidate.effectId,
    `resource effect ${candidate.effectId} is already terminal as ${recorded.outcome}`,
  );
}

function duplicateReceipt(receipt, identity) {
  return Object.freeze({
    kind: "duplicateSettlement",
    effectId: identity.effectId,
    responseId: identity.responseId,
    originalKind: receipt?.kind ?? "responseInFlight",
    originalReceipt: receipt,
  });
}

function requireResponse(responses, responseId) {
  const response = responses.get(responseId);
  if (response === undefined) {
    throw new TypeError(`unknown resource effect response ${responseId}`);
  }
  return response;
}

function settlementDenial(code, effectId, detail) {
  const error = new TypeError(detail);
  error.name = "ResourceEffectSettlementDenial";
  error.code = code;
  error.effectId = effectId;
  return error;
}

export { createResourceEffectSettlementRegistry };
