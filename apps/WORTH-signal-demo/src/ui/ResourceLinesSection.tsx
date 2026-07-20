import React from "react";
import { createSignals } from "worth-signals-wasm";
import { createReactSignalsStore, useResourceLine } from "worth-signals-wasm/react";
import { DxCorner } from "./DxCorner";
import { ResourceLinesSectionCodeSample } from "./ResourceLinesSectionCodeSample";

const RESOURCE_LINES_DX_SAMPLE = `export function ProductPrice({ productId }: { productId: string }) {
  const view = useResourceView(product.line({ productId }), store);

  if (view.contentState === "loading") return <Skeleton />;

  return (
    <span data-stale={view.freshness.kind !== "fresh"}>
      {money(view.value.price)}
      {view.isRefreshing ? <Spinner /> : null}
    </span>
  );
}`;
import {
  createStorefrontServer,
  describeLifecycleEntry,
  type LifecycleEntryView,
  type ServerPushPacket,
  type StorefrontProduct,
  type StorefrontServer,
} from "./resourceLinesSectionSupport";
import "./resourceLinesSection.css";

interface ResourceLinesSectionProps {
  onNavigate: (path: string) => void;
}

type SignalsRuntime = Awaited<ReturnType<typeof createSignals>>;

type Phase = "booting" | "settled" | "ambush" | "revealed";

interface ProductLineHandle {
  value: () => StorefrontProduct | null;
  status: () => { kind: string };
  freshness: () => { kind: string; reason?: string };
  refresh: () => unknown;
  invalidate: () => unknown;
  deliver: (packet: unknown) => unknown;
  diagnostics: () => { lastEffect: Record<string, unknown> | null };
  history: () => { lifecycle: unknown[] };
}

interface StorefrontWORTH {
  signals: SignalsRuntime;
  server: StorefrontServer;
  productFamily: {
    line: (params: { productId: string }) => ProductLineHandle;
    delivery: { field: (options: Record<string, unknown>) => unknown };
  };
  catalogFamily: { line: (params: Record<string, never>) => unknown };
  catalogLine: {
    value: () => readonly StorefrontProduct[] | null;
    status: () => { kind: string };
    refresh: () => unknown;
    history: () => { lifecycle: unknown[] };
  };
}

interface ShockInfo {
  field: string;
  from: number;
  to: number;
  packetId: string;
}

const FOCUS_PRODUCT_ID = "p-204";
const AMBUSH_DELAY_MS = 4_500;

const money = (value: number): string => `$${value.toLocaleString("en-US")}`;

function buildWORTH(signals: SignalsRuntime, server: StorefrontServer): StorefrontWORTH {
  const api = (signals as unknown as {
    api: (options: Record<string, unknown>) => {
      url: (path: string) => {
        detail: (declaration: Record<string, unknown>) => StorefrontWORTH["productFamily"];
        response: (contract: unknown) => {
          list: (declaration: Record<string, unknown>) => StorefrontWORTH["catalogFamily"];
        };
      };
    };
  }).api({
    baseUrl: "/api/storefront",
    effects: (signals as unknown as {
      resource: { effects: { branchNative: () => unknown } };
    }).resource.effects.branchNative(),
  });

  const resource = (signals as unknown as {
    resource: {
      detailFields: (fields: Record<string, unknown>) => unknown;
      response: { array: (options: Record<string, unknown>) => unknown };
    };
  }).resource;

  const productFields = resource.detailFields({
    price: {
      read: (value: StorefrontProduct) => value.price,
      write: (value: StorefrontProduct, price: number) => ({ ...value, price }),
    },
    inventory: {
      read: (value: StorefrontProduct) => value.inventory,
      write: (value: StorefrontProduct, inventory: number) => ({ ...value, inventory }),
    },
    status: {
      read: (value: StorefrontProduct) => value.status,
      write: (value: StorefrontProduct, status: StorefrontProduct["status"]) => ({ ...value, status }),
    },
  });

  const productFamily = api.url("/products/:productId").detail({
    reconcile: productFields,
    load: ({ productId }: { productId: string }) => server.fetchProduct(productId),
  });

  const catalogFamily = api
    .url("/products")
    .response(resource.response.array({ itemId: (product: StorefrontProduct) => product.id }))
    .list({ load: () => server.fetchProducts() });

  const catalogLine = (catalogFamily as { line: (params: Record<string, never>) => StorefrontWORTH["catalogLine"] }).line({});

  return { signals, server, productFamily, catalogFamily, catalogLine };
}

