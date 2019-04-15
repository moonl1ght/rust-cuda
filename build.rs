extern crate cc;

fn main() {
  cc::Build::new()
    .cuda(true)
    .cpp(true)
    .flag("-cudart=shared")
    .files(&["./src/dot.cpp", "./src/dot_gpu.cu"])
    .compile("dot.a");
  println!("cargo:rustc-link-search=native=/Developer/NVIDIA/CUDA-10.1/lib");
  println!("cargo:rustc-link-search=/Developer/NVIDIA/CUDA-10.1/lib");
  println!("cargo:rustc-env=LD_LIBRARY_PATH=/Developer/NVIDIA/CUDA-10.1/lib");
  println!("cargo:rustc-link-lib=dylib=cudart");
}