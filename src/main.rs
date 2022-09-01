use std::thread;

mod gui_glium;
mod gui_glow;
mod gui_wgpu;
mod imgui_wgpu;

pub fn main() {
    let runner_thread = thread::Builder::new()
        .name("thread1".to_string())
        .spawn(move || gui_wgpu::run_gui())
        .unwrap();

    runner_thread.join().unwrap();
}
