import React from "react";
import { useReducedMotion } from "motion/react";
import "./compositionGlueArtifact.css";

type Chip = { height: number; kind: "library" | "glue"; label: string; width: number };
type Body = Chip & { angle: number; burstX: number; burstY: number; opacity: number; scale: number; vx: number; vy: number; x: number; y: number };
type FragmentSeed = { angle: number; burstX: number; burstY: number; lineX: number; lineY: number; scale: number; startX: number; startY: number };
type Fragment = FragmentSeed & { opacity: number; x: number; y: number };

const chips: readonly Chip[] = [
  { height: 34, kind: "library", label: "React", width: 86 },
  { height: 34, kind: "library", label: "Formik", width: 92 },
  { height: 34, kind: "library", label: "TanStack Query", width: 140 },
  { height: 34, kind: "library", label: "Router", width: 92 },
  { height: 32, kind: "glue", label: "syncStatus()", width: 126 },
  { height: 32, kind: "glue", label: "mapErrors()", width: 120 },
  { height: 32, kind: "glue", label: "invalidate()", width: 124 },
  { height: 32, kind: "glue", label: "hydrate()", width: 104 },
  { height: 32, kind: "glue", label: "onSubmit()", width: 112 },
  { height: 32, kind: "glue", label: "rollbackRef", width: 116 },
];

const cycleSeconds = 6.6;
const revealGap = 0.3;
const revealHold = 0.28;
const chaosStart = 3.05;
const popAt = 4.45;
const resolveAt = 5.18;
const restitution = 0.98;

function seededUnit(seed: number) {
  const raw = Math.sin(seed * 918.71) * 10000;
  return raw - Math.floor(raw);
}

function initialBodies(width: number, height: number): Body[] {
  const centerX = width / 2;
  const centerY = height / 2;
  const ring = Math.min(width, height) * 0.28;
  return chips.map((chip, index) => {
    const angle = -Math.PI / 2 + (index / chips.length) * Math.PI * 2;
    const jitter = (seededUnit(index + 3) - 0.5) * 18;
    return {
      ...chip,
      angle,
      burstX: Math.cos(angle) * (width * 0.42 + seededUnit(index + 11) * 28),
      burstY: Math.sin(angle) * (height * 0.38 + seededUnit(index + 19) * 18),
      opacity: 0,
      scale: 0.92,
      vx: Math.cos(angle + 1.7) * 26,
      vy: Math.sin(angle + 1.7) * 22,
      x: centerX + Math.cos(angle) * (ring + jitter),
      y: centerY + Math.sin(angle) * (ring * 0.64 + jitter * 0.5),
    };
  });
}

function buildConfettiSeeds(bodies: readonly Body[], width: number, height: number): FragmentSeed[] {
  const shardsPerChip = 7;
  return bodies.flatMap((body, chipIndex) =>
    Array.from({ length: shardsPerChip }, (_, shardIndex) => {
      const index = chipIndex * shardsPerChip + shardIndex;
      const edgeAngle = (shardIndex / shardsPerChip) * Math.PI * 2 + body.angle * 0.18;
      const startX = body.x + Math.cos(edgeAngle) * body.width * 0.42;
      const startY = body.y + Math.sin(edgeAngle) * body.height * 0.44;
      const burstAngle = edgeAngle + (seededUnit(index + 31) - 0.5) * 0.95;
      const burst = 38 + seededUnit(index + 43) * Math.min(width, height) * 0.36;
      return {
        angle: burstAngle,
        burstX: startX + Math.cos(burstAngle) * burst,
        burstY: startY + Math.sin(burstAngle) * burst * 0.74,
        lineX: width / 2 + (seededUnit(index + 67) - 0.5) * 48,
        lineY: height / 2 + (seededUnit(index + 71) - 0.5) * 22,
        scale: 0.55 + seededUnit(index + 97) * 0.9,
        startX,
        startY,
      };
    }),
  );
}

function visibleConfetti(seeds: readonly FragmentSeed[], progress: number): Fragment[] {
  const eased = 1 - (1 - progress) ** 3;
  return seeds.map((seed) => ({
    ...seed,
    opacity: Math.min(1, progress * 4) * (1 - Math.max(0, progress - 0.82) * 2.8),
    x: seed.startX + (seed.burstX - seed.startX) * eased,
    y: seed.startY + (seed.burstY - seed.startY) * eased,
  }));
}

function resolvedConfetti(seeds: readonly FragmentSeed[], progress: number): Fragment[] {
  const eased = progress * progress * (3 - 2 * progress);
  return seeds.map((seed) => ({
    ...seed,
    opacity: 0.34 * (1 - eased),
    x: seed.burstX + (seed.lineX - seed.burstX) * eased,
    y: seed.burstY + (seed.lineY - seed.burstY) * eased,
  }));
}

