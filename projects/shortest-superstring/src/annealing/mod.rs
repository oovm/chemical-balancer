use rand::{rngs::SmallRng, thread_rng, Rng, SeedableRng};
use serde::{Deserialize, Serialize};
use serde_json::{ser::PrettyFormatter, Error, Serializer};
use std::{io::Read, path::Path};

const GROUP_STEPS: usize = 256;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SimulatedAnnealing {
    /// 初始温度
    t: f64,
    /// 终止温度
    t_end: f64,
    // 温度衰减率
    t_decay: f64,
    steps: usize,
    words: Vec<String>,
    best: Vec<usize>,
    rng: SmallRng,
}

impl SimulatedAnnealing {
    pub fn new(words: Vec<String>) -> Self {
        Self {
            t: 1e8,
            t_end: 1e-8,
            t_decay: 0.98,
            steps: 300,
            words: words.to_vec(),
            best: vec![],
            rng: SmallRng::from_rng(thread_rng()).unwrap(),
        }
    }
    pub fn with_template(mut self, start: f64, end: f64, decay: f64) -> Self {
        self.t = start;
        self.t_end = end;
        self.t_decay = decay;
        self
    }
    pub fn save_checkpoints<P>(&self, checkpoints: P) -> Result<(), Error>
    where
        P: AsRef<Path>,
    {
        let file = std::fs::File::create(checkpoints).unwrap();
        let mut ser = Serializer::with_formatter(file, PrettyFormatter::with_indent(b"    "));
        self.serialize(&mut ser)
    }

    pub fn load_checkpoints<P>(mut self, checkpoints: P) -> Result<Self, Error>
    where
        P: AsRef<Path>,
    {
        let mut file = std::fs::File::open(checkpoints).unwrap();
        let mut content = String::new();
        file.read_to_string(&mut content).unwrap();
        let mut sa: Self = serde_json::from_str(&content).unwrap();
        sa.rng = SmallRng::from_rng(thread_rng()).unwrap();
        sa
    }
    pub fn load_or_restart<P>(problem: Vec<String>, checkpoints: P) -> Result<Self, Error>
    where
        P: AsRef<Path>,
    {
        if checkpoints.as_ref().exists() { Self::load_checkpoints(checkpoints) } else { Ok(Self::new(problem)) }
    }

    pub fn short_super_string(&mut self) -> String {
        let n = self.words.len();
        let mut overlap = vec![vec![0; n]; n];
        let mut length = vec![0; n];
        let mut l = 0;
        for i in 0..n {
            length[i] = self.words[i].len();
            l += length[i];
        }
        for i in 0..n {
            for j in 0..n {
                if i == j {
                    continue;
                }
                let mut s = String::new();
                s.push_str(&self.words[j]);
                s.push('A');
                s.push_str(&self.words[i]);
                overlap[i][j] = search_next(s.as_bytes(), length[i] + length[j] + 1);
            }
        }
        let mut t = self.t;
        let mut value = 0;
        let mut test_list = vec![0; n];
        let mut best_list = vec![0; n];

        for i in 0..n {
            best_list[i] = i;
            test_list[i] = i;
        }
        for i in 1..n {
            value += overlap[i - 1][i];
        }
        while t > self.t_end {
            for _ in 0..GROUP_STEPS {
                self.random_swap(&mut test_list);
                let mut newvalue = 0;
                for i in 1..n {
                    newvalue += overlap[test_list[i - 1]][test_list[i]];
                }
                if newvalue > value {
                    for i in 0..n {
                        best_list[i] = test_list[i];
                    }
                    value = newvalue;
                }
                else {
                    let r = self.rng.gen::<f64>();
                    if (-((value - newvalue) as f64) / t).exp() <= r {
                        self.random_swap(&mut test_list);
                    }
                }
            }
            t *= self.t_decay;
        }
        let mut answer = String::new();
        answer.push_str(&self.words[best_list[0]]);
        for i in 1..n {
            answer.push_str(&self.words[best_list[i]][overlap[best_list[i - 1]][best_list[i]]..]);
        }
        answer
    }
    fn random_swap(&mut self, tests: &mut Vec<usize>) {
        let n = self.words.len();
        loop {
            let left = self.rng.gen_range(0..n);
            let right = self.rng.gen_range(0..n);
            if left != right {
                tests.swap(left, right);
                self.steps += 1;
                break;
            }
        }
    }
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
