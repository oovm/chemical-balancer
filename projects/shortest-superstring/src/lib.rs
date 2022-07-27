use std::{cmp::min, time::Instant};

fn common(x: &String, y: &String) -> i32 {
    let a = x.as_bytes();
    let b = y.as_bytes();
    let la = a.len();
    let lb = b.len();
    for k in (1..=la.min(lb)).rev() {
        if a[la - k..la] == b[0..k] {
            return k as i32;
        }
    }
    0
}

fn f(dp: &mut Vec<Vec<i32>>, a: &Vec<Vec<i32>>, i: usize, m: usize, n: usize) -> i32 {
    if m == 0 {
        return 0;
    }
    if dp[i][m] >= 0 {
        return dp[i][m];
    }
    let mut res = -1;
    for j in 0..n {
        let bit = 1 << j;
        if (m & bit) == 0 {
            continue;
        }
        let w = a[i][j] + f(dp, a, j, m - bit, n);
        if res < w {
            res = w;
        }
    }
    dp[i][m] = res;
    res
}

fn next(dp: &mut Vec<Vec<i32>>, a: &Vec<Vec<i32>>, i: usize, m: usize, n: usize) -> usize {
    let mut res = -1;
    let mut best = n;
    for j in 0..n {
        let bit = 1 << j;
        if (m & bit) == 0 {
            continue;
        }
        let w = a[i][j] + f(dp, a, j, m - bit, n);
        if res < w {
            res = w;
            best = j;
        }
    }
    best
}
pub fn shortest_super_string(words: &[String]) -> String {
    let n = words.len();
    let mut a = Vec::new();
    for i in 0..n {
        let mut b = Vec::new();
        for j in 0..n {
            b.push(if i == j { 0 } else { common(&words[i], &words[j]) });
        }
        a.push(b);
    }
    let bits = (1 << n) - 1;
    let mut dp = vec![vec![-1; 1 << n]; n];
    let mut best = n;
    let mut res = -1;
    for i in 0..n {
        let w = f(&mut dp, &a, i, bits - (1 << i), n);
        if res < w {
            res = w;
            best = i;
        }
    }
    let mut ans = words[best].clone();
    let mut m = bits - (1 << best);
    while m > 0 {
        let j = next(&mut dp, &a, best, m, n);
        let (_, t) = words[j].split_at(a[best][j] as usize);
        ans.push_str(t);
        best = j;
        m -= 1 << j;
    }
    ans
}
pub struct Solution;

impl Solution {
    pub fn shortest_superstring(words: &[String]) -> String {
        let n = words.len();
        let mut graph = vec![vec![0; n]; n];
        for i in 0..n {
            for j in 0..n {
                if i != j {
                    let mut k = words[j].len();
                    while !words[i].ends_with(&words[j][..k]) {
                        k -= 1;
                    }
                    graph[i][j] = k;
                }
            }
        }
        let mut dp = vec![vec![0; n]; 1 << n];
        let mut parent = vec![vec![None; n]; 1 << n];
        for mask in 0..1 << n {
            for b in 0..n {
                if mask >> b & 1 == 0 {
                    continue;
                }
                let prev = mask ^ 1 << b;
                for (i, row) in graph.iter().enumerate() {
                    if prev >> i & 1 == 0 {
                        continue;
                    }
                    let val = dp[prev][i] + row[b];
                    if val > dp[mask][b] {
                        dp[mask][b] = val;
                        parent[mask][b] = Some(i);
                    }
                }
            }
        }
        let mut mask = (1 << n) - 1;
        let mut perm = Vec::with_capacity(n);
        let mut seen = vec![false; n];
        if let Some(mut p) = (0..n).max_by_key(|&i| dp[(1 << n) - 1][i]) {
            loop {
                perm.push(p);
                seen[p] = true;
                if let Some(t) = parent[mask][p] {
                    mask ^= 1 << p;
                    p = t;
                }
                else {
                    break;
                }
            }
        }
        perm.extend((0..n).filter(|&i| !seen[i]));
        (1..perm.len()).rev().fold(words[perm[n - 1]].clone(), |acc, i| {
            let overlap = graph[perm[i]][perm[i - 1]];
            acc + &words[perm[i - 1]][overlap..]
        })
    }
}

// 0120118011701160115011401130112011101101090108010701060105010401030102010119

pub fn days_in_year(leap_year: bool) -> Vec<String> {
    let days_in_month = vec![31, if leap_year { 29 } else { 28 }, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
    let mut days = Vec::new();
    for (month, &days_in_month) in days_in_month.iter().enumerate() {
        for day in 1..=days_in_month {
            days.push(format!("{:02}{:02}", month + 1, day));
        }
    }
    days
}
