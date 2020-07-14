use std::path::Path;
use std::time::SystemTime;
use chrono::Local;

fn main() {
    let _t = xi_trace::enable_tracing();
    let res = fibonacci(7);
    println!("{}", res);
    let date = Local::now();
    let file_name = format!("{}.json", date.format("%Y-%m-%d-%H-%M-%S"));
    println!("{}", file_name);
    xi_trace::save(
        Path::new(&file_name),
        true,
    )
    .expect("Failed to save data");
}

fn fibonacci(n: u32) -> u32 {
    let _t = xi_trace::trace_block(format!("{}", n), &["fib"]);
    if n == 0 || n == 1 {
        1
    } else {
        fibonacci(n - 1) + fibonacci(n - 2)
    }
}
