use std::io;

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();

    let input = if args.is_empty() {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("값을 입력하세요");
        input
    } else {
        args.join(" ")
    };

    let nums = Stats::parse_input(input.trim());
    let stats = Stats::new(&nums);

    println!(
        "mean={}, median={}, max={}, min={}",
        stats.mean, stats.median, stats.max, stats.min
    );
}

struct Stats {
    mean: f64,
    median: f64,
    max: f64,
    min: f64,
}

impl Stats {
    pub fn new(nums: &[f64]) -> Self {
        Self {
            mean: Self::get_mean(nums),
            median: Self::get_median(nums),
            max: Self::get_max(nums),
            min: Self::get_min(nums),
        }
    }
    pub fn parse_input(input: &str) -> Vec<f64> {
        input
            .split_ascii_whitespace()
            .map(|s| s.parse().expect("숫자를 입력하세요"))
            .collect()
    }

    fn get_mean(nums: &[f64]) -> f64 {
        if nums.is_empty() {
            panic!("숫자를 1개 이상 입력하세요");
        }
        nums.iter().sum::<f64>() / nums.len() as f64
    }

    fn get_max(nums: &[f64]) -> f64 {
        if nums.is_empty() {
            panic!("숫자를 1개 이상 입력하세요");
        }

        nums.iter()
            .max_by(|a, b| a.partial_cmp(b).unwrap())
            .copied()
            .unwrap()
    }

    fn get_min(nums: &[f64]) -> f64 {
        if nums.is_empty() {
            panic!("숫자를 1개 이상 입력하세요");
        }

        nums.iter()
            .min_by(|a, b| a.partial_cmp(b).unwrap())
            .copied()
            .unwrap()
    }

    fn get_median(nums: &[f64]) -> f64 {
        if nums.is_empty() {
            panic!("숫자를 1개 이상 입력하세요");
        }

        let mut sorted = nums.to_vec();
        sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());

        let length = sorted.len();

        if length % 2 == 0 {
            let left = sorted[(length / 2) - 1];
            let right = sorted[length / 2];
            (left + right) / 2.0
        } else {
            sorted[length / 2]
        }
    }
}
