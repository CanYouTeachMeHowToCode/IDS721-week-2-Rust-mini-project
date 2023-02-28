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

## Week 4

A longest palidromic substring finder that takes in an string and returns its longest substring such that the substring is also a palindrome. 

### Usage
> Run `cargo run -- longest-parlindromic-substring -i [s]` to get the longest palidromic substring of the input string [s]. `s` is `String` type; e.g. the output of running command `cargo run -- longest-parlindromic-substring -i "babadba"` is `bab`, and the output of running command `cargo run -- longest-parlindromic-substring -i "asdwfesddsfsdsdsdsdsdsgvqwerwqsdfsadas"` is `sdsdsdsdsds`. 

## Week 5

An IP address restorer that takes in a string and return all possible valid IP addresses (in standard format, e.g. 245.255.255.255)

### Usage
> Run `cargo run -- restore-ip-addresses -i [s]` to get the all possible restored IP addresses of the input string [s]. `s` is `String` type; e.g. the output of running command `cargo run -- restore-ip-addresses -i "25525511135"` is 
```
[
    "255.255.11.135",
    "255.255.111.35",
]
```
> , and the output of running command `cargo run -- restore-ip-addresses -i "1010238"` is 
```
[
    "1.0.10.238",
    "1.0.102.38",
    "10.102.3.8",
    "10.10.2.38",
    "10.10.23.8",
    "10.1.0.238",
    "101.0.2.38",
    "101.0.23.8",
]
```
> ; if the input string is unable to form any valid IP addresses, it will return an empty list `[]`.

## Week 6

An Excel sheet column title converter that takes in a column title in string format and return its corresponding column number (e.g. "A" -> 1, "C" -> 3, "Z" -> 26, "AA" -> 27, "ASD" -> 1174)

### Usage
> Run `cargo run -- convert-title-to-num -i [title]` to get the  corresponding column number for input title string `title`. `title` is `String` type; e.g. the output of running command `cargo run -- convert-title-to-num -i "AA"` is `27` and the output of running command `cargo run -- convert-title-to-num -i "ASD"` is `1174`

> Note that the input string `title` will be automatically converted to upper case in order to align with the Excel column title format. For instance, the outputs of running command `cargo run -- convert-title-to-num -i "ASD"` and `cargo run -- convert-title-to-num -i "asd"` are both `1174`.