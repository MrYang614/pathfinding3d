import * as THREE from 'three';
import { OrbitControls } from 'three/addons/controls/OrbitControls.js';
import { GLTFLoader } from 'three/addons/loaders/GLTFLoader.js';
import { RoomEnvironment } from 'three/addons/environments/RoomEnvironment.js';
import { PathfindingHelper } from 'three-pathfinding-3d';
import { PathfindingWasm } from 'pathfinding3d';

const viewport = document.getElementById('app');

const Color = {
	GROUND: 0x606060,
	NAVMESH: 0xffffff,
};

const ZONE = 'level';
const SPEED = 5;
const PATH_OUTPUT = new Float32Array(1024 * 3);

let navmesh: THREE.Mesh;

let groupID: number | undefined;
let path: THREE.Vector3[] | null;

const playerPosition = new THREE.Vector3(-3.5, 0.5, 5.5);
const targetPosition = new THREE.Vector3();

const pathfinder = new PathfindingWasm();
const helper = new PathfindingHelper();
const clock = new THREE.Clock();
const mouse = new THREE.Vector2();
const mouseDown = new THREE.Vector2();
const raycaster = new THREE.Raycaster();

const renderer = new THREE.WebGLRenderer();
renderer.setPixelRatio(window.devicePixelRatio);
renderer.setSize(window.innerWidth, window.innerHeight);
renderer.setClearColor(0xffffff);
renderer.toneMapping = THREE.ACESFilmicToneMapping;
viewport.appendChild(renderer.domElement);

const environment = new RoomEnvironment();
const pmremGenerator = new THREE.PMREMGenerator(renderer);

const scene = new THREE.Scene();
scene.background = new THREE.Color(0xbbbbbb);
scene.environment = pmremGenerator.fromScene(environment).texture;
scene.add(helper);
environment.dispose();

const camera = new THREE.PerspectiveCamera(45, window.innerWidth / window.innerHeight, 1, 2000);
camera.position.x = -10;
camera.position.y = 14;
camera.position.z = 10;

const controls = new OrbitControls(camera, renderer.domElement);
controls.dampingFactor = 0.2;

const ambient = new THREE.AmbientLight(0x101030);
scene.add(ambient);

const directionalLight = new THREE.DirectionalLight(0xffeedd);
directionalLight.position.set(0, 0.5, 0.5);
scene.add(directionalLight);

init();
animate();

async function init() {
	const gltfLoader = new GLTFLoader();

	gltfLoader.load(
		'/level.glb',
		(gltf) => {
			const levelMesh = gltf.scene.getObjectByName('Cube') as THREE.Mesh;
			const levelMat = new THREE.MeshStandardMaterial({
				color: Color.GROUND,
				flatShading: true,
				roughness: 1,
				metalness: 0,
			});
			const mesh = new THREE.Mesh(levelMesh.geometry, levelMat);
			scene.add(mesh);
		},
		null,
	);

	gltfLoader.load(
		'/level.nav.glb',
		(gltf) => {
			const _navmesh = gltf.scene.getObjectByName('Navmesh_Mesh') as THREE.Mesh;

			console.time('createZone()');
			const positions = _navmesh.geometry.attributes.position.array as Float32Array;
			const indices = _navmesh.geometry.index.array as Uint32Array;
			pathfinder.create_zone(ZONE,positions, indices, 0.001);
			console.timeEnd('createZone()');

			const navWireframe = new THREE.Mesh(
				_navmesh.geometry,
				new THREE.MeshBasicMaterial({
					color: 0x808080,
					wireframe: true,
					// depthTest: false,
					// transparent: true,
				}),
			);
			navWireframe.position.y = 0.1;
			scene.add(navWireframe);

			navmesh = new THREE.Mesh(
				_navmesh.geometry,
				new THREE.MeshBasicMaterial({
					color: Color.NAVMESH,
					side: THREE.DoubleSide,
				}),
			);

			scene.add(navmesh);

			groupID = pathfinder.get_group(
				ZONE,
				playerPosition.x,
				playerPosition.y,
				playerPosition.z,
				true,
			);
		},
		null,
	);

	helper.setPlayerPosition(new THREE.Vector3(-3.5, 0.5, 5.5)).setTargetPosition(new THREE.Vector3(-3.5, 0.5, 5.5));

	document.addEventListener('pointerdown', onDocumentPointerDown, false);
	document.addEventListener('pointerup', onDocumentPointerUp, false);
	window.addEventListener('resize', onWindowResize, false);
}

