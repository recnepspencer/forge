import {
  type CameraState,
  RENDER_HEIGHT,
  RENDER_WIDTH,
  type RenderAspects,
  type RenderStats,
  type SceneState,
} from "./types";

const CAMERA_SPEED = 0.11;
const LOOK_SPEED = 0.0024;
const TAU = Math.PI * 2;

type Vec3 = { x: number; y: number; z: number };
type Mat4 = Float32Array;
type Mesh = { vertex: WebGLBuffer; normal: WebGLBuffer; index: WebGLBuffer; indexCount: number };
type Renderer = {
  canvas: OffscreenCanvas;
  gl: WebGL2RenderingContext;
  bgProgram: WebGLProgram;
  surfaceProgram: WebGLProgram;
  screen: WebGLBuffer;
  composeTexture: WebGLTexture;
  composePixels: Float32Array;
  composeScratch: Float32Array;
  composeColumns: number;
  composeRows: number;
  composeTextureColumns: number;
  composeTextureRows: number;
  floor: Mesh;
  gear: Mesh | null;
  gearKey: string | null;
};

type UploadStats = {
  dirtyTiles: number;
  uploadedTiles: number;
  uploadSpans: number;
  uploadBytes: number;
};

let renderer: Renderer | null = null;

export function defaultSceneState(): SceneState {
  const camera = cameraLookAt({ x: 1.15, y: 1.55, z: -4.8 }, { x: 0, y: 0.05, z: 0 });
  return {
    camera,
    light: { x: -2.4, y: 2.7, z: -2.2, intensity: 1.32 },
    gear: { teeth: 16, outerRadius: 1.18, innerRadius: 0.38, thickness: 0.42, rotation: 0.18 },
  };
}

export function movementStep(keys: Set<string>, camera: CameraState) {
  const forward = { x: -Math.sin(camera.yaw), y: 0, z: Math.cos(camera.yaw) };
  const right = { x: Math.cos(camera.yaw), y: 0, z: Math.sin(camera.yaw) };
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
  if (deltaX === 0 && deltaY === 0) return camera;
  return {
    ...camera,
    yaw: camera.yaw + deltaX * LOOK_SPEED,
    pitch: clamp(camera.pitch + deltaY * LOOK_SPEED, -1.1, 1.1),
  };
}

export function renderScene(scene: SceneState, aspects: RenderAspects): { frame: ImageBitmap; stats: RenderStats } {
  const startedAt = performance.now();
  const state = ensureRenderer();
  const { gl, canvas } = state;
  const camera = buildCamera(scene.camera);
  const gear = ensureGearMesh(state, aspects);
  const uploadStats = uploadTileTexture(state, aspects);
  const floorModel = multiplyMany(
    translation(0, -1.06, 0),
    scale(9.6, 1, 8.2),
  );
  const gearModel = multiplyMany(
    translation(0, 0.02, 0),
    rotationY(0.2 + scene.gear.rotation * 0.2),
    rotationX(-0.08),
    rotationZ(scene.gear.rotation),
  );

  gl.viewport(0, 0, canvas.width, canvas.height);
  gl.enable(gl.DEPTH_TEST);
  gl.enable(gl.CULL_FACE);
  gl.clearColor(0.02, 0.03, 0.045, 1);
  gl.clear(gl.COLOR_BUFFER_BIT | gl.DEPTH_BUFFER_BIT);

  drawBackground(state, camera.eye, aspects);
  drawMesh(state, state.floor, floorModel, camera, scene, aspects, {
    base: [0.13, 0.16, 0.19],
    edge: [0.23, 0.29, 0.33],
    metalness: 0.12,
    roughness: 0.94,
    shadowed: 1,
  });
  drawMesh(state, gear, gearModel, camera, scene, aspects, {
    base: [0.74, 0.78, 0.83],
    edge: [0.98, 1.0, 1.0],
    metalness: 0.88,
    roughness: 0.24,
    shadowed: 0,
  });

  gl.flush();
  const frame = canvas.transferToImageBitmap();
  return {
    frame,
    stats: {
      frameIndex: 0,
      raysMarched: aspects.mesh.triangleCount * aspects.tileGrid.tileCount,
      averageSteps: aspects.profile.toothDepth * aspects.tileGrid.tileCount,
      hits: aspects.dirtyTileIndices.length,
      misses: Math.max(aspects.tileGrid.tileCount - aspects.dirtyTileIndices.length, 0),
      renderMs: performance.now() - startedAt,
      tileCount: aspects.tileGrid.tileCount,
      tileColumns: aspects.tileGrid.columns,
      tileRows: aspects.tileGrid.rows,
      dirtyTiles: uploadStats.dirtyTiles,
      uploadedTiles: uploadStats.uploadedTiles,
      uploadSpans: uploadStats.uploadSpans,
      uploadBytes: uploadStats.uploadBytes,
      changedDetails: 0,
    },
  };
}

