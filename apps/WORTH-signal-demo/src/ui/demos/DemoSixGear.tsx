import { useEffect, useRef } from "react";
import * as THREE from "three";

export interface GearRenderParams {
  innerRadius: number;
  outerRadius: number;
  thickness: number;
  teeth: number;
  material?: "steel" | "titanium" | "ceramic";
  rotation?: number;
}

interface DemoSixGearProps {
  params: GearRenderParams;
}

function createGearShape(params: GearRenderParams): THREE.Shape {
  const shape = new THREE.Shape();
  const points = params.teeth * 2;
  for (let index = 0; index <= points; index += 1) {
    const angle = (index / points) * Math.PI * 2;
    const radius = index % 2 === 0 ? params.outerRadius : params.outerRadius * 0.82;
    const x = Math.cos(angle) * radius;
    const y = Math.sin(angle) * radius;
    if (index === 0) shape.moveTo(x, y);
    else shape.lineTo(x, y);
  }
  const hole = new THREE.Path();
  hole.absarc(0, 0, params.innerRadius, 0, Math.PI * 2, true);
  shape.holes.push(hole);
  return shape;
}

export function DemoSixGear({ params }: DemoSixGearProps) {
  const hostRef = useRef<HTMLDivElement | null>(null);
  const initialParamsRef = useRef(params);
  const sceneRef = useRef<{
    renderer: THREE.WebGLRenderer;
    scene: THREE.Scene;
    camera: THREE.PerspectiveCamera;
    mesh: THREE.Mesh;
    material: THREE.MeshStandardMaterial;
    frame: number;
  } | null>(null);

  useEffect(() => {
    const host = hostRef.current;
    if (!host) return;
    const container = host;
    const scene = new THREE.Scene();
    const camera = new THREE.PerspectiveCamera(38, 1, 0.1, 100);
    camera.position.set(0, -5, 4);
    camera.lookAt(0, 0, 0);
    const renderer = new THREE.WebGLRenderer({ antialias: true, alpha: true });
    renderer.setPixelRatio(Math.min(window.devicePixelRatio, 2));
    container.appendChild(renderer.domElement);

    const material = new THREE.MeshStandardMaterial({
      color: "#ff9c9c",
      metalness: 0.72,
      roughness: 0.24,
    });
    const initialParams = initialParamsRef.current;
    const mesh = new THREE.Mesh(new THREE.ExtrudeGeometry(createGearShape(initialParams), {
      depth: initialParams.thickness,
      bevelEnabled: true,
      bevelSize: 0.04,
      bevelThickness: 0.03,
      bevelSegments: 2,
    }), material);
    mesh.rotation.x = Math.PI / 2;
    mesh.geometry.center();
    scene.add(mesh);
    scene.add(new THREE.AmbientLight("#ffffff", 0.6));
    const key = new THREE.DirectionalLight("#ffffff", 2.1);
    key.position.set(3, -4, 5);
    scene.add(key);

    function resize() {
      const width = container.clientWidth;
      const height = Math.max(300, container.clientHeight);
      renderer.setSize(width, height);
      camera.aspect = width / height;
      camera.updateProjectionMatrix();
    }

    function animate() {
      mesh.rotation.z += 0.006;
      renderer.render(scene, camera);
      const current = sceneRef.current;
      if (current) current.frame = window.requestAnimationFrame(animate);
    }

    resize();
    window.addEventListener("resize", resize);
    sceneRef.current = {
      renderer,
      scene,
      camera,
      mesh,
      material,
      frame: window.requestAnimationFrame(animate),
    };
    return () => {
      const current = sceneRef.current;
      if (current) window.cancelAnimationFrame(current.frame);
      window.removeEventListener("resize", resize);
      mesh.geometry.dispose();
      material.dispose();
      renderer.dispose();
      renderer.domElement.remove();
      sceneRef.current = null;
    };
  }, []);

  useEffect(() => {
    const current = sceneRef.current;
    if (!current) return;
    const oldGeometry = current.mesh.geometry;
    const nextGeometry = new THREE.ExtrudeGeometry(createGearShape(params), {
      depth: params.thickness,
      bevelEnabled: true,
      bevelSize: 0.04,
      bevelThickness: 0.03,
      bevelSegments: 2,
    });
    nextGeometry.center();
    current.mesh.geometry = nextGeometry;
    oldGeometry.dispose();
  }, [params]);

  useEffect(() => {
    const current = sceneRef.current;
    if (!current) return;
    current.material.color.set(materialColor(params.material));
    if (typeof params.rotation === "number") {
      current.mesh.rotation.z = THREE.MathUtils.degToRad(params.rotation);
    }
  }, [params.material, params.rotation]);

  return <div className="demo-six-gear-stage" ref={hostRef} />;
}

function materialColor(material: DemoSixGearProps["params"]["material"]) {
  return ({
    steel: "#f1c76f",
    titanium: "#7dd3fc",
    ceramic: "#f0abfc",
  } as const)[material ?? "steel"];
}
