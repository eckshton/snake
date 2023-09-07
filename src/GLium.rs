use std::fs;

use glium::{
    implement_vertex, 
    Display, 
    glutin::{
        event_loop::EventLoop, 
        window::WindowBuilder, 
        ContextBuilder, 
    }, 
    VertexBuffer, 
    IndexBuffer, index::PrimitiveType, Program, Surface, uniforms::EmptyUniforms, 
};

#[derive(Copy, Clone)]
pub struct TexVert {
    pub pos: [f32; 2],
    pub tex: [f32; 2]
}
#[derive(Copy, Clone)]
pub struct ColVert {
    pub pos: [f32; 2],
    pub col: [f32; 4]
}
implement_vertex!(TexVert, pos, tex);
implement_vertex!(ColVert, pos, col);

fn new_program(d: &Display, v: &str, f: &str) -> Program {
    let vsrc = fs::read_to_string(v).unwrap();
    let fsrc = fs::read_to_string(f).unwrap();
    Program::from_source(d, &vsrc, &fsrc, None).unwrap()
}

pub struct ColBuffer {
    pub v: VertexBuffer<ColVert>,
    pub i: IndexBuffer<u16>
}
pub struct GL {
    pub d: Display,
    tp: Program,
    cp: Program,
    bg: [f32; 3]
}
impl GL {
    pub fn new(events_loop: &EventLoop<()>, bg: [f32; 3]) -> Self {
        let wb = WindowBuilder::new().with_title("snek");
        let cb = ContextBuilder::new().with_vsync(true);
        let d = Display::new(wb, cb, events_loop).unwrap();
        let tp = new_program(&d, "src/shader/vertex/tex.glsl", "src/shader/fragment/tex.glsl");
        let cp = new_program(&d, "src/shader/vertex/solid.glsl", "src/shader/fragment/solid.glsl");
        GL {
            d,
            tp,
            cp,
            bg
        }
    }
    pub fn new_colbuf(self: &Self, v: &[ColVert], i: &[u16]) -> ColBuffer {
        ColBuffer { 
            v: VertexBuffer::immutable(&self.d, v).unwrap(), 
            i: IndexBuffer::new(&self.d, PrimitiveType::TrianglesList, i).unwrap() 
        }
    }
    pub fn new_colbuf_dyn(self: &Self, v: &[ColVert], i: &[u16]) -> ColBuffer {
        ColBuffer { 
            v: VertexBuffer::dynamic(&self.d, v).unwrap(), 
            i: IndexBuffer::new(&self.d, PrimitiveType::TrianglesList, i).unwrap() 
        }
    }
    pub fn draw(self: &Self, colbufs: &[&ColBuffer]) {
        let mut f = self.d.draw();
        f.clear_color(self.bg[0], self.bg[1], self.bg[2], 1.0);
        for buf in colbufs.iter() {
            f.draw(&buf.v, &buf.i, &self.cp, &EmptyUniforms, &Default::default()).unwrap();
        }
        f.finish().unwrap();
    }
}
