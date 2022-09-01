use winit::platform::unix::EventLoopBuilderExtUnix;

use winit::dpi::LogicalSize;
use winit::event::ElementState;
use winit::event::KeyboardInput;
use winit::event::VirtualKeyCode;
use winit::{
    event::{Event, WindowEvent},
    event_loop::EventLoop,
    window::Window,
    window::WindowBuilder,
};

pub fn run_gui() {
    //let event_loop = EventLoop::new();
    //winit::window::Window::new(&event_loop);
    /*let mut loop_builder = winit::event_loop::EventLoopBuilder::new();
    let event_loop = loop_builder.with_any_thread(true).build();
    winit::window::Window::new(&event_loop).unwrap();

    event_loop.run(move |event, _, control_flow| {
        control_flow.set_wait();

        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => control_flow.set_exit(),
            _ => (),
        }
    });*/

    let mut loop_builder = winit::event_loop::EventLoopBuilder::new();
    let event_loop = loop_builder.with_any_thread(true).build();

    let _window = WindowBuilder::new()
        .with_title("this is a window")
        //.with_inner_size(LogicalSize::new(600.0, 300.0))
        //.with_min_inner_size(LogicalSize::new(400.0, 200.0))
        //.with_max_inner_size(LogicalSize::new(800.0, 400.0))
        //.with_resizable(resizable)
        .build(&event_loop)
        .unwrap();

    event_loop.run(move |event, _, control_flow| {
        control_flow.set_wait();

        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => control_flow.set_exit(),

                _ => (),
            },
            _ => (),
        };
    });
}
