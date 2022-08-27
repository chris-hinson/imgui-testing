use glow::HasContext;
use imgui::Condition;
use imgui::Context;
use imgui_glow_renderer::AutoRenderer;
use imgui_sdl2_support::SdlPlatform;
use rand::Rng;
use sdl2::{
    event::Event,
    keyboard::Keycode,
    video::{GLProfile, Window},
};

// Create a new glow context.
fn glow_context(window: &Window) -> glow::Context {
    unsafe {
        glow::Context::from_loader_function(|s| window.subsystem().gl_get_proc_address(s) as _)
    }
}

pub fn run_gui() {
    /* initialize SDL and its video subsystem */
    let sdl = sdl2::init().unwrap();
    let video_subsystem = sdl.video().unwrap();

    /* hint SDL to initialize an OpenGL 3.3 core profile context */
    let gl_attr = video_subsystem.gl_attr();
    //TODO:REMOVE ME IF YOU DONT WANT An SDL DEBUG CONTEXT
    gl_attr.set_context_flags().debug().set();

    gl_attr.set_context_version(3, 3);
    gl_attr.set_context_profile(GLProfile::Core);

    /* create a new window, be sure to call opengl method on the builder when using glow! */
    let window = video_subsystem
        .window("Hello imgui-rs!", 1280, 720)
        .allow_highdpi()
        .opengl()
        .position_centered()
        .resizable()
        .build()
        .unwrap();

    /* create a new OpenGL context and make it current */
    let gl_context = window.gl_create_context().unwrap();
    window.gl_make_current(&gl_context).unwrap();

    /* enable vsync to cap framerate */
    window.subsystem().gl_set_swap_interval(1).unwrap();

    /* create new glow and imgui contexts */
    let gl = glow_context(&window);

    let mut imgui = Context::create();
    imgui.set_ini_filename(None);
    imgui.set_log_filename(None);

    /* setup platform and renderer, and fonts to imgui */
    imgui
        .fonts()
        .add_font(&[imgui::FontSource::DefaultFontData { config: None }]);

    let mut platform = SdlPlatform::init(&mut imgui);
    let mut renderer = AutoRenderer::initialize(gl, &mut imgui).unwrap();
    let mut event_pump = sdl.event_pump().unwrap();

    let mut textures = imgui::Textures::<glow::Texture>::default();

    let mut frame: Vec<u8> = vec![200; 184_320];

    mutate(&mut frame);

    unsafe {
        renderer.gl_context().enable(glow::DEBUG_OUTPUT);
        renderer.gl_context().enable(glow::DEBUG_OUTPUT_SYNCHRONOUS);
        renderer.gl_context().debug_message_callback(debug_handler);
    }

    //let test_texture = Screen::new(&renderer.gl_context(), &mut textures, &frame);
    'main: loop {
        for event in event_pump.poll_iter() {
            /* pass all events to imgui platfrom */
            platform.handle_event(&mut imgui, &event);

            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'main,
                _ => {}
            }
        }

        //just change 1000 pixels in our framebuffer to be a randomly chosen color
        mutate(&mut frame);

        //clear the color buffer (i.e. screen)
        unsafe {
            renderer.gl_context().clear(glow::COLOR_BUFFER_BIT);
        }

        //prepare this frame
        platform.prepare_frame(&mut imgui, &window, &event_pump);
        //make a new frame and append whatever to it
        let ui = imgui.new_frame();

        ui.window("test texture")
            .size([256.0, 240.0], Condition::FirstUseEver)
            .build(|| {
                let mut test_texture = Screen::new(&renderer.gl_context(), &mut textures, &frame);
                //test_texture.update(&renderer.gl_context(), &frame);
                //imgui::Image::new(test_texture.texture_id, test_texture.size).build(ui);
                test_texture.show(ui);
            });

        ui.show_metrics_window(&mut true);

        //turn our frame into drawing data to be passed to our AutoRenderer
        let draw_data = imgui.render();
        //finally, execute the draw commands for the ui
        renderer.render(draw_data).unwrap();

        //swap into the buffer we were just drawing in
        window.gl_swap_window();
    }
}

struct Screen {
    texture_id: imgui::TextureId,
    size: [f32; 2],
}

impl Screen {
    fn new(
        gl: &glow::Context,
        textures: &mut imgui::Textures<glow::Texture>,
        pixels: &[u8],
    ) -> Self {
        //frame is 256 x 240
        const WIDTH: usize = 256;
        const HEIGHT: usize = 240;

        let gl_texture = unsafe { gl.create_texture() }.expect("unable to create GL texture");

        unsafe {
            gl.bind_texture(glow::TEXTURE_2D, Some(gl_texture));
            gl.tex_parameter_i32(
                glow::TEXTURE_2D,
                glow::TEXTURE_MIN_FILTER,
                glow::LINEAR as _,
            );

            gl.tex_parameter_i32(
                glow::TEXTURE_2D,
                glow::TEXTURE_MAG_FILTER,
                glow::LINEAR as _,
            );
            gl.tex_image_2d(
                glow::TEXTURE_2D,
                0,
                glow::RGB as i32,
                WIDTH as i32,
                HEIGHT as i32,
                0,
                glow::RGB,
                glow::UNSIGNED_BYTE,
                Some(pixels),
            )
        }

        Self {
            texture_id: textures.insert(gl_texture),
            size: [WIDTH as f32, HEIGHT as f32],
        }
    }

    fn update(&mut self, gl: &glow::Context, pixels: &[u8]) {
        unsafe {
            gl.tex_sub_image_2d(
                glow::TEXTURE_2D,
                0,
                0,
                240,
                256,
                240,
                glow::RGB,
                glow::UNSIGNED_BYTE,
                glow::PixelUnpackData::Slice(pixels),
            );
        }
    }

    fn show(&self, ui: &imgui::Ui) {
        imgui::Image::new(self.texture_id, self.size).build(ui);
    }
}

fn mutate(buffer: &mut Vec<u8>) {
    let mut rng = rand::thread_rng();

    for _i in 0..1000 {
        let new_x = rng.gen_range(0..256);
        let new_y = rng.gen_range(0..240);
        let new_r = rng.gen_range(0..u8::MAX);
        let new_g = rng.gen_range(0..u8::MAX);
        let new_b = rng.gen_range(0..u8::MAX);

        buffer[new_y * 3 * 256 + new_x * 3] = new_r;
        buffer[new_y * 3 * 256 + new_x * 3 + 1] = new_g;
        buffer[new_y * 3 * 256 + new_x * 3 + 2] = new_b;
    }
}

fn debug_handler(_source: u32, _err_type: u32, _id: u32, severity: u32, msg: &str) {
    match severity {
        glow::DEBUG_SEVERITY_NOTIFICATION => {
            println!("NOTIFICATION: {}", msg)
        }
        glow::DEBUG_SEVERITY_LOW => {
            println!("INFO: {}", msg)
        }
        glow::DEBUG_SEVERITY_MEDIUM => {
            println!("WARNING: {}", msg)
        }
        glow::DEBUG_SEVERITY_HIGH => {
            println!("ERROR: {}", msg)
        }
        _ => println!("got an error with an invalid severity level"),
    }
}
