use std::{io::{stdout, Write}, time::Instant};
use rayon::prelude::*;
use fastrand::Rng;

const LOOK_RANGE: i32 = 3;
const RULE_SIZE: usize = 2_i32.pow(2 * LOOK_RANGE as u32 + 1) as usize;
const WORLD_SIZE: usize = 149;

#[derive(Clone, Copy)]
pub struct AlgoConfig {
    pub generations: u32,
    pub world_no: usize,
    pub rule_no: usize,
    pub keep: usize,
    pub reroll: usize,
    pub max_iter: usize,
    pub mutations: u8,
    pub crossover: u8,
    pub verbosity: u8
}

impl AlgoConfig {
    pub const DEFAULT: AlgoConfig = AlgoConfig {
        generations: 100,
        world_no: 100,
        rule_no: 100,
        keep: 20,
        reroll: 10,
        max_iter: 2 * WORLD_SIZE,
        mutations: 4,
        crossover: 1,
        verbosity: 0
    };
}

fn uniform(arr: &mut [bool], rng: &mut Rng) {
    let p = rng.f32_inclusive();
    arr.iter_mut().for_each(|b| *b = rng.f32_inclusive() < p);
}

type Rule = [bool; RULE_SIZE];

fn rule_generate(rng: &mut Rng) -> Rule {
    let mut rule = [false; RULE_SIZE];
    uniform(&mut rule, rng);
    rule
}

fn rule_output(r: &Rule) -> String {
    let hex = ['0','1','2','3','4','5','6','7','8','9','a','b','c','d','e','f'];
    let mut out = String::new();
    for i in (0..RULE_SIZE).step_by(4) {
        let mut digit = 0;
        for b in r.iter().skip(i).take(4) {
            digit <<= 1;
            if *b { digit += 1; }
        }
        out.push(hex[digit]);
    }

    out
}

#[derive(Clone, Copy)]
struct World { data: [bool; WORLD_SIZE] }

impl World {
    fn new() -> World {
        World { data: [false; WORLD_SIZE] }
    }

    fn generate(rng: &mut Rng) -> (bool, World) {
        let mut w = World::new();
        uniform(&mut w.data, rng);
        let num_t = w.data.iter().filter(|b| **b).count();
        (num_t > WORLD_SIZE / 2, w)
    }

    fn get(&self, index: i32) -> bool {
        let ws_i32 = WORLD_SIZE as i32;
        if index < 0 {
            self.data[(index + ws_i32) as usize]
        } else if index >= ws_i32 {
            self.data[(index - ws_i32) as usize]
        } else {
            self.data[index as usize]
        }
    }

    fn neighbourhood(&self, index: i32) -> usize {
        let mut out = 0;
        for i in (index-LOOK_RANGE)..=(index+LOOK_RANGE) {
            out <<= 1;
            if self.get(i) { out += 1; }
        }
        out
    }
    
    fn apply_rule(&mut self, rule: &Rule) {
        let mut data = [false; WORLD_SIZE];
        data.iter_mut().enumerate().for_each(|(i, b)|
            *b = rule[self.neighbourhood(i as i32)]);
        self.data = data;
    }

    fn result(&self, rule: &Rule, max_iter: usize) -> Option<bool> {
        let mut w = *self;
        let mut prev;

        for _ in 0..max_iter {
            prev = w.data;
            w.apply_rule(rule);
            if w.data.iter().zip(prev.iter()).all(|(a, b)| *a == *b) {
                break;
            }
        }

        let ans = w.data[0];
        if w.data.iter().all(|b| *b == ans) {
            Some(ans)
        } else {
            None
        }
    }
}

fn genetic_algo(cfg: &AlgoConfig, rng: &mut Rng) -> (usize, Rule) {
    let mut rules: Vec<(usize, Rule)> = Vec::with_capacity(cfg.rule_no);
    for _ in 0..cfg.rule_no { rules.push((0, rule_generate(rng))); }

    let mut best = (0, [false; RULE_SIZE]);
    for gen_n in 1..=cfg.generations {
        let timer = Instant::now();

        for _ in 0..cfg.world_no {
            let (ans, w) = World::generate(rng);
            rules.par_iter_mut().for_each(|(score, rule)| {
                let res = w.result(rule, cfg.max_iter);
                if let Some(b) = res && ans == b { *score += 1; }});
        }

        rules.par_sort_by_key(|(score, _)| cfg.world_no - *score);
        
        best = rules[0];
        let a = rules[0].0;
        let b = rules[cfg.keep - 1].0;

        for i in cfg.keep..cfg.rule_no - cfg.reroll {
            let mut r = rules[rng.usize(..cfg.keep)].1;
            let p2 = &rules[rng.usize(..cfg.keep)].1;

            for _ in 0..cfg.crossover {
                let loc = rng.usize(..RULE_SIZE);
                r[loc] = p2[loc];
            }

            for _ in 0..cfg.mutations {
                let loc = rng.usize(..RULE_SIZE);
                r[loc] = !r[loc];
            }

            rules[i].1 = r;
        }

        rules[cfg.rule_no-cfg.reroll..].iter_mut().for_each(|(_, r)| *r = rule_generate(rng));

        rules.iter_mut().for_each(|(score, _)| *score = 0);
        
        if cfg.verbosity >= 2 {
            let time = timer.elapsed().as_secs_f32();
            let a_prop= a as f32 / cfg.world_no as f32 * 100.0;
            let b_prop= b as f32 / cfg.world_no as f32 * 100.0;
            print!("{gen_n:>3} | top: [{a_prop:.1}%-{b_prop:.1}%]");
            if cfg.verbosity >= 3 { print!(", best: {}", rule_output(&best.1)) }
            println!(" ({time:.2}s)");
        } else if cfg.verbosity == 1 {
            print!("\x1b[4D\x1b[K");
            print!("{:>3}%", gen_n * 100 / cfg.generations);
            let _ = stdout().flush();
        }
    }
    
    best
}

pub fn run(gather: usize, tests: usize, display: usize, cfg: &AlgoConfig, rng: &mut Rng) {
    let mut rules: Vec<(usize, Rule)> = Vec::with_capacity(gather);
    for i in 1..=gather {
        if cfg.verbosity >= 1 {
            print!("\r\x1b[2K");
            print!("gather {i}/{gather}:");
            if cfg.verbosity == 1 {
                print!("   0%");
                let _ = stdout().flush();
            } else {
                println!();
            }
        }
        rules.push(genetic_algo(cfg, rng));
    }

    print!("\r\x1b[2K");

    for i in 1..=tests {
        if cfg.verbosity >= 1 {
            print!("\r\x1b[2K");
            print!("testing {:>3}%", i * 100 / tests);
            let _ = stdout().flush();
        }
        let (ans, w) = World::generate(rng);
        rules.par_iter_mut().for_each(|(score, rule)| {
            let res = w.result(rule, cfg.max_iter);
            if let Some(b) = res && ans == b { *score += 1; }});
    }

    if cfg.verbosity == 1 {
        print!("\r\x1b[2K");
        let _ = stdout().flush();
    } else {
        println!();
    }
    
    rules.sort_by_key(|r| tests - r.0);
    for (i, (score, r)) in rules[..display].iter().enumerate() {
        println!("{:>2}. ({:.1}%) {}", i+1, *score as f32 / tests as f32 * 100.0, rule_output(r));
    }
}
