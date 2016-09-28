extern crate lalrpop;

fn main() {
    lalrpop::Configuration::new()
        .emit_comments(false) // Don't emit comments to birlscript.rs. This makes the file smaller (no one will understand it anyway)
        .process_current_dir()
        .unwrap();
}
