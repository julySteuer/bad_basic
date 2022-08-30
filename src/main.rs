mod Tokenizer;
mod Parser;
mod Interpreter;
mod Shell;

fn main() {
    Shell::Shell::env_starter();
    Shell::Shell::new().cont_runner();
}
