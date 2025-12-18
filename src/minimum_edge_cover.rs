pub fn main() {
    let n = 6;
    let mut k = [2, 2, 3, 2, 2].to_vec(); // the distances between i and i+1

    let mut dp = vec![i32::MAX; n];
    dp[1] = k[0];

    for i in 2..n {
        dp[i] = std::cmp::min(dp[i-2], dp[i-1]) + k[i-1];
    }

    println!("{}", dp[n-1]);
}
