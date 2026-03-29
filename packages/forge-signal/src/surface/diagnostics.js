export class SignalDiagnostics {
  constructor(inner) {
    this.inner = inner;
  }

  why(id) {
    return this.inner.why(id);
  }

  health() {
    return this.inner.health();
  }
}
