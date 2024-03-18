use std::{
    io::{stdin, BufRead, Write},
    process::exit,
};

pub fn flush_stdout() {
    std::io::stdout().flush().unwrap_or_else(|e| {
        println!("\nUnable to flush stdout: {}", e);
        if let Some(c) = e.raw_os_error() {
            exit(c);
        }
        exit(1);
    });
}

pub fn question(msg: &str) -> String {
    print!("{}", msg);
    flush_stdout();
    stdin()
        .lock()
        .lines()
        .next()
        .unwrap_or_else(|| exit_msg_code("\nFailed to read stdin. Stream ended.", 1))
        .unwrap_or_else(|e| exit_err_msg("\nFailed to read stdin", e))
}

pub fn question_password(msg: &str) -> String {
    print!("{}", msg);
    flush_stdout();
    rpassword::read_password().unwrap_or_else(|e| exit_err_msg("\nFailed to read stdin", e))
}

pub fn exit_msg_code(msg: &str, code: i32) -> ! {
    println!("{}", msg);
    exit(code);
}

pub fn exit_msg(msg: &str) -> ! {
    exit_msg_code(msg, 1);
}

pub fn exit_err_msg<E: Into<anyhow::Error>>(msg: &str, err: E) -> ! {
    let error = err.into();
    exit_msg(&format!("{}: {}", msg, &error))
}