function ensureRenderer(): Renderer {
  if (renderer) return renderer;
  const canvas = new OffscreenCanvas(RENDER_WIDTH, RENDER_HEIGHT);
  const gl = canvas.getContext("webgl2", {
    alpha: false, antialias: true, depth: true, desynchronized: true, premultipliedAlpha: false,
  });
  if (!gl) throw new Error("WebGL2 OffscreenCanvas is required for the demo renderer.");
  const bgProgram = program(gl, bgVertex, bgFragment);
  const surfaceProgram = program(gl, surfaceVertex, surfaceFragment);
  renderer = {
    canvas,
    gl,
    bgProgram,
    surfaceProgram,
    screen: buffer(gl, gl.ARRAY_BUFFER, new Float32Array([-1, -1, 3, -1, -1, 3])),
    composeTexture: createTileTexture(gl),
    composePixels: new Float32Array(4),
    composeScratch: new Float32Array(4),
    composeColumns: 1,
    composeRows: 1,
    composeTextureColumns: 1,
    composeTextureRows: 1,
    floor: buildFloor(gl),
    gear: null,
    gearKey: null,
  };
  return renderer;
}

function ensureGearMesh(state: Renderer, aspects: RenderAspects): Mesh {
  const key = [
    aspects.dimensions.teeth,
    aspects.dimensions.outerRadius.toFixed(4),
    aspects.dimensions.innerRadius.toFixed(4),
    aspects.dimensions.thickness.toFixed(4),
  ].join("|");
  if (state.gear && state.gearKey === key) return state.gear;
  if (state.gear) {
    state.gl.deleteBuffer(state.gear.vertex);
    state.gl.deleteBuffer(state.gear.normal);
    state.gl.deleteBuffer(state.gear.index);
  }
  state.gear = buildGear(state.gl, aspects);
  state.gearKey = key;
  return state.gear;
}

function buildFloor(gl: WebGL2RenderingContext): Mesh {
  return mesh(gl,
    new Float32Array([-1, 0, -1, 1, 0, -1, 1, 0, 1, -1, 0, 1]),
    new Float32Array([0, 1, 0, 0, 1, 0, 0, 1, 0, 0, 1, 0]),
    new Uint32Array([0, 1, 2, 0, 2, 3]),
  );
}

function buildGear(gl: WebGL2RenderingContext, aspects: RenderAspects): Mesh {
  const teeth = Math.max(6, aspects.dimensions.teeth);
  const segments = Math.max(96, teeth * 12);
  const root = aspects.profile.rootRadius;
  const tip = aspects.profile.tipRadius;
  const inner = Math.max(0.08, aspects.dimensions.innerRadius);
  const half = Math.max(0.08, aspects.dimensions.thickness * 0.5);
  const positions: number[] = [];
  const normals: number[] = [];
  const indices: number[] = [];

  for (let i = 0; i < segments; i += 1) {
    const a = (i / segments) * TAU;
    const r = outerRadius(a, teeth, root, tip);
    const ox = Math.cos(a) * r;
    const oy = Math.sin(a) * r;
    const ix = Math.cos(a) * inner;
    const iy = Math.sin(a) * inner;
    push(positions, normals, ox, oy, half, 0, 0, 1);
    push(positions, normals, ix, iy, half, 0, 0, 1);
    push(positions, normals, ox, oy, -half, 0, 0, -1);
    push(positions, normals, ix, iy, -half, 0, 0, -1);
  }

  for (let i = 0; i < segments; i += 1) {
    const n = (i + 1) % segments;
    const a = i * 4;
    const b = n * 4;
    indices.push(a, b, b + 1, a, b + 1, a + 1);
    indices.push(a + 2, b + 3, b + 2, a + 2, a + 3, b + 3);
  }

  appendWall(positions, normals, indices, segments, (a) => outerRadius(a, teeth, root, tip), half, false);
  appendWall(positions, normals, indices, segments, () => inner, half, true);
  return mesh(gl, new Float32Array(positions), new Float32Array(normals), new Uint32Array(indices));
}

