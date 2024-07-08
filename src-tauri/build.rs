use std::process::Command;
use std::path::{Path, PathBuf};

fn main() {
  let tiks_command_path = "../tiks_command";

  let status = Command::new("cargo")
  .args(&["build","--release", "--manifest-path", &format!("{}/Cargo.toml", tiks_command_path)])
  .status()
  .expect("Error: Can't build the tiks command");

  if !status.success(){
    panic!("Failed to build tiks_command")
  }
  #[allow(unused_assignments)]
  let mut exe_path = PathBuf::new();

  if cfg!(target_os="windows"){
    exe_path = Path::new(tiks_command_path).join("target/release/tiks.exe");
  }else{
    exe_path = Path::new(tiks_command_path).join("target/release/tiks");
  }

  // 设置环境变量
  println!("cargo:rustc-env=Tiks={}", exe_path.clone().display());

  // 运行生成的可执行文件
  let status = Command::new(exe_path)
      .status()
      .expect("Failed to execute tiks_command");

  if !status.success() {
      panic!("tiks_command execution failed");
  }
  // 在构建时重新运行此脚本
  println!("cargo:rerun-if-changed=build.rs");

  tauri_build::build()
}