use std::cmp;

// pub fn main() {
    // let n = 6; // the number of computers
    // let mut k = [2, 2, 3, 2, 2]; // the distance between computer i and computer i-1
    //
    // let mut dp = vec![0; n+1];
    // dp[2] = k[0];
    // dp[3] = k[1] + k[0];
    //
    // for i in 4..=n {
    //     dp[i] = cmp::min(dp[i-2] + k[i-2], dp[i-1] + k[i-2]);
    // }

    // println!("{}", dp[n]);
// }

pub fn main() {
    let n = 6;
    let a = [2, 2, 3, 2, 2];

    let mut dp = vec![vec![i64::MAX; 2]; n - 1];
    dp[0][1] = a[0];

    for i in 1..(n - 1) {
        dp[i][0] = dp[i - 1][1];
        dp[i][1] = dp[i - 1][0].min(dp[i - 1][1]) + a[i];
    }

    println!("{}", dp[n-2][1]);
}

