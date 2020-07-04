use std::path::Path;

fn main() {
    let _t = xi_trace::enable_tracing();
    let res = fibonacci(7);
    println!("{}", res);
    xi_trace::save(
        Path::new("C:\\Users\\joshi\\Documents\\Dummy\\fibonnaci.json"),
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
