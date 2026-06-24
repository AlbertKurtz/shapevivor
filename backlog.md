# Backlog

## Sprint 1: Core Combat & Kinematics Framework (MVP)

- **Task 1.1: Plugin Decoupling Architecture**
  Refactor existing initialization and loop logic into an independent, decoupled `GamePlugin` spanning modular files (`player.rs`, `enemy.rs`, `ui.rs`).
- **Task 1.2: Shared Component Convergence**
  Unify data layout models (`Health`, `Speed`, `RadiusCollider`) across distinct player, obstacle, and enemy entities to enforce strict composition over inheritance.
- **Task 1.3: Linear Projectile Spawning**
  Implement a timer-driven automated weapon module that periodically targets the closest entity and fires linear `Projectile` objects.
- **Task 1.4: Squared-Distance Intersection System**
  Develop a cache-efficient instant-hit system using `.distance_squared()` to handle damage allocation and safe projectile deletion.
- **Task 1.5: Static Collision Resolution (Wall-Sliding)**
  Write custom kinematic math to intercept player movement against static entities. Push the player transform out of intersection bounds along the tangent vector to allow seamless corner-sliding without stickiness.
- **Task 1.6: Dynamic XP Harvest Loop**
  Design an automated drop table system spawning localized `XP` or resource tokens upon enemy destruction. Equip the player with a `PickupRadius` component that dynamically attracts these tokens when nearby.
- **Task 1.7: Monitor-Anchored HUD Layout**
  Migrate loose world-space print debugging strings into absolute screen-space UI nodes (`Node` layouts in Bevy 0.15+) displaying live player health and resource metrics.

## Sprint 2: The Two-Tier Spatial Partitioning Grid (Scale & Density)

- **Task 2.1: Dual-Resolution Spatial Grid Resource**
  Create a global `SpatialGrid` resource utilizing nested integer hash maps tracking two specific layers: Macro Cells (~10,000px bounds for LOD tracking) and Micro Cells (~200px bounds for local math tracking).
- **Task 2.2: Parallelized Grid Mapping**
  Write a high-performance system executing every frame to convert continuous floating-point player/enemy `Transforms` to integer cell hashes, registering them dynamically inside buckets.
- **Task 2.3: Optimized Multi-Entity Push Resolution**
  Leverage Micro Grid bucket references to restrict enemy-on-enemy collision checks exclusively to immediate neighbors. Apply bidirectional penetration vector shifting to handle realistic crowd crowding/pushing without full engine physics overhead.
- **Task 2.4: Grid-Driven Balanced Spawning & Density Verification**
  Utilize the grid cells as a heatmap matrix. Prior to executing spawning routines, scan local cell density indexes to evenly distribute spawn points, preventing artificial enemy/item overlapping and clustering spikes.

## Sprint 3: Continuous Infinite Procedural World (The "Ark" Generation)

- **Task 3.1: Chunk Life Cycle Manager**
  Establish continuous chunk allocation algorithms linked directly to Macro Grid threshold crossings as the player camera wanders through coordinates.
- **Task 3.2: Noise-Mapped Resource Generation**
  Incorporate coherent procedural pseudo-random noise fields to scatter harvestable nodes (trees, ore) and permanent enemy structures based on density parameters.
- **Task 3.3: Async Visual Despawning & Preservation**
  Utilize Bevy's background task pools to unload heavy rendering components (sprites, materials) of off-screen entities to clean GPU memory, while preserving raw spatial data arrays in RAM for persistence.

## Sprint 4: Inventories, Building Trees, & State Persistence

- **Task 4.1: Material Inventory Controller**
  Construct a structural resource ledger recording individual raw material storage values (wood, stone, iron).
- **Task 4.2: Placement Projection System**
  Create a building mode state projecting a semi-transparent structural "ghost blueprint" mapped directly to mouse cursor coordinates.
- **Task 4.3: Bevy 0.19 BSN Structure Hierarchies**
  Build modular structure profiles (walls, variations of autoturrets) using the new declarative `bsn!` scene format to allow clean composability and patchable stat progression.
- **Task 4.4: Simulation Serialization System (Save/Load)**
  Implement runtime persistence by leveraging Bevy 0.19's updated scene serialization mechanisms or the `bevy_save` framework to extract active grid array structures to a compressed binary disk file.
- **Task 4.5: Global State Machine Separation**
  Establish strict boundaries using a Bevy state driver enum separating `MainMenu`, `InGame`, `BuildingMode`, and `GameOver` operations to safely pause runtime systems.
