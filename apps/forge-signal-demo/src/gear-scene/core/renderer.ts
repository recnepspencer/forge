import {
  RENDER_HEIGHT,
  RENDER_WIDTH,
  type CameraState,
  type RenderAspects,
  type RenderStats,
  type SceneState,
} from "./types";

const CAMERA_SPEED = 0.11;
const LOOK_SPEED = 0.0024;
const TAU = Math.PI * 2;
const FLOOR_Y = -0.34;

type Vec3 = { x: number; y: number; z: number };
type Vec2 = { x: number; y: number };
type Triangle = {
  a: Vec3;
  b: Vec3;
  c: Vec3;
  normal: Vec3;
  base: number;
  roughness: number;
};

type ProjectedTriangle = {
  points: [Vec2, Vec2, Vec2];
  depth: number;
  fill: string;
  stroke: string;
};

let scratchCanvas: OffscreenCanvas | null = null;
let scratchContext: OffscreenCanvasRenderingContext2D | null = null;

export function defaultSceneState(): SceneState {
  return {
    camera: {
      x: 0,
      y: 0.25,
      z: -4.2,
      yaw: 0.18,
      pitch: -0.1,
    },
    light: {
      x: -2.2,
      y: 2.8,
      z: -3.1,
      intensity: 1.18,
    },
    gear: {
      teeth: 16,
      outerRadius: 1.18,
      innerRadius: 0.38,
      thickness: 0.42,
      rotation: 0.2,
    },
  };
}

export function movementStep(keys: Set<string>, camera: CameraState) {
  const forward = {
    x: -Math.sin(camera.yaw),
    y: 0,
    z: Math.cos(camera.yaw),
  };
  const right = {
    x: Math.cos(camera.yaw),
    y: 0,
    z: Math.sin(camera.yaw),
  };

  let next = { ...camera };

  if (keys.has("w")) next = moveCamera(next, forward, CAMERA_SPEED);
  if (keys.has("s")) next = moveCamera(next, forward, -CAMERA_SPEED);
  if (keys.has("a")) next = moveCamera(next, right, -CAMERA_SPEED);
  if (keys.has("d")) next = moveCamera(next, right, CAMERA_SPEED);
  if (keys.has("shift")) next.y -= CAMERA_SPEED;
  if (keys.has("space")) next.y += CAMERA_SPEED;

  return next;
}

export function applyLookDelta(camera: CameraState, deltaX: number, deltaY: number) {
  if (deltaX === 0 && deltaY === 0) {
    return camera;
  }

  return {
    ...camera,
    yaw: camera.yaw + deltaX * LOOK_SPEED,
    pitch: clamp(camera.pitch + deltaY * LOOK_SPEED, -1.1, 1.1),
  };
}

export function renderScene(scene: SceneState, aspects: RenderAspects): {
  frame: ImageBitmap;
  stats: RenderStats;
} {
  const startedAt = performance.now();
  const ctx = ensureScratchContext();
  const canvas = scratchCanvas!;

  ctx.clearRect(0, 0, canvas.width, canvas.height);
  drawBackdrop(ctx);

  const mesh = buildGearMesh(scene, aspects);
  drawFloor(ctx, scene, aspects);
  drawProjectedShadow(ctx, scene, mesh, aspects);
  const triangles = projectTriangles(scene, aspects, mesh);
  drawTriangles(ctx, triangles, aspects);

  const frame = scratchCanvas!.transferToImageBitmap();

  return {
    frame,
    stats: {
      frameIndex: 0,
      raysMarched: aspects.mesh.triangleCount,
      averageSteps: aspects.profile.toothDepth,
      hits: aspects.topology.profilePointCount,
      misses: aspects.topology.ringSegments,
      renderMs: performance.now() - startedAt,
    },
  };
}

function ensureScratchContext() {
  if (!scratchCanvas) {
    scratchCanvas = new OffscreenCanvas(RENDER_WIDTH, RENDER_HEIGHT);
    scratchContext = scratchCanvas.getContext("2d");
  }
  if (!scratchContext) {
    throw new Error("2D OffscreenCanvas context is required for the gear renderer.");
  }
  return scratchContext;
}

