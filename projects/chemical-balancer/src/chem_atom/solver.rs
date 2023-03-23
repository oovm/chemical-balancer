use super::*;

impl ChemicalBalancer {
    pub fn solve(&self) -> Vec<Vec<f64>> {
        let mut matrix = Vec::new();
        for i in &self.lhs {
            matrix.push(i.count_elements(&self.elements));
        }
        for i in &self.rhs {
            matrix.push(i.count_elements(&self.elements));
        }
        // transpose matrix
        null_space(transpose(matrix))
    }
    pub fn matrix(&self) -> Vec<Vec<f64>> {
        let mut matrix = Vec::new();
        for i in &self.lhs {
            matrix.push(i.count_elements(&self.elements));
        }
        for i in &self.rhs {
            matrix.push(i.count_elements(&self.elements));
        }
        matrix
    }

    pub fn solve_integers(&self) -> Vec<Vec<isize>> {
        let a = arr2(&[[1., 2., 3.], [4., 5., 6.]]);
        let (_, _, v_t) = a.svd(true, true).unwrap();
        let null_space = v_t.rows(2..).into_owned();
        println!("{:?}", null_space);
    }
}

fn transpose(matrix: Vec<Vec<f64>>) -> Vec<Vec<f64>> {
    let mut result = vec![vec![0.0; matrix.len()]; matrix[0].len()];
    for i in 0..matrix.len() {
        for j in 0..matrix[0].len() {
            result[j][i] = matrix[i][j];
        }
    }
    result
}

// Find the least common multiple that can be reduced to an integer
// eg. [1.0, 1.5, 1.0] => [2,3,2]
fn lcm_ints(input: &[f64], scale: f64) -> Vec<isize> {
    let denominators = input.iter().map(|&x| (x * scale) as isize).collect::<Vec<_>>();
    let lcm = lcm(&denominators);
    denominators.iter().map(|&x| lcm / x).collect()
}

fn lcm(input: &[isize]) -> isize {
    use num::integer::lcm;
    input.iter().fold(input[0], |a, &b| lcm(a, b))
}

fn null_space(mut matrix: Vec<Vec<f64>>) -> Vec<Vec<f64>> {
    let mut null_space = Vec::new();
    let mut row = 0;
    let mut col = 0;
    let mut rank = 0;
    let rows = matrix.len();
    let cols = matrix[0].len();
    while row < rows && col < cols {
        let mut pivot = row;
        for i in row..rows {
            if matrix[i][col].abs() > matrix[pivot][col].abs() {
                pivot = i;
            }
        }
        if matrix[pivot][col].abs() < 1e-10 {
            col += 1;
            continue;
        }
        matrix.swap(row, pivot);
        let scale = 1.0 / matrix[row][col];
        for j in col..cols {
            matrix[row][j] *= scale;
        }
        for i in 0..rows {
            if i != row {
                let scale = matrix[i][col];
                for j in col..cols {
                    matrix[i][j] -= scale * matrix[row][j];
                }
            }
        }
        row += 1;
        col += 1;
        rank += 1;
    }
    for i in rank..cols {
        let mut null_vector = vec![0.0; cols];
        null_vector[i] = 1.0;
        for j in 0..rank {
            null_vector[j] = -matrix[j][i];
        }
        null_space.push(null_vector);
    }
    keep_first_positive(&mut null_space);
    null_space
}

fn keep_first_positive(matrix: &mut Vec<Vec<f64>>) {
    for i in 0..matrix.len() {
        for j in 0..matrix[i].len() {
            if matrix[i][j] < 0.0 {
                matrix[i][j] = -matrix[i][j];
            }
        }
    }
}

#[test]
fn test() {
    let out = null_space(vec![vec![1.0, 0.0, 0.0, -1.0], vec![0.0, 2.0, -1.0, 0.0], vec![-2.0, 0.0, 1.0, 0.0]]);
    // to wolfram
    // NullSpace[{{1, 0, 0, -1}, {0, 2, -1, 0}, {-2, 0, 1, 0}}]

    // [1.0, 1.5, 1.0]

    println!("{:#?}", out);
    assert_eq!(out, vec![vec![1.0, 1.0, 2.0, 1.0]])
}
