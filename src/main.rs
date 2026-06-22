// TODO:
// - output stuff to a file

use std::time::Instant;
use rayon::prelude::*;
use fastrand::Rng;

const LOOK_RANGE: i32 = 3;
const RULE_SIZE: usize = 2_i32.pow(2 * LOOK_RANGE as u32 + 1) as usize;
const WORLD_SIZE: usize = 149;

struct AlgoConfig {
    generations: u32,
    world_no: usize,
    rule_no: usize,
    keep: usize,
    max_iter: usize,
    mutations: u8,
    crossover: u8,
    debug: bool
}

impl Default for AlgoConfig {
    fn default() -> AlgoConfig {
        AlgoConfig {
            generations: 100,
            world_no: 100,
            rule_no: 100,
            keep: 20,
            max_iter: 2 * WORLD_SIZE,
            mutations: 2,
            crossover: 1,
            debug: true
        }
    }
}

fn uniform(arr: &mut [bool], rng: &mut Rng) {
    let p = rng.f32();
    for i in 0..arr.len() {
        arr[i] = rng.f32() < p;
    }
}

type Rule = [bool; RULE_SIZE];

fn rule_generate(rng: &mut Rng) -> Rule {
    let mut rule = [false; RULE_SIZE];
    uniform(&mut rule, rng);
    rule
}

fn rule_from_parents(p1: &Rule, p2: &Rule, mutations: u8, crossover: u8, rng: &mut Rng) -> Rule {
    let mut out = *p1;

    for _ in 0..crossover {
        let loc = rng.usize(..RULE_SIZE);
        out[loc] = p2[loc];
    }

    for _ in 0..mutations {
        let loc = rng.usize(..RULE_SIZE);
        out[loc] = !out[loc];
    }

    out
}

fn rule_output(r: &Rule) -> String {
    let hex = ['0','1','2','3','4','5','6','7','8','9','a','b','c','d','e','f'];
    let mut out = String::new();
    for i in (0..RULE_SIZE).step_by(4) {
        let mut digit = 0;
        for i in i..i+4 {
            digit <<= 1;
            if r[i] { digit += 1; }
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
        let mut data_copy = self.data;
        for i in 0..WORLD_SIZE {
            data_copy[i] = rule[self.neighbourhood(i as i32)];
        }
        self.data = data_copy;
    }

    fn result(&self, rule: &Rule, max_iter: usize) -> Option<bool> {
        let mut w = *self;

        for _ in 0..max_iter { w.apply_rule(rule); }

        let ans = w.data[0];
        if w.data.iter().all(|b| *b == ans) {
            Some(ans)
        } else {
            None
        }
    }
}


fn genetic_algo(cfg: &AlgoConfig, rng: &mut Rng) {
    let mut worlds: Vec<(bool, World)> = Vec::with_capacity(cfg.world_no);
    let mut rules: Vec<(usize, Rule)> = Vec::with_capacity(cfg.rule_no);
    for _ in 0..cfg.rule_no { rules.push((0, rule_generate(rng))); }

    for gen_n in 1..=cfg.generations {
        let time = Instant::now();

        worlds.clear();
        for _ in 0..cfg.world_no { worlds.push(World::generate(rng)); }

        for (score, rule) in &mut rules {
            *score = worlds.par_iter().filter(|(ans, world)| {
                let b = world.result(rule, cfg.max_iter);
                b.is_some() && b.unwrap() == *ans })
                .count();
        }

        rules.sort_by_key(|(score, _)| cfg.world_no - *score);
        
        let a = rules[0].0;
        let b = rules[cfg.keep - 1].0;

        for i in cfg.keep..cfg.rule_no {
            let p1_loc = rng.usize(..cfg.keep);
            let mut p2_loc = rng.usize(..cfg.keep);
            while p2_loc == p1_loc { p2_loc = rng.usize(..cfg.keep); }

            rules[i].1 = rule_from_parents(&rules[p1_loc].1, &rules[p2_loc].1, cfg.mutations, cfg.crossover, rng);
        }

        rules.iter_mut().for_each(|(score, _)| *score = 0);
        
        if cfg.debug {
            let toc = time.elapsed();
            let a_prop= a as f32 / cfg.world_no as f32 * 100.0;
            let b_prop= b as f32 / cfg.world_no as f32 * 100.0;
            println!("({:.2}s) {gen_n} top: {a_prop:.1}% - {b_prop:.1}% (best: {})",
                toc.as_secs_f32(), rule_output(&rules[0].1));
        }
    }
}

fn main() {
    let mut cfg = AlgoConfig::default();

    let args: Vec<String> = std::env::args().collect();

    macro_rules! cfg_field {
        ($field:ident, $i:ident) => { cfg.$field = args[$i].parse().expect("bad args") }
    }

    let mut i = 1;
    while i < args.len() {
        let opt = args[i].as_str();
        i += 1;
        match opt {
            "-g" | "--generations" => cfg_field!(generations, i),
            "-w" | "--worlds"      => cfg_field!(world_no, i),
            "-r" | "--rules"       => cfg_field!(rule_no, i),
            "-k" | "--keep"        => cfg_field!(keep, i),
            "-M" | "--max-iter"    => cfg_field!(max_iter, i),
            "-m" | "--mutations"   => cfg_field!(mutations, i),
            "-c" | "--crossover"   => cfg_field!(crossover, i),
            "-q" | "--quiet"       => { cfg.debug = false; i -= 1 },
            _ => panic!("bad args")
        }
        i += 1;
    }
    
    genetic_algo(&cfg, &mut Rng::new());
}