function drawBackdrop(ctx: OffscreenCanvasRenderingContext2D) {
  const sky = ctx.createLinearGradient(0, 0, 0, RENDER_HEIGHT);
  sky.addColorStop(0, "#0b1217");
  sky.addColorStop(0.46, "#141e25");
  sky.addColorStop(1, "#1a242b");
  ctx.fillStyle = sky;
  ctx.fillRect(0, 0, RENDER_WIDTH, RENDER_HEIGHT);

  const vignette = ctx.createRadialGradient(
    RENDER_WIDTH * 0.5,
    RENDER_HEIGHT * 0.5,
    RENDER_WIDTH * 0.18,
    RENDER_WIDTH * 0.5,
    RENDER_HEIGHT * 0.5,
    RENDER_WIDTH * 0.72,
  );
  vignette.addColorStop(0, "rgba(255,255,255,0)");
  vignette.addColorStop(1, "rgba(0,0,0,0.34)");
  ctx.fillStyle = vignette;
  ctx.fillRect(0, 0, RENDER_WIDTH, RENDER_HEIGHT);
}

function drawFloor(ctx: OffscreenCanvasRenderingContext2D, scene: SceneState, aspects: RenderAspects) {
  const floorCorners = [
    { x: -4.8, y: FLOOR_Y, z: 0.8 },
    { x: 4.8, y: FLOOR_Y, z: 0.8 },
    { x: 8.4, y: FLOOR_Y, z: 11.2 },
    { x: -8.4, y: FLOOR_Y, z: 11.2 },
  ]
    .map((point) => projectPoint(worldToCamera(point, scene.camera)))
    .filter(Boolean) as Array<{ point: Vec2; depth: number }>;

  if (floorCorners.length === 4) {
    const floorGradient = ctx.createLinearGradient(0, RENDER_HEIGHT * 0.54, 0, RENDER_HEIGHT);
    floorGradient.addColorStop(0, "#4a5762");
    floorGradient.addColorStop(1, "#273039");
    ctx.fillStyle = floorGradient;
    ctx.beginPath();
    ctx.moveTo(floorCorners[0].point.x, floorCorners[0].point.y);
    for (let i = 1; i < floorCorners.length; i += 1) {
      ctx.lineTo(floorCorners[i].point.x, floorCorners[i].point.y);
    }
    ctx.closePath();
    ctx.fill();
  }

  const lines: Array<[Vec3, Vec3]> = [];
  for (let i = -7; i <= 7; i += 1) {
    lines.push([
      { x: i * 0.45, y: FLOOR_Y, z: 0.3 },
      { x: i * 0.95, y: FLOOR_Y, z: 8.4 },
    ]);
  }
  for (let i = 0; i <= 10; i += 1) {
    const z = 0.6 + i * 0.72;
    lines.push([
      { x: -4.2, y: FLOOR_Y, z },
      { x: 4.2, y: FLOOR_Y, z },
    ]);
  }

  ctx.save();
  ctx.lineWidth = 1;
  for (const [start, end] of lines) {
    const a = projectPoint(worldToCamera(start, scene.camera));
    const b = projectPoint(worldToCamera(end, scene.camera));
    if (!a || !b) continue;
    const alpha = a.depth < 8 ? aspects.shading.floorGridOpacity : aspects.shading.floorGridOpacity * 0.5;
    ctx.strokeStyle = `rgba(218, 230, 238, ${alpha.toFixed(3)})`;
    ctx.beginPath();
    ctx.moveTo(a.point.x, a.point.y);
    ctx.lineTo(b.point.x, b.point.y);
    ctx.stroke();
  }
  ctx.restore();
}

