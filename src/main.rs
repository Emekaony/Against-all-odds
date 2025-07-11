#[allow(unused)]
fn main() {
    let mat1 = vec![
        vec![1, 2, 3, 4],
        vec![1, 2, 3, 4],
        vec![1, 2, 3, 4],
        vec![1, 2, 3, 4],
    ];
    let mat2 = vec![vec![8, 2, 3], vec![1, 2, 3], vec![1, 2, 3]];
    let mat3 = vec![
        vec![1, 2, 3, 4, 9],
        vec![1, 2, 3, 4, 0],
        vec![1, 2, 3, 4, 4],
        vec![1, 2, 3, 4, 9],
        vec![1, 2, 3, 4, 2],
    ];
    let mat4 = vec![vec![9, 4], vec![1, -2]];
    println!("{}", matrix_diag_sum(&mat4));

    use_foo();
}

fn matrix_diag_sum(mat: &Vec<Vec<i32>>) -> i32 {
    // let tt = &mat[1];
    let mut sum: i32 = 0;
    // square matrix
    for i in 0..mat.len() {
        sum += mat[i][i];
    }
    sum
}

fn use_foo() {
    let mut vec = vec![vec![1, 2, 3], vec![1, 2, 3]];
    foo(&mut vec);
    println!("{:?}", vec);
}

fn foo(a: &mut Vec<Vec<i32>>) {
    let tt = &mut a[0];
    tt[0] = 22;
}
