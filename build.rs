use std::env;
use std::path::Path;

fn main() {
    println!("cargo:rerun-if-changed=grammar/OutboundAPILexer.g4");
    println!("cargo:rerun-if-changed=grammar/OutboundAPIParser.g4");

    let dest_path = Path::new("src/infrastructure/generated");

    // Ensure the destination directory exists
    let _ = std::fs::create_dir_all(&dest_path);

    // Run antlr-rust generator
    // Note: In a real environment, you might use antlr_rust::generate
    // For now, we print instructions as antlr4 requires the Java runtime 
    // and antlr4-rust jar to be available to build properly.
    
    // As a fallback for this migration, we assume the user will manually generate
    // the files using the instructions in `grammar/README.md` if `build.rs` fails
    // to find the local Java/ANTLR environment.
}
