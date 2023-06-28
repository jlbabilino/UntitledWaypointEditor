fn main() {
  let path = std::path::PathBuf::from("target/debug");
  println!("cargo:extra-link-arg=-Wl,-rpath={}", path.display());
  tauri_build::build()
}
