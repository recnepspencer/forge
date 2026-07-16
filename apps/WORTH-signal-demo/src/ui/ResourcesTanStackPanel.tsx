import { useEffect, useMemo, useRef, useState } from "react";
import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query";

import { CacheTimelineStrip, type CacheEventDatum, type WrongWindow } from "./ResourcesModelStrips";
import { PlatformOwner, PoPanel } from "./ResourcesSectionParts";
import { pushPanelEvent, type PanelController, type PanelProps } from "./resourcesSectionPanels";
import {
  HEALING_REFETCH_DELAY_MS,
  computeAgreement,
  createPanelEvent,
  createPoServer,
  type PanelEvent,
  type PoLine,
} from "./resourcesSectionSupport";

function cacheEventId(): string {
  return `${Date.now().toString(36)}-${Math.random().toString(36).slice(2, 8)}`;
}

export function ResourcesTanStackPanel({
  baseMs,
  highlightId,
  onAgreement,
  onController,
  phase,
  serverTruth,
}: PanelProps) {
  const store = useMemo(() => createPoServer(), []);
  const queryClient = useQueryClient();
  const [events, setEvents] = useState<PanelEvent[]>([]);
  const [cacheEvents, setCacheEvents] = useState<CacheEventDatum[]>([]);
  const [wrongWindows, setWrongWindows] = useState<WrongWindow[]>([]);
  const dependentsByLine = useRef(new Map<string, Set<string>>());
  const snapshotAtByLine = useRef(new Map<string, number>());
  const wasFetchingAfterMount = useRef(false);
  const refetchEvidencePending = useRef(false);

  const pushCacheEvent = (event: Omit<CacheEventDatum, "id">) => {
    setCacheEvents((current) => [...current, { ...event, id: cacheEventId() }]);
  };

  const query = useQuery({
    queryKey: ["po", "lines"],
    queryFn: async () => {
      const hasCachedValue = queryClient.getQueryData(["po", "lines"]) !== undefined;
      const fetched = await store.fetchLines(hasCachedValue ? HEALING_REFETCH_DELAY_MS : undefined);
      if (wasFetchingAfterMount.current) {
        wasFetchingAfterMount.current = false;
        refetchEvidencePending.current = true;
        const completedAt = performance.now();
        pushCacheEvent({ kind: "refetch", atMs: completedAt, label: "refetch all" });
        setWrongWindows((current) => (current.length > 0 && current[current.length - 1].toMs === null
          ? [...current.slice(0, -1), { ...current[current.length - 1], toMs: completedAt }]
          : current));
      }
      return fetched;
    },
  });

  const mutation = useMutation({
    mutationFn: (line: PoLine) => store.save(line),
    onMutate: async (line) => {
      await queryClient.cancelQueries({ queryKey: ["po", "lines"] });
      const previous = queryClient.getQueryData<readonly PoLine[]>(["po", "lines"]) ?? [];
      snapshotAtByLine.current.set(line.id, performance.now());
      queryClient.setQueryData(["po", "lines"], (current: readonly PoLine[] = []) => [
        ...current,
        { ...line, sync: "syncing" as const },
      ]);
      pushCacheEvent({ kind: "optimistic", atMs: performance.now(), label: "add" });
      pushPanelEvent(
        setEvents,
        createPanelEvent("info", `Adding ${line.label}…`, "onMutate snapshotted the cache and inserted the row."),
      );
      return { previous };
    },
    onSuccess: (saved) => {
      queryClient.setQueryData(["po", "lines"], (current: readonly PoLine[] = []) =>
        current.map((line) => (line.id === saved.id ? saved : line)),
      );
      pushCacheEvent({ kind: "confirmed", atMs: performance.now(), label: "confirm" });
      pushPanelEvent(
        setEvents,
        createPanelEvent("success", `${saved.label} confirmed`, "onSuccess replaced the optimistic row."),
      );
    },
    onError: (_error, line, context) => {
      queryClient.setQueryData(["po", "lines"], context?.previous ?? []);
      setWrongWindows((current) => (current.length > 0 && current[current.length - 1].toMs === null
        ? current
        : [...current, { fromMs: performance.now(), toMs: null }]));
      pushCacheEvent({
        kind: "restore",
        atMs: performance.now(),
        label: "restore",
        restoreToMs: snapshotAtByLine.current.get(line.id),
      });
      pushPanelEvent(
        setEvents,
        createPanelEvent("error", `${line.label} failed`, "onError restored this mutation's cache snapshot."),
      );
    },
    onSettled: () => {
      if (queryClient.isMutating() === 1) {
        void queryClient.invalidateQueries({ queryKey: ["po", "lines"] });
      }
    },
  });

  const lines = (query.data ?? null) as readonly PoLine[] | null;
  const agreement = useMemo(() => computeAgreement(lines, serverTruth), [lines, serverTruth]);

  useEffect(() => {
    onAgreement("tanstack", agreement, "live");
  }, [agreement, onAgreement]);

  const refetching = query.isFetching && !query.isLoading;
  useEffect(() => {
    if (refetching) {
      wasFetchingAfterMount.current = true;
      return;
    }
    if (refetchEvidencePending.current) {
      refetchEvidencePending.current = false;
      onAgreement("tanstack", agreement, "refetchCompleted");
    }
  }, [agreement, onAgreement, refetching]);

  const controller = useMemo<PanelController>(() => ({
    addLine: (line, options = {}) => {
      if (options.dependsOnLineId) {
        const dependents = dependentsByLine.current.get(options.dependsOnLineId) ?? new Set<string>();
        dependents.add(line.id);
        dependentsByLine.current.set(options.dependsOnLineId, dependents);
      }
      mutation.mutate({ ...line });
    },
    settle: (lineId, accepted) => {
      store.settle(lineId, accepted);
      if (!accepted) {
        // The server delivers dependency cancellations as separate responses,
        // one tick after the parent's rejection.
        for (const dependentId of dependentsByLine.current.get(lineId) ?? []) {
          window.setTimeout(() => store.cancel(dependentId), 120);
        }
      }
    },
    reset: async () => {
      store.reset();
      dependentsByLine.current.clear();
      snapshotAtByLine.current.clear();
      setEvents([]);
      setCacheEvents([]);
      setWrongWindows([]);
      await queryClient.resetQueries({ queryKey: ["po", "lines"] });
    },
  }), [mutation, queryClient, store]);

  useEffect(() => {
    onController(controller);
    return () => onController(null);
  }, [controller, onController]);

  const live = phase === "arming" || phase === "diverged" || phase === "batchRunning";

  return (
    <div className="po-column">
      <PlatformOwner
        description="One shared query cache · snapshot rollback · full-list refetch"
        title="TanStack Query"
        variant="tanstack"
      />
      <PoPanel
        agreement={agreement}
        caption='useQuery({ queryKey: ["po", "lines"] }) · onMutate / onError / onSettled'
        error={query.error instanceof Error ? query.error.message : null}
        events={events}
        highlightId={highlightId}
        lines={lines}
        loading={query.isLoading}
        refetching={refetching}
        serverTruth={serverTruth}
        title="TanStack Query"
        variant="tanstack"
      />
      <CacheTimelineStrip baseMs={baseMs} events={cacheEvents} live={live} wrongWindows={wrongWindows} />
    </div>
  );
}
