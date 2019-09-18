/*
 * This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
 * the MPL was not distributed with this file, You can obtain one at https://mozilla.org/MPL/2.0/.
 * This Source Code Form is “Incompatible With Secondary Licenses”, as defined by the Mozilla
 * Public License, v. 2.0.
 */

//! # Terminator
//!
//! A small library to have `Display` output for `fn main() -> Result<(), E>`
//!
//! ## The Problem
//! In RFC 1937 Rust added a long wanted feature: using `?` in `fn main()`.
//! In the RFC it had a trait bound for `E` to be `Display`, but in the actual
//! version that was stabilized it had `E` be `Debug`. While fine if one wants to
//! have `Debug` output in say tests, it has no use for those who want to use it
//! in a binary program that people would want to use. To get around this people
//! continue to use the same pattern we had before `rustc 1.26.0`, namely:
//!
//! ```ignore
//! fn main() {
//!   if let Err(e) = run() {
//!     eprintln!("{}", e);
//!     std::process::exit(1);
//!   }
//! }
//!
//! fn run() -> Result<(), Box<dyn Error>> {
//!   my_possible_failure_fn()?;
//!   Ok(())
//! }
//! ```
//!
//! What we want is this code, but it outputs a `Display` value:
//!
//! ```ignore
//! fn main() -> Result<(), SomeDisplayError> {
//!   my_possible_failure_fn()?;
//!   Ok(())
//! }
//! ```
//!
//! That's where the `Terminator` library comes in. The code for this library is
//! all in one file so it's fairly easy to read, take a look at it for an
//! explanation of how it works exactly. As long as your error implements
//! `std::error::Error` then this should work!
//!
//! ## How to use it
//!
//! Just have your `main` function return `Return<(), Terminator<YourError>>` or
//! if you need to use it's never type implementation `Return<!,
//! Terminator<YourError>>`. You can leave off the explicit error type if you
//! want `Box<dyn Error>`.
//!
//! Your code should look something like this:
//!
//! ```ignore
//! use terminator::Terminator;
//! fn main() -> Result<(), Terminator> {
//!   your_possible_failure_fn()?;
//!   // your other code
//!   Ok(())
//! }
//! ```
//!
//! ## Minimum version
//! We support a minimum `rustc` version of `1.26.0` as this was when the question
//! mark in main feature was stabilized. However, for versions less than `1.31.0`
//! you'll need to set the feature flag `rust2015` in your `Cargo.toml` like so:
//!
//! ```toml
//! [dependencies]
//! terminator = { version = "0.1", default-features = false, features = "rust2015" }
//! ```

use std::error::Error;
use std::fmt::{self, Debug, Display};

#[doc(hidden)]
#[cfg(not(rust2015))]
type DefaultError = Box<dyn Error>;

#[doc(hidden)]
#[cfg(rust2015)]
type DefaultError = Box<Error>;

/// A type that lets you output your error as `Display` for `fn main() -> Result<(), Error>`
pub struct Terminator<E = DefaultError> {
    err: E
}

impl<T: Into<E> + Display, E> From<T> for Terminator<E> {
    fn from(err: T) -> Self {
        Self {
            err: err.into(),
        }
    }
}

/// A manually implemented implementation of `Debug` that writes the error out to stderr as if it
/// was `Display`
impl<E: Display> Debug for Terminator<E> {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{}", self.err)
    }
}

#[test]
fn terminator_can_be_used_with_send_and_sync() {
    let x: Box<dyn Error + Send + Sync> = "hi".into();
    assert_eq!("hi", format!("{:?}", Terminator::<Box<dyn Error + Send + Sync>>::from(x)));
}

#[test]
fn terminator_can_be_used_anywhere_question_mark_can() {
    struct MyError(String);

    impl<T: Into<String>> From<T> for MyError {
        fn from(s: T) -> Self {
            MyError(s.into())
        }
    }

    impl Display for MyError {
        fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
            write!(f, "oh no: {}", self.0)
        }
    }

    fn return_my_error() -> Result<(), Terminator<MyError>> {
        Err("hi")?
    }

    assert_eq!("oh no: hi", format!("{:?}", return_my_error().unwrap_err()));
}