function drawProjectedShadow(ctx: OffscreenCanvasRenderingContext2D, scene: SceneState, mesh: Triangle[], aspects: RenderAspects) {
  const points: Vec2[] = [];
  for (const triangle of mesh) {
    for (const vertex of [triangle.a, triangle.b, triangle.c]) {
      const shadowWorld = projectShadowPoint(vertex, scene.light);
      const projected = projectPoint(worldToCamera(shadowWorld, scene.camera));
      if (projected) {
        points.push(projected.point);
      }
    }
  }

  if (points.length === 0) {
    return;
  }

  const hull = convexHull(points);
  if (hull.length < 3) {
    return;
  }

  ctx.save();
  ctx.filter = "blur(12px)";
  ctx.fillStyle = `rgba(5, 8, 11, ${aspects.shading.shadowOpacity.toFixed(3)})`;
  ctx.beginPath();
  ctx.moveTo(hull[0].x, hull[0].y);
  for (let i = 1; i < hull.length; i += 1) {
    ctx.lineTo(hull[i].x, hull[i].y);
  }
  ctx.closePath();
  ctx.fill();
  ctx.restore();
}

function drawTriangles(ctx: OffscreenCanvasRenderingContext2D, triangles: ProjectedTriangle[], aspects: RenderAspects) {
  for (const triangle of triangles) {
    ctx.beginPath();
    ctx.moveTo(triangle.points[0].x, triangle.points[0].y);
    ctx.lineTo(triangle.points[1].x, triangle.points[1].y);
    ctx.lineTo(triangle.points[2].x, triangle.points[2].y);
    ctx.closePath();
    ctx.fillStyle = triangle.fill;
    ctx.fill();
    ctx.strokeStyle = triangle.stroke;
    ctx.lineWidth = aspects.shading.edgeContrast;
    ctx.stroke();
  }
}

function buildGearMesh(_scene: SceneState, aspects: RenderAspects): Triangle[] {
  const outer = createOuterRing(aspects);
  const inner = createInnerRing(aspects);
  const half = aspects.dimensions.thickness * 0.5;
  const topOuter = outer.map((point) => ({ ...point, y: half }));
  const bottomOuter = outer.map((point) => ({ ...point, y: -half }));
  const topInner = inner.map((point) => ({ ...point, y: half }));
  const bottomInner = inner.map((point) => ({ ...point, y: -half }));

  const triangles: Triangle[] = [];
  appendRingSurface(triangles, topOuter, topInner, "#bcc7d0", 0.24, false);
  appendRingSurface(triangles, bottomOuter, bottomInner, "#707d89", 0.3, true);
  appendSideWalls(triangles, topOuter, bottomOuter, "#9ba8b3", 0.4, false);
  appendSideWalls(triangles, topInner, bottomInner, "#596571", 0.28, true);
  return triangles;
}

function createOuterRing(aspects: RenderAspects): Vec3[] {
  const ring: Vec3[] = [];
  const teeth = Math.max(aspects.dimensions.teeth, 8);
  const step = aspects.profile.toothStep;
  const root = aspects.profile.rootRadius;
  const tip = aspects.profile.tipRadius;
  const shoulder = aspects.profile.shoulderRadius;
  const rotation = aspects.dimensions.rotation;

  for (let tooth = 0; tooth < teeth; tooth += 1) {
    const base = rotation + tooth * step;
    const samples = [
      { angle: base - step * 0.5, radius: root },
      { angle: base - step * 0.26, radius: shoulder },
      { angle: base - step * 0.1, radius: tip },
      { angle: base + step * 0.1, radius: tip },
      { angle: base + step * 0.26, radius: shoulder },
      { angle: base + step * 0.5, radius: root },
    ];

    for (const sample of samples) {
      ring.push({
        x: Math.cos(sample.angle) * sample.radius,
        y: 0,
        z: Math.sin(sample.angle) * sample.radius,
      });
    }
  }

  return ring;
}

