use progressing::mapping::Bar as MappingBar;
use progressing::Baring;
use std::io::Write;
use std::sync::mpsc::Receiver;
use std::thread;
use std::thread::JoinHandle;

pub(crate) fn create_progress_thread(
    max: usize,
    message: &str,
    rx: Receiver<usize>,
) -> JoinHandle<()> {
    let message = message.to_string(); // todo Is this the proper way?
    thread::spawn(move || {
        let mut progress_bar = MappingBar::with_range(0, max).timed();
        loop {
            let increment = rx.recv().expect("receiving progress");
            progress_bar.add(increment);
            print!("\r{} {}", message, progress_bar);
            std::io::stdout().flush().expect("flushing stdout");
            if progress_bar.progress() == max {
                println!();
                break;
            }
        }
    })
}