function appendWall(
  positions: number[],
  normals: number[],
  indices: number[],
  segments: number,
  radiusAt: (angle: number) => number,
  half: number,
  invert: boolean,
) {
  for (let i = 0; i < segments; i += 1) {
    const n = (i + 1) % segments;
    const a0 = (i / segments) * TAU;
    const a1 = (n / segments) * TAU;
    const r0 = radiusAt(a0);
    const r1 = radiusAt(a1);
    const x0 = Math.cos(a0) * r0;
    const y0 = Math.sin(a0) * r0;
    const x1 = Math.cos(a1) * r1;
    const y1 = Math.sin(a1) * r1;
    let nx = y1 - y0;
    let ny = -(x1 - x0);
    const nl = Math.hypot(nx, ny) || 1;
    nx /= nl;
    ny /= nl;
    if (invert) { nx *= -1; ny *= -1; }
    const base = positions.length / 3;
    push(positions, normals, x0, y0, half, nx, ny, 0);
    push(positions, normals, x1, y1, half, nx, ny, 0);
    push(positions, normals, x1, y1, -half, nx, ny, 0);
    push(positions, normals, x0, y0, -half, nx, ny, 0);
    indices.push(base, base + 1, base + 2, base, base + 2, base + 3);
  }
}

function outerRadius(angle: number, teeth: number, root: number, tip: number) {
  const step = TAU / teeth;
  const local = ((angle % step) + step) % step / step;
  const rise = smoothstep(0.08, 0.28, local);
  const fall = 1 - smoothstep(0.72, 0.92, local);
  return root + (tip - root) * Math.min(rise, fall);
}

function drawBackground(state: Renderer, eye: Vec3, aspects: RenderAspects) {
  const { gl, bgProgram, screen, composeTexture } = state;
  gl.useProgram(bgProgram);
  disableUnusedVertexAttributes(gl, [gl.getAttribLocation(bgProgram, "a_position")]);
  gl.bindBuffer(gl.ARRAY_BUFFER, screen);
  const pos = gl.getAttribLocation(bgProgram, "a_position");
  if (pos >= 0) {
    gl.enableVertexAttribArray(pos);
    gl.vertexAttribPointer(pos, 2, gl.FLOAT, false, 0, 0);
  }
  gl.uniform3f(gl.getUniformLocation(bgProgram, "u_eye"), eye.x, eye.y, eye.z);
  gl.uniform2f(gl.getUniformLocation(bgProgram, "u_tileGrid"), aspects.tileGrid.columns, aspects.tileGrid.rows);
  gl.activeTexture(gl.TEXTURE0);
  gl.bindTexture(gl.TEXTURE_2D, composeTexture);
  gl.uniform1i(gl.getUniformLocation(bgProgram, "u_composeTexture"), 0);
  gl.disable(gl.DEPTH_TEST);
  gl.drawArrays(gl.TRIANGLES, 0, 3);
  gl.enable(gl.DEPTH_TEST);
}

