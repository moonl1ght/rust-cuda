#include <iostream>
#include "dot_gpu.h"

using namespace std;

void display_vector(float *v, size_t N) {
  cout << "[";
  for (size_t i = 0; i < N; i++) {
    cout << v[i];
    if (i != N - 1) {
      cout << ", ";
    }
  }
  cout << "]" << endl;
}

extern "C" {
  float dot(float *v1, float *v2, size_t N) {
    cout << "Calling gpu dot product" << endl;
    cout << "Got two vectors from rust:" << endl;
    display_vector(v1, N);
    display_vector(v2, N);
    float *gpu_res;
    float res = 0.0;
    gpu_res = gpu_dot(v1, v2, N);
    for (int i = 0; i < blocksPerGrid; i++) {
      res += gpu_res[i];
    }
    free(gpu_res);
    return res;
  }
}