import {
  decodeDefinitions,
  decodeRuntimeEnvelope,
  normalizeRuntimeEnvelope,
} from "../internal/codec.ts";

export class SignalAdapters {
  inner: any;

  constructor(inner: any) {
    this.inner = inner;
  }

  exportDefinitions() {
    return decodeDefinitions(this.inner.export_definitions());
  }

  exportRuntimeEnvelope() {
    return decodeRuntimeEnvelope(this.inner.export_runtime_envelope());
  }

  replaceRuntimeEnvelope(envelope: any) {
    return this.inner.replace_runtime_envelope(normalizeRuntimeEnvelope(envelope));
  }
}
