declare const WORTHSignalRawLocationAuthorityBrand: unique symbol;
declare const WORTHSignalCanonicalUrlAuthorityBrand: unique symbol;
declare const WORTHSignalRawLocationVerificationPackageBrand: unique symbol;
declare const WORTHSignalCanonicalUrlVerificationPackageBrand: unique symbol;

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
  readonly [WORTHSignalRawLocationVerificationPackageBrand]: "rawLocationVerificationPackage";
}

export interface CanonicalUrlVerificationPackage {
  readonly canonicalUrlDigest: string;
  readonly equivalenceDigest: string;
  readonly searchDigest: string;
  readonly hashDigest: string;
  readonly [WORTHSignalCanonicalUrlVerificationPackageBrand]: "canonicalUrlVerificationPackage";
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
  readonly [WORTHSignalCanonicalUrlAuthorityBrand]: "canonicalUrlAuthority";
}

export interface RawLocationAuthority {
  readonly href: string;
  readonly pathname: string;
  readonly searchParams: ReadonlyArray<UrlSearchParamEntry>;
  readonly hashFragment: string | undefined;
  readonly navigationType: RawLocationNavigationType;
  canonical(): CanonicalUrlAuthority;
  verification(): RawLocationVerificationPackage;
  readonly [WORTHSignalRawLocationAuthorityBrand]: "rawLocationAuthority";
}
