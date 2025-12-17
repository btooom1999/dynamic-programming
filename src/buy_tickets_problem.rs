use std::cmp;

pub fn main() {
    let n = 5;
    let t = [2, 5, 7, 8, 4];
    let mut r = [4, 9, 10, 10].to_vec();
    r.insert(0, 0);

    let mut dp = vec![0; n+1];

    dp[1] = t[0];
    for i in 2..=n {
        dp[i] = cmp::min(dp[i-1] + t[i-1], dp[i-2] + r[i-1]);
    }

    println!("{:?}", dp[n]);
}
