import {
  decodeDefinitions,
  decodeRuntimeEnvelope,
  normalizeRuntimeEnvelope
} from "../internal/codec.js";

export class SignalAdapters {
  constructor(inner) {
    this.inner = inner;
  }

  exportDefinitions() {
    return decodeDefinitions(this.inner.export_definitions());
  }

  exportRuntimeEnvelope() {
    return decodeRuntimeEnvelope(this.inner.export_runtime_envelope());
  }

  replaceRuntimeEnvelope(envelope) {
    return this.inner.replace_runtime_envelope(normalizeRuntimeEnvelope(envelope));
  }
}
