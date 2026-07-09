import React from "react";
import { useReducedMotion } from "motion/react";

type Particle = {
  cooldown: number;
  direction: 1 | -1;
  impact: number;
  radius: number;
  value: number;
  vx: number;
  vy: number;
  x: number;
  y: number;
};

interface BouncingValueFieldProps {
  seed: number;
  values: number[];
}

const restitution = 0.96;
const tokenRadius = 18;

const seededUnit = (seed: number) => {
  const raw = Math.sin(seed * 999.17) * 10000;
  return raw - Math.floor(raw);
};

const initialParticles = (seed: number, values: number[], width: number, height: number): Particle[] =>
  values.map((value, index) => ({
    cooldown: 0,
    direction: 1,
    impact: 0,
    radius: tokenRadius,
    value,
    vx: (seededUnit(seed + index * 3) > 0.5 ? 1 : -1) * (72 + seededUnit(seed + index * 7) * 58),
    vy: (seededUnit(seed + index * 5) > 0.5 ? 1 : -1) * (54 + seededUnit(seed + index * 11) * 48),
    x: tokenRadius + seededUnit(seed + index * 13) * Math.max(1, width - tokenRadius * 2),
    y: tokenRadius + seededUnit(seed + index * 17) * Math.max(1, height - tokenRadius * 2),
  }));

const nextValue = (particle: Particle, entropy: number): Particle => {
  const factor = 1 + entropy;
  if (particle.direction === 1) {
    const value = Math.min(1000, Math.round(particle.value * factor));
    return { ...particle, direction: value >= 1000 ? -1 : 1, value };
  }

  const value = Math.max(1, Math.round(particle.value / factor));
  return { ...particle, direction: value <= 1 ? 1 : -1, value };
};

const resolveCollision = (left: Particle, right: Particle, entropy: number): [Particle, Particle] => {
  const dx = right.x - left.x;
  const dy = right.y - left.y;
  const distance = Math.max(Math.hypot(dx, dy), 0.001);
  const minDistance = left.radius + right.radius;
  if (distance >= minDistance || left.cooldown > 0 || right.cooldown > 0) return [left, right];

  const nx = dx / distance;
  const ny = dy / distance;
  const tx = -ny;
  const ty = nx;
  const leftNormal = left.vx * nx + left.vy * ny;
  const rightNormal = right.vx * nx + right.vy * ny;

  if (rightNormal - leftNormal >= 0) return [left, right];

  const leftTangent = left.vx * tx + left.vy * ty;
  const rightTangent = right.vx * tx + right.vy * ty;
  const overlap = (minDistance - distance) / 2;
  const bouncedLeft = nextValue(left, entropy);
  const bouncedRight = nextValue(right, 1 - entropy);

  return [
    {
      ...bouncedLeft,
      cooldown: 0.24,
      impact: 1,
      vx: (rightNormal * nx + leftTangent * tx) * restitution,
      vy: (rightNormal * ny + leftTangent * ty) * restitution,
      x: left.x - nx * overlap,
      y: left.y - ny * overlap,
    },
    {
      ...bouncedRight,
      cooldown: 0.24,
      impact: 1,
      vx: (leftNormal * nx + rightTangent * tx) * restitution,
      vy: (leftNormal * ny + rightTangent * ty) * restitution,
      x: right.x + nx * overlap,
      y: right.y + ny * overlap,
    },
  ];
};

export const BouncingValueField: React.FC<BouncingValueFieldProps> = ({ seed, values }) => {
  const prefersReducedMotion = useReducedMotion();
  const fieldRef = React.useRef<HTMLDivElement | null>(null);
  const [size, setSize] = React.useState({ height: 176, width: 480 });
  const [particles, setParticles] = React.useState(() => initialParticles(seed, values, size.width, size.height));

  React.useEffect(() => {
    const field = fieldRef.current;
    if (!field) return;

    const observer = new ResizeObserver(([entry]) => {
      setSize({
        height: entry.contentRect.height,
        width: entry.contentRect.width,
      });
    });

    observer.observe(field);
    return () => observer.disconnect();
  }, []);

  React.useEffect(() => {
    setParticles(initialParticles(seed, values, size.width, size.height));
  }, [seed, size.height, size.width, values]);

  React.useEffect(() => {
    if (prefersReducedMotion) return;

    let frame = 0;
    let last = performance.now();

    const step = (now: number) => {
      const dt = Math.min((now - last) / 1000, 0.04);
      last = now;
      setParticles((current) => {
        const next = current.map((particle) => {
          let x = particle.x + particle.vx * dt;
          let y = particle.y + particle.vy * dt;
          let vx = particle.vx;
          let vy = particle.vy;

          if (x < particle.radius || x > size.width - particle.radius) {
            vx *= -1;
            x = Math.min(size.width - particle.radius, Math.max(particle.radius, x));
          }

          if (y < particle.radius || y > size.height - particle.radius) {
            vy *= -1;
            y = Math.min(size.height - particle.radius, Math.max(particle.radius, y));
          }

          return {
            ...particle,
            cooldown: Math.max(0, particle.cooldown - dt),
            impact: Math.max(0, particle.impact - dt * 3.2),
            vx,
            vy,
            x,
            y,
          };
        });

        for (let left = 0; left < next.length; left += 1) {
          for (let right = left + 1; right < next.length; right += 1) {
            const entropy = seededUnit(now + seed + left * 31 + right * 47);
            [next[left], next[right]] = resolveCollision(next[left], next[right], entropy);
          }
        }

        return next;
      });
      frame = requestAnimationFrame(step);
    };

    frame = requestAnimationFrame(step);
    return () => cancelAnimationFrame(frame);
  }, [prefersReducedMotion, seed, size.height, size.width]);

  return (
    <div className="xai-value-field" aria-hidden="true" ref={fieldRef}>
      {particles.map((particle, index) => (
        <span
          className="xai-value-token"
          key={index}
          style={
            {
              "--impact": particle.impact.toFixed(2),
              left: `${particle.x}px`,
              top: `${particle.y}px`,
            } as React.CSSProperties
          }
        >
          {particle.value.toString().padStart(2, "0")}
        </span>
      ))}
    </div>
  );
};
