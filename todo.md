2024-03-08
- expirement with glMultiDrawElementsIndexed -> https://nickmcd.me/2021/04/04/high-performance-voxel-engine/

2024-03-10
+ generate a chunk of 16 by 16 transparent cubes using normal rendering
  + fix render with indices this needs to be added still
  + generate cube
  + generate 16x16 chunk
  + generate chunk correctly
  + generate transparent cubes
  + fix winding because some sides not shown

2024-03-13
+ center origin of generated chunk so rotation works on own axis
x implement greedy mesh

2024-03-14
+ add fps counter
x add 10x10x2 chunks

2024-03-16
+ create technique where we do not mesh anymore but use big faces to index in big 3d textures, where a 8192x8192x255 voxel world only contains 8192x2 + 8192 * 2 + 255 * 2 + 255 * 2 number of faces
  + create to 6 faces
  + generate all faces and do a binary color 3d rendering
  + create a global 3d texture of 1 byte per voxel and index into it and sample from a color palette (256 colors)
  + implement randomizer of mesh so we can see how fast we can update the global lookup table

2024-03-21
+ create a freelook camera
  + create transform class
+ set uniforms on shaders for viewing and other stuff

2024-03-22
+ start using non-indexed rendering
+ maybe compress vertices, as we only need the direction and x or y or z position
+ write buffer wgpu::Queue::write_texture to gpu to sample from
+ generate triangles from vertex index and unforms only
  + first do fixed sizes without uniforms
  + now with uniforms

2024-05-24
+ start implementing what we have currently in cpp in rust/wgpu
+ fix lattice color lookup  

2024-07-24
- investigate morton codes for sparse voxel octree (cache locality)
- investigate sdt for traversal
- investagate DDA traversal with octree
- split in 4x4x4 blocks so we can store a mask in a 64bit value which is one read for traversing one block
