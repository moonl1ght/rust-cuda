#include <stdio.h>

const int threadsPerBlock = 512;
const int blocksPerGrid = 64;

float * gpu_dot(float *v1, float *v2, size_t N);