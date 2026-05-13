use std::env;
use std::hint::black_box;
use std::thread;
use std::time::{Duration, Instant};

#[derive(Debug, Clone)]
struct Config {
    workers: usize,
    tickers: usize,
    rounds: usize,
    compute_iters: u64,
    ticker_period_us: u64,
    ticker_work_iters: u64,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            workers: 4,
            tickers: 2,
            rounds: 50,
            compute_iters: 80_000_000,
            ticker_period_us: 2_000,
            ticker_work_iters: 200_000,
        }
    }
}

fn parse_args() -> Config {
    let mut cfg = Config::default();
    let args: Vec<String> = env::args().collect();

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--workers" => {
                i += 1;
                cfg.workers = args[i].parse().unwrap();
            }
            "--tickers" => {
                i += 1;
                cfg.tickers = args[i].parse().unwrap();
            }
            "--rounds" => {
                i += 1;
                cfg.rounds = args[i].parse().unwrap();
            }
            "--compute-iters" => {
                i += 1;
                cfg.compute_iters = args[i].parse().unwrap();
            }
            "--ticker-period-us" => {
                i += 1;
                cfg.ticker_period_us = args[i].parse().unwrap();
            }
            "--ticker-work-iters" => {
                i += 1;
                cfg.ticker_work_iters = args[i].parse().unwrap();
            }
            other => {
                eprintln!("unknown arg: {}", other);
                std::process::exit(2);
            }
        }
        i += 1;
    }
    cfg
}

fn cpu_burn(iters: u64) -> u64 {
    let mut x = 0x1234_5678_9abc_def0u64;
    let mut acc = 0u64;
    for _ in 0..iters {
        x = x
            .wrapping_mul(2862933555777941757)
            .wrapping_add(3037000493);
        acc ^= x.rotate_left(13);
        acc = acc.wrapping_mul(0x9E3779B185EBCA87);
        black_box(acc);
    }
    acc
}

fn percentile(sorted: &[f64], p: f64) -> f64 {
    if sorted.is_empty() {
        return 0.0;
    }
    let idx = ((sorted.len() - 1) as f64 * p).round() as usize;
    sorted[idx]
}

fn main() {
    let cfg = parse_args();
    eprintln!("config = {:?}", cfg);

    let global_start = Instant::now();
    let mut round_ms = Vec::with_capacity(cfg.rounds);

    for round in 0..cfg.rounds {
        let round_start = Instant::now();

        let mut handles = Vec::new();

        for _ in 0..cfg.workers {
            let iters = cfg.compute_iters;
            handles.push(thread::spawn(move || cpu_burn(iters)));
        }

        for _ in 0..cfg.tickers {
            let period = Duration::from_micros(cfg.ticker_period_us);
            let work_iters = cfg.ticker_work_iters;
            handles.push(thread::spawn(move || {
                let start = Instant::now();
                let mut v = 0u64;
                while start.elapsed() < Duration::from_millis(200) {
                    thread::sleep(period);
                    v ^= cpu_burn(work_iters);
                }
                v
            }));
        }

        let mut checksum = 0u64;
        for h in handles {
            checksum ^= h.join().unwrap();
        }
        black_box(checksum);

        let elapsed = round_start.elapsed().as_secs_f64() * 1000.0;
        round_ms.push(elapsed);
        println!("round={round} elapsed_ms={elapsed:.3}");
    }

    let total_ms = global_start.elapsed().as_secs_f64() * 1000.0;
    round_ms.sort_by(|a, b| a.partial_cmp(b).unwrap());

    println!("--- summary ---");
    println!("rounds={}", cfg.rounds);
    println!("total_ms={:.3}", total_ms);
    println!("p50_ms={:.3}", percentile(&round_ms, 0.50));
    println!("p95_ms={:.3}", percentile(&round_ms, 0.95));
    println!("p99_ms={:.3}", percentile(&round_ms, 0.99));
    println!("max_ms={:.3}", round_ms.last().copied().unwrap_or(0.0));
}