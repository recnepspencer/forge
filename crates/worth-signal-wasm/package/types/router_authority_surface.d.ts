declare const WorthSignalRawLocationAuthorityBrand: unique symbol;
declare const WorthSignalCanonicalUrlAuthorityBrand: unique symbol;
declare const WorthSignalRawLocationVerificationPackageBrand: unique symbol;
declare const WorthSignalCanonicalUrlVerificationPackageBrand: unique symbol;

export type RawLocationNavigationType =
  | "load"
  | "push"
  | "replace"
  | "pop"
  | "manual"
  | "external";

export interface RawLocationOptions {
  navigationType?: RawLocationNavigationType;
}

export interface UrlSearchParamEntry {
  readonly key: string;
  readonly value: string;
}

export interface RawLocationVerificationPackage {
  readonly rawLocationDigest: string;
  readonly canonicalUrlDigest: string;
  readonly equivalenceDigest: string;
  readonly [WorthSignalRawLocationVerificationPackageBrand]: "rawLocationVerificationPackage";
}

export interface CanonicalUrlVerificationPackage {
  readonly canonicalUrlDigest: string;
  readonly equivalenceDigest: string;
  readonly searchDigest: string;
  readonly hashDigest: string;
  readonly [WorthSignalCanonicalUrlVerificationPackageBrand]: "canonicalUrlVerificationPackage";
}

export interface CanonicalUrlAuthority {
  readonly href: string;
  readonly pathname: string;
  readonly searchParams: ReadonlyArray<UrlSearchParamEntry>;
  readonly hashFragment: string | undefined;
  readonly canonicalUrlDigest: string;
  readonly equivalenceDigest: string;
  readonly searchDigest: string;
  readonly hashDigest: string;
  verification(): CanonicalUrlVerificationPackage;
  readonly [WorthSignalCanonicalUrlAuthorityBrand]: "canonicalUrlAuthority";
}

export interface RawLocationAuthority {
  readonly href: string;
  readonly pathname: string;
  readonly searchParams: ReadonlyArray<UrlSearchParamEntry>;
  readonly hashFragment: string | undefined;
  readonly navigationType: RawLocationNavigationType;
  canonical(): CanonicalUrlAuthority;
  verification(): RawLocationVerificationPackage;
  readonly [WorthSignalRawLocationAuthorityBrand]: "rawLocationAuthority";
}
