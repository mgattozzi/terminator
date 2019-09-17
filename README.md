# Terminator
[![Contributor Covenant](https://img.shields.io/badge/Contributor%20Covenant-v1.4%20adopted-ff69b4.svg)](CODE_OF_CONDUCT.md)
[![GitHub](https://img.shields.io/github/license/mgattozzi/terminator)](LICENSE)

A small library to have `Display` output for `fn main() -> Result<(), E>`

## The Problem
In [RFC 1937][RFC] Rust added a long wanted feature: using `?` in `fn main()`.
In the RFC it had a trait bound for `E` to be `Display`, but in the actual
version that was stabilized it had `E` be `Debug`. While fine if one wants to
have `Debug` output in say tests, it has no use for those who want to use it
in a binary program that people would want to use. To get around this people
continue to use the same pattern we had before `rustc 1.26.0`, namely:

```rust
fn main() {
  if let Err(e) = run() {
    eprintln!("{}", e);
    std::process::exit(1);
  }
}

fn run() -> Result<(), Box<dyn Error>> {
  my_possible_failure_fn()?;
  Ok(())
}
```

What we want is this code, but it outputs a `Display` value:

```rust
fn main() -> Result<(), SomeDisplayError> {
  my_possible_failure_fn()?;
  Ok(())
}
```

That's where the `Terminator` library comes in. The code for this library is
all in one file so it's fairly easy to read, take a look at it for an
explanation of how it works exactly. As long as your error implements
`std::error::Error` then this should work!

## How to use it

Just have your `main` function return `Return<(), Terminator>` or if you need
to use it's never type implementation `Return<!, Terminator>`. Your code should
look something like this:

```rust
use terminator::Terminator;
fn main() -> Result<(), Terminator> {
  your_possible_failure_fn()?;
  // your other code
  Ok(())
}
```

## Minimum version
We support a minimum `rustc` version of `1.26.0` as this was when the question
mark in main feature was stabilized.

## License

See the [LICENSE](LICENSE) file for a copy of the `MPL 2.0` distributed with
this source code. By contributing to this code repository you agree to have your
code also distributed and released under these terms.

## Support

This is a small library but if it put a smile on your face or made your day just
a little bit better then consider buying me a cup of coffee or getting my dog
a chew toy.

[![ko-fi](https://www.ko-fi.com/img/githubbutton_sm.svg)](https://ko-fi.com/L3L413K3Q)

<!-- Links --/>
[RFC]: https://github.com/rust-lang/rfcs/blob/master/text/1937-ques-in-main.md
