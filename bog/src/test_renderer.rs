//! Test renderer implementation



use wgpu::util::DeviceExt as _;



pub struct Renderer {
    vertices: Vec<Vertex>,
    indices: Vec<u32>,
}

impl Renderer {
    pub fn start() -> Self {
        Self {
            vertices: Vec::new(),
            indices: Vec::new(),
        }
    }

    pub fn finish(self, device: &wgpu::Device, ) -> (wgpu::Buffer, wgpu::Buffer) {
        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(&self.vertices),
            usage: wgpu::BufferUsages::VERTEX,
        });
        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Index Buffer"),
            contents: bytemuck::cast_slice(&self.indices),
            usage: wgpu::BufferUsages::INDEX,
        });

        (vertex_buffer, index_buffer)
    }

    pub fn render_quad(&mut self, quad: &Quad, color: [f32; 3]) {
        self.indices.reserve_exact(Quad::num_indices() as usize);
        self.indices.extend(Quad::indices_u32().into_iter());
        self.vertices.reserve_exact(Quad::num_vertices() as usize);
        quad.push_with_color(color, &mut self.vertices);
    }
}



#[repr(C)]
#[derive(Clone, Copy, Debug)]
#[derive(bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    pos: [f32; 2],
    color: [f32; 3],
}


pub struct Quad {
    pos: [f32; 2],
    size: [f32; 2],
}

// Constants.
impl Quad {
    pub const fn num_indices() -> u32 {
        6
    }

    pub const fn num_vertices() -> u32 {
        4
    }

    pub const fn indices_u16() -> [u16; 6] {
        [0, 1, 2, 2, 1, 3]
    }

    pub const fn indices_u32() -> [u32; 6] {
        [0, 1, 2, 2, 1, 3]
    }
}

impl Quad {
    pub fn push_with_color(&self, color: [f32; 3], out: &mut Vec<Vertex>) {
        out.extend([
            Vertex { pos: self.pos, color },
            Vertex { pos: [self.pos[0] + self.size[0], self.pos[1]], color },
            Vertex { pos: [self.pos[0], self.pos[1] + self.size[1]], color },
            Vertex { pos: [self.pos[0] + self.size[0], self.pos[1] + self.size[1]], color },
        ]);
    }
}
