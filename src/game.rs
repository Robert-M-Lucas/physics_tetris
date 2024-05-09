mod tetromino;

use std::collections::{HashMap, HashSet, VecDeque};
use rand::distributions::{Distribution, Standard};
use rand::Rng;
use rand::rngs::ThreadRng;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::surface::Surface;
use sdl2::video::Window;
use crate::game::tetromino::Tetromino;

pub struct Toggle {
    toggle: bool
}

impl Toggle {
    pub fn new() -> Self {
        Toggle { toggle: false }
    }

    pub fn toggle(&mut self) -> bool {
        self.toggle = !self.toggle;
        self.toggle
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum ColourCode {
    Red,
    Green,
    Blue,
    Violet,
}

impl ColourCode {
    pub fn to_rgb(&self) -> Color {
        match self {
            ColourCode::Blue => Color::BLUE,
            ColourCode::Red => Color::RED,
            ColourCode::Green => Color::GREEN,
            ColourCode::Violet => Color::RGB(195, 0, 255),
        }
    }
}

impl Distribution<ColourCode> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> ColourCode {
        match rng.gen_range(0..4) {
            0 => ColourCode::Red,
            1 => ColourCode::Green,
            2 => ColourCode::Blue,
            3 => ColourCode::Violet,
            _ => panic!()
        }
    }
}

#[derive(Copy, Clone)]
pub enum ColourType {
    Empty,
    Colour(ColourCode),
    NoPhysicsColour(ColourCode),
    Deleting(ColourCode, u32)
}

impl ColourType {
    pub fn to_rgb(&self, background: Color) -> Color {
        match self {
            ColourType::Empty => background,
            ColourType::Colour(code) | ColourType::NoPhysicsColour(code) => {
                code.to_rgb()
            }
            ColourType::Deleting(code, time_left) => {
                if (time_left / 10) % 2 == 1 {
                    background
                }
                else {
                    code.to_rgb()
                }
            }
        }
    }

    pub fn is_empty(&self) -> bool {
        match self {
            ColourType::Empty => true,
            _ => false
        }
    }

    pub fn is_physics(&self) -> bool {
        match self {
            ColourType::Colour(_) => true,
            _ => false
        }
    }

    pub fn with_physics(&mut self) -> ColourType {
        match self {
            ColourType::NoPhysicsColour(code) | ColourType::Colour(code) => {
                ColourType::Colour(code.clone())
            }
            _ => panic!()
        }
    }

    pub fn set_deleting(&mut self, time_left: u32) {
        *self = match self {
            ColourType::Empty => panic!("Tried to delete empty!"),
            ColourType::Colour(code) | ColourType::NoPhysicsColour(code) | ColourType::Deleting(code, _) => {
                ColourType::Deleting(code.clone(), time_left)
            }
        };
    }
}



pub struct Game<const W: usize, const H: usize> {
    pixel_size: u32,
    screen_size: (u32, u32),
    screen_position: (i32, i32),
    square_pixel_width: u32,
    square_width: u32,
    square_height: u32,
    background: Color,
    board: [[ColourType; H]; W],
    tetromino: Option<Tetromino>,
    time_since_last: u64
}

impl<const W: usize, const H: usize> Game<W, H> {
    pub fn new(pixel_size: u32,
               screen_size: (u32, u32),
               screen_position: (i32, i32),
               square_pixel_width: u32,
               background: Color) -> Self {

        if W % square_pixel_width as usize != 0 || H % square_pixel_width as usize != 0 {
            panic!("Square pixel width must fit into width and height");
        }

        Self {
            pixel_size,
            screen_size,
            screen_position,
            square_pixel_width,
            square_width: W as u32 / square_pixel_width,
            square_height: H as u32 / square_pixel_width,
            background,
            board: [[ColourType::Empty; H]; W],
            tetromino: None,
            time_since_last: 0
        }
    }

