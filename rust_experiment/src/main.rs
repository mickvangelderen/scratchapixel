extern crate gl;
extern crate glutin;
extern crate libc;

fn main() {
    let window = glutin::WindowBuilder::new()
        .with_title("Rust Experiment!")
        .with_gl(glutin::GlRequest::Specific(glutin::Api::OpenGl, (3, 3)))
        .with_gl_profile(glutin::GlProfile::Core)
        .build()
        .unwrap();

    unsafe {
        window.make_current().unwrap();

        gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);
    }

    // Create shaders.
    let vertex_shader_id = file_to_shader(gl::VERTEX_SHADER, "assets/vertex_shader.glsl").unwrap();
    let fragment_shader_id = file_to_shader(gl::FRAGMENT_SHADER, "assets/fragment_shader.glsl").unwrap();
    unsafe {
        let program_id: u32 = gl::CreateProgram();
        gl::AttachShader(program_id, vertex_shader_id);
        gl::AttachShader(program_id, fragment_shader_id);
        gl::LinkProgram(program_id);
        gl::UseProgram(program_id);
    }

    let mut vertex_array_id: u32 = 0;

    unsafe { gl::GenVertexArrays(1, &mut vertex_array_id); }

    unsafe { gl::BindVertexArray(vertex_array_id); }

    let vertex_buffer_data: [f32; 9] = [
        -1.0, -1.0, 0.0,
         1.0, -1.0, 0.0,
         0.0,  1.0, 0.0
    ];

    let mut vertex_buffer_id: u32 = 0;

    unsafe { gl::GenBuffers(1, &mut vertex_buffer_id); }

    unsafe { gl::BindBuffer(gl::ARRAY_BUFFER, vertex_buffer_id); }

    // unsafe {
    //     let x = &vertex_buffer_data as *const std::os::raw::c_void;
    // }
    unsafe {
        gl::BufferData(
            gl::ARRAY_BUFFER,
            std::mem::size_of_val(&vertex_buffer_data) as gl::types::GLsizeiptr,
            vertex_buffer_data.as_ptr() as *const std::os::raw::c_void,
            gl::STATIC_DRAW
        );
    }

    let mut tint = 0.0;

    'application: loop {
        for event in window.poll_events() {
            match event {
                glutin::Event::Closed => break 'application,
                glutin::Event::KeyboardInput(glutin::ElementState::Pressed, _, Some(key)) => {
                    match key {
                        glutin::VirtualKeyCode::Q | glutin::VirtualKeyCode::Escape =>
                            break 'application,
                        _ => ()
                    }
                },
                _ => ()
            }
        }

        // Updating.
        tint = (tint + 0.016) % 1.0;

        // Drawing.
        unsafe {
            gl::ClearColor(tint, tint, tint, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        };

        unsafe { gl::EnableVertexAttribArray(0); }

        unsafe { gl::BindBuffer(gl::ARRAY_BUFFER, vertex_buffer_id); }

        unsafe { gl::VertexAttribPointer(
            // Attribute 0, must match shader layout
            0,
            // Size.
            3,
            gl::FLOAT,
            // Normalized?
            gl::FALSE,
            // Stride
            0,
            // Buffer offset
            std::ptr::null()
        ); }

        unsafe { gl::DrawArrays(gl::TRIANGLES, 0, 3); }

        unsafe { gl::DisableVertexAttribArray(0); }

        window.swap_buffers().unwrap();
        std::thread::sleep(std::time::Duration::from_millis(16));
    }
}

fn file_to_shader<P: AsRef<std::path::Path>>(shader_type: u32, path: P) -> Result<u32, std::io::Error> {
    file_to_string(path)
        .map(|source| {
            unsafe {
                let id: u32 = gl::CreateShader(shader_type);
                gl::ShaderSource(
                    id,
                    1,
                    [source.as_ptr() as *const i8].as_ptr(),
                    [source.len() as i32].as_ptr()
                );
                gl::CompileShader(id);
                id
            }
        })
}

fn file_to_string<P: AsRef<std::path::Path>>(path: P) -> Result<String, std::io::Error> {
    use std::fs::File;
    use std::io::BufReader;
    use std::io::Read;

    File::open(path)
        .and_then(|file| {
            let mut contents = String::new();
            BufReader::new(file)
                .read_to_string(&mut contents)
                .map(|_| contents)
        })
}
