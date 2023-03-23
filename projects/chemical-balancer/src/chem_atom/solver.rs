use super::*;

fn null_space(input: &[Vec<f64>]) -> Result<Vec<Vec<f64>>, String> {
    let n = input[0].len();
    let m = input.len();

    let mut rref = input.to_vec();
    let mut lead = 0;

    for row in 0..m {
        if n <= lead {
            break;
        }
        let mut i = row;
        while rref[i][lead] == 0.0 {
            i += 1;
            if m == i {
                i = row;
                lead += 1;
                if n == lead {
                    break;
                }
            }
        }
        if n == lead {
            break;
        }

        rref.swap(row, i);

        let lv = rref[row][lead];
        rref[row] = rref[row].iter().map(|&x| x / lv).collect();

        for i in 0..m {
            if i != row {
                let lv = rref[i][lead];
                rref[i] = rref[i]
                    .iter()
                    .zip(rref[row].iter())
                    .map(|(&x, &y)| x - y * lv)
                    .collect();
            }
        }
        lead += 1;
    }

    let mut null_space_basis = Vec::new();
    for i in 0..n {
        let mut is_zero_column = true;
        let mut basis_vector = vec![0.0; n];
        for j in 0..m {
            if rref[j][i] != 0.0 {
                is_zero_column = false;
                basis_vector[j] = -rref[j][i];
                break;
            }
        }
        if is_zero_column {
            basis_vector[i] = 1.0;
        }
        null_space_basis.push(basis_vector);
    }

    Ok(null_space_basis)
}

#[test]
fn test() {
    let out = null_space(&[
        vec![1.0, 0.0, 0.0, -1.0],
        vec![0.0, 2.0, -1.0,0.0],
        vec![-2.0,0.0,1.0,0.0],
    ]);
    println!("{:#?}", out)
}