function createInnerRing(aspects: RenderAspects): Vec3[] {
  const segments = aspects.topology.ringSegments;
  const ring: Vec3[] = [];
  const radius = aspects.dimensions.innerRadius;
  for (let i = 0; i < segments; i += 1) {
    const angle = aspects.dimensions.rotation + (i / segments) * TAU;
    ring.push({
      x: Math.cos(angle) * radius,
      y: 0,
      z: Math.sin(angle) * radius,
    });
  }
  return ring;
}

function appendRingSurface(
  triangles: Triangle[],
  outer: Vec3[],
  inner: Vec3[],
  base: string,
  roughness: number,
  invert: boolean,
) {
  const count = outer.length;
  for (let i = 0; i < count; i += 1) {
    const next = (i + 1) % count;
    const innerIndex = Math.floor((i / count) * inner.length) % inner.length;
    const innerNext = Math.floor((next / count) * inner.length) % inner.length;
    if (invert) {
      triangles.push(makeTriangle(outer[i], inner[innerIndex], inner[innerNext], base, roughness));
      triangles.push(makeTriangle(outer[i], inner[innerNext], outer[next], base, roughness));
    } else {
      triangles.push(makeTriangle(outer[i], inner[innerNext], inner[innerIndex], base, roughness));
      triangles.push(makeTriangle(outer[i], outer[next], inner[innerNext], base, roughness));
    }
  }
}

function appendSideWalls(
  triangles: Triangle[],
  top: Vec3[],
  bottom: Vec3[],
  base: string,
  roughness: number,
  invert: boolean,
) {
  const count = top.length;
  for (let i = 0; i < count; i += 1) {
    const next = (i + 1) % count;
    if (invert) {
      triangles.push(makeTriangle(top[i], bottom[next], bottom[i], base, roughness));
      triangles.push(makeTriangle(top[i], top[next], bottom[next], base, roughness));
    } else {
      triangles.push(makeTriangle(top[i], bottom[i], bottom[next], base, roughness));
      triangles.push(makeTriangle(top[i], bottom[next], top[next], base, roughness));
    }
  }
}

function makeTriangle(a: Vec3, b: Vec3, c: Vec3, base: string, roughness: number): Triangle {
  return {
    a,
    b,
    c,
    normal: normalize(cross(subtract(b, a), subtract(c, a))),
    base: parseHex(base),
    roughness,
  };
}

function projectTriangles(scene: SceneState, aspects: RenderAspects, triangles: Triangle[]): ProjectedTriangle[] {
  const light = normalize(scene.light);
  const projected: ProjectedTriangle[] = [];

  for (const triangle of triangles) {
    const ca = worldToCamera(triangle.a, scene.camera);
    const cb = worldToCamera(triangle.b, scene.camera);
    const cc = worldToCamera(triangle.c, scene.camera);

    const screenA = projectPoint(ca);
    const screenB = projectPoint(cb);
    const screenC = projectPoint(cc);
    if (!screenA || !screenB || !screenC) {
      continue;
    }

    const worldNormal = triangle.normal;
    const diffuse = clamp(dot(worldNormal, light), 0, 1);
    const ambient = aspects.shading.ambient;
    const view = clamp(dot(worldNormal, { x: 0, y: 0, z: -1 }), 0, 1);
    const highlight =
      Math.pow(clamp(diffuse * aspects.lighting.highlightBoost + view * 0.3, 0, 1), aspects.shading.specularPower) *
      (1 - triangle.roughness);
    const brightness = ambient + diffuse * aspects.shading.diffuseBoost * scene.light.intensity + highlight * 0.26;

    projected.push({
      points: [screenA.point, screenB.point, screenC.point],
      depth: (ca.z + cb.z + cc.z) / 3,
      fill: shade(triangle.base, brightness),
      stroke: shade(triangle.base, Math.max(brightness - 0.18, 0.16)),
    });
  }

  projected.sort((left, right) => right.depth - left.depth);
  return projected;
}

