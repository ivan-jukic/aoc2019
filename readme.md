# AoC 2019

> Here we go again, down another rabbit hole!

## Using `aoc-runner`

### Getting the input

To get the latest day's input, run

```
cargo aoc input
```

...or to get N's day input.

```
cargo aoc input -d N
```


### Testing

A good way to solve the AoC puzzles would be to write the tests for your code
first, before running your code on the actual inputs.

For example we'd write some tests for `dayX` like this

```
#[cfg(test)]
mod tests {
    use super::{part1_solver};

    #[test]
    fn dayX_part1() {
        assert_eq!(part1_solver("some test input"), "expected test output");
    }
}
```

Then we can run the tests by filtering on the function name

```
cargo test dayX
```

In this example, `cargo test` will run only those test functions which start
with `dayX` in their names. Running `cargo test` without providing a filter
will run all the tests in your codebase.


### Running your code

Once you've tested your `dayX` relevant code, it's time to run your code, and
get the final solutions.

To run the code for the latest puzzle

```
cargo aoc
```

To run the code for a specific day puzzle

```
cargo aoc -d X
```

Runing only part one or two can be achieved by calling

```
cargo aoc -p <1 or 2>
```

It can also be mixed and matched with the `-d` flag.


### More info

More info about using `cargo-aoc`:
[https://github.com/gobanos/cargo-aoc](https://github.com/gobanos/cargo-aoc)


## Desctructuring in `rust`

How to destructure in rust:
[https://pzol.github.io/getting_rusty/posts/20140417_destructuring_in_rust/](https://pzol.github.io/getting_rusty/posts/20140417_destructuring_in_rust/)

Though, there seems to be an issue with it:
[https://github.com/rust-lang/rust/issues/62254](https://github.com/rust-lang/rust/issues/62254)
