use piston_window::{Context, DrawState, Image};
use opengl_graphics::GlGraphics;
use opengl_graphics::Texture as GlTexture;
use crate::entity::{Block, Entity, Lightbulb, Player};
use crate::color::Color;

const ONE_START_MSG: &str = "level must have exactly one starting position";
const LEVEL2: &[u8] = include_bytes!("../bin/levels/level3.skb");
// const LEVEL2: &[u8] = include_bytes!("../bin/levels/playground.skb");
const TILE_SIZE: f64 = 16.;
const WALL: [f64; 4] = [32., 0., TILE_SIZE, TILE_SIZE];
const FLOOR: [f64; 4] = [32., 16., TILE_SIZE, TILE_SIZE];
const EXIT: [f64; 4] = [16., 0., TILE_SIZE, TILE_SIZE];

#[derive(Clone)]
pub enum Tile {
    Floor,
    Wall,
    Exit,
}
use Tile::*;

impl Tile {
    fn from_chr(chr: char) -> Self {
        match chr {
            '#' => Wall,
            'z' => Exit,
            _ => Floor,
        }
    }

    pub fn sprite(&self, x: usize, y: usize) -> Image {
        let src = match self {
            Wall => WALL,
            Floor => FLOOR,
            Exit => EXIT,
        };
        Image::new()
            .src_rect(src)
            .rect([x as f64 * TILE_SIZE, y as f64 * TILE_SIZE, TILE_SIZE, TILE_SIZE])
    }

    pub fn is_passable(&self) -> bool {
        match self {
            Wall => false,
            Floor => true,
            Exit => true,
        }
    }
}

type Game = (Room, Player, Vec<Entity>);

pub struct Room {
    width: usize,
    height: usize,
    tiles: Vec<Tile>,
}

impl Room {
    pub fn new() -> Game {
        Room::from_file(LEVEL2)
    }

    pub fn from_file(bytes: &[u8]) -> Game {
        let width = bytes.iter().position(|&c| c == '\n' as u8).unwrap();
        let tiles: Vec<_> = bytes.iter()
            .filter_map(|&c| {
                let c = c as char;
                (c != '\n').then(|| Tile::from_chr(c as char))
            }).collect();
        let height = tiles.len() / width;

        let mut x = 0;
        let mut y = 0;
        let mut player = None;
        let mut entities = Vec::new();
        for &byte in bytes {
            match byte as char {
                'a' => {
                    if player.is_some() { panic!("{}", ONE_START_MSG); }
                    player = Some(Player::new(x, y));
                },
                'b' => { entities.push(Entity::Block(Block::new(x, y, Color::Gray))); },
                'r' => { entities.push(Entity::Block(Block::new(x, y, Color::Red))); },
                'w' => { entities.push(Entity::Block(Block::new(x, y, Color::White))); },
                'R' => { entities.push(Entity::Lightbulb(Lightbulb::new(x, y, Color::Red))); },
                'G' => { entities.push(Entity::Lightbulb(Lightbulb::new(x, y, Color::Green))); },
                'B' => { entities.push(Entity::Lightbulb(Lightbulb::new(x, y, Color::Blue))); },
                '\n' => {
                    x = 0;
                    y += 1;
                    continue;
                },
                _ => (),
            }
            x += 1;
        }
        (
            Room { width, height, tiles },
            player.expect(ONE_START_MSG),
            entities,
        )
    }

    pub fn render(&self,
                  texture: &GlTexture,
                  draw_state: &DrawState,
                  context: &Context,
                  gl: &mut GlGraphics) {
        for (i, elem) in self.tiles.iter().enumerate() {
            let x = i % self.width;
            let y = i / self.width;
            elem.sprite(x, y)
                .draw(texture, draw_state, context.transform, gl);
        }
    }

    pub fn tile_at(&self, x: i32, y: i32) -> Option<Tile> {
        if x < 0 || y < 0 { return None; }
        let idx = self.width * (y as usize) + x as usize;
        self.tiles.get(idx).cloned()
    }

    pub fn pixel_width(&self) -> i64 {
        self.width as i64 * 16
    }

    pub fn pixel_height(&self) -> i64 {
        self.height as i64 * 16
    }
}
