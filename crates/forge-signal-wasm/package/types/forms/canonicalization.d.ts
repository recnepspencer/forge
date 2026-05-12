import type { SignalValue } from "../model.js";

export interface FormCanonicalizationArtifact {
  readonly kind: "canonicalization";
  readonly canonicalizationId: number;
  readonly operationId: number;
  readonly action: string | null;
  readonly planDigest: string | null;
  readonly previousSourceDigest: string;
  readonly previousDraftDigest: string;
  readonly sourceBasisDigest: string;
  readonly canonicalSourceDigest: string;
  readonly canonicalValue: SignalValue;
  readonly draftReset: true;
  readonly sourceProjection: "serverCanonicalUntilAuthoritativeSourceDrift";
  readonly reason: string;
  readonly canonicalizationDigest: string;
}
