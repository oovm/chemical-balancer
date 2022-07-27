const T_START: f64 = 100000.0; //初始温度
const T_END: f64 = 1e-8; //终止温度
const T_DECAY: f64 = 0.98; //温度衰减率
use rand::{rngs::SmallRng, thread_rng, Rng, SeedableRng};

pub fn short_super_string(words: &[String]) -> String {
    let mut rng = SmallRng::from_rng(thread_rng()).unwrap();

    let n = words.len();

    let mut overlap = vec![vec![0; n]; n];
    let mut length = vec![0; n];
    let mut l = 0;
    for i in 0..n {
        length[i] = words[i].len();
        l += length[i];
    }
    for i in 0..n {
        for j in 0..n {
            if i == j {
                continue;
            }
            let mut s = String::new();
            s.push_str(&words[j]);
            s.push('A');
            s.push_str(&words[i]);
            overlap[i][j] = search_next(s.as_bytes(), length[i] + length[j] + 1);
        }
    }
    let mut t = T_START;
    let mut value = 0;
    let mut testlist = vec![0; n];
    let mut list = vec![0; n];
    let mut left = 0;
    let mut right = 0;

    for i in 0..n {
        list[i] = i;
        testlist[i] = i;
    }
    for i in 1..n {
        value += overlap[i - 1][i];
    }
    let P = 300;
    while t > T_END {
        for _ in 0..P {
            loop {
                left = rng.gen_range(0..n);
                right = rng.gen_range(0..n);
                if left != right {
                    break;
                }
            }
            testlist.swap(left, right);
            let mut newvalue = 0;
            for i in 1..n {
                newvalue += overlap[testlist[i - 1]][testlist[i]];
            }
            if newvalue > value {
                for i in 0..n {
                    list[i] = testlist[i];
                }
                value = newvalue;
            }
            else {
                let r = rng.gen::<f64>();
                if (-((value - newvalue) as f64) / t).exp() <= r {
                    testlist.swap(left, right);
                }
            }
        }
        t *= T_DECAY;
    }
    let mut res = String::new();
    res.push_str(&words[list[0]]);
    for i in 1..n {
        res.push_str(&words[list[i]][overlap[list[i - 1]][list[i]]..]);
    }
    res
}

fn search_next(s: &[u8], l: usize) -> usize {
    let mut next = vec![0; l];
    let mut i = 0;
    let mut j = 1;
    while j < l {
        if s[i] == s[j] {
            next[j] = i + 1;
            i += 1;
            j += 1;
        }
        else {
            if i == 0 {
                next[j] = 0;
                i = 0;
                j += 1;
            }
            else {
                i = next[i - 1];
            }
        }
    }
    next[l - 1]
}