function drawMesh(
  state: Renderer,
  object: Mesh,
  model: Mat4,
  camera: { eye: Vec3; viewProjection: Mat4 },
  scene: SceneState,
  aspects: RenderAspects,
  material: { base: [number, number, number]; edge: [number, number, number]; metalness: number; roughness: number; shadowed: number },
) {
  const { gl, surfaceProgram } = state;
  gl.useProgram(surfaceProgram);
  bind(gl, surfaceProgram, object);
  uniformMat4(gl, surfaceProgram, "u_model", model);
  uniformMat4(gl, surfaceProgram, "u_viewProjection", camera.viewProjection);
  gl.uniform3f(gl.getUniformLocation(surfaceProgram, "u_eye"), camera.eye.x, camera.eye.y, camera.eye.z);
  gl.uniform3f(gl.getUniformLocation(surfaceProgram, "u_light"), scene.light.x, scene.light.y, scene.light.z);
  gl.uniform3f(gl.getUniformLocation(surfaceProgram, "u_base"), ...material.base);
  gl.uniform3f(gl.getUniformLocation(surfaceProgram, "u_edge"), ...material.edge);
  gl.uniform1f(gl.getUniformLocation(surfaceProgram, "u_lightIntensity"), scene.light.intensity);
  gl.uniform1f(gl.getUniformLocation(surfaceProgram, "u_metalness"), material.metalness);
  gl.uniform1f(gl.getUniformLocation(surfaceProgram, "u_roughness"), material.roughness);
  gl.uniform1f(gl.getUniformLocation(surfaceProgram, "u_shadowed"), material.shadowed);
  gl.uniform1f(gl.getUniformLocation(surfaceProgram, "u_shadowRadius"), aspects.dimensions.outerRadius * 1.3);
  gl.uniform1f(gl.getUniformLocation(surfaceProgram, "u_shadowOpacity"), 0.22 + aspects.shading.shadowOpacity * 0.46);
  gl.uniform1f(gl.getUniformLocation(surfaceProgram, "u_rim"), 0.24 + aspects.lighting.highlightBoost * 0.12);
  gl.uniform2f(gl.getUniformLocation(surfaceProgram, "u_resolution"), RENDER_WIDTH, RENDER_HEIGHT);
  gl.uniform2f(gl.getUniformLocation(surfaceProgram, "u_tileGrid"), aspects.tileGrid.columns, aspects.tileGrid.rows);
  gl.activeTexture(gl.TEXTURE0);
  gl.bindTexture(gl.TEXTURE_2D, state.composeTexture);
  gl.uniform1i(gl.getUniformLocation(surfaceProgram, "u_composeTexture"), 0);
  gl.drawElements(gl.TRIANGLES, object.indexCount, gl.UNSIGNED_INT, 0);
}

function mesh(gl: WebGL2RenderingContext, positions: Float32Array, normals: Float32Array, indices: Uint32Array): Mesh {
  return {
    vertex: buffer(gl, gl.ARRAY_BUFFER, positions),
    normal: buffer(gl, gl.ARRAY_BUFFER, normals),
    index: buffer(gl, gl.ELEMENT_ARRAY_BUFFER, indices),
    indexCount: indices.length,
  };
}

function createTileTexture(gl: WebGL2RenderingContext) {
  const texture = gl.createTexture();
  if (!texture) throw new Error("Failed to create tile texture.");
  gl.bindTexture(gl.TEXTURE_2D, texture);
  gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_MIN_FILTER, gl.NEAREST);
  gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_MAG_FILTER, gl.NEAREST);
  gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_WRAP_S, gl.CLAMP_TO_EDGE);
  gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_WRAP_T, gl.CLAMP_TO_EDGE);
  gl.texImage2D(gl.TEXTURE_2D, 0, gl.RGBA32F, 1, 1, 0, gl.RGBA, gl.FLOAT, new Float32Array([0, 0, 0, 0]));
  return texture;
}

function uploadTileTexture(state: Renderer, aspects: RenderAspects): UploadStats {
  const fullUpload = aspects.fullComposeUpload;
  return uploadPackedComposeTexture(
    state.gl,
    state,
    state.composeTexture,
    aspects.tileGrid.columns,
    aspects.tileGrid.rows,
    aspects.dirtyTileIndices,
    aspects.tileUploadBuffer,
    aspects.dirtyTileRects,
    fullUpload,
  );
}

function uploadPackedComposeTexture(
  gl: WebGL2RenderingContext,
  state: Renderer,
  texture: WebGLTexture,
  columns: number,
  rows: number,
  dirtyTileIndices: number[],
  packedUploadBuffer: Float32Array,
  dirtyTileRects: RenderAspects["dirtyTileRects"],
  fullUpload: boolean,
): UploadStats {
  gl.bindTexture(gl.TEXTURE_2D, texture);
  ensureComposePixels(state, columns, rows);
  ensureComposeTextureStorage(gl, state, columns, rows);
  if (fullUpload) {
    state.composePixels.set(packedUploadBuffer);
    gl.texImage2D(gl.TEXTURE_2D, 0, gl.RGBA32F, columns, rows, 0, gl.RGBA, gl.FLOAT, state.composePixels);
    state.composeTextureColumns = columns;
    state.composeTextureRows = rows;
    return {
      dirtyTiles: dirtyTileIndices.length,
      uploadedTiles: dirtyTileIndices.length,
      uploadSpans: 1,
      uploadBytes: state.composePixels.byteLength,
    };
  }
  writeDirtyComposePixels(state.composePixels, packedUploadBuffer, dirtyTileIndices);
  const rectangles = dirtyTileRects;
  gl.pixelStorei(gl.UNPACK_ALIGNMENT, 1);
  for (const rectangle of rectangles) {
    const clamped = clampUploadRect(rectangle, columns, rows);
    if (!clamped) {
      continue;
    }
    const rectPixels = buildRectUploadPixels(state, clamped, columns);
    gl.texSubImage2D(
      gl.TEXTURE_2D,
      0,
      clamped.startColumn,
      clamped.row,
      clamped.width,
      clamped.height,
      gl.RGBA,
      gl.FLOAT,
      rectPixels,
    );
  }
  return {
    dirtyTiles: dirtyTileIndices.length,
    uploadedTiles: dirtyTileIndices.length,
    uploadSpans: rectangles.length,
    uploadBytes: rectangles.reduce((total, rectangle) => total + rectangle.width * rectangle.height * 16, 0),
  };
}

