# IDS721 Weekly Rust Mini Project

## Week 2

A Basic Calculater that supports plus('+'), minus('-'), multiply('*') and division('/') (will round down to nearest integer). Unfortunately, it currently does not support computations with parentheses ('('). 

### Usage
> Run `cargo run -- basic-calculator -i [expr]` to get the value of the final result of expression `expr` after evaluation. `expr` is `String` type; e.g. the output of running command `cargo run -- basic-calculator -i "2*2+3*4"` is 
`16`, since $ 2 \times 2 + 3 \times 4 = 4 + 12 = 16$ by doing some quick math.

## Week 3

A Integer-to-Roman-Integer converter that takes in an Integer and return a Roman Integer (e.g. it will convert 19 to 'XIX')

### Usage
> Run `cargo run -- integer-to-roman -i [num]` to get the Roman Integer representation of the input number [num]. `num` is `i32` type; e.g. the output of running command `cargo run -- integer-to-roman -i 19` is `XIX`, and the output of running command `cargo run -- integer-to-roman -i 514` is `DXIV`. 