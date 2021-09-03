# Data Matrix

Display data from STDIN in a matrix-style cascade!

## Installation

### Binary

Binaries are available from the [releases page](https://github.com/jsmailes/data-matrix/releases).
Currently only tested on Fedora, testing on other systems welcome!

### Manual

Requires rust and cargo, installation instructions can be found [here](https://www.rust-lang.org/tools/install).

Clone this repository:
```
git clone https://github.com/jsmailes/data-matrix.git
cd data-matrix
```

Build and install using cargo:
```
cargo install --path .
```

### Crate

TBD

## Usage

```
data-matrix [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -s, --speed <fps>                 Animation speed, in frames per second
    -t, --trail <length>              Maximum trail length
    -n, --num_inputs <num_inputs>     Maximum number of lines to process per frame
    -r, --randomness <probability>    Randomness of trails
```

Run a script and pipe it into `data-matrix`:
```
script | data-matrix
```