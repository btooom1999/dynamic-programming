use std::cmp;

pub fn main() {
    let n = 6; // the number of computers
    let mut k = [2, 2, 3, 2, 2]; // the distance between computer i and computer i-1

    let mut dp = vec![0; n+1];
    dp[2] = k[0];
    dp[3] = k[1] + k[0];

    for i in 4..=n {
        dp[i] = cmp::min(dp[i-2] + k[i-2], dp[i-1] + k[i-2]);
    }

    println!("{}", dp[n]);
}