function writeDirtyComposePixels(
  target: Float32Array,
  packedUploadBuffer: Float32Array,
  dirtyTileIndices: number[],
) {
  for (let dirtyIndex = 0; dirtyIndex < dirtyTileIndices.length; dirtyIndex += 1) {
    const tileIndex = dirtyTileIndices[dirtyIndex];
    const sourceIndex = tileIndex * 4;
    const targetIndex = tileIndex * 4;
    target[targetIndex] = clamp(packedUploadBuffer[sourceIndex], 0, 1);
    target[targetIndex + 1] = clamp(packedUploadBuffer[sourceIndex + 1], 0, 1);
    target[targetIndex + 2] = clamp(packedUploadBuffer[sourceIndex + 2], 0, 1);
    target[targetIndex + 3] = clamp(packedUploadBuffer[sourceIndex + 3], 0, 1);
  }
}

function ensureComposePixels(state: Renderer, columns: number, rows: number) {
  if (state.composeColumns === columns && state.composeRows === rows && state.composePixels.length === columns * rows * 4) {
    return;
  }
  state.composeColumns = columns;
  state.composeRows = rows;
  state.composePixels = new Float32Array(columns * rows * 4);
  state.composeScratch = new Float32Array(4);
}

function ensureComposeTextureStorage(
  gl: WebGL2RenderingContext,
  state: Renderer,
  columns: number,
  rows: number,
) {
  if (
    state.composeTextureColumns === columns
    && state.composeTextureRows === rows
  ) {
    return;
  }
  gl.texImage2D(
    gl.TEXTURE_2D,
    0,
    gl.RGBA32F,
    columns,
    rows,
    0,
    gl.RGBA,
    gl.FLOAT,
    state.composePixels,
  );
  state.composeTextureColumns = columns;
  state.composeTextureRows = rows;
}

function buildRectUploadPixels(
  state: Renderer,
  rectangle: RenderAspects["dirtyTileRects"][number],
  columns: number,
) {
  const pixelCount = rectangle.width * rectangle.height * 4;
  if (state.composeScratch.length !== pixelCount) {
    state.composeScratch = new Float32Array(pixelCount);
  }
  const rowWidth = rectangle.width * 4;
  for (let rowOffset = 0; rowOffset < rectangle.height; rowOffset += 1) {
    const sourceStart =
      ((rectangle.row + rowOffset) * columns + rectangle.startColumn) * 4;
    state.composeScratch.set(
      state.composePixels.subarray(sourceStart, sourceStart + rowWidth),
      rowOffset * rowWidth,
    );
  }
  return state.composeScratch;
}

function bind(gl: WebGL2RenderingContext, program: WebGLProgram, object: Mesh) {
  const pos = gl.getAttribLocation(program, "a_position");
  const normal = gl.getAttribLocation(program, "a_normal");
  disableUnusedVertexAttributes(gl, [pos, normal]);
  gl.bindBuffer(gl.ARRAY_BUFFER, object.vertex);
  if (pos >= 0) {
    gl.enableVertexAttribArray(pos);
    gl.vertexAttribPointer(pos, 3, gl.FLOAT, false, 0, 0);
  }
  gl.bindBuffer(gl.ARRAY_BUFFER, object.normal);
  if (normal >= 0) {
    gl.enableVertexAttribArray(normal);
    gl.vertexAttribPointer(normal, 3, gl.FLOAT, false, 0, 0);
  }
  gl.bindBuffer(gl.ELEMENT_ARRAY_BUFFER, object.index);
}

