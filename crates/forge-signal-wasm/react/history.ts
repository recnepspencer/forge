import { useMemo, useRef, useSyncExternalStore } from "react";

import type {
  BrowserHistoryStoryReactLike,
  BrowserHistoryStoryView,
  SignalsHistoryReactLike,
  SignalsHistoryView,
} from "./model.js";

function isBranchLike(value: unknown): value is {
  id: number;
  name: string;
  parent_branch_id: number | null;
  head_snapshot_id: number | null;
} {
  return !!value && typeof value === "object" && "id" in value && "name" in value;
}

function sameBranch(left: unknown, right: unknown): boolean {
  if (Object.is(left, right)) {
    return true;
  }
  if (!isBranchLike(left) || !isBranchLike(right)) {
    return false;
  }
  return left.id === right.id
    && left.name === right.name
    && left.parent_branch_id === right.parent_branch_id
    && left.head_snapshot_id === right.head_snapshot_id;
}

function sameBranchList(left: readonly unknown[], right: readonly unknown[]): boolean {
  if (left.length !== right.length) {
    return false;
  }
  for (let index = 0; index < left.length; index += 1) {
    if (!sameBranch(left[index], right[index])) {
      return false;
    }
  }
  return true;
}

function canUndoBranch(branch: unknown): boolean {
  return isBranchLike(branch) && branch.parent_branch_id !== null;
}

function canRedoBranch(branches: readonly unknown[], currentBranch: unknown): boolean {
  if (!isBranchLike(currentBranch)) {
    return false;
  }
  return branches.some((branch) =>
    isBranchLike(branch) && branch.parent_branch_id === currentBranch.id
  );
}

export function useSignalsHistory<
  TBranch = unknown,
  THistory extends SignalsHistoryReactLike<TBranch> = SignalsHistoryReactLike<TBranch>,
>(
  history: THistory,
): SignalsHistoryView<TBranch> {
  const cacheRef = useRef<{
    hasValue: boolean;
    currentBranch: TBranch | null;
    branches: readonly TBranch[];
    snapshot: SignalsHistoryView<TBranch> | null;
  }>({
    hasValue: false,
    currentBranch: null,
    branches: [],
    snapshot: null,
  });

  const subscribe = useMemo(
    () => (listener: () => void) => history.subscribe(listener),
    [history],
  );
  const getSnapshot = useMemo(
    () => () => {
      const currentBranch = history.current_branch();
      const branches = history.branches();
      const cache = cacheRef.current;
      if (
        cache.hasValue
        && sameBranch(cache.currentBranch, currentBranch)
        && sameBranchList(cache.branches as readonly unknown[], branches as readonly unknown[])
      ) {
        return cache.snapshot as SignalsHistoryView<TBranch>;
      }
      const canUndo = canUndoBranch(currentBranch);
      const canRedo = canRedoBranch(branches as readonly unknown[], currentBranch);
      const snapshot = Object.freeze({
        currentBranch,
        branches,
        canUndo,
        canRedo,
      });
      cache.hasValue = true;
      cache.currentBranch = currentBranch;
      cache.branches = branches;
      cache.snapshot = snapshot;
      return snapshot;
    },
    [history],
  );

  return useSyncExternalStore(subscribe, getSnapshot, getSnapshot);
}

export function useBrowserHistoryStory<
  TEntry = unknown,
  TEvent = unknown,
  TBreadcrumbTrail = unknown,
  TBackProvenance = unknown,
  TStory extends BrowserHistoryStoryReactLike<
    TEntry,
    TEvent,
    TBreadcrumbTrail,
    TBackProvenance
  > = BrowserHistoryStoryReactLike<TEntry, TEvent, TBreadcrumbTrail, TBackProvenance>,
>(
  story: TStory,
): BrowserHistoryStoryView<TEntry, TEvent, TBreadcrumbTrail, TBackProvenance> {
  const cacheRef = useRef<{
    latestBoundaryEvent: TEvent | null | undefined;
    snapshot:
      | BrowserHistoryStoryView<TEntry, TEvent, TBreadcrumbTrail, TBackProvenance>
      | null;
  }>({
    latestBoundaryEvent: undefined,
    snapshot: null,
  });

  const subscribe = useMemo(
    () => (listener: () => void) => story.subscribe(listener),
    [story],
  );
  const getSnapshot = useMemo(
    () => () => {
      const latestBoundaryEvent = story.latestBoundaryEvent();
      const cache = cacheRef.current;
      if (
        cache.snapshot !== null
        && Object.is(cache.latestBoundaryEvent, latestBoundaryEvent)
      ) {
        return cache.snapshot;
      }
      const snapshot = Object.freeze({
        current: story.current(),
        entries: story.admittedEntries(),
        breadcrumbTrail: story.breadcrumbTrail(),
        backProvenance: story.backProvenance(),
        events: story.events(),
      });
      cache.latestBoundaryEvent = latestBoundaryEvent;
      cache.snapshot = snapshot;
      return snapshot;
    },
    [story],
  );

  return useSyncExternalStore(subscribe, getSnapshot, getSnapshot);
}
