use crate::input::UserInput;
use nalgebra_glm as glm;

use crate::canvas::CanvasData;
use crate::transform::Transform;
use crate::RenderObjectTrait;
use web_sys::WebGlBuffer;
use web_sys::WebGlProgram;
use web_sys::WebGlRenderingContext as GL;
use web_sys::WebGlUniformLocation;

pub struct AttributeLocations {
    pub vertex_position: i32,
}

pub struct UniformLocations {
    projection_matrix: WebGlUniformLocation,
    model_view_matrix: WebGlUniformLocation,
}

pub struct Box2D {
    buffer: WebGlBuffer,
    program: WebGlProgram,
    attribute_locations: AttributeLocations,
    uniform_locations: UniformLocations,
    pub canvas: CanvasData,
    pub transform: Transform,
    pub input: UserInput,
}

impl Box2D {

    fn init_buffers(gl: &GL) -> WebGlBuffer {
        let position_buffer = gl.create_buffer().unwrap();
        gl.bind_buffer(GL::ARRAY_BUFFER, Some(&position_buffer));
        let positions = [-1., 1., 1., 1., -1., -1., 1., -1.];

        unsafe {
            let vert_array = js_sys::Float32Array::view(&positions);

            gl.buffer_data_with_array_buffer_view(GL::ARRAY_BUFFER, &vert_array, GL::STATIC_DRAW);
        }
        position_buffer
    }
}

impl RenderObjectTrait for Box2D {
    fn new(gl: &GL, program: WebGlProgram, canvas: CanvasData, transform: Transform) -> Box2D {
        let attribute_locations = AttributeLocations {
            vertex_position: gl.get_attrib_location(&program, "aVertexPosition"),
        };
        let uniform_locations = UniformLocations {
            projection_matrix: gl
                .get_uniform_location(&program, "uProjectionMatrix")
                .unwrap(),
            model_view_matrix: gl
                .get_uniform_location(&program, "uModelViewMatrix")
                .unwrap(),
        };

        let buffer = Box2D::init_buffers(&gl);

        let input = UserInput::new();

        Box2D {
            buffer,
            attribute_locations,
            uniform_locations,
            program,
            canvas,
            transform,
            input,
        }
    }

    fn draw_scene(&self, gl: &GL) {
        gl.clear_color(0., 0., 0., 1.);
        gl.clear_depth(1.);
        gl.enable(GL::DEPTH_TEST);
        gl.depth_func(GL::LEQUAL);
        gl.clear(GL::COLOR_BUFFER_BIT | GL::DEPTH_BUFFER_BIT);

        let z_near: f32 = 0.1;
        let z_far: f32 = 100.0;

        let projection_matrix = glm::perspective(
            self.canvas.get_aspect(),
            self.canvas.get_fov(),
            z_near,
            z_far,
        );
        let mut empty_matrix = glm::mat4x4(
            0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0.,
        );
        empty_matrix.fill_with_identity();
        let translation_vector = glm::vec3(
            self.transform.get_trans_x(),
            self.transform.get_trans_y(),
            self.transform.get_trans_z(),
        );
        let model_view_matrix = glm::translate(&empty_matrix, &translation_vector);

        let number_components = 2;
        let buffer_type = GL::FLOAT;
        let normalize = false;
        let stride = 0;
        let offset = 0;

        gl.bind_buffer(GL::ARRAY_BUFFER, Some(&self.buffer));
        gl.vertex_attrib_pointer_with_i32(
            self.attribute_locations.vertex_position as u32,
            number_components,
            buffer_type,
            normalize,
            stride,
            offset,
        );
        gl.enable_vertex_attrib_array(self.attribute_locations.vertex_position as u32);

        gl.use_program(Some(&self.program));
        let transpose = false;
        gl.uniform_matrix4fv_with_f32_array(
            Some(&self.uniform_locations.projection_matrix),
            transpose,
            projection_matrix.as_slice(),
        );
        gl.uniform_matrix4fv_with_f32_array(
            Some(&self.uniform_locations.model_view_matrix),
            transpose,
            model_view_matrix.as_slice(),
        );

        let offset = 0;
        let vertex_count = 4;
        gl.draw_arrays(GL::TRIANGLE_STRIP, offset, vertex_count);
    }

    fn canvas(&mut self) -> &mut CanvasData {
        &mut self.canvas
    }
    fn input(&mut self) -> &mut UserInput {
        &mut self.input
    }
    fn transform(&mut self) -> &mut Transform {
        &mut self.transform
    }
    fn set_canvas(&mut self, canvas: CanvasData) {
        self.canvas = canvas;
    }
    fn set_transform(&mut self, transform: Transform) {
        self.transform = transform;
    }
    fn set_input(&mut self, input: UserInput) {
        self.input = input;
    }
}
