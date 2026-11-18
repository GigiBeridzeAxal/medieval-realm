//! Tile map: pure data + logic, no rendering. Fully unit-tested.
//!
//! Maps are plain ASCII text files in `assets/maps/` so designers can edit
//! them in any text editor. See [`TileKind::from_char`] for the legend.

use bevy::math::Vec2;

/// Size of one tile in world units (pixels at zoom 1.0).
pub const TILE_SIZE: f32 = 32.0;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TileKind {
    Grass,
    Path,
    Water,
    Bridge,
    CastleWall,
    CastleFloor,
    Gate,
    Tree,
    House,
}

impl TileKind {
    /// Map legend. Spawn markers (P, V, $) sit on grass.
    pub fn from_char(c: char) -> Option<Self> {
        Some(match c {
            '.' | 'P' | 'V' | '$' => TileKind::Grass,
            ',' => TileKind::Path,
            '~' => TileKind::Water,
            '=' => TileKind::Bridge,
            '#' => TileKind::CastleWall,
            '_' => TileKind::CastleFloor,
            'G' => TileKind::Gate,
            'T' => TileKind::Tree,
            'H' => TileKind::House,
            _ => return None,
        })
    }

    /// Can characters walk on this tile?
    pub fn is_walkable(self) -> bool {
        !matches!(
            self,
            TileKind::Water | TileKind::CastleWall | TileKind::Tree | TileKind::House
        )
    }
}

/// Something to spawn on top of the terrain, read from the map file.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpawnMarker {
    Player,
    Villager,
    Coin,
}

#[derive(Debug, PartialEq, Eq)]
pub enum MapError {
    Empty,
    RaggedRow {
        row: usize,
        expected: usize,
        found: usize,
    },
    UnknownTile {
        row: usize,
        col: usize,
        ch: char,
    },
    PlayerCount(usize),
}

impl std::fmt::Display for MapError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MapError::Empty => write!(f, "map is empty"),
            MapError::RaggedRow {
                row,
                expected,
                found,
            } => {
                write!(f, "row {row} has {found} tiles, expected {expected}")
            }
            MapError::UnknownTile { row, col, ch } => {
                write!(f, "unknown tile '{ch}' at row {row}, col {col}")
            }
            MapError::PlayerCount(n) => {
                write!(f, "map needs exactly 1 player spawn 'P', found {n}")
            }
        }
    }
}

impl std::error::Error for MapError {}

#[derive(Debug, Clone)]
pub struct TileMap {
    width: usize,
    height: usize,
    tiles: Vec<TileKind>,
    pub spawns: Vec<(SpawnMarker, usize, usize)>,
}

impl TileMap {
    pub fn parse(text: &str) -> Result<Self, MapError> {
        let rows: Vec<&str> = text
            .lines()
            .map(str::trim_end)
            .filter(|l| !l.is_empty())
            .collect();
        let width = rows.first().ok_or(MapError::Empty)?.chars().count();
        let mut tiles = Vec::with_capacity(width * rows.len());
        let mut spawns = Vec::new();

        for (row, line) in rows.iter().enumerate() {
            let found = line.chars().count();
            if found != width {
                return Err(MapError::RaggedRow {
                    row,
                    expected: width,
                    found,
                });
            }
            for (col, ch) in line.chars().enumerate() {
                let kind = TileKind::from_char(ch).ok_or(MapError::UnknownTile { row, col, ch })?;
                tiles.push(kind);
                let marker = match ch {
                    'P' => Some(SpawnMarker::Player),
                    'V' => Some(SpawnMarker::Villager),
                    '$' => Some(SpawnMarker::Coin),
                    _ => None,
                };
                if let Some(m) = marker {
                    spawns.push((m, col, row));
                }
            }
        }

        let players = spawns.iter().filter(|s| s.0 == SpawnMarker::Player).count();
        if players != 1 {
            return Err(MapError::PlayerCount(players));
        }

        Ok(Self {
            width,
            height: rows.len(),
            tiles,
            spawns,
        })
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    /// Tile at (col, row). Out of bounds returns `None`.
    pub fn get(&self, col: i32, row: i32) -> Option<TileKind> {
        if col < 0 || row < 0 || col as usize >= self.width || row as usize >= self.height {
            return None;
        }
        Some(self.tiles[row as usize * self.width + col as usize])
    }

    /// Out-of-bounds tiles count as blocked, so nobody can leave the map.
    pub fn is_walkable(&self, col: i32, row: i32) -> bool {
        self.get(col, row).is_some_and(TileKind::is_walkable)
    }

    pub fn player_spawn(&self) -> (usize, usize) {
        self.spawns
            .iter()
            .find(|s| s.0 == SpawnMarker::Player)
            .map(|s| (s.1, s.2))
            .expect("validated in parse")
    }

    pub fn iter(&self) -> impl Iterator<Item = (usize, usize, TileKind)> + '_ {
        self.tiles
            .iter()
            .enumerate()
            .map(|(i, &k)| (i % self.width, i / self.width, k))
    }

