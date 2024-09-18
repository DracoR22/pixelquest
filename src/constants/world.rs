pub const CHUNK_SIZE: i32 = 16;
pub const OVERLAP: i32 = 1; // Amount of overlap with neighboring chunks
pub const CUBE_INDICES: [u16; 36] = [
    0,  1,  2,  2,  3,  0, // front
    4,  5,  6,  6,  7,  4, // back
    8,  9, 10, 10, 11,  8, // top
    12, 13, 14, 14, 15, 12, // bottom
    16, 17, 18, 18, 19, 16, // right
    20, 21, 22, 22, 23, 20  // left
];