function hiddenConfetti(width: number, height: number): Fragment[] {
  return Array.from({ length: chips.length * 7 }, (_, index) => {
    const angle = (index / (chips.length * 7)) * Math.PI * 2;
    return {
      angle,
      burstX: width / 2,
      burstY: height / 2,
      lineX: width / 2,
      lineY: height / 2,
      opacity: 0,
      scale: 0.5,
      startX: width / 2,
      startY: height / 2,
      x: width / 2,
      y: height / 2,
    };
  });
}

function resolveCollision(left: Body, right: Body) {
  if (left.opacity < 0.18 || right.opacity < 0.18) return;
  const dx = right.x - left.x;
  const dy = right.y - left.y;
  const overlapX = (left.width + right.width) / 2 - Math.abs(dx);
  const overlapY = (left.height + right.height) / 2 - Math.abs(dy);
  if (overlapX <= 0 || overlapY <= 0) return;
  const impact = 44 + Math.min(94, Math.hypot(left.vx - right.vx, left.vy - right.vy) * 0.17);

  if (overlapX < overlapY) {
    const direction = dx >= 0 ? 1 : -1;
    left.x -= (overlapX / 2) * direction;
    right.x += (overlapX / 2) * direction;
    const leftVelocity = left.vx;
    left.vx = right.vx * restitution - direction * impact;
    right.vx = leftVelocity * restitution + direction * impact;
    left.vy += (seededUnit(left.angle + right.angle + 13) - 0.5) * impact * 0.32;
    right.vy -= (seededUnit(left.angle + right.angle + 29) - 0.5) * impact * 0.32;
    return;
  }

  const direction = dy >= 0 ? 1 : -1;
  left.y -= (overlapY / 2) * direction;
  right.y += (overlapY / 2) * direction;
  const leftVelocity = left.vy;
  left.vy = right.vy * restitution - direction * impact * 0.82;
  right.vy = leftVelocity * restitution + direction * impact * 0.82;
  left.vx += (dx >= 0 ? -1 : 1) * impact * 0.48;
  right.vx += (dx >= 0 ? 1 : -1) * impact * 0.48;
}

function advanceBodies(current: Body[], width: number, height: number, elapsed: number, dt: number) {
  const chaos = Math.max(0, Math.min(1, (elapsed - chaosStart) / (popAt - chaosStart)));
  const speed = 0.82 + chaos * chaos * chaos * 5.8;

  for (let index = 0; index < current.length; index += 1) {
    const body = current[index];
    const revealProgress = Math.max(0, Math.min(1, (elapsed - index * revealGap) / revealHold));
    const visible = revealProgress * revealProgress * (3 - 2 * revealProgress);
    const pullToCenter = chaos * chaos * 42;
    const orbitDirection = index % 2 === 0 ? 1 : -1;
    body.opacity += (visible - body.opacity) * Math.min(1, dt * 12);
    body.scale += ((0.88 + visible * 0.12) - body.scale) * Math.min(1, dt * 10);
    body.vx += Math.cos(elapsed * (1.9 + index * 0.1) + body.angle) * (34 + chaos * 230) * dt;
    body.vy += Math.sin(elapsed * (1.55 + index * 0.07) - body.angle) * (16 + chaos * 96) * dt;
    body.vx += Math.cos(body.angle + Math.PI / 2) * orbitDirection * chaos * 112 * dt;
    body.vy += Math.sin(body.angle + Math.PI / 2) * orbitDirection * chaos * 34 * dt;
    body.vx += (width / 2 - body.x) * pullToCenter * dt * 0.017;
    body.vy += (height / 2 - body.y) * pullToCenter * dt * 0.008;
    body.vx *= 1 - Math.max(0, 0.16 - chaos * 0.13) * dt;
    body.vy *= 1 - Math.max(0, 0.16 - chaos * 0.13) * dt;
    body.x += body.vx * dt * speed;
    body.y += body.vy * dt * speed;

    const halfWidth = body.width / 2;
    const halfHeight = body.height / 2;
    if (body.x < halfWidth || body.x > width - halfWidth) {
      body.vx *= -0.9;
      body.x = Math.min(width - halfWidth, Math.max(halfWidth, body.x));
    }
    if (body.y < halfHeight || body.y > height - halfHeight) {
      body.vy *= -0.9;
      body.y = Math.min(height - halfHeight, Math.max(halfHeight, body.y));
    }
  }

  for (let left = 0; left < current.length; left += 1) {
    for (let right = left + 1; right < current.length; right += 1) resolveCollision(current[left], current[right]);
  }
}

