use std::thread;

mod gui;

pub fn main() {
    let runner_thread = thread::Builder::new()
        .name("thread1".to_string())
        .spawn(move || gui::run_gui())
        .unwrap();

    runner_thread.join().unwrap();
}