function disableUnusedVertexAttributes(gl: WebGL2RenderingContext, keep: number[]) {
  const preserved = new Set(keep.filter((index) => index >= 0));
  const maxAttributes = gl.getParameter(gl.MAX_VERTEX_ATTRIBS) as number;
  for (let index = 0; index < maxAttributes; index += 1) {
    if (!preserved.has(index)) {
      gl.disableVertexAttribArray(index);
    }
  }
}

function clampUploadRect(
  rectangle: RenderAspects["dirtyTileRects"][number],
  columns: number,
  rows: number,
): RenderAspects["dirtyTileRects"][number] | null {
  if (rectangle.row >= rows || rectangle.startColumn >= columns) {
    return null;
  }
  const width = Math.min(rectangle.width, columns - rectangle.startColumn);
  const height = Math.min(rectangle.height, rows - rectangle.row);
  if (width <= 0 || height <= 0) {
    return null;
  }
  return {
    row: rectangle.row,
    startColumn: rectangle.startColumn,
    width,
    height,
  };
}

function buildCamera(camera: CameraState) {
  const eye = { x: camera.x, y: camera.y, z: camera.z };
  const forward = {
    x: -Math.sin(camera.yaw) * Math.cos(camera.pitch),
    y: -Math.sin(camera.pitch),
    z: Math.cos(camera.yaw) * Math.cos(camera.pitch),
  };
  const target = { x: eye.x + forward.x, y: eye.y + forward.y, z: eye.z + forward.z };
  return { eye, viewProjection: multiply(perspective(0.66, RENDER_WIDTH / RENDER_HEIGHT, 0.1, 60), lookAt(eye, target, { x: 0, y: 1, z: 0 })) };
}

function perspective(fovy: number, aspect: number, near: number, far: number): Mat4 {
  const f = 1 / Math.tan(fovy / 2);
  const nf = 1 / (near - far);
  return new Float32Array([f / aspect, 0, 0, 0, 0, f, 0, 0, 0, 0, (far + near) * nf, -1, 0, 0, 2 * far * near * nf, 0]);
}

function lookAt(eye: Vec3, target: Vec3, up: Vec3): Mat4 {
  const z = norm({ x: eye.x - target.x, y: eye.y - target.y, z: eye.z - target.z });
  const x = norm(cross(up, z));
  const y = cross(z, x);
  return new Float32Array([x.x, y.x, z.x, 0, x.y, y.y, z.y, 0, x.z, y.z, z.z, 0, -dot(x, eye), -dot(y, eye), -dot(z, eye), 1]);
}

function identity(): Mat4 { return new Float32Array([1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1]); }
function translation(x: number, y: number, z: number): Mat4 { const out = identity(); out[12] = x; out[13] = y; out[14] = z; return out; }
function scale(x: number, y: number, z: number): Mat4 { return new Float32Array([x, 0, 0, 0, 0, y, 0, 0, 0, 0, z, 0, 0, 0, 0, 1]); }
function rotationX(a: number): Mat4 { const c = Math.cos(a), s = Math.sin(a); return new Float32Array([1, 0, 0, 0, 0, c, s, 0, 0, -s, c, 0, 0, 0, 0, 1]); }
function rotationY(a: number): Mat4 { const c = Math.cos(a), s = Math.sin(a); return new Float32Array([c, 0, -s, 0, 0, 1, 0, 0, s, 0, c, 0, 0, 0, 0, 1]); }
function rotationZ(a: number): Mat4 { const c = Math.cos(a), s = Math.sin(a); return new Float32Array([c, s, 0, 0, -s, c, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1]); }
function multiply(left: Mat4, right: Mat4): Mat4 {
  const out = new Float32Array(16);
  for (let c = 0; c < 4; c += 1) for (let r = 0; r < 4; r += 1) out[c * 4 + r] = left[r] * right[c * 4] + left[4 + r] * right[c * 4 + 1] + left[8 + r] * right[c * 4 + 2] + left[12 + r] * right[c * 4 + 3];
  return out;
}
function multiplyMany(...matrices: Mat4[]) { return matrices.reduce((acc, m) => multiply(acc, m), identity()); }
function dot(a: Vec3, b: Vec3) { return a.x * b.x + a.y * b.y + a.z * b.z; }
function cross(a: Vec3, b: Vec3): Vec3 { return { x: a.y * b.z - a.z * b.y, y: a.z * b.x - a.x * b.z, z: a.x * b.y - a.y * b.x }; }
function norm(v: Vec3): Vec3 { const l = Math.hypot(v.x, v.y, v.z) || 1; return { x: v.x / l, y: v.y / l, z: v.z / l }; }
function push(pos: number[], normArr: number[], x: number, y: number, z: number, nx: number, ny: number, nz: number) { pos.push(x, y, z); normArr.push(nx, ny, nz); }
function smoothstep(e0: number, e1: number, x: number) { const t = clamp((x - e0) / (e1 - e0 || 1), 0, 1); return t * t * (3 - 2 * t); }

