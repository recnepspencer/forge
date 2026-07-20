import { deepFreeze } from "./canonical.js";

export function success(value) {
  return deepFreeze({ posture: "success", value });
}

export function reviewRequired(review) {
  return deepFreeze({ posture: "reviewRequired", review });
}

export function denied(code, message, evidence = null) {
  return deepFreeze({ posture: "denied", code, message, evidence });
}

export function unavailable(code, message, evidence = null) {
  return deepFreeze({ posture: "unavailable", code, message, evidence });
}

export function failed(code, message, evidence = null) {
  return deepFreeze({ posture: "failed", code, message, evidence });
}

export function advisory(value, advisories) {
  return deepFreeze({ posture: "advisory", value, advisories });
}