export function CompositionGlueArtifact() {
  const prefersReducedMotion = useReducedMotion();
  const ref = React.useRef<HTMLDivElement | null>(null);
  const bodiesRef = React.useRef<Body[]>([]);
  const popRef = React.useRef<Body[] | null>(null);
  const confettiRef = React.useRef<FragmentSeed[]>([]);
  const [size, setSize] = React.useState({ height: 198, width: 512 });
  const [bodies, setBodies] = React.useState<Body[]>(initialBodies(512, 198));
  const [fragments, setFragments] = React.useState<Fragment[]>(hiddenConfetti(512, 198));
  const [phase, setPhase] = React.useState<"build" | "chaos" | "pop" | "resolve">(prefersReducedMotion ? "resolve" : "build");

  React.useEffect(() => {
    const element = ref.current;
    if (!element) return;
    const observer = new ResizeObserver(([entry]) => {
      const next = { height: Math.max(176, entry.contentRect.height), width: Math.max(320, entry.contentRect.width) };
      setSize(next);
      bodiesRef.current = initialBodies(next.width, next.height);
      popRef.current = null;
      confettiRef.current = [];
      setBodies(bodiesRef.current);
      setFragments(hiddenConfetti(next.width, next.height));
    });
    observer.observe(element);
    return () => observer.disconnect();
  }, []);

  React.useEffect(() => {
    if (prefersReducedMotion) {
      setPhase("resolve");
      return;
    }

    bodiesRef.current = initialBodies(size.width, size.height);
    let frame = 0;
    let last = performance.now();
    const started = last;

    const step = (now: number) => {
      const elapsed = ((now - started) / 1000) % cycleSeconds;
      const dt = Math.min((now - last) / 1000, 0.033);
      last = now;

      if (elapsed < 0.05) {
        bodiesRef.current = initialBodies(size.width, size.height);
        popRef.current = null;
        confettiRef.current = [];
        setFragments(hiddenConfetti(size.width, size.height));
      }

      if (elapsed < popAt) {
        advanceBodies(bodiesRef.current, size.width, size.height, elapsed, dt);
        setBodies(bodiesRef.current.map((body) => ({ ...body })));
        setPhase(elapsed < chaosStart ? "build" : "chaos");
      } else {
        if (!popRef.current) {
          popRef.current = bodiesRef.current.map((body) => ({ ...body }));
          confettiRef.current = buildConfettiSeeds(popRef.current, size.width, size.height);
        }
        const progress = Math.min(1, (elapsed - popAt) / 0.62);
        setBodies(
          popRef.current.map((body) => ({
            ...body,
            opacity: 1 - progress,
            scale: 1 - progress * 0.55,
            x: body.x + body.burstX * progress,
            y: body.y + body.burstY * progress,
          })),
        );
        if (elapsed < resolveAt) {
          setFragments(visibleConfetti(confettiRef.current, progress));
        } else {
          setFragments(resolvedConfetti(confettiRef.current, Math.min(1, (elapsed - resolveAt) / 0.78)));
        }
        setPhase(elapsed < resolveAt ? "pop" : "resolve");
      }

      frame = requestAnimationFrame(step);
    };

    frame = requestAnimationFrame(step);
    return () => cancelAnimationFrame(frame);
  }, [prefersReducedMotion, size.height, size.width]);

  return (
    <div className={`xai-composition-glue-artifact is-${phase}`} aria-hidden="true" ref={ref}>
      <div className="xai-glue-field">
        <div className="xai-glue-pressure" />
        {bodies.map((body) => (
          <span
            className={`xai-glue-chip is-${body.kind}`}
            key={body.label}
            style={
              {
                "--chip-opacity": body.opacity.toFixed(3),
                "--chip-scale": body.scale.toFixed(3),
                "--chip-height": `${body.height}px`,
                "--chip-x": `${body.x}px`,
                "--chip-y": `${body.y}px`,
                "--chip-width": `${body.width}px`,
              } as React.CSSProperties
            }
          >
            {body.label}
          </span>
        ))}
        {fragments.map((fragment, index) => (
          <span
            className="xai-glue-fragment"
            key={index}
            style={
              {
                "--angle": `${fragment.angle}rad`,
                "--frag-line-x": `${fragment.lineX}px`,
                "--frag-line-y": `${fragment.lineY}px`,
                "--frag-opacity": fragment.opacity.toFixed(3),
                "--frag-scale": fragment.scale.toFixed(2),
                "--frag-x": `${fragment.x}px`,
                "--frag-y": `${fragment.y}px`,
              } as React.CSSProperties
            }
          />
        ))}
        <div className="xai-glue-resolution">
          <span className="xai-glue-node xai-glue-node-center">Forge</span>
        </div>
      </div>
    </div>
  );
}
