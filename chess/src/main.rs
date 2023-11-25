use std::path::Path;
use sdl2::event::Event;
use sdl2::pixels::Color;
use sdl2::render::{Texture, TextureCreator, WindowCanvas};
use sdl2::EventPump;
use sdl2::rect::Rect;
use sdl2::surface::Surface;
use sdl2::video::WindowContext;

const WINDOW_SIZE: u32 = 800;
const BOARD_SIZE: i32 = 8;
const TILE_SIZE: i32 = 100;
const PIECE_SIZE: i32 = 90;
const PIECE_OFFSET: i32 = 5;

#[derive(PartialEq)]
enum PieceType {
    Pawn,
    Queen,
    Rook,
    Knight,
    Bishop,
    King
}

#[derive(PartialEq)]
enum PieceColor { Black, White }

struct Pawn<'a> {
    texture: Texture<'a>,
    rect: Rect,
    row: i32,
    col: i32,
    is_selected: bool,
    has_moved: bool,
    color: PieceColor,
    piece_type: PieceType,
}

impl Pawn<'_> {
    fn new(texture_creator: &TextureCreator<WindowContext>, is_white: bool, row: i32, col: i32) -> Self {
        let surface_path = if is_white { "assets/white_pawn.bmp" } else { "assets/black_pawn.bmp" };
        let surface = Surface::load_bmp(Path::new(surface_path)).expect("Failed to create surface");

        let texture = texture_creator.create_texture_from_surface(surface).expect("Failed to create texture");
        let rect = Rect::new(col * TILE_SIZE, row * TILE_SIZE, PIECE_SIZE as u32, PIECE_SIZE as u32);

        let color = if is_white { PieceColor::White } else { PieceColor::Black };

        Pawn {
            texture,
            rect,
            row,
            col,
            is_selected: false,
            has_moved: false,
            color,
            piece_type: PieceType::Pawn,
        }
    }
}

#[derive(PartialEq)]
enum TileColor { Black, White }

struct Tile {
    rect: Rect,
    tile_color: TileColor,
    is_occupied: bool,
    is_highlighted: bool
}

struct Board<'a> {
    tiles: Vec<Vec<Tile>>,
    pawns: Vec<Pawn<'a>>
}

impl Board<'_> {
    fn new(texture_creator: &TextureCreator<WindowContext>) -> Self {
        let mut tiles: Vec<Vec<Tile>> = vec!();
        for row in 0..BOARD_SIZE {
            let mut tile_row: Vec<Tile> = vec!();
            for col in 0..BOARD_SIZE {
                let tile: Tile = Tile {
                    rect: Rect::new(col * TILE_SIZE, row * TILE_SIZE, TILE_SIZE as u32, TILE_SIZE as u32),
                    tile_color: if (row + col) % 2 == 0 { TileColor::White } else { TileColor::Black },
                    is_occupied: false,
                    is_highlighted: false
                };

                tile_row.push(tile);
            }
            tiles.push(tile_row);
        }

        let mut pawns: Vec<Pawn> = vec!();
        for col in 0..BOARD_SIZE {
            pawns.push(Pawn::new(texture_creator, false, 7, col))
        }
        
        Board {
            tiles,
            pawns
        }
    }

    fn render(&mut self, canvas: &mut WindowCanvas) {
        for row in 0..BOARD_SIZE as usize {
            for col in 0..BOARD_SIZE as usize {
                if !self.tiles[row][col].is_highlighted {
                    if self.tiles[row][col].tile_color == TileColor::White {
                        canvas.set_draw_color(Color::RGB(234, 221, 202));
                    } else {
                        canvas.set_draw_color(Color::RGB(111, 78, 55));
                    }
                } else {
                    if self.tiles[row][col].tile_color == TileColor::White {
                        canvas.set_draw_color(Color::RGB(137, 196, 244));
                    } else {
                        canvas.set_draw_color(Color::RGB(112, 169, 215));
                    }
                }
                canvas.fill_rect(self.tiles[row][col].rect).expect("Failed to fill rect");
            }
        }
    }

    fn handle_click(&mut self, mouse_x: i32, mouse_y: i32) {
        let row = (mouse_x / TILE_SIZE) as usize;
        let col = (mouse_y / TILE_SIZE) as usize;

        if self.valid_move(row, col) {
            if !self.tiles[row][col].is_highlighted {
                self.clear_selections();
                self.clear_highlights();
                if self.tiles[row][col].is_occupied {
                    // TODO: FINISH THIS
                }
            }
        }
    }

    fn valid_move(&mut self, row: usize, col: usize) -> bool {
        row < BOARD_SIZE as usize && col < BOARD_SIZE as usize
    }

    fn clear_selections(&mut self) {

    }

    fn clear_highlights(&mut self) {
        for row in self.tiles.iter_mut() {
            for tile in row.iter_mut() {
                tile.is_highlighted = false;
            }
        }
    }
}

struct WindowManager {
    canvas: WindowCanvas,
    texture_creator: TextureCreator<WindowContext>,
}

impl WindowManager {
    fn new(width: u32, height: u32, sdl_context: &sdl2::Sdl) -> Self {
        let video = sdl_context.video().expect("Failed to create video");
        let window = video.window("Chess", width, height)
            .position_centered()
            .allow_highdpi()
            .build()
            .expect("Failed to create window");

        let canvas = window.into_canvas()
            .accelerated()
            .present_vsync()
            .build()
            .expect("Failed to create canvas");

        let texture_creator = canvas.texture_creator();

        WindowManager {
            canvas,
            texture_creator,
        }
    }
}


struct Game<'a> {
    window_manager: WindowManager,
    board: Board<'a>,
    is_running: bool,
    events: EventPump,
}

impl Game<'_> {
    pub fn new() -> Self {
        let sdl = sdl2::init().expect("Failed to initialize SDL");
        let window_manager = WindowManager::new(800, 800, &sdl);
        let board = Board::new(&window_manager.texture_creator);
        let events = sdl.event_pump().expect("Failed to create event pump");

        Game {
            window_manager,
            board,
            is_running: false,
            events,
        }
    }

    pub fn run(&mut self) {
        self.is_running = true;
        while self.is_running {
            self.process_events();
            self.render();

            // Cap the fps at 60
            std::thread::sleep(std::time::Duration::from_millis(16));
        }
    }

    fn process_events(&mut self) {
        for event in self.events.poll_iter() {
            match event {
                Event::Quit { .. } => self.is_running = false,
                _ => {}
            }
        }
    }

    fn render(&mut self) {
        let mut canvas = &mut self.window_manager.canvas;

        canvas.set_draw_color(Color::BLACK);
        canvas.clear();

        self.board.render(&mut canvas);

        canvas.present();
    }
}

fn main() {
    let mut game = Game::new();
    game.run();
}
