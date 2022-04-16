# simon-bf-bench-util

Program requested by [Simon From Jakobsen](https://github.com/simonfj20), which gets the delta between two inputs and collects it into their chosen index.

I don't know the specifics of what this is used for, other than it is used for benchmarking [brainfuck interpreter](https://github.com/camper0008/brainfuck-interpreter-rust), hence the name.

## How to use

Run the program through your preferred method, either with `cargo install` or `cargo run` and give the filename as the first argument: `bf-bench-util profile.txt`

It will then print the output.

Note that the files have to end with a newline, as I have not added edge cases for when the final character isn't a newline.

- Works:

```
0    0000000000100
0    0000000000200

```

- Does not work

```
0    0000000000100
0    0000000000200
```

## Specs

- Example input/output given in `EXAMPLE.txt` file

- File could potentially be several gb, so implement a buffer instead of reading it all into memory at once.

- First argument should be a file to read

- Blazing fast 🚀🚀🚀