function program(gl: WebGL2RenderingContext, vertexSource: string, fragmentSource: string) {
  const p = gl.createProgram();
  if (!p) throw new Error("Failed to create WebGL program.");
  const vs = shader(gl, gl.VERTEX_SHADER, vertexSource);
  const fs = shader(gl, gl.FRAGMENT_SHADER, fragmentSource);
  gl.attachShader(p, vs);
  gl.attachShader(p, fs);
  gl.linkProgram(p);
  if (!gl.getProgramParameter(p, gl.LINK_STATUS)) throw new Error(gl.getProgramInfoLog(p) ?? "WebGL link failed.");
  gl.deleteShader(vs);
  gl.deleteShader(fs);
  return p;
}

function shader(gl: WebGL2RenderingContext, type: number, source: string) {
  const s = gl.createShader(type);
  if (!s) throw new Error("Failed to create shader.");
  gl.shaderSource(s, source);
  gl.compileShader(s);
  if (!gl.getShaderParameter(s, gl.COMPILE_STATUS)) throw new Error(gl.getShaderInfoLog(s) ?? "WebGL compile failed.");
  return s;
}

function buffer(
  gl: WebGL2RenderingContext,
  target: number,
  data: Float32Array | Uint32Array,
) {
  const b = gl.createBuffer();
  if (!b) throw new Error("Failed to create buffer.");
  gl.bindBuffer(target, b);
  gl.bufferData(target, data as unknown as BufferSource, gl.STATIC_DRAW);
  return b;
}

function uniformMat4(gl: WebGL2RenderingContext, program: WebGLProgram, name: string, value: Mat4) {
  gl.uniformMatrix4fv(gl.getUniformLocation(program, name), false, value);
}

function moveCamera(base: CameraState, vector: Vec3, scalar: number): CameraState {
  return { ...base, x: base.x + vector.x * scalar, y: base.y + vector.y * scalar, z: base.z + vector.z * scalar };
}

function cameraLookAt(position: Vec3, target: Vec3): CameraState {
  const dx = target.x - position.x;
  const dy = target.y - position.y;
  const dz = target.z - position.z;
  const flat = Math.hypot(dx, dz) || 1;
  return { x: position.x, y: position.y, z: position.z, yaw: Math.atan2(-dx, dz), pitch: -Math.atan2(dy, flat) };
}

function clamp(value: number, min: number, max: number) { return Math.min(Math.max(value, min), max); }

const bgVertex = `#version 300 es
in vec2 a_position;
out vec2 v_uv;
void main() {
  v_uv = a_position * 0.5 + 0.5;
  gl_Position = vec4(a_position, 0.0, 1.0);
}`;

