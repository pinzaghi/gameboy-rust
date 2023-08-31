use sdl2::{VideoSubsystem, video::{Window, WindowPos}, render::Canvas, pixels::Color, rect::Point};

use crate::{gameboy::io::lcd::{SCREEN_WIDTH, ColoredPixel}, screen::Screen};

pub(crate) type TileDataFrame = [[[[ColoredPixel; 8]; 8]; TILEDATA_ROWS]; TILEDATA_COLS];

pub(crate) const TILEDATA_COLS: usize = 16;
pub(crate) const TILEDATA_ROWS: usize = 24;

pub(crate) const TILEDATA_WIDTH: usize = TILEDATA_COLS * 8;
pub(crate) const TILEDATA_HEIGHT: usize = TILEDATA_ROWS * 8;

pub(crate) const BLACK_TDFRAME: TileDataFrame = [[[[ColoredPixel::Black; 8]; 8]; TILEDATA_ROWS]; TILEDATA_COLS];

pub(crate) struct TileDataDebug {
    canvas: Canvas<Window>
}

impl TileDataDebug {
    pub fn new(video: &VideoSubsystem) -> TileDataDebug {
        
        let scale = 2;

        let mut window = video.window("Game Boy", TILEDATA_WIDTH as u32 * scale, TILEDATA_HEIGHT as u32 * scale)
            .resizable()
            .position_centered()
            .build()
            .unwrap();

        let (x, y) = window.position();
        window.set_position(WindowPos::Positioned(x+(SCREEN_WIDTH as i32)*4), WindowPos::Positioned(y));

        let mut canvas = window.into_canvas().build().unwrap();

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.set_scale(scale as f32, scale as f32).unwrap();
        canvas.clear();

        TileDataDebug { canvas }
    }

    pub(crate) fn clear(&mut self) {
        self.canvas.clear();
    }

    pub(crate) fn present(&mut self) {
        self.canvas.present();
    }

    pub(crate) fn render(&mut self, frame: TileDataFrame) {
        for tx in 0..TILEDATA_COLS {
            for ty in 0..TILEDATA_ROWS {
                for px in 0..8{
                    for py in 0..8{
                        let pixel = frame[tx][ty][px][py];
                        self.canvas.set_draw_color(Screen::color(pixel));
                        let point = Point::new((tx*8 + py) as i32, (ty*8 + px) as i32);
                        self.canvas.draw_point(point).unwrap();
                    }
                }
            }
        }
    }
}