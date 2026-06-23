# cell-auto-evo
Genetic algorithm that evolves a binary cellular automaton to distinguish densities. See https://melaniemitchell.me/PapersContent/evca-review.pdf.

## Building
Use `cargo`.

## Usage
```
$ cargo run --release [options]
```
or
```
$ cargo build --release
$ ./target/release/cellular [options]
```

| Option | Long Option   | Values           | Description                                                                    | Default            |
| :---   | :---          | :---             | :---                                                                           | :---               |
| -G | --gather <n>      | n >= 1           | Number of times to run the genetic algorithm to produce the final set of rules | 300                |
| -t | --tests <n>       | n >= 1           | Number of times to test each of the final set of rules                         | 10000              |
| -d | --display <n>     | 1 <= n <= gather | Number of rules to display at the end                                          | 10                 |
| -s | --seed <n>        | Valid `u64`      | Seed for the RNG                                                               | Randomly generated |
| -g | --generations <n> | n >= 1           | Generations per run of the genetic algorithm                                   | 100                |
| -w | --worlds <n>      | n >= 1           | Number of initial conditions per generation                                    | 100                |
| -r | --rules <n>       | n >= 1           | Number of rules present per run of the genetic algorithm                       | 100                |
| -k | --keep <n>        | 1 <= n <= rules  | Number of rules to keep between generations                                    | 20                 |
| -M | --max-iter <n>    | n >= 1           | Maximum times a rule can be applied before it is counted as a fail             | 298                |
| -m | --mutations <n>   | 0 <= n           | Number of mutations to add when generating new rules via "breeding"            | 2                  |
| -c | --crossover <n>   | 0 <= n           | Number of crossovers to do when "breeding" two existing rules                  | 1                  |
| -v | --verbosity <n>   | 0 <= n <= 3      | Verbosity of output                                                            | 0                  |

**Note:** The size of each world (149 cells) and the lookaround range (3) are hard-coded as constants for optimisation reasons. They can be edited in `src/main.rs`.