function nodeCenter(zoneId: string, groupId: number, nodeId: number): THREE.Vector3 | null {
	const center = pathfinder.node_center(zoneId, groupId, nodeId);
	if (!center) return null;
	return new THREE.Vector3(center[0], center[1], center[2]);
}

function closestNodeCenter(
	zoneId: string,
	groupId: number,
	x: number,
	y: number,
	z: number,
): THREE.Vector3 | null {
	const nodeId = pathfinder.get_closest_node_id(zoneId, groupId, x, y, z, true);
	if (nodeId === undefined) return null;
	return nodeCenter(zoneId, groupId, nodeId);
}

function onDocumentPointerDown(event: PointerEvent) {
	mouseDown.x = (event.clientX / window.innerWidth) * 2 - 1;
	mouseDown.y = -(event.clientY / window.innerHeight) * 2 + 1;
}

function onDocumentPointerUp(event: PointerEvent) {
	mouse.x = (event.clientX / window.innerWidth) * 2 - 1;
	mouse.y = -(event.clientY / window.innerHeight) * 2 + 1;

	if (Math.abs(mouseDown.x - mouse.x) > 0 || Math.abs(mouseDown.y - mouse.y) > 0) return; // Prevent unwanted click when rotate camera.

	camera.updateMatrixWorld();

	raycaster.setFromCamera(mouse, camera);

	const intersects = raycaster.intersectObject(navmesh);

	if (!intersects.length) return;

	targetPosition.copy(intersects[0].point);

	helper.reset().setPlayerPosition(playerPosition);

	// Teleport on ctrl/cmd click or RMB.
	if (event.metaKey || event.ctrlKey || event.button === 2) {
		path = null;
		groupID = pathfinder.get_group(
			ZONE,
			targetPosition.x,
			targetPosition.y,
			targetPosition.z,
			true,
		);

		helper.setPlayerPosition(playerPosition.copy(targetPosition));
		if (groupID !== undefined) {
			const center = closestNodeCenter(ZONE, groupID, playerPosition.x, playerPosition.y, playerPosition.z);
			if (center) helper.setNodePosition(center);
		}

		return;
	}

	const targetGroupID = pathfinder.get_group(
		ZONE,
		targetPosition.x,
		targetPosition.y,
		targetPosition.z,
		true,
	);

	helper.setTargetPosition(targetPosition);
	if (targetGroupID !== undefined) {
		const center = closestNodeCenter(ZONE, targetGroupID, targetPosition.x, targetPosition.y, targetPosition.z);
		if (center) helper.setNodePosition(center);
	}

	if (groupID === undefined) return;

	const pointCount = pathfinder.find_path(
		ZONE,
		groupID,
		playerPosition.x,
		playerPosition.y,
		playerPosition.z,
		targetPosition.x,
		targetPosition.y,
		targetPosition.z,
		PATH_OUTPUT,
	);

	if (pointCount > 0) {
		path = [];
		for (let i = 0; i < pointCount; i++) {
			path.push(
				new THREE.Vector3(PATH_OUTPUT[i * 3], PATH_OUTPUT[i * 3 + 1], PATH_OUTPUT[i * 3 + 2]),
			);
		}

		helper.setPath(path);
	} else {
		path = null;
		const step =
			targetGroupID !== undefined
				? closestNodeCenter(ZONE, targetGroupID, targetPosition.x, targetPosition.y, targetPosition.z)
				: null;
		helper.setStepPosition(step ?? targetPosition);
	}
}

function onWindowResize() {
	camera.aspect = window.innerWidth / window.innerHeight;
	camera.updateProjectionMatrix();

	renderer.setSize(window.innerWidth, window.innerHeight);
}

function animate() {
	requestAnimationFrame(animate);
	tick(clock.getDelta());
	renderer.render(scene, camera);
}

function tick(dt: number) {
	if (!path || !path.length) return;

	const targetPosition = path[0];
	const velocity = targetPosition.clone().sub(playerPosition);

	if (velocity.lengthSq() > 0.05 * 0.05) {
		velocity.normalize();
		// Move player to target
		playerPosition.add(velocity.multiplyScalar(dt * SPEED));
		helper.setPlayerPosition(playerPosition);
	} else {
		// Remove node from the path we calculated
		path.shift();
	}
}