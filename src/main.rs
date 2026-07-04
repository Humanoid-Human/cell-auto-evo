// TODO:
// - output stuff to a file

use std::time::Instant;

mod algo;
use algo::AlgoConfig;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let mut cfg = AlgoConfig::DEFAULT;
    let mut gather = 300;
    let mut tests = 100000;
    let mut display = 10;
    let mut seed = fastrand::u64(0..u64::MAX);

    let mut i = 1;
    
    macro_rules! opt {
        ($field:expr) => { $field = args[i].parse().expect("bad args") }
    }

    while i < args.len() {
        let opt = args[i].as_str();
        i += 1;
        match opt {
            "-G" | "--gather"      => opt!(gather),
            "-t" | "--tests"       => opt!(tests),
            "-d" | "--display"     => opt!(display),
            "-s" | "--seed"        => opt!(seed),
            "-g" | "--generations" => opt!(cfg.generations),
            "-w" | "--worlds"      => opt!(cfg.world_no),
            "-r" | "--rules"       => opt!(cfg.rule_no),
            "-k" | "--keep"        => opt!(cfg.keep),
            "-M" | "--max-iter"    => opt!(cfg.max_iter),
            "-m" | "--mutations"   => opt!(cfg.mutations),
            "-c" | "--crossover"   => opt!(cfg.crossover),
            "-v" | "--verbosity"   => opt!(cfg.verbosity),
            _ => panic!("bad args")
        }
        i += 1;
    }

    if cfg.verbosity >= 1 {
        let timer = Instant::now();
        algo::run(gather, tests, display, &cfg, &mut fastrand::Rng::with_seed(seed));
        println!("Completed in {:.2}s", timer.elapsed().as_secs_f32());
    } else {
        algo::run(gather, tests, display, &cfg, &mut fastrand::Rng::with_seed(seed));
    }
}
