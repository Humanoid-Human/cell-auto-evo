# cell-auto-evo
Genetic algorithm that evolves a binary cellular automaton to distinguish densities. See https://melaniemitchell.me/PapersContent/evca-review.pdf.

## Building
1. Clone this repository
2. Build with `cargo build -r` (see the [`cargo` docs](https://doc.rust-lang.org/cargo/) for installation and usage)

## Usage
```
$ cargo run -r -- [options]
```
or (Unix only)
```
$ cargo build -r
$ ./target/release/cellular [options]
```
or (Windows only)
```
$ cargo build -r
$ start ./target/release/cellular [options]
```

### Options

**Note:** All options take an integer value, i.e. `--gather 300`.
| Option | Long Option   | Values           | Description                                                                    | Default            |
| :---   | :---          | :---                | :---                                                                           | :---               |
| `-G` | `--gather`      | `n` >= 1             | Number of times to run the genetic algorithm to produce the final set of rules | 300                |
| `-t` | `--tests`       | `n` >= 1             | Number of times to test each of the final set of rules                         | 10000              |
| `-d` | `--display`     | 1 <= `n` <= `gather` | Number of rules to display at the end                                          | 10                 |
| `-s` | `--seed`        | Valid 64-bit int    | Seed for the RNG                                                               | Randomly generated |
| `-g` | `--generations` | `n` >= 1             | Generations per run of the genetic algorithm                                   | 100                |
| `-w` | `--worlds`      | `n` >= 1             | Number of initial conditions per generation                                    | 100                |
| `-r` | `--rules`       | `n` >= 1             | Number of rules present per run of the genetic algorithm                       | 100                |
| `-k` | `--keep`        | 1 <= `n` <= `rules`  | Number of rules to keep between generations                                    | 20                 |
| `-M` | `--max-iter`    | `n` >= 1             | Maximum times a rule can be applied before it is counted as a fail             | 298                |
| `-m` | `--mutations`   | 0 <= `n`            | Number of mutations to add when generating new rules via "breeding"            | 2                  |
| `-c` | `--crossover`   | 0 <= `n`            | Number of crossovers to do when "breeding" two existing rules                  | 1                  |
| `-v` | `--verbosity`   | 0 <= `n` <= 3        | Verbosity of output                                                            | 0                  |

**Note:** The size of each world (149 cells) and the lookaround range (3 cells) are hard-coded as constants for optimisation reasons. They can be edited in `src/algo.rs`.