    pub fn draw(&self, canvas: &mut Canvas<Window>) {
        if self.pixel_size != 0 {
            let mut pixel_data = Vec::with_capacity((W * self.pixel_size as usize) * (H * self.pixel_size as usize) * 3);


            for y in (0..H).rev() {
                for _ in 0..self.pixel_size {
                    for x in 0..W {
                        let colour = self.board[x][y].to_rgb(self.background).rgb();
                        for _ in 0..self.pixel_size {
                            pixel_data.push(colour.0);
                            pixel_data.push(colour.1);
                            pixel_data.push(colour.2);
                        }
                    }
                }
            }

            let surface = Surface::from_data(&mut pixel_data,
                                             W as u32 * self.pixel_size,
                                             H as u32 * self.pixel_size,
                                             (3 * W) as u32 * self.pixel_size,
                                             sdl2::pixels::PixelFormatEnum::RGB24
            ).unwrap();

            canvas.copy(&surface.as_texture(&canvas.texture_creator()).unwrap(), None, Rect::new(self.screen_position.0, self.screen_position.1, W as u32 * self.pixel_size, H as u32 * self.pixel_size)).unwrap();
        }
        else {
            for x in 0..W {
                for y in 0..H {
                    let colour = self.board[x][y].to_rgb(self.background);
                    canvas.set_draw_color(colour);

                    if self.pixel_size == 1 {
                        canvas.draw_point((self.screen_position.0 + x as i32, self.screen_position.1 + (H - y - 1) as i32)).unwrap();
                    }
                    else {
                        canvas.fill_rect(
                            Rect::new(
                                self.screen_position.0 + (x as u32 * self.pixel_size) as i32, self.screen_position.1 + ((H - y - 1) as u32 * self.pixel_size) as i32,
                                self.pixel_size, self.pixel_size
                            )
                        ).unwrap();
                    }
                }
            }
        }

        canvas.set_draw_color(Color::WHITE);

        for y in -1..(H as i32) {
            for x in [-1, W as i32] {
                if self.pixel_size == 1 {
                    canvas.draw_point((self.screen_position.0 + x, self.screen_position.1 + (H as i32 - y - 1))).unwrap();
                }
                else {
                    canvas.fill_rect(
                        Rect::new(
                            self.screen_position.0 + (x * self.pixel_size as i32), self.screen_position.1 + ((H as i32 - y - 1) * self.pixel_size as i32),
                            self.pixel_size, self.pixel_size
                        )
                    ).unwrap();
                }
            }
        }

        for x in 0..(W as i32) {
            let y = -1;
            if self.pixel_size == 1 {
                canvas.draw_point((self.screen_position.0 + x, self.screen_position.1 + (H as i32 - y - 1))).unwrap();
            }
            else {
                canvas.fill_rect(
                    Rect::new(
                        self.screen_position.0 + (x * self.pixel_size as i32), self.screen_position.1 + ((H as i32 - y - 1) * self.pixel_size as i32),
                        self.pixel_size, self.pixel_size
                    )
                ).unwrap();
            }
        }
    }

    fn ant(&mut self, cell: (usize, usize), target_colour: ColourCode, origins_visited: &mut HashSet<usize>) -> (bool, HashSet<(usize, usize)>) {
        fn check_cell_colour(current: ColourType, target: ColourCode) -> bool {
            match current {
                ColourType::Colour(colour) => colour == target,
                _ => false
            }
        }

        let mut open_set = VecDeque::new();
        open_set.push_back(cell);
        let mut history = HashSet::new();
        history.insert(cell);
        let mut found = false;

        loop {
            let cell = match open_set.pop_front() {
                Some(c) => c,
                None => break
            };

            if cell.0 < W - 1 {
                let cell = (cell.0 + 1, cell.1);

                if check_cell_colour(self.board[cell.0][cell.1], target_colour) && !history.contains(&cell) {
                    history.insert(cell);
                    open_set.push_back(cell);
                    if cell.0 == W - 1 { found = true; }
                    else if cell.0 == 0 { origins_visited.insert(cell.1); }
                }
            }
            if cell.0 > 0 {
                let cell = (cell.0 - 1, cell.1);

                if check_cell_colour(self.board[cell.0][cell.1], target_colour) && !history.contains(&cell) {
                    history.insert(cell);
                    open_set.push_back(cell);
                    if cell.0 == W - 1 { found = true; }
                    else if cell.0 == 0 { origins_visited.insert(cell.1); }
                }
            }
            if cell.1 < H - 1 {
                let cell = (cell.0, cell.1 + 1);

                if check_cell_colour(self.board[cell.0][cell.1], target_colour) && !history.contains(&cell) {
                    history.insert(cell);
                    open_set.push_back(cell);
                    if cell.0 == W - 1 { found = true; }
                    else if cell.0 == 0 { origins_visited.insert(cell.1); }
                }
            }
            if cell.1 > 0 {
                let cell = (cell.0, cell.1 - 1);

                if check_cell_colour(self.board[cell.0][cell.1], target_colour) && !history.contains(&cell) {
                    history.insert(cell);
                    open_set.push_back(cell);
                    if cell.0 == W - 1 { found = true; }
                    else if cell.0 == 0 { origins_visited.insert(cell.1); }
                }
            }
        }

        (found, history)
    }