    /// World-space bounds (min, max) covering every tile edge to edge.
    pub fn world_bounds(&self) -> (Vec2, Vec2) {
        let half = TILE_SIZE / 2.0;
        let min = Vec2::new(-half, -(self.height as f32 - 0.5) * TILE_SIZE);
        let max = Vec2::new((self.width as f32 - 0.5) * TILE_SIZE, half);
        (min, max)
    }
}

/// Centre of tile (col, row) in world space. Row 0 is the top of the map.
pub fn tile_to_world(col: usize, row: usize) -> Vec2 {
    Vec2::new(col as f32 * TILE_SIZE, -(row as f32) * TILE_SIZE)
}

/// Which tile contains this world position.
pub fn world_to_tile(pos: Vec2) -> (i32, i32) {
    (
        (pos.x / TILE_SIZE).round() as i32,
        (-pos.y / TILE_SIZE).round() as i32,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    const SMALL: &str = "TTTT\nT.PT\nT~$T\nTTTT\n";

    #[test]
    fn parses_small_map() {
        let map = TileMap::parse(SMALL).unwrap();
        assert_eq!((map.width(), map.height()), (4, 4));
        assert_eq!(map.get(1, 2), Some(TileKind::Water));
        assert_eq!(map.player_spawn(), (2, 1));
        assert!(map.spawns.contains(&(SpawnMarker::Coin, 2, 2)));
    }

    #[test]
    fn walkability() {
        let map = TileMap::parse(SMALL).unwrap();
        assert!(map.is_walkable(1, 1)); // grass
        assert!(map.is_walkable(2, 1)); // player spawn sits on grass
        assert!(!map.is_walkable(1, 2)); // water
        assert!(!map.is_walkable(0, 0)); // tree
        assert!(!map.is_walkable(-1, 0)); // off map
        assert!(!map.is_walkable(4, 1)); // off map
    }

    #[test]
    fn rejects_ragged_rows() {
        let err = TileMap::parse("TTT\nT.P\nTT\n").unwrap_err();
        assert_eq!(
            err,
            MapError::RaggedRow {
                row: 2,
                expected: 3,
                found: 2
            }
        );
    }

    #[test]
    fn rejects_unknown_tiles() {
        let err = TileMap::parse("TP\nTX\n").unwrap_err();
        assert_eq!(
            err,
            MapError::UnknownTile {
                row: 1,
                col: 1,
                ch: 'X'
            }
        );
    }

    #[test]
    fn requires_exactly_one_player() {
        assert_eq!(
            TileMap::parse("..\n..").unwrap_err(),
            MapError::PlayerCount(0)
        );
        assert_eq!(
            TileMap::parse("PP\n..").unwrap_err(),
            MapError::PlayerCount(2)
        );
        assert_eq!(TileMap::parse("").unwrap_err(), MapError::Empty);
    }

    #[test]
    fn tile_world_round_trip() {
        for (c, r) in [(0, 0), (3, 7), (39, 19)] {
            let p = tile_to_world(c, r);
            assert_eq!(world_to_tile(p), (c as i32, r as i32));
            // Anywhere inside the tile maps back to it.
            assert_eq!(
                world_to_tile(p + Vec2::splat(TILE_SIZE * 0.49)),
                (c as i32, r as i32)
            );
        }
    }

    #[test]
    fn shipped_village_map_is_valid() {
        let map = TileMap::parse(include_str!("../assets/maps/village.txt")).unwrap();
        let (c, r) = map.player_spawn();
        assert!(map.is_walkable(c as i32, r as i32));
        // Every spawn must be on a walkable tile.
        for &(m, c, r) in &map.spawns {
            assert!(
                map.is_walkable(c as i32, r as i32),
                "{m:?} at ({c},{r}) is blocked"
            );
        }
        // The border must be solid so nothing escapes.
        for c in 0..map.width() as i32 {
            assert!(!map.is_walkable(c, 0));
            assert!(!map.is_walkable(c, map.height() as i32 - 1));
        }
    }
}
