use lexer::{preprocesser::{self, preprocesser, read_source}, scanner::scanner};
use log::debug;

mod lexer;


fn main() {
    env_logger::init_from_env(env_logger::Env::new().filter("RUST_LOG").write_style("debug"));

    let mut input = String::from("sourceProgram8");
    let source_file_path = std::env::current_dir().unwrap().join(input);
    debug!("path: {:?}", source_file_path);

    let preprocesser_output = preprocesser(&read_source(source_file_path));
    debug!("source: {:?}", preprocesser_output);

    let scanner_output = scanner(preprocesser_output);
    debug!("tokens: {:?}", scanner_output);
}
