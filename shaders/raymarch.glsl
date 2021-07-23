#version 450 core

layout(local_size_x = 32, local_size_y = 32, local_size_z = 1) in;

layout(rgba32f, binding = 0) buffer img_output;
layout(std430, binding = 1) buffer voxel_SSBO
{
    int data_SSBO[];
};

void main() {

}
