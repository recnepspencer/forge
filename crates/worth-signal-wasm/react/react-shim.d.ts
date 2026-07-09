declare module "react" {
  export type ReactNode =
    | JSX.Element
    | string
    | number
    | boolean
    | null
    | undefined
    | readonly ReactNode[];

  export namespace JSX {
    interface Element {}
    interface IntrinsicElements {
      [elementName: string]: unknown;
    }
  }

  export interface Context<TValue> {
    Provider(props: {
      readonly value: TValue;
      readonly children?: ReactNode;
    }): ReactNode;
  }

  export interface MutableRefObject<TValue> {
    current: TValue;
  }

  export type Dispatch<TValue> = (value: TValue | ((previous: TValue) => TValue)) => void;

  export function createContext<TValue>(defaultValue: TValue): Context<TValue>;
  export function useContext<TValue>(context: Context<TValue>): TValue;
  export function useMemo<T>(factory: () => T, deps: ReadonlyArray<unknown>): T;
  export function useRef<T>(initialValue: T): MutableRefObject<T>;
  export function useState<T>(initialValue: T): [T, Dispatch<T>];
  export function useCallback<TCallback extends (...args: never[]) => unknown>(
    callback: TCallback,
    deps: ReadonlyArray<unknown>,
  ): TCallback;
  export function useEffect(
    effect: () => void | (() => void),
    deps?: ReadonlyArray<unknown>,
  ): void;
  export function useSyncExternalStore<T>(
    subscribe: (listener: () => void) => () => void,
    getSnapshot: () => T,
    getServerSnapshot?: () => T,
  ): T;

  const React: {
    readonly createElement: (...args: readonly unknown[]) => JSX.Element;
    readonly createContext: typeof createContext;
    readonly useContext: typeof useContext;
    readonly useMemo: typeof useMemo;
    readonly useRef: typeof useRef;
    readonly useState: typeof useState;
    readonly useCallback: typeof useCallback;
    readonly useEffect: typeof useEffect;
    readonly useSyncExternalStore: typeof useSyncExternalStore;
  };

  export default React;
}