function worldToCamera(point: Vec3, camera: CameraState): Vec3 {
  const translated = {
    x: point.x - camera.x,
    y: point.y - camera.y,
    z: point.z - camera.z,
  };

  const yawCos = Math.cos(-camera.yaw);
  const yawSin = Math.sin(-camera.yaw);
  const yawed = {
    x: translated.x * yawCos - translated.z * yawSin,
    y: translated.y,
    z: translated.x * yawSin + translated.z * yawCos,
  };

  const pitchCos = Math.cos(-camera.pitch);
  const pitchSin = Math.sin(-camera.pitch);
  return {
    x: yawed.x,
    y: yawed.y * pitchCos - yawed.z * pitchSin,
    z: yawed.y * pitchSin + yawed.z * pitchCos,
  };
}

function projectPoint(point: Vec3): { point: Vec2; depth: number } | null {
  if (point.z <= 0.1) {
    return null;
  }
  const focal = 520;
  const x = RENDER_WIDTH * 0.5 + (point.x / point.z) * focal;
  const y = RENDER_HEIGHT * 0.5 - (point.y / point.z) * focal;
  return { point: { x, y }, depth: point.z };
}

function projectShadowPoint(point: Vec3, light: Vec3): Vec3 {
  const direction = subtract(point, light);
  const t = (FLOOR_Y - light.y) / direction.y;
  return {
    x: light.x + direction.x * t,
    y: FLOOR_Y,
    z: light.z + direction.z * t,
  };
}

function convexHull(points: Vec2[]): Vec2[] {
  const sorted = [...points].sort((a, b) => (a.x === b.x ? a.y - b.y : a.x - b.x));
  if (sorted.length <= 1) {
    return sorted;
  }

  const lower: Vec2[] = [];
  for (const point of sorted) {
    while (lower.length >= 2 && cross2d(lower[lower.length - 2], lower[lower.length - 1], point) <= 0) {
      lower.pop();
    }
    lower.push(point);
  }

  const upper: Vec2[] = [];
  for (let i = sorted.length - 1; i >= 0; i -= 1) {
    const point = sorted[i];
    while (upper.length >= 2 && cross2d(upper[upper.length - 2], upper[upper.length - 1], point) <= 0) {
      upper.pop();
    }
    upper.push(point);
  }

  lower.pop();
  upper.pop();
  return lower.concat(upper);
}

function moveCamera(base: CameraState, vector: { x: number; y: number; z: number }, scalar: number): CameraState {
  return {
    ...base,
    x: base.x + vector.x * scalar,
    y: base.y + vector.y * scalar,
    z: base.z + vector.z * scalar,
  };
}

function subtract(a: Vec3, b: Vec3): Vec3 {
  return { x: a.x - b.x, y: a.y - b.y, z: a.z - b.z };
}

function cross(a: Vec3, b: Vec3): Vec3 {
  return {
    x: a.y * b.z - a.z * b.y,
    y: a.z * b.x - a.x * b.z,
    z: a.x * b.y - a.y * b.x,
  };
}

function dot(a: Vec3, b: Vec3): number {
  return a.x * b.x + a.y * b.y + a.z * b.z;
}

function normalize(vector: Vec3): Vec3 {
  const length = Math.hypot(vector.x, vector.y, vector.z) || 1;
  return {
    x: vector.x / length,
    y: vector.y / length,
    z: vector.z / length,
  };
}

function cross2d(a: Vec2, b: Vec2, c: Vec2): number {
  return (b.x - a.x) * (c.y - a.y) - (b.y - a.y) * (c.x - a.x);
}

function parseHex(hex: string): number {
  return Number.parseInt(hex.replace("#", ""), 16);
}

function shade(base: number, brightness: number) {
  const r = (base >> 16) & 255;
  const g = (base >> 8) & 255;
  const b = base & 255;
  return `rgb(${clamp(Math.round(r * brightness), 0, 255)}, ${clamp(Math.round(g * brightness), 0, 255)}, ${clamp(Math.round(b * brightness), 0, 255)})`;
}

function clamp(value: number, min: number, max: number) {
  return Math.min(Math.max(value, min), max);
}