const bgFragment = `#version 300 es
precision highp float;
in vec2 v_uv;
uniform vec3 u_eye;
uniform sampler2D u_composeTexture;
uniform vec2 u_tileGrid;
out vec4 outColor;
float radial(vec2 uv, vec2 center, float radius) { return clamp(1.0 - distance(uv, center) / radius, 0.0, 1.0); }
void main() {
  vec2 tileUv = floor(v_uv * u_tileGrid) / u_tileGrid;
  vec4 composeTile = texture(u_composeTexture, tileUv);
  vec3 top = vec3(0.025, 0.045, 0.07);
  vec3 bottom = vec3(0.11, 0.13, 0.16);
  vec3 color = mix(bottom, top, smoothstep(0.02, 0.86, v_uv.y));
  float blueKey = radial(v_uv, vec2(0.46, 0.31), 0.46);
  float amberKick = radial(v_uv, vec2(0.84, 0.22), 0.30);
  float floorGlow = radial(v_uv, vec2(0.50, 0.92), 0.62);
  color += vec3(0.08, 0.18, 0.24) * blueKey * (0.65 + composeTile.z * 0.55);
  color += vec3(0.20, 0.11, 0.05) * amberKick * (0.28 + composeTile.w * 0.55);
  color += vec3(0.06, 0.08, 0.09) * floorGlow * (0.25 + composeTile.w * 0.75);
  color = mix(color, vec3(0.18, 0.205, 0.225), smoothstep(0.56, 1.0, v_uv.y) * (0.58 + composeTile.w * 0.42));
  float vignette = radial(v_uv, vec2(0.5, 0.48), 0.92);
  color *= 0.70 + vignette * 0.30;
  outColor = vec4(color, 1.0);
}`;

const surfaceVertex = `#version 300 es
precision highp float;
in vec3 a_position;
in vec3 a_normal;
uniform mat4 u_model;
uniform mat4 u_viewProjection;
out vec3 v_worldPosition;
out vec3 v_worldNormal;
void main() {
  vec4 world = u_model * vec4(a_position, 1.0);
  v_worldPosition = world.xyz;
  v_worldNormal = normalize(mat3(u_model) * a_normal);
  gl_Position = u_viewProjection * world;
}`;

const surfaceFragment = `#version 300 es
precision highp float;
in vec3 v_worldPosition;
in vec3 v_worldNormal;
uniform vec3 u_eye;
uniform vec3 u_light;
uniform vec3 u_base;
uniform vec3 u_edge;
uniform float u_lightIntensity;
uniform float u_metalness;
uniform float u_roughness;
uniform float u_shadowed;
uniform float u_shadowRadius;
uniform float u_shadowOpacity;
uniform float u_rim;
uniform sampler2D u_composeTexture;
uniform vec2 u_tileGrid;
uniform vec2 u_resolution;
out vec4 outColor;
void main() {
  vec2 tileUv = floor(gl_FragCoord.xy / u_resolution * u_tileGrid) / u_tileGrid;
  vec4 composeTile = texture(u_composeTexture, tileUv);
  vec3 n = normalize(v_worldNormal);
  vec3 l = normalize(u_light - v_worldPosition);
  vec3 v = normalize(u_eye - v_worldPosition);
  vec3 h = normalize(l + v);
  float diffuse = max(dot(n, l), 0.0);
  float specular = pow(max(dot(n, h), 0.0), mix(10.0, 72.0, 1.0 - u_roughness)) * (0.52 + composeTile.z * 1.15);
  float fresnel = pow(1.0 - max(dot(n, v), 0.0), 4.0);
  float toothMask = composeTile.y;
  float boreMask = composeTile.y;
  float faceMask = composeTile.x;
  float reflectionMask = composeTile.z;
  float edgeBoost = composeTile.x;
  vec3 baseMetal = mix(u_base * vec3(0.86, 0.88, 0.92), u_edge, faceMask * 0.22);
  vec3 toothMetal = mix(baseMetal, vec3(0.94, 0.97, 1.0), toothMask * 0.42);
  vec3 boreMetal = mix(toothMetal, vec3(0.18, 0.20, 0.24), boreMask * 0.72);
  vec3 color = boreMetal * (0.15 + diffuse * (0.62 + u_lightIntensity * 0.24)) * (0.58 + edgeBoost * 0.58);
  color += mix(vec3(0.10), u_edge, u_metalness) * specular * (0.84 + u_lightIntensity * 0.18);
  color += u_edge * fresnel * (u_rim + composeTile.z * 0.25 + toothMask * 0.18);
  color += vec3(0.18, 0.22, 0.26) * reflectionMask * 0.16;
  color += vec3(0.05, 0.06, 0.07) * composeTile.w * 0.12;
  if (u_shadowed > 0.5) {
    float shadow = 1.0 - smoothstep(u_shadowRadius * 0.45, u_shadowRadius * 1.05, distance(v_worldPosition.xz, vec2(0.0)));
    color *= 1.0 - shadow * (u_shadowOpacity * (0.65 + composeTile.w * 0.35));
    color += vec3(0.03, 0.04, 0.05) * composeTile.w * 0.30;
  }
  outColor = vec4(color, 1.0);
}`;
