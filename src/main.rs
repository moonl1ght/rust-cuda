extern crate libc;
extern crate rand;

use libc::{c_float, size_t};
use rand::Rng;

const VEC_SIZE: usize = 10;
const MAX: f32 = 10.;
const MIN: f32 = 0.;


extern "C" {
  fn dot(v1: *mut c_float, v2: *mut c_float, N: size_t) -> c_float;
}

fn cpu_dot(v1: Vec<f32>, v2: Vec<f32>) -> f32 {
  let mut res: f32 = 0.;
  for i in 0..v1.len() {
    res += v1[i] * v2[i];
  }
  return res;
}

fn main() {
  let mut v1: Vec<f32> = Vec::new();
  let mut v2: Vec<f32> = Vec::new();
  let mut gpu_res: c_float;
  let mut cpu_res: f32 = 0.;

  let mut rng = rand::thread_rng();
  for _ in 0..VEC_SIZE {
    v1.push(rng.gen_range(MIN, MAX));
    v2.push(rng.gen_range(MIN, MAX));
  }

  println!("{:?}", v1);
  println!("{:?}", v2);

  println!("GPU computing started");
  unsafe {
    gpu_res = dot(v1.as_mut_ptr(), v2.as_mut_ptr(), VEC_SIZE);
  }
  println!("GPU computing finished");
  println!("GPU dot product result: {}", gpu_res);
  
  cpu_res = cpu_dot(v1, v2);
  println!("CPU dot product result: {}", cpu_res);
}