/** Bumps a counter on a short schedule after an action so imperative line reads re-render through transitions. */
function useLinePulse(): [number, () => void] {
  const [pulse, setPulse] = React.useState(0);
  const timeoutsRef = React.useRef<number[]>([]);

  React.useEffect(() => {
    return () => timeoutsRef.current.forEach((handle) => window.clearTimeout(handle));
  }, []);

  const schedule = React.useCallback(() => {
    setPulse((current) => current + 1);
    [120, 400, 800, 1400].forEach((delay) => {
      timeoutsRef.current.push(window.setTimeout(() => setPulse((current) => current + 1), delay));
    });
  }, []);

  return [pulse, schedule];
}

function StatusPill({ kind }: { kind: string }): React.ReactElement {
  const tone = kind === "fulfilled" ? "ok" : kind === "rejected" ? "bad" : "busy";
  return <span className={`rl-pill rl-pill-status rl-pill-${tone}`}>{kind}</span>;
}

function FreshnessPill({ freshness }: { freshness: { kind: string; reason?: string } }): React.ReactElement {
  const stale = freshness.kind !== "fresh";
  return (
    <span className={`rl-pill rl-pill-freshness${stale ? " rl-pill-stale" : " rl-pill-ok"}`}>
      {freshness.kind}
      {stale && freshness.reason ? <em> · {freshness.reason}</em> : null}
    </span>
  );
}

function TapeRow({ entry }: { entry: LifecycleEntryView }): React.ReactElement {
  const isDelivery = entry.event === "delivered";
  const isInvalidation = entry.event === "invalidated";
  return (
    <li className={`rl-tape-row${isDelivery ? " is-delivery" : ""}${isInvalidation ? " is-invalidation" : ""}`}>
      <span className="rl-tape-seq">{String(entry.sequence).padStart(2, "0")}</span>
      <div className="rl-tape-body">
        <p className="rl-tape-headline">{entry.headline}</p>
        {entry.detail ? <p className="rl-tape-detail">{entry.detail}</p> : null}
        <details className="rl-raw">
          <summary>raw lifecycle entry</summary>
          <pre>{JSON.stringify(entry.raw, null, 2)}</pre>
        </details>
      </div>
      <span className="rl-tape-meta">
        {entry.statusKind} · {entry.freshnessKind}
      </span>
    </li>
  );
}

function exportLineHistory(line: ProductLineHandle, productId: string): void {
  const artifact = {
    exportedAt: new Date().toISOString(),
    lineIdentity: `product.line({ productId: "${productId}" })`,
    source: "line.history().lifecycle + line.diagnostics().lastEffect, read from the Worth runtime",
    value: line.value(),
    status: line.status(),
    freshness: line.freshness(),
    lifecycle: line.history().lifecycle,
    lastEffect: line.diagnostics().lastEffect,
  };
  const blob = new Blob([JSON.stringify(artifact, null, 2)], { type: "application/json" });
  const url = URL.createObjectURL(blob);
  const anchor = document.createElement("a");
  anchor.href = url;
  anchor.download = `worth-line-history-${productId}.json`;
  anchor.click();
  URL.revokeObjectURL(url);
}

