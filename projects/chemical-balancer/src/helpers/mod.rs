/// Cast isize matrix to f64 matrix
pub fn cast_isize_to_f64(v: Vec<Vec<isize>>) -> Vec<Vec<f64>> {
    v.iter().map(|v| v.iter().map(|&x| x as f64).collect()).collect()
}
