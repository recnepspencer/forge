import { reserveAuthoringSignalId } from "./scopes.js";

function withReservedSignalId(rawSignals, family, id, callback) {
  const release = reserveAuthoringSignalId(rawSignals, family, id);
  try {
    return callback();
  } catch (error) {
    release();
    throw error;
  }
}

export { withReservedSignalId };
