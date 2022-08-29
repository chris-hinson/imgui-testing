use std::thread;

mod gui_glium;
mod gui_glow;

pub fn main() {
    let runner_thread = thread::Builder::new()
        .name("thread1".to_string())
        .spawn(move || gui_glium::run_gui())
        .unwrap();

    runner_thread.join().unwrap();
}