    // TODO: Collect all movements into one method
    pub fn move_left(&mut self) {
        if let Some(tetromino) = &mut self.tetromino {
            let mut moved: usize = 0;

            'move_loop: for _ in 0..self.square_pixel_width {
                for pos in tetromino.get_all_pixels(self.square_pixel_width) {
                    let pos = (pos.0 as usize, pos.1 as usize);
                    if pos.0 <= moved {
                        break 'move_loop;
                    }

                    match self.board[pos.0 - (moved + 1)][pos.1] {
                        ColourType::Colour(_) | ColourType::Deleting(_, _) => {
                            break 'move_loop;
                        }
                        _ => {}
                    }
                }

                moved += 1;
            }

            if moved > 0 {
                for pos in tetromino.get_all_pixels(self.square_pixel_width) {
                    let pos = (pos.0 as usize, pos.1 as usize);
                    self.board[pos.0 - moved][pos.1] = self.board[pos.0][pos.1];
                    self.board[pos.0][pos.1] = ColourType::Empty;
                }
            }

            tetromino.position = (tetromino.position.0 - moved as i64, tetromino.position.1);
        }
    }

    pub fn move_right(&mut self) {
        if let Some(tetromino) = &mut self.tetromino {
            let mut moved: usize = 0;

            'move_loop: for _ in 0..self.square_pixel_width {
                for pos in tetromino.get_all_pixels(self.square_pixel_width) {
                    let pos = (pos.0 as usize, pos.1 as usize);
                    if pos.0 >= W - moved - 1 {
                        break 'move_loop;
                    }

                    match self.board[pos.0 + (moved + 1)][pos.1] {
                        ColourType::Colour(_) | ColourType::Deleting(_, _) => {
                            break 'move_loop;
                        }
                        _ => {}
                    }
                }

                moved += 1;
            }

            if moved > 0 {
                let mut iter = tetromino.get_all_pixels(self.square_pixel_width);
                let start_pos = iter.next().unwrap();
                let start_pos = (start_pos.0 as usize, start_pos.1 as usize);

                let colour = self.board[start_pos.0][start_pos.1];
                self.board[start_pos.0][start_pos.1] = ColourType::Empty;

                for pos in iter {
                    let pos = (pos.0 as usize, pos.1 as usize);
                    self.board[pos.0][pos.1] = ColourType::Empty;
                }

                for pos in tetromino.get_all_pixels(self.square_pixel_width) {
                    let pos = (pos.0 as usize, pos.1 as usize);
                    self.board[pos.0 + moved][pos.1] = colour;
                }
            }

            tetromino.position = (tetromino.position.0 + moved as i64, tetromino.position.1);
        }
    }

    pub fn move_down(&mut self) {
        self.move_down_amount(self.square_pixel_width);
    }

    pub fn move_down_amount(&mut self, amount: u32) {
        if self.tetromino.is_none() { return; }
        let tetromino = self.tetromino.as_mut().unwrap();

        let mut collided = false;
        let mut moved: usize = 0;

        'move_loop: for _ in 0..amount {
            for pos in tetromino.get_all_pixels(self.square_pixel_width) {
                let pos = (pos.0 as usize, pos.1 as usize);
                if pos.1 <= moved {
                    collided = true;
                    break 'move_loop;
                }

                match self.board[pos.0][pos.1 - (moved + 1)] {
                    ColourType::Colour(_) | ColourType::Deleting(_, _) => {
                        collided = true;
                        break 'move_loop;
                    }
                    _ => {}
                }
            }

            moved += 1;
        }

        if moved > 0 {
            for pos in tetromino.get_all_pixels(self.square_pixel_width) {
                let pos = (pos.0 as usize, pos.1 as usize);
                if !collided {
                    self.board[pos.0][pos.1 - moved] = self.board[pos.0][pos.1];
                }
                else {
                    self.board[pos.0][pos.1 - moved] = self.board[pos.0][pos.1].with_physics();
                }
                self.board[pos.0][pos.1] = ColourType::Empty;
            }
        }
        else if collided {
            for pos in tetromino.get_all_pixels(self.square_pixel_width) {
                let pos = (pos.0 as usize, pos.1 as usize);
                self.board[pos.0][pos.1 - moved] = self.board[pos.0][pos.1].with_physics();
            }
        }

        if collided {
            self.tetromino = None;
        }
        else {
            tetromino.position = (tetromino.position.0, tetromino.position.1 - moved as i64);
        }
    }

    pub fn rotate(&mut self) {
        if self.tetromino.is_none() { return; }
        let tetromino = self.tetromino.as_mut().unwrap();
        let original_rotation = tetromino.rotation;
        let mut modified_rotation = original_rotation + 1;
        if modified_rotation >= 4 {
            modified_rotation = 0;
        }
        tetromino.rotation = modified_rotation;

        let mut cancel = false;
        for pos in tetromino.get_all_pixels(self.square_pixel_width) {
            if pos.0 < 0 || pos.1 < 0 || pos.0 >= W as i64 || pos.1 >= H as i64 { cancel = true; break; }
            let pos = (pos.0 as usize, pos.1 as usize);

            match self.board[pos.0][pos.1] {
                ColourType::Colour(_) | ColourType::Deleting(_, _) => {
                    cancel = true;
                    break;
                }
                _ => {}
            }
        }

        if cancel {
            tetromino.rotation = original_rotation;
            return;
        }

        println!("{}", original_rotation);
        tetromino.rotation = original_rotation;
        let mut iter = tetromino.get_all_pixels(self.square_pixel_width);
        let start_pos = iter.next().unwrap();
        let start_pos = (start_pos.0 as usize, start_pos.1 as usize);

        let colour = self.board[start_pos.0][start_pos.1];
        self.board[start_pos.0][start_pos.1] = ColourType::Empty;

        for pos in iter {
            let pos = (pos.0 as usize, pos.1 as usize);
            self.board[pos.0][pos.1] = ColourType::Empty;
        }

        tetromino.rotation = modified_rotation;

        for pos in tetromino.get_all_pixels(self.square_pixel_width) {
            let pos = (pos.0 as usize, pos.1 as usize);
            self.board[pos.0][pos.1] = colour;
        }
    }

    pub fn game_update(&mut self, rng: &mut ThreadRng, frame_count: u64) {
        if self.tetromino.is_none() && frame_count - self.time_since_last > 30 {
            let tetromino = Tetromino::new(rng.gen(), ((self.square_pixel_width * ((self.square_width / 2) - 2)) as i64, self.square_pixel_width as i64 * 19), 0);
            let colour = ColourType::NoPhysicsColour(rng.gen());
            for pos in tetromino.get_all_pixels(self.square_pixel_width) {
                let pos = (pos.0 as usize, pos.1 as usize);
                self.board[pos.0][pos.1] = colour;
            }
            self.tetromino = Some(tetromino);
            self.time_since_last = frame_count
        }
        else if self.tetromino.is_some() {
            if (frame_count - self.time_since_last) % 30 == 29 {
                self.move_down();
                if self.tetromino.is_none() {
                    self.time_since_last = frame_count;
                }
            }
        }

        let mut origins_visited: HashSet<usize> = HashSet::new();
        for y in 0..H {
            let colour_code = match self.board[0][y] {
                ColourType::Colour(code) => code,
                _ => continue,
            };
            if origins_visited.contains(&y) { continue; }

            let (found, visited) = self.ant((0, y), colour_code, &mut origins_visited);

            if found {
                for pos in visited {
                    self.board[pos.0][pos.1].set_deleting(40);
                }
            }
        }
    }

    fn move_pixel(&mut self, from: (usize, usize), to: (usize, usize)) {
        self.board[to.0][to.1] = self.board[from.0][from.1];
        self.board[from.0][from.1] = ColourType::Empty;
    }

    pub fn physics_update(&mut self) {
        let mut toggle = Toggle::new();

        let mut game_over = false;

        for y in 0..H {
            for x in 0..W {
                match self.board[x][y] {
                    ColourType::Deleting(_, time_left) => {
                        if time_left > 0 {
                            self.board[x][y].set_deleting(time_left - 1)
                        }
                        else {
                            self.board[x][y] = ColourType::Empty;
                        }
                    },
                    ColourType::Colour(_) => {
                        if y >= (19 * self.square_pixel_width) as usize {
                            game_over = true;
                        }
                    }
                    _ => {}
                }
            }

            if game_over {
                for y in 0..H {
                    for x in 0..W {
                        match self.board[x][y] {
                            ColourType::Colour(_) => {
                                if y >= (18 * self.square_pixel_width) as usize {
                                    self.board[x][y] = ColourType::Empty;
                                }
                                else {
                                    self.board[x][y].set_deleting((H - y) as u32 / 10);
                                }
                            }
                            _ => {}
                        }
                    }
                }
                return;
            }

            if y == 0 { continue; }


            if y < H - 1 {
                for x in 0..W {
                    match self.board[x][y] {
                        ColourType::Colour(_) => {
                            if toggle.toggle() {
                                if x > 0 && x < W - 1 && self.board[x+1][y-1].is_physics() && self.board[x][y+1].is_physics() && self.board[x-1][y-1].is_empty() {
                                    self.move_pixel((x, y), (x - 1, y - 1));
                                }
                                else if x > 0 && x < W - 1 && self.board[x-1][y-1].is_physics() && self.board[x][y+1].is_physics() && self.board[x+1][y-1].is_empty() {
                                    self.move_pixel((x, y), (x + 1, y - 1));
                                }
                            }
                            else {
                                if x > 0 && x < W - 1 && self.board[x-1][y-1].is_physics() && self.board[x][y+1].is_physics() && self.board[x+1][y-1].is_empty() {
                                    self.move_pixel((x, y), (x + 1, y - 1));
                                }
                                else if x > 0 && x < W - 1 && self.board[x+1][y-1].is_physics() && self.board[x][y+1].is_physics() && self.board[x-1][y-1].is_empty() {
                                    self.move_pixel((x, y), (x - 1, y - 1));
                                }
                            }
                        }
                        _ => {}
                    }
                }
            }


            for x in 0..W {
                match self.board[x][y] {
                    ColourType::Colour(_) => {
                        if self.board[x][y-1].is_empty() {
                            self.move_pixel((x, y), (x, y - 1));
                        }
                    }
                    _ => {}
                }
            }

            for x in 0..W {
                match self.board[x][y] {
                    ColourType::Colour(_) => {
                        if toggle.toggle() {
                            if x > 0 && self.board[x-1][y-1].is_empty() {
                                self.move_pixel((x, y), (x - 1, y - 1));
                            }
                            else if x < W - 1 && self.board[x+1][y-1].is_empty() {
                                self.move_pixel((x, y), (x + 1, y - 1));
                            }
                        }
                        else {
                            if x < W - 1 && self.board[x+1][y-1].is_empty() {
                                self.move_pixel((x, y), (x + 1, y - 1));
                            }
                            else if x > 0 && self.board[x-1][y-1].is_empty() {
                                self.move_pixel((x, y), (x - 1, y - 1));
                            }
                        }
                    }
                    _ => {}
                }
            }
        }
    }
}