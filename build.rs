// fn main() {
//     // Tell Cargo to rerun this build script if any .proto file changes
//     println!("cargo:rerun-if-changed=proto");

//     // Compile the .proto files
//     prost_build::compile_protos(&["proto/database.proto"], &["proto/"]).unwrap();
// }

// fn main() {
//     println!("cargo:rerun-if-changed=proto");
//     eprintln!("Running build script...");

//     if let Err(error) = prost_build::compile_protos(&["proto/database.proto"], &["proto/"]) {
//         eprintln!("Error compiling protos: {:?}", error);
//     }
// }


// build.rs
fn main() {
    prost_build::Config::new()
        .out_dir("src/generated") // Specify your output directory
        .compile_protos(&["proto/database.proto"], &["proto"])
        .expect("Failed to compile protobuf files");
}
