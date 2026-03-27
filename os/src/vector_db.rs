//! Vector Database Service
//! Simple vector storage and similarity search for AI memory

extern crate libm;

pub const VECTOR_DIMENSION: usize = 128;
pub const MAX_VECTORS: usize = 1024;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Vector {
    pub id: u32,
    pub data: [f32; VECTOR_DIMENSION],
    pub metadata: [u8; 64],
}

impl Vector {
    pub fn new(id: u32) -> Self {
        Vector {
            id,
            data: [0.0; VECTOR_DIMENSION],
            metadata: [0; 64],
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct SearchResult {
    pub vector_id: u32,
    pub similarity: f32,
    pub metadata: [u8; 64],
}

static mut VECTOR_COUNT: u32 = 0;
static mut VECTORS: [Option<Vector>; MAX_VECTORS] = unsafe {
    [None; MAX_VECTORS]
};

pub fn init() {
}

pub fn insert(data: &[f32; VECTOR_DIMENSION], metadata: &[u8; 64]) -> Option<u32> {
    unsafe {
        if VECTOR_COUNT >= MAX_VECTORS as u32 {
            return None;
        }

        let id = VECTOR_COUNT;
        VECTOR_COUNT += 1;

        let mut vector = Vector::new(id);
        vector.data = *data;
        vector.metadata = *metadata;

        for i in 0..MAX_VECTORS {
            if VECTORS[i].is_none() {
                VECTORS[i] = Some(vector);
                break;
            }
        }

        Some(id)
    }
}

pub fn cosine_similarity(a: &[f32; VECTOR_DIMENSION], b: &[f32; VECTOR_DIMENSION]) -> f32 {
    let mut dot_product: f32 = 0.0;
    let mut norm_a: f32 = 0.0;
    let mut norm_b: f32 = 0.0;

    for i in 0..VECTOR_DIMENSION {
        dot_product += a[i] * b[i];
        norm_a += a[i] * a[i];
        norm_b += b[i] * b[i];
    }

    let sqrt_a = unsafe { libm::sqrtf(norm_a) };
    let sqrt_b = unsafe { libm::sqrtf(norm_b) };

    if sqrt_a > 0.0 && sqrt_b > 0.0 {
        dot_product / (sqrt_a * sqrt_b)
    } else {
        0.0
    }
}

pub fn get_vector(id: u32) -> Option<&'static Vector> {
    unsafe {
        for i in 0..MAX_VECTORS {
            if let Some(ref vector) = VECTORS[i] {
                if vector.id == id {
                    return Some(vector);
                }
            }
        }
    }
    None
}

pub fn delete(id: u32) -> bool {
    unsafe {
        for i in 0..MAX_VECTORS {
            if let Some(ref vector) = VECTORS[i] {
                if vector.id == id {
                    VECTORS[i] = None;
                    return true;
                }
            }
        }
    }
    false
}

pub fn get_count() -> u32 {
    unsafe { VECTOR_COUNT }
}

pub fn get_capacity() -> usize {
    MAX_VECTORS
}
