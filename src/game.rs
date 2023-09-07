#[derive(Clone, PartialEq)]
pub enum BMember {
    Snake,
    Empty,
}
#[derive(Clone, Copy)]
pub enum Dir {
    Up,
    Down,
    Left,
    Right
}
pub enum Scrn {
    Game
}
struct Instruction {
    dir: Dir,
    x: i64,
    y: i64
}
pub struct StartPos {
    pub snake_x: i64,
    pub snake_y: i64,
    pub apple_x: i64,
    pub apple_y: i64
}
pub struct Board {
    pub w: i64,
    pub h: i64,
    pub b: Vec<Vec<BMember>>
}
pub struct Snake {
    pub x: i64,
    pub y: i64,
    pub l: u16,
    pub d: Dir,
    pub tx: i64,
    pub ty: i64,
    pub td: Dir,
    tailinst: Vec<Instruction>
}
pub struct Apple {
    pub x: i64,
    pub y: i64
}
pub struct Game {
    pub s: Snake,
    pub b: Board,
    pub a: Apple,
    pub scrn: Scrn,
    pub lost: bool,
    pub steptime: u16,
    pub seed: f64,
    pub apple_ct: u32
}

fn prng(seed: f64, inc: f64) -> f32 {
    let res = (inc * seed)
        .sqrt()
        .powf(3.141592653589793);
    (res - res.floor()) as f32
}

impl Game {
    pub fn new(w: i64, h: i64, start_pos: &StartPos, seed: f64) -> Self {
        let s = Snake {
            x: start_pos.snake_x,
            y: start_pos.snake_y,
            l: 3,
            d: Dir::Right,
            td: Dir::Right,
            tailinst: vec![],
            tx: start_pos.snake_x,
            ty: start_pos.snake_y
        };
        let a = Apple {
            x: start_pos.apple_x,
            y: start_pos.apple_y
        };
        let b = Board {
            w,
            h,
            b: vec![vec![BMember::Empty; h as usize]; w as usize]
        };

        Game {
            s,
            b,
            a,
            scrn: Scrn::Game,
            lost: false,
            steptime: 100,
            seed,
            apple_ct: 0
        }
    }
    fn oob(s: &mut Snake, b: &Board) -> bool {
        if s.x >= b.w {
            s.x = b.w - 1;
            return true
        }
        if s.x < 0 {
            s.x = 0;
            return true
        }
        if s.y >= b.h {
            s.y = b.h - 1;
            return true
        }
        if s.y < 0 {
            s.y = 0;
            return true
        }
        false
    }
    fn eat_apple(self: &mut Self) {
        self.s.l = 3;

        let mut ax = (prng(self.seed, self.apple_ct as f64) * self.b.w as f32) as i64;
        let mut ay = (prng(self.seed, self.apple_ct as f64 + 0.5) * self.b.h as f32) as i64;
        for _ in 0..self.b.w { for _ in 0..self.b.h {
            if ax != self.a.x && ay != self.a.y {
                if self.b.b[ax as usize][ay as usize] == BMember::Empty {
                    self.a.x = ax;
                    self.a.y = ay;
                    return
                }
            }
            ax += 1;
            if ax >= self.b.w {
                ax = 0;
                ay += 1;
            }
            if ay >= self.b.h {
                ay = 0;
            }
        }}
        self.lost = true
    }
    pub fn step(self: &mut Self, i: Option<Dir>) {
        if self.lost { return }
        self.b.b[self.s.x as usize][self.s.y as usize] = BMember::Snake;
        match i {
            Some(dir) => {
                self.s.d = dir;
                self.s.tailinst.push(Instruction { dir, x: self.s.x, y: self.s.y });
            },
            None => ()
        }
        match self.s.d {
            Dir::Up => self.s.y += 1,
            Dir::Down => self.s.y -= 1,
            Dir::Left => self.s.x -= 1,
            Dir::Right => self.s.x += 1,
        }
        if Game::oob(&mut self.s, &self.b) {
            self.lost = true
        }
        match self.b.b[self.s.x as usize][self.s.y as usize] {
            BMember::Snake => self.lost = true,
            _ => ()
        }
        if self.s.x == self.a.x && self.s.y == self.a.y {
            self.apple_ct += 1;
            self.eat_apple();
        }
        self.b.b[self.s.tx as usize][self.s.ty as usize] = BMember::Empty;
        if self.s.l > 0 {
            self.s.l -= 1;
            return;
        }
        if self.s.tailinst.len() != 0 {
            if self.s.tailinst[0].x == self.s.tx && self.s.tailinst[0].y == self.s.ty {
                self.s.td = self.s.tailinst[0].dir;
                self.s.tailinst.remove(0);
            }
        }
        match self.s.td {
            Dir::Up => self.s.ty += 1,
            Dir::Down => self.s.ty -= 1,
            Dir::Left => self.s.tx -= 1,
            Dir::Right => self.s.tx += 1,
        }
    }
}
impl StartPos {
    pub fn init(w: i64, h: i64) -> Self {
        StartPos { 
            snake_x: (w as f32 * 0.25).floor() as i64, 
            snake_y: (h as f32 * 0.5).floor() as i64, 
            apple_x: (w as f32 * 0.75).floor() as i64, 
            apple_y: (h as f32 * 0.5).floor() as i64 
        }
    }
}
