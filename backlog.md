# Backlog

## Sprint 1: Core Combat Framework (Minimum Viable Product)

- Task 1.1: Refactor existing code into an decoupled GamePlugin structure across modular files (player.rs, enemy.rs, ui.rs).

- Task 1.2: Unify shared data components (Health, Speed) to eliminate duplicate code across distinct entity types.

- Task 1.3: Implement a timer-based auto-attack system shooting Projectile entities toward the closest obstacle.

- Task 1.4: Build an instant-hit collision module utilizing .distance_squared() to handle safe projectile despawning and enemy damage.

- Task 1.5: Implement an automated XP/Resource drop pool with a PickupRadius magnetic attraction loop.

- Task 1.6: Migrate textual debugging tracking into a monitor-anchored Bevy UI Node HUD.

## Sprint 2: The Macro-Square Spatial Grid (Optimization Scale)

- Task 2.1: Create a custom global SpatialGrid resource mapping entity IDs directly to chunk coordinate squares.

- Task 2.2: Write a parallelized registration system updating entity grid blocks every frame.

- Task 2.3: Redesign collision queries to strictly cross-examine entities within identical or immediately adjacent grid spaces.

## Sprint 3: Infinite Procedural World Generation (The "Ark" Layer)

- Task 3.1: Implement modular 32x32 terrain tile chunks tracking spatial parameters.

- Task 3.2: Introduce coherent noise algorithms to spawn trees, iron deposits, and permanent enemy hives.

- Task 3.3: Design background chunk loading/unloading logic using Bevy's async task pool to optimize memory.

## Sprint 4: Inventories, Building Trees, & State Machines

- Task 4.1: Build a global resource inventory tracking counts for raw materials.

- Task 4.2: Construct a transparent structural "ghost layout" following the crosshair inside a dedicated structural state.

- Task 4.3: Leverage Bevy 0.19 BSN scenes to construct modular building layouts (walls, turret variations).

- Task 4.4: Establish a robust state manager decoupling MainMenu, InGame, and GameOver operations.
