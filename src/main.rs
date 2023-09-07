use std::{time::{Instant, Duration}, thread};

use GLium::{GL, ColVert, ColBuffer};
use bot::Bot;
use game::{Game, Scrn, Dir, BMember, StartPos};
use glium::glutin::{event_loop::EventLoop, event::{Event, WindowEvent, DeviceEvent, ElementState, VirtualKeyCode}};

mod GLium;
mod game;
mod bot;

const WIDTH: i8 = 127;
const HEIGHT: i8 = 127;
const SEED: f64 = 123.0;

struct Coords {
    x: i8,
    y: i8
}
struct Dims {
    w: i8,
    h: i8
}
struct Dynamic {
    apple: ColBuffer,
    head: ColBuffer,
    tail: ColBuffer
}
struct State {
    start_pos: StartPos,
    scrn: Scrn,
    time: Instant,
    static_colbuf: ColBuffer,
    dynamic_colbuf: Dynamic,
    input: Option<Dir>,
    buffer: Option<Dir>,
    reset: bool,
    game: Game
}

fn board_coords(b: &Dims, x: i8, y: i8) -> (f32, f32) {
    return (
        -1.0 + x as f32 * (2.0 / b.w as f32),
        -1.0 + y as f32 * (2.0 / b.h as f32)
    )
}
fn make_board_rect(b: &Dims, x: i8, y: i8, col: [f32; 4], voff: u16) -> ([ColVert; 4], [u16; 6]) {
    let (cx, cy) = board_coords(b, x, y);
    make_rect(
        cx, 
        cy, 
        cx + 2.0 / b.w as f32,
        cy + 2.0 / b.h as f32,
        col, 
        voff
    )
}
fn make_board_vert(b: &Dims, x: i8, y: i8, col: [f32; 4]) -> [ColVert; 4] {
    let (cx, cy) = board_coords(b, x, y);
    [
        ColVert { pos: [cx, cy], col },
        ColVert { pos: [cx + 2.0 / b.w as f32, cy], col },
        ColVert { pos: [cx, cy + 2.0 / b.h as f32], col },
        ColVert { pos: [cx + 2.0 / b.w as f32, cy + 2.0 / b.h as f32], col }
    ]
}
fn make_rect(x: f32, y: f32, x2: f32, y2: f32, col: [f32; 4], voff: u16) -> ([ColVert; 4], [u16; 6]) {
    let fv = [
        ColVert { pos: [x, y], col },
        ColVert { pos: [x2, y], col },
        ColVert { pos: [x, y2], col },
        ColVert { pos: [x2, y2], col }
    ];
    let fi = [0u16, 2, 3, 0, 1, 3];
    (fv, fi.map(|x| x + voff))
}
fn init_bg(w: i8, h: i8, col: [f32; 4]) -> (Vec<ColVert>, Vec<u16>) {
    let mut fv: Vec<ColVert> = vec![];
    let mut fi: Vec<u16> = vec![];
    let sw = 2.0 / w as f32;
    let sh = 2.0 / h as f32;

    for x in 0..(w/2) { for y in 0..(h/2) {
        let fx = x as f32 * 2.0;
        let fy = y as f32 * 2.0;
        let (fv1, fi1) = make_rect(
            -1.0 + fx * sw,
            1.0 - fy * sh,
            -1.0 + sw + fx * sw, 
            1.0 - sh - fy * sh,
            col, fv.len() as u16
        );
        let (fv2, fi2) = make_rect(
            -1.0 + sw + fx * sw,
            1.0 - sh - fy * sh,
            -1.0 + sw + sw + fx * sw, 
            1.0 - sh - sh - fy * sh,
            col, fv.len() as u16 + 4
        );
        fv.extend(fv1);
        fv.extend(fv2);
        fi.extend(fi1);
        fi.extend(fi2);
    }}

    (fv, fi)
}
fn init_scene_game(gl: &GL, w: i8, h: i8) -> ColBuffer {
    let bg = init_bg(w, h, [0.01, 0.01, 0.01, 1.0]);
    gl.new_colbuf(&bg.0, &bg.1)
}
fn recalc_board(gl: &GL, game: &Game, res: &mut Dynamic, growing: bool) {
    let dims = Dims { w: game.b.w, h: game.b.h };
    let sv = make_board_vert(&dims, game.s.x, game.s.y, [0.25, 0.25, 0.1, 1.0]);
    let av = make_board_vert(&dims, game.a.x, game.a.y, [0.25, 0.1, 0.1, 1.0]);
    let mut tv = vec![];
    let mut ti = vec![];

    match growing {
        true => for x in 0..game.b.b.len() { for y in 0..game.b.b[x].len() {
            let bm = match game.b.b[x][y] {
                BMember::Snake => Some(make_board_rect(&dims, x as i8, y as i8, [0.1, 0.1, 0.04, 1.0], tv.len() as u16)),
                _ => None
            };
            match bm {
                Some((v, i)) => {
                    tv.extend(v);
                    ti.extend(i);
                },
                _ => ()
            }
        }},
        false => for x in 0..game.b.b.len() { for y in 0..game.b.b[x].len() {
            let bm = match game.b.b[x][y] {
                BMember::Snake => Some(make_board_vert(&dims, x as i8, y as i8, [0.1, 0.1, 0.04, 1.0])),
                _ => None
            };
            match bm {
                Some(v) => {
                    tv.extend(v);
                },
                _ => ()
            }
        }}
    }

    res.apple.v.write(&av);
    res.head.v.write(&sv);
    if growing {
        res.tail = gl.new_colbuf_dyn(&tv, &ti);
        return
    }
    res.tail.v.write(&tv);
}
fn prep_board(gl: &GL, start_pos: &StartPos, b: &Dims) -> (ColBuffer, ColBuffer, ColBuffer) {
    let (sv, si) = make_board_rect(b, start_pos.snake_x, start_pos.snake_y, [0.25, 0.25, 0.1, 1.0], 0);
    let (av, ai) = make_board_rect(b, start_pos.apple_x, start_pos.apple_y, [0.25, 0.1, 0.1, 1.0], 0);
    let tv: [ColVert; 0] = [];
    let ti: [u16; 0] = [];

    (
        gl.new_colbuf_dyn(&sv, &si),
        gl.new_colbuf_dyn(&av, &ai),
        gl.new_colbuf_dyn(&tv, &ti)
    )
}
fn mainloop(gl: &GL, s: &mut State) {
    if s.reset {
        s.game = Game::new(WIDTH, HEIGHT, &s.start_pos, SEED);
        s.reset = false;
    }
    s.game.step(s.input);
    s.input = s.buffer;
    s.buffer = None;
    s.time = Instant::now();
    recalc_board(&gl, &s.game, &mut s.dynamic_colbuf, true);
    gl.d.gl_window().window().request_redraw();
}
fn input(s: &mut State, k: Option<VirtualKeyCode>) {
    let res = match k {
        Some(VirtualKeyCode::A) => Some(Dir::Left),
        Some(VirtualKeyCode::W) => Some(Dir::Up),
        Some(VirtualKeyCode::S) => Some(Dir::Down),
        Some(VirtualKeyCode::D) => Some(Dir::Right),
        Some(VirtualKeyCode::R) => {
            s.reset = true;
            None
        },
        _ => None
    };
    match s.input {
        Some(_) => s.buffer = res,
        None => s.input = res
    }
}
fn main() {
    let events_loop = EventLoop::new();
    let gl = GL::new(&events_loop, [0.1, 0.1, 0.1]);
    let start_pos = StartPos::init(WIDTH, HEIGHT);
    let dynamic = prep_board(&gl, &start_pos, &Dims { w: WIDTH, h: HEIGHT });
    let mut state = State {
        game: Game::new(WIDTH, HEIGHT, &start_pos, SEED),
        start_pos,
        scrn: Scrn::Game,
        time: Instant::now(),
        static_colbuf: init_scene_game(&gl, WIDTH, HEIGHT),
        dynamic_colbuf: Dynamic { apple: dynamic.0, head: dynamic.1, tail: dynamic.2 },
        input: None,
        buffer: None,
        reset: false
    };
    events_loop.run(move |e, _, c| {
        c.set_poll();
        match e {
            Event::WindowEvent { 
                event: WindowEvent::CloseRequested, 
                .. 
            } => c.set_exit(),
            Event::DeviceEvent {
                event: DeviceEvent::Key(k),
                ..
            } => match k.state {
                ElementState::Pressed => input(&mut state, k.virtual_keycode),
                _ => ()
            },
            Event::MainEventsCleared => mainloop(&gl, &mut state),
            Event::RedrawRequested(_) => gl.draw(&[
                &state.static_colbuf, 
                &state.dynamic_colbuf.tail,
                &state.dynamic_colbuf.apple,
                &state.dynamic_colbuf.head
            ]),
            _ => ()
        }
        if (state.time.elapsed().as_millis() as u16) < state.game.steptime {
            thread::sleep(Duration::from_millis(
                (state.game.steptime as u128 - state.time.elapsed().as_millis()) as u64
            ));
        }
    })
}
