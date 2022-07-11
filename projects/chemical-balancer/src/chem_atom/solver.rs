use super::*;

fn null_space(mut matrix: Vec<Vec<f64>>) -> Vec<Vec<f64>> {
    let n = matrix.len();
    let m = matrix[0].len();
    let mut null_basis = vec![];
    let mut pivot_cols = vec![];
    let mut row = 0;
    for col in 0..m {
        let mut pivot_row = None;
        for i in row..n {
            if matrix[i][col] != 0.0 {
                pivot_row = Some(i);
                break;
            }
        }
        if let Some(pivot_row) = pivot_row {
            if pivot_row != row {
                matrix.swap(pivot_row, row);
            }
            pivot_cols.push(col);
            for i in (row + 1)..n {
                let factor = matrix[i][col] / matrix[row][col];
                for j in col..m {
                    matrix[i][j] -= factor * matrix[row][j];
                }
            }
            row += 1;
        }
    }
    for col in 0..m {
        if !pivot_cols.contains(&col) {
            let mut null_vec = vec![0.0; m];
            null_vec[col] = 1.0;
            for (i, &pivot_col) in pivot_cols.iter().enumerate() {
                let factor = matrix[i][col];
                null_vec[pivot_col] = -factor;
            }
            null_basis.push(null_vec);
        }
    }
    null_basis
}


#[test]
fn test() {
    let out = null_space(vec![
        vec![1.0, 0.0, 0.0, -1.0],
        vec![0.0, 2.0, -1.0, 0.0],
        vec![-2.0, 0.0, 1.0, 0.0],
    ]);
    println!("{:#?}", out);
    assert_eq!(out, vec![vec![1.0, 1.0, 2.0, 1.0]])
}