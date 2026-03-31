export class SignalDiagnostics {
  inner: any;

  constructor(inner: any) {
    this.inner = inner;
  }

  why(id: string) {
    return this.inner.why(id);
  }

  health() {
    return this.inner.health();
  }
}
