#include "dot_gpu.h"

__global__ void dot__(float *v1, float *v2, float *res, int N) {
  __shared__ float cache [threadsPerBlock];
  int tid = threadIdx.x + blockIdx.x * blockDim.x;
  int cacheIndex = threadIdx.x;
  float temp = 0.0;
  while (tid < N) {
    temp += v1[tid] * v2[tid];
    tid += blockDim.x * gridDim.x;
  }
  cache[cacheIndex] = temp;

  __syncthreads();

  int i = blockDim.x / 2;
  while (i != 0) {
    if (cacheIndex < i) {
      cache[cacheIndex] += cache[cacheIndex + i];
    }
    __syncthreads();
    i /= 2;   
  }

  if (cacheIndex == 0) {
    res[blockIdx.x] = cache[0];
  }
}

float * gpu_dot (float *v1, float *v2, size_t N) {
	float *dev_v1, *dev_v2, *dev_res, *res;
  res = new float[blocksPerGrid];
  
	cudaMalloc((void**)&dev_v1, N * sizeof(float));
  cudaMalloc((void**)&dev_v2, N * sizeof(float));
	cudaMalloc((void**)&dev_res, blocksPerGrid * sizeof(float));
	
	cudaMemcpy(dev_v1, v1, N * sizeof(float), cudaMemcpyHostToDevice);
	cudaMemcpy(dev_v2, v2, N * sizeof(float), cudaMemcpyHostToDevice);
	
	dot__<<<blocksPerGrid, threadsPerBlock>>>(dev_v1, dev_v2, dev_res, (int)N);
	cudaMemcpy(res, dev_res, blocksPerGrid * sizeof(float), cudaMemcpyDeviceToHost);

	cudaFree(dev_v1);
  cudaFree(dev_v2);
	cudaFree(dev_res);
	
	return res;
}