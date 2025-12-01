// use std::fs;
// use std::path::Path;
// fn main() {
//     tonic_prost_build::configure()
//     .out_dir("src/generated")
//     .compile_protos(&["../proto/hello.proto"], &["../proto"])
//     .unwrap();
// }


fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let out_dir = "src/generated";
    // fs::create_dir_all(Path::new(out_dir))?;

    let out_dir = std::env::var("OUT_DIR")?;
    println!("cargo:warning=OUT_DIR is : {}", out_dir);

    tonic_prost_build::configure()
    //    .out_dir(out_dir)
        .build_server(true)
        .build_client(true)
        .out_dir(std::env::var("OUT_DIR").unwrap())
//        .out_dir(out_dir)
        .compile_protos(
            &["../proto/hello.proto"],
            &["../proto"],
        )?;
   Ok(())
}

// ./target/debug/build/proto-defs-c9c645741b3a0923/out/hello.rs
// ./target/debug/build/proto-defs-c9c645741b3a0923/out/hello.rs