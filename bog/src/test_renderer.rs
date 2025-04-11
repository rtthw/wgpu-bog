//! Test renderer implementation



use wgpu::util::DeviceExt as _;



pub struct Shader {
    pipeline: wgpu::RenderPipeline,
}

impl Shader {
    // TODO: Error handling.
    pub fn new(device: &wgpu::Device, desc: ShaderDescriptor) -> Result<Self, String> {
        let module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: desc.label,
            source: desc.source,
        });
        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: desc.pipeline_label,
            bind_group_layouts: &[],
            push_constant_ranges: &[],
        });
        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &module,
                entry_point: desc.vertex_entry_point,
                buffers: desc.vertex_buffers,
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &module,
                entry_point: desc.fragment_entry_point,
                targets: desc.fragment_targets,
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            }),
            primitive: desc.primitive,
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
            cache: None,
        });

        Ok(Self {
            pipeline,
        })
    }
}

pub struct ShaderDescriptor<'a> {
    pub source: wgpu::ShaderSource<'a>,
    pub label: Option<&'a str>,
    pub pipeline_label: Option<&'a str>,
    pub vertex_entry_point: Option<&'a str>,
    pub vertex_buffers: &'a [wgpu::VertexBufferLayout<'a>],
    pub fragment_entry_point: Option<&'a str>,
    pub fragment_targets: &'a [Option<wgpu::ColorTargetState>],
    pub primitive: wgpu::PrimitiveState,
}

impl<'a> Default for ShaderDescriptor<'a> {
    fn default() -> Self {
        Self {
            source: wgpu::ShaderSource::Dummy(std::marker::PhantomData),
            label: None,
            pipeline_label: None,
            vertex_entry_point: None,
            vertex_buffers: &[],
            fragment_entry_point: None,
            fragment_targets: &[],
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                // NOTE: Setting this to anything other than `Fill` requires
                //       `Features::NON_FILL_POLYGON_MODE`.
                polygon_mode: wgpu::PolygonMode::Fill,
                // NOTE: Requires `Features::DEPTH_CLIP_CONTROL`.
                unclipped_depth: false,
                // NOTE: Requires `Features::CONSERVATIVE_RASTERIZATION`.
                conservative: false,
            },
        }
    }
}



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

    pub fn finish(self, device: &wgpu::Device, ) -> (wgpu::Buffer, wgpu::Buffer, u32) {
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

        (vertex_buffer, index_buffer, self.indices.len() as u32)
    }

    pub fn add_quad(&mut self, quad: &Quad, color: [f32; 3]) {
        self.indices.reserve_exact(Quad::num_indices() as usize);
        self.vertices.reserve_exact(Quad::num_vertices() as usize);

        self.indices.extend(Quad::indices_u32().into_iter());
        quad.push_with_color(color, &mut self.vertices);
    }

    pub fn add_quads(&mut self, quads: &[Quad], color: [f32; 3]) {
        self.indices.reserve_exact(quads.len() * Quad::num_indices() as usize);
        self.vertices.reserve_exact(quads.len() * Quad::num_vertices() as usize);

        self.indices.extend(Quad::indices_u32().repeat(quads.len()));
        for quad in quads {
            quad.push_with_color(color, &mut self.vertices);
        }
    }
}



#[repr(C)]
#[derive(Clone, Copy, Debug)]
#[derive(bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    pos: [f32; 2],
    color: [f32; 3],
}

impl Vertex {
    pub fn desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x2,
                },
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 2]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x3,
                },
            ]
        }
    }
}

pub struct Quad {
    pub pos: [f32; 2],
    pub size: [f32; 2],
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
    pub const fn new(pos: [f32; 2], size: [f32; 2]) -> Self {
        Self { pos, size }
    }

    pub fn push_with_color(&self, color: [f32; 3], out: &mut Vec<Vertex>) {
        out.extend([
            Vertex { pos: self.pos, color },
            Vertex { pos: [self.pos[0] + self.size[0], self.pos[1]], color },
            Vertex { pos: [self.pos[0], self.pos[1] + self.size[1]], color },
            Vertex { pos: [self.pos[0] + self.size[0], self.pos[1] + self.size[1]], color },
        ]);
    }
}
