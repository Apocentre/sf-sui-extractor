fn main() {
  println!("cargo:rerun-if-changed=proto");
  tonic_build::configure()
    .out_dir("src/pb")
    .protoc_arg("--experimental_allow_proto3_optional")
    .compile(
      &[
        "proto/checkpoint.proto",
      ],
      &["proto"],
    )
    .expect("Failed to compile proto(s)");
}
