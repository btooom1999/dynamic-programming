use std::cmp;

pub fn main() {
    let n = 5;
    let k = 7;
    let matrix = [
        [9, -2, 6, 2, 1, 3, 4],
        [0, -1, 6, 7, 1, 3, 3],
        [8, -2, 8, 2, 5, 3, 2],
        [1, -1, 6, 2, 1, 6, 1],
        [7, 2, 6, 2, 1, 3, 7]
    ].to_vec();

    let mut results = vec![vec![i32::MIN; k]; n];

    let mut max = i32::MIN;

    for i in 0..n {
        results[i][0] = matrix[i][0];
    }

    for j in 1..k {
        for i in 0..n {
            let prev_val = results[i][j-1];

            let ai = (cmp::max(i.saturating_sub(1), 0), j);
            results[ai.0][ai.1] = cmp::max(results[ai.0][ai.1], prev_val + matrix[ai.0][ai.1]);

            let mi = (i, j);
            results[mi.0][mi.1] = cmp::max(results[mi.0][mi.1], prev_val + matrix[mi.0][mi.1]);
            

            let bi = (cmp::min(i.saturating_add(1), n - 1), j);                
            results[bi.0][bi.1] = cmp::max(results[bi.0][bi.1], prev_val + matrix[bi.0][bi.1]);

            max = cmp::max(max, results[i][j]);
        }
    }

    for item in results.iter() {
        println!("{:?}", item);
    }

    println!("maximum path sum is: {}", max);
}
