use rand::distributions::{Distribution, Standard};
use rand::Rng;

#[derive(Debug)]
pub enum TetrominoShape {
    LShape,
    ReverseLShape,
    ZShape,
    ReverseZShape,
    Line,
    Square,
}

impl Distribution<TetrominoShape> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> TetrominoShape {
        match rng.gen_range(0..6) {
            0 => TetrominoShape::LShape,
            1 => TetrominoShape::ReverseLShape,
            2 => TetrominoShape::ZShape,
            3 => TetrominoShape::ReverseZShape,
            4 => TetrominoShape::Line,
            _ => TetrominoShape::Square,
        }
    }
}

pub struct Tetromino {
    shape: TetrominoShape,
    pub position: (i64, i64),
    pub rotation: u8,
}

impl Tetromino {
    pub fn new(shape: TetrominoShape,
               position: (i64, i64),
               rotation: u8) -> Self {
        Self {
            shape,
            position,
            rotation
        }
    }

    pub fn get_shape(&self) -> [[bool; 4]; 4] {
        match self.shape {
            TetrominoShape::LShape =>
                match self.rotation {
                    0 => [
                        [false, false, false, false],
                        [false, true , false, false],
                        [false, true , false, false],
                        [false, true , true , false],
                    ],
                    1 => [
                        [false, false, false, false],
                        [false, false, false, false],
                        [false, true , true , true ],
                        [false, true , false, false],
                    ],
                    2 => [
                        [false, false, false, false],
                        [false, true , true , false],
                        [false, false, true , false],
                        [false, false, true , false],
                    ],
                    _ => [
                        [false, false, false, false],
                        [false, false, false, false],
                        [false, false, true , false],
                        [true , true , true , false],
                    ],
                }

            TetrominoShape::ReverseLShape => match self.rotation {
                0 => [
                    [false, false, false, false],
                    [false, false, true , false],
                    [false, false, true , false],
                    [false, true , true , false],
                ],
                1 => [
                    [false,  false, false, false],
                    [false,  false, false, false],
                    [false,  true , false, false],
                    [false,  true , true , true ],
                ],
                2 => [
                    [false, false, false, false],
                    [false, true , true , false],
                    [false, true , false, false],
                    [false, true , false, false],
                ],
                _ => [
                    [false, false, false, false],
                    [false, false, false, false],
                    [true , false, false, false],
                    [true , true , true , false],
                ],
            }
            TetrominoShape::ZShape => match self.rotation {
                0 | 2 =>
                    [
                        [false, false, false, false],
                        [false, false, true , false],
                        [false, true , true , false],
                        [false, true , false, false],
                    ],
                _ =>
                    [
                        [false, false, false, false],
                        [false, false, false, false],
                        [false, true , true , false],
                        [false, false, true , true ],
                    ],
            }

            TetrominoShape::ReverseZShape => match self.rotation {
                0 | 2 =>
                    [
                        [false, false, false, false],
                        [false, true , false, false],
                        [false, true , true , false],
                        [false, false, true , false],
                    ],
                _ =>
                    [
                        [false, false, false, false],
                        [false, false, false, false],
                        [false, false, true , true ],
                        [false, true , true , false],
                    ],
            }
            TetrominoShape::Line => match self.rotation {
                0 | 2 =>
                    [
                        [false, true , false, false],
                        [false, true , false, false],
                        [false, true , false, false],
                        [false, true , false, false],
                    ],
                _ =>
                    [
                        [false, false, false, false],
                        [false, false, false, false],
                        [true , true , true , true ],
                        [false, false, false, false],
                    ],
            }
            TetrominoShape::Square => [
                [false, false, false, false],
                [false, true , true , false],
                [false, true , true , false],
                [false, false, false, false],
            ],
        }
    }
    
    pub fn get_all_pixels(&self, square_pixel_width: u32) -> PixelIterator {
        let shape = self.get_shape();

        let mut i = PixelIterator {
            shape,
            square_pixel_width: square_pixel_width as usize,
            position_x: self.position.0,
            position_y: self.position.1,
            shape_x: 0,
            shape_y: 0,
            pixel_x: 0,
            pixel_y: 0,
            is_next: true,
        };
        i.skip_until_valid(false);
        i
    }
}

#[derive(Debug)]
pub struct PixelIterator {
    shape: [[bool; 4]; 4],
    square_pixel_width: usize,
    position_x: i64,
    position_y: i64,
    shape_x: usize,
    shape_y: usize,
    pixel_x: usize,
    pixel_y: usize,
    is_next: bool,
}

impl PixelIterator {
    pub fn skip_until_valid(&mut self, mut guarantee_first: bool) {
        while guarantee_first || !self.shape[self.shape_y][self.shape_x] {
            guarantee_first = false;
            self.shape_y += 1;

            if self.shape_y >= 4 {
                self.shape_y = 0;
                self.shape_x += 1;

                if self.shape_x == 4 {
                    self.is_next = false;
                    return;
                }
            }
        }
    }
}

impl Iterator for PixelIterator {
    type Item = (i64, i64);

    fn next(&mut self) -> Option<(i64, i64)> {
        if !self.is_next { return None; }

        let ret = Some(
            (
                (self.position_x + (self.shape_x * self.square_pixel_width) as i64 + self.pixel_x as i64),
                (self.position_y + (self.shape_y * self.square_pixel_width) as i64 + self.pixel_y as i64)
            )
        );

        self.pixel_y += 1;
        if self.pixel_y == self.square_pixel_width {
            self.pixel_y = 0;
            self.pixel_x += 1;

            if self.pixel_x == self.square_pixel_width {
                self.pixel_x = 0;

                self.skip_until_valid(true);
            }
        }

        if ret.unwrap().0 > 1_000_000 {
            println!("{:?}", self);
        }

        ret
    }
}