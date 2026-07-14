import { useMemo, useState } from "react";
import { DemoPreface } from "./DemoPreface";
import { ResourceLinesSectionCodeSample } from "./ResourceLinesSectionCodeSample";
import "./resourceLinesSection.css";

interface ResourceLinesSectionProps {
  onNavigate: (path: string) => void;
}

type Product = {
  id: string;
  name: string;
  status: "active" | "review";
  price: number;
  inventory: number;
};

type LineEvent = {
  kind: "initialLoad" | "refresh" | "deliveryPatch" | "invalidate";
  locus: string;
  message: string;
};

type ActiveLine = "catalog" | "detail";

const sourceProducts: Product[] = [
  { id: "p-204", name: "Northstar Carry-On", status: "active", price: 184, inventory: 18 },
  { id: "p-381", name: "Garment sleeve", status: "active", price: 72, inventory: 41 },
  { id: "p-672", name: "Packing cubes", status: "review", price: 38, inventory: 7 },
];

function money(value: number): string {
  return `$${value}`;
}

function terminalOutput({
  activeLine,
  catalogFreshness,
  detailFreshness,
  detail,
  events,
  products,
  selectedId,
}: {
  activeLine: ActiveLine;
  catalogFreshness: string;
  detailFreshness: string;
  detail: Product;
  events: LineEvent[];
  products: Product[];
  selectedId: string;
}): string {
  const lowStock = products.filter((product) => product.inventory <= 10).length;
  const inspectedLine =
    activeLine === "catalog"
      ? "products.line({})"
      : `product.line({ productId: "${selectedId}" })`;
  const detailEvent = events.find((event) => event.locus.includes(selectedId)) ?? events[0] ?? null;
  const catalogEvent = events.find((event) => event.locus === "products.line({})") ?? events[0] ?? null;
  return [
    `> inspecting`,
    `"${inspectedLine}"`,
    "",
    "> products.line({}).summary()",
    JSON.stringify({ status: "fulfilled", freshness: catalogFreshness, total: products.length, lowStock }, null, 2),
    "",
    `> product.line({ productId: "${selectedId}" }).value()`,
    JSON.stringify({ ...detail, freshness: detailFreshness }, null, 2),
    "",
    `> ${inspectedLine}.diagnostics().lastEffect`,
    JSON.stringify(activeLine === "catalog" ? catalogEvent : detailEvent, null, 2),
    "",
    `> ${inspectedLine}.history().entries()`,
    JSON.stringify(events.slice(0, 5), null, 2),
  ].join("\n");
}

