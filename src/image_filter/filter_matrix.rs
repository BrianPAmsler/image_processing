pub struct FilterMatrix {
    dim: usize,
    mat: Box<[f32]>
}

impl FilterMatrix {
    pub fn new<const N: usize>(matrix: [[f32; N]; N]) -> FilterMatrix {
        if N % 2 == 0 {
            panic!("FilterMatrix must have odd dimensions.");
        }

        FilterMatrix { dim: N, mat: matrix.into_iter().flat_map(|arr| arr.into_iter()).collect() }
    }

    pub fn get(&self, x: usize, y: usize) -> f32 {
        self.mat[x + y * self.dim]
    }
}