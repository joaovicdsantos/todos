use std::process;

pub fn load_args() -> GenericArguments {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.len() == 0 {
        eprint!("Usage: todos <action> <args>");
        process::exit(1)
    }
    GenericArguments {
        action: args[0].to_string(),
        params: args[1..].to_vec(),
    }
}

#[derive(Debug)]
pub struct GenericArguments {
    pub action: String,
    pub params: Vec<String>,
}