export function ResourceLinesSection({ onNavigate }: ResourceLinesSectionProps) {
  const [products, setProducts] = useState<Product[]>(sourceProducts);
  const [selectedId, setSelectedId] = useState(sourceProducts[0].id);
  const [activeLine, setActiveLine] = useState<ActiveLine>("catalog");
  const [catalogFreshness, setCatalogFreshness] = useState("fresh");
  const [detailFreshness, setDetailFreshness] = useState("fresh");
  const [serverVersion, setServerVersion] = useState(1);
  const [events, setEvents] = useState<LineEvent[]>([
    { kind: "initialLoad", locus: "products.line({})", message: "Catalog line loaded three products." },
  ]);

  const selected = products.find((product) => product.id === selectedId) ?? products[0];
  const output = useMemo(
    () => terminalOutput({ activeLine, catalogFreshness, detail: selected, detailFreshness, events, products, selectedId }),
    [activeLine, catalogFreshness, detailFreshness, events, products, selected, selectedId],
  );

  function pushEvent(event: LineEvent): void {
    setEvents((current) => [event, ...current].slice(0, 6));
  }

  function refreshCatalog(): void {
    setServerVersion((version) => version + 1);
    setProducts((current) =>
      current.map((product, index) => ({
        ...product,
        inventory: product.inventory + (index === 2 ? 3 : 1),
      })),
    );
    setCatalogFreshness("fresh");
    setActiveLine("catalog");
    pushEvent({ kind: "refresh", locus: "products.line({})", message: "Catalog line refreshed from server truth." });
  }

  function deliverPatch(): void {
    const nextVersion = serverVersion + 1;
    setServerVersion(nextVersion);
    setProducts((current) =>
      current.map((product) =>
        product.id === selectedId
          ? { ...product, inventory: product.inventory + 6 + nextVersion, price: product.price + 4 }
          : product,
      ),
    );
    setCatalogFreshness("fresh");
    setDetailFreshness("fresh");
    setActiveLine("detail");
    pushEvent({
      kind: "deliveryPatch",
      locus: `product.line({ productId: "${selectedId}" })`,
      message: "Backend delivery patched price and inventory for the selected product.",
    });
  }

  function invalidateDetail(): void {
    setDetailFreshness("invalidated");
    setActiveLine("detail");
    pushEvent({
      kind: "invalidate",
      locus: `product.line({ productId: "${selectedId}" })`,
      message: "Detail line was marked stale without rewriting catalog truth.",
    });
  }

  function selectProduct(productId: string): void {
    setSelectedId(productId);
    setActiveLine("detail");
    setDetailFreshness("fresh");
    pushEvent({
      kind: "initialLoad",
      locus: `product.line({ productId: "${productId}" })`,
      message: "Detail line materialized for the selected product.",
    });
  }

  return (
    <div className="xai-section-band accent-resources resource-lines-section">
      <div className="xai-section-heading">
        <span className="xai-section-eyebrow">02 / Resource Lines</span>
        <h2>Every read is a line.</h2>
        <p>
          A resource line is the runtime-owned unit of server truth: value,
          status, freshness, diagnostics, and history attached to one identity.
        </p>
      </div>

      <DemoPreface demoId={2} />

      <ResourceLinesSectionCodeSample />

      <div className="resource-lines-workbench">
        <article className="resource-lines-panel">
          <div className="forms-card-topline"><span>Materialized lines</span></div>
          <div className="resource-lines-tabs" aria-label="Resource lines">
            <button className={activeLine === "catalog" ? "is-active" : ""} onClick={() => setActiveLine("catalog")} type="button">
              products.line({"{}"})
            </button>
            <button className={activeLine === "detail" ? "is-active" : ""} onClick={() => setActiveLine("detail")} type="button">
              {`product.line({ productId: "${selectedId}" })`}
            </button>
          </div>
          <p className="resource-lines-help">
            Click a product to materialize its detail line. Use the actions to
            see freshness, diagnostics, and history move independently.
          </p>

          <div className="resource-lines-list">
            {products.map((product) => (
              <button
                className={product.id === selectedId ? "is-selected" : ""}
                key={product.id}
                onClick={() => selectProduct(product.id)}
                type="button"
              >
                <span>
                  <strong>{product.name}</strong>
                  <em>{`${money(product.price)} / ${product.inventory} in stock`}</em>
                </span>
                <b>{product.status}</b>
              </button>
            ))}
          </div>
        </article>

        <article className="resource-lines-panel resource-lines-detail">
          <div className="forms-card-topline"><span>Selected detail line</span></div>
          <h3>{selected.name}</h3>
          <dl>
            <div><dt>Status</dt><dd>{selected.status}</dd></div>
            <div><dt>Price</dt><dd>{money(selected.price)}</dd></div>
            <div><dt>Inventory</dt><dd>{selected.inventory}</dd></div>
            <div><dt>Catalog freshness</dt><dd>{catalogFreshness}</dd></div>
            <div><dt>Detail freshness</dt><dd>{detailFreshness}</dd></div>
          </dl>
          <div className="resource-lines-actions">
            <button onClick={refreshCatalog} type="button">
              <strong>Refresh catalog</strong>
              <span>Reload list summary and row inventory.</span>
            </button>
            <button onClick={deliverPatch} type="button">
              <strong>Deliver backend patch</strong>
              <span>Patch the selected detail line from server truth.</span>
            </button>
            <button onClick={invalidateDetail} type="button">
              <strong>Invalidate detail</strong>
              <span>Mark only the selected detail line stale.</span>
            </button>
          </div>
        </article>

        <article className="resource-lines-panel resource-lines-terminal-card">
          <div className="forms-card-topline"><span>Line inspector</span></div>
          <pre className="resource-lines-terminal">{output}</pre>
        </article>
      </div>

      <div className="signals-cta-row">
        <div className="signals-cta-copy">Lines make server truth inspectable before you even talk about writes.</div>
        <div className="xai-section-actions">
          <button className="xai-button xai-button-primary" onClick={() => onNavigate("#/demos/2")} type="button">
            Open resource lines demo
          </button>
          <button className="xai-button xai-button-secondary" onClick={() => onNavigate("#/docs/resources/index")} type="button">
            Read resources docs
          </button>
        </div>
      </div>
    </div>
  );
}
