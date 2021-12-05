use std::env;
mod xor_functions;

fn main() {
    let argv: Vec<String> = {
        let mut t: Vec<String> = Vec::new();
        for arg in env::args() {
            t.push(arg);
        };
        t
    };

    if argv.len() != 4 { return }

    match xor_functions::xor_encrypt_file(&argv[1], &argv[2], &argv[3]) {
        _ => return
    }

}