function StorefrontWorkbench({ WORTH }: { WORTH: StorefrontWORTH }): React.ReactElement {
  const reactStore = React.useMemo(() => createReactSignalsStore(WORTH.signals), [WORTH]);
  const [selectedId, setSelectedId] = React.useState(FOCUS_PRODUCT_ID);
  const [phase, setPhase] = React.useState<Phase>("booting");
  const [shock, setShock] = React.useState<ShockInfo | null>(null);
  const [pulse, schedulePulse] = useLinePulse();
  const ambushArmedRef = React.useRef(false);
  const recorderRef = React.useRef<HTMLElement | null>(null);

  const selection = React.useMemo(() => ({ productId: selectedId }), [selectedId]);
  const catalogSelection = React.useMemo(() => ({}), []);

  const line = React.useMemo(
    () => WORTH.productFamily.line(selection),
    [WORTH, selection],
  );

  const lineView = useResourceLine<StorefrontProduct | null, null>(
    WORTH.productFamily as unknown as Parameters<typeof useResourceLine>[0],
    selection,
    reactStore as Parameters<typeof useResourceLine>[2],
    { inactiveValue: null },
  );
  const catalogView = useResourceLine<readonly StorefrontProduct[] | null, null>(
    WORTH.catalogFamily as unknown as Parameters<typeof useResourceLine>[0],
    catalogSelection,
    reactStore as Parameters<typeof useResourceLine>[2],
    { inactiveValue: null },
  );

  // the hook provides the subscription; the imperative read guarantees delivered
  // patches are visible even when they land outside a tracked write
  const product = line.value() ?? lineView.value ?? null;
  const catalog = catalogView.value ?? null;

  const refreshedLinesRef = React.useRef(new Set<string>());
  React.useEffect(() => {
    if (refreshedLinesRef.current.has(selectedId)) return;
    refreshedLinesRef.current.add(selectedId);
    if (line.value() === null) {
      line.refresh();
      schedulePulse();
    }
  }, [line, selectedId, schedulePulse]);

  const catalogRefreshedRef = React.useRef(false);
  React.useEffect(() => {
    if (catalogRefreshedRef.current) return;
    catalogRefreshedRef.current = true;
    if (WORTH.catalogLine.value() === null) {
      WORTH.catalogLine.refresh();
      schedulePulse();
    }
  }, [WORTH, schedulePulse]);

  const deliverServerPush = React.useCallback(
    (packet: ServerPushPacket, targetId: string) => {
      const targetLine = WORTH.productFamily.line({ productId: targetId });
      const send = (basisId: string | null): { kind?: string; expectedBasisId?: string | null } =>
        targetLine.deliver(
          WORTH.productFamily.delivery.field({
            packetId: packet.packetId,
            basisId,
            nextBasisId: packet.nextBasisId,
            field: packet.field,
            value: packet.value,
          }),
        ) as { kind?: string; expectedBasisId?: string | null };

      // deliveries chain on the line's basis; if the line disagrees (fresh load
      // resets the chain), re-send against the basis it expects
      const report = send(packet.basisId);
      if (report?.kind === "basisRejected") {
        send(report.expectedBasisId ?? null);
      }
      schedulePulse();
    },
    [WORTH, schedulePulse],
  );

  React.useEffect(() => {
    if (phase !== "booting" || product === null) return;
    setPhase("settled");
  }, [phase, product]);

  React.useEffect(() => {
    if (phase !== "settled" || ambushArmedRef.current) return;
    ambushArmedRef.current = true;
    const handle = window.setTimeout(() => {
      const packet = WORTH.server.pushChange(FOCUS_PRODUCT_ID, "price");
      deliverServerPush(packet, FOCUS_PRODUCT_ID);
      setShock({
        field: packet.field,
        from: packet.previousValue,
        to: packet.value,
        packetId: packet.packetId,
      });
      setPhase("ambush");
    }, AMBUSH_DELAY_MS);
    return () => window.clearTimeout(handle);
  }, [phase, WORTH, deliverServerPush]);

  const reveal = (): void => {
    setPhase("revealed");
    window.setTimeout(() => {
      recorderRef.current?.scrollIntoView({ behavior: "smooth", block: "start" });
    }, 60);
  };

  const status = line.status();
  const freshness = line.freshness();
  const lastEffect = line.diagnostics().lastEffect;
  const lifecycle = React.useMemo(
    () => line.history().lifecycle.map(describeLifecycleEntry).reverse(),
    // pulse re-reads the tape after actions land
    [line, pulse, product],
  );

  const revealed = phase === "revealed";
  const ambush = phase === "ambush";

  const lastDelivery = lastEffect && lastEffect.provenance === "deliveredPatch"
    ? {
        field: String((lastEffect.patch as { field?: string } | null)?.field ?? "?"),
        packetId: String((lastEffect.delivery as { packetId?: string } | null)?.packetId ?? "?"),
        nextBasisId: String((lastEffect.delivery as { nextBasisId?: string } | null)?.nextBasisId ?? "?"),
      }
    : null;

  const liveCodeLine = lastEffect
    ? `// → "${String(lastEffect.provenance ?? "unknown")}"${lastDelivery ? ` · packet ${lastDelivery.packetId} · ${lastDelivery.field} patched` : ""}`
    : null;

  return (
    <>
      <section className="rl-catalog-strip" aria-label="Catalog line">
        <span className="rl-line-identity">products.line({"{}"})</span>
        <div className="rl-catalog-products">
          {(catalog ?? []).map((item) => (
            <button
              className={item.id === selectedId ? "is-selected" : ""}
              key={item.id}
              onClick={() => {
                setSelectedId(item.id);
                schedulePulse();
              }}
              type="button"
            >
              {item.name}
            </button>
          ))}
          {catalog === null ? <span className="rl-catalog-loading">loading catalog…</span> : null}
        </div>
        <span className="rl-catalog-meta">
          {WORTH.catalogLine.history().lifecycle.length} recorded events · its own recorder, its own truth
        </span>
      </section>

      <section className="rl-focus" aria-label="Product detail line">
        <article className={`rl-product-card${ambush ? " is-alarmed" : ""}`}>
          <header className="rl-product-head">
            <div>
              <h3>{product?.name ?? "Loading product…"}</h3>
              <span className="rl-line-identity">{`product.line({ productId: "${selectedId}" })`}</span>
            </div>
            <StatusPill kind={status.kind} />
          </header>

          <dl className="rl-product-fields">
            <div className={shock && shock.field === "price" && (ambush || revealed) ? "rl-field-flash" : ""} key={`price-${product?.price ?? "…"}`}>
              <dt>Price</dt>
              <dd>{product ? money(product.price) : "—"}</dd>
            </div>
            <div key={`inv-${product?.inventory ?? "…"}`}>
              <dt>Inventory</dt>
              <dd>{product ? `${product.inventory} in stock` : "—"}</dd>
            </div>
            <div>
              <dt>Status</dt>
              <dd>{product?.status ?? "—"}</dd>
            </div>
          </dl>

          <div className="rl-familiar-divider" aria-hidden="true">
            <span>everything above is any fetch — everything below exists because this read is a line</span>
          </div>

          <div className="rl-line-facts">
            <FreshnessPill freshness={freshness} />
            <span className="rl-pill rl-pill-provenance">
              last effect: {lastEffect ? String(lastEffect.provenance ?? "unknown") : "none"}
            </span>
            <code className="rl-facts-caption">line.freshness() · line.diagnostics().lastEffect</code>
          </div>

          {revealed ? (
            <div className="rl-action-row">
              <button onClick={() => { line.refresh(); schedulePulse(); }} type="button">
                Refresh
                <span>line.refresh()</span>
              </button>
              <button onClick={() => { line.invalidate(); schedulePulse(); }} type="button">
                Mark stale
                <span>line.invalidate()</span>
              </button>
              <button
                onClick={() => {
                  const packet = WORTH.server.pushChange(selectedId, Math.random() > 0.5 ? "price" : "inventory");
                  deliverServerPush(packet, selectedId);
                }}
                type="button"
              >
                Server: push another change
                <span>line.deliver(…)</span>
              </button>
            </div>
          ) : null}

          {phase === "settled" ? (
            <p className="rl-quiet-caption">An ordinary product page. Nothing to see here.</p>
          ) : null}
        </article>

        {ambush && shock ? (
          <button className="rl-shock-button" onClick={reveal} type="button">
            <strong>Wait. Why did that change?</strong>
            <span>
              {shock.field} just went {money(shock.from)} → {money(shock.to)} and you didn’t touch anything.
            </span>
          </button>
        ) : null}
      </section>

      {revealed ? (
        <section className="rl-recorder" aria-label="Line recorder" ref={recorderRef}>
          {shock && selectedId === FOCUS_PRODUCT_ID ? (
            <div className="rl-answer-banner">
              <p className="rl-answer-main">
                A backend delivery patched <strong>{shock.field}</strong> {money(shock.from)} → {money(shock.to)} — packet{" "}
                <code>{shock.packetId}</code>
                {lastDelivery ? <> · basis <code>{lastDelivery.nextBasisId}</code></> : null}. No user action involved.
              </p>
              <p className="rl-answer-sub">
                The value on screen changed behind your back, and the read itself kept the receipt. Every stack does this
                to its users — most just have nothing to consult afterwards.
              </p>
              <code className="rl-facts-caption">line.diagnostics().lastEffect</code>
            </div>
          ) : null}

          <header className="signals-panel-head">
            <h3>Flight recorder</h3>
            <code>line.history().lifecycle</code>
            <button
              className="signals-export-button"
              onClick={() => exportLineHistory(line, selectedId)}
              type="button"
            >
              Export line history (JSON)
            </button>
          </header>
          <ul className="rl-tape">
            {lifecycle.map((entry) => (
              <TapeRow entry={entry} key={entry.sequence} />
            ))}
          </ul>
          <p className="rl-recorder-footnote">
            Every row is read from the runtime. Switch products above — each line carries its own recorder.
          </p>
        </section>
      ) : null}

      {revealed ? (
        <section className="signals-code-section" aria-labelledby="rl-code-title">
          <h2 id="rl-code-title">The wiring, and the receipt it leaves behind</h2>
          <ResourceLinesSectionCodeSample liveLine={liveCodeLine} />
        </section>
      ) : null}

      {revealed ? (
        <DxCorner
          code={RESOURCE_LINES_DX_SAMPLE}
          filename="product-price.tsx"
          subtitle="Audit-grade server state with a smaller day-to-day surface than a query library: one handle owns fetch, freshness, patches, and history — no query keys, no cache-sync glue."
          receipts={[
            {
              claim: "No query keys to invent.",
              api: "product.line({ productId }) · identity is the params",
            },
            {
              claim: "Actions live on the handle you already hold.",
              api: "line.refresh() · line.invalidate() · line.deliver(…)",
            },
            {
              claim: "Loading, refreshing, empty, error — already decided.",
              api: "useResourceView(line).contentState",
            },
          ]}
        />
      ) : null}
    </>
  );
}

export function ResourceLinesSection({ onNavigate }: ResourceLinesSectionProps): React.ReactElement {
  const [WORTH, setWORTH] = React.useState<StorefrontWORTH | null>(null);
  const [bootError, setBootError] = React.useState<string | null>(null);

  React.useEffect(() => {
    let active = true;
    createSignals({ deployment: "mainThreadCompatibility" })
      .then((signals) => {
        if (!active) return;
        setWORTH(buildWORTH(signals, createStorefrontServer()));
      })
      .catch((error: unknown) => {
        if (active) setBootError(error instanceof Error ? error.message : "Could not start the Worth runtime.");
      });
    return () => {
      active = false;
    };
  }, []);

  return (
    <div className="accent-resources rl-section">
      {bootError ? <div className="signals-runtime-message">{bootError}</div> : null}
      {!WORTH && !bootError ? <div className="signals-runtime-message">Connecting to the Worth runtime…</div> : null}
      {WORTH ? <StorefrontWorkbench WORTH={WORTH} /> : null}

      <div className="signals-docs-row">
        <button onClick={() => onNavigate("#/docs/resources/debugging/README")} type="button">
          Read debugging and recovery <span aria-hidden="true">→</span>
        </button>
      </div>
    </div>
  );
}
