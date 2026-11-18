# Medieval Realm (Rust + Bevy 0.20, 2D)

A 2D top-down medieval game starter. A knight explores a village, crosses the river
and collects gold. Villagers wander around, and the camera follows the knight.

## Run

```bash
cargo run            # debug build (dependencies are optimized, so it runs smoothly)
cargo run --release  # shipping build
cargo test           # 20 tests, no window or GPU needed
```

Linux needs: `sudo apt install pkg-config libudev-dev libasound2-dev libwayland-dev libxkbcommon-dev`.
The first build takes a few minutes. Later builds take seconds.

**Controls:** WASD / arrow keys to move.

## Project layout

| File | What it does | Tested |
|---|---|---|
| `assets/maps/village.txt` | The level as ASCII. Designers edit this file | ✅ validated by a test |
| `src/map.rs` | Parses maps, tile types, walkability, tile and world coordinates | ✅ |
| `src/movement.rs` | Box-vs-tile collision with wall sliding and no tunnelling | ✅ |
| `src/rng.rs` | Deterministic RNG, so bugs can be reproduced | ✅ |
| `src/world.rs` | Spawns terrain, props, the knight, villagers and coins | |
| `src/player.rs` | Knight input and movement | |
| `src/villager.rs` | Villager wandering AI | ✅ 60-second headless simulation |
| `src/treasure.rs` | Coin pickup and gold | ✅ headless ECS test |
| `src/camera.rs` | Smooth follow camera, clamped to the map | ✅ |
| `src/hud.rs` | Gold counter and help text | ✅ |
| `tests/gameplay.rs` | Runs real game systems with no window | |

Each feature is a Bevy **plugin**, so you add a new feature as a new file and register it in `src/lib.rs`.

## Map legend

| Char | Tile | Walkable |
|---|---|---|
| `.` | Grass | yes |
| `,` | Dirt path | yes |
| `~` | Water | no |
| `=` | Bridge | yes |
| `#` | Castle wall | no |
| `_` | Castle floor | yes |
| `G` | Castle gate | yes |
| `T` | Tree | no |
| `H` | House | no |
| `P` | Player spawn (exactly one) | yes |
| `V` | Villager spawn | yes |
| `$` | Gold coin | yes |

## Art

All graphics are placeholder coloured rectangles, so the project runs with no asset files.
To add real pixel art, put PNGs in `assets/` and replace `Sprite::from_color(...)` with
`Sprite::from_image(asset_server.load("knight.png"))` in `src/world.rs`.
Nearest-neighbour filtering is already enabled, so pixel art stays crisp.

## Next steps (suggested)
1. Sprite sheets and walk animations (knight, villagers)
2. Combat: sword swing, health, enemies (bandits, wolves)
3. Dialogue with villagers
4. Inventory and shop
5. Multiple maps and transitions (village, castle interior, forest)
6. Save and load
7. Sound and music (enable Bevy's `audio` feature)
