#![forbid(unsafe_code)]

mod review_cli;
mod worker;
mod worker_io;
mod worker_task;

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let result = if args.first().is_some_and(|arg| arg == "review-v2") {
        review_cli::run(&args[1..])
    } else {
        worker::run()
    };
    if let Err(error) = result {
        eprintln!("hat-software-architect-worker: {error}");
        std::process::exit(1);
    }
}
