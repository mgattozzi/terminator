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
//! ```rust
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
//! ```rust
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
//! Just have your `main` function return `Return<(), Terminator>` or if you need
//! to use it's never type implementation `Return<!, Terminator>`. Your code should
//! look something like this:
//!
//! ```rust
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
use std::fmt::{self, Debug};

/// A type that lets you output your error as `Display` for `fn main() -> Result<(), Error>`
pub struct Terminator {
    err: String
}

/// This impl block is what does all of the work. Rust has some rules regarding what traits can be
/// implemented for what. Initially the this was what was written:
///
/// ```rust
/// use std::error::Error;
/// impl<T: Error> From<T> for Terminator {
/// ```
///
/// However, we run into issues of cohesion and specialization. There exists an impl defined in
/// stdlib of this form:
///
/// ```rust
/// impl<T> From<T> for T {
/// ```
///
/// Which impl then should Rust use? While we know it's more specialized `rustc` does not. On top
/// of this we are violating coherence meaning that we have more than one impl for each given type.
/// What's a programmer to do then in this case? We want to be able to say I want to restrict this
/// to types that implement `std::error::Error`. Well the good news is that we can with `Into` and
/// some `Box` magic.
///
/// This is the actual impl that does all our magic:
///
/// ```rust
/// use std::error::Error;
/// impl<T: Into<Box<dyn Error>>> From<T> for Terminator {
/// ```
///
/// We are not asking for any given `T` that impls `std::error::Error`! We are asking only the ones
/// that can turn into a `Box<dyn Error>`. Since we are then turning into another type we're not
/// actually breaking cohesion. Dynamic dispatch allows us to do what we originally wanted with
/// only some overhead. With this impl `?` now works for any type that can turn into `Box<dyn
/// Error>` which should be all error types that implement `std::error::Error`.
///
/// Why do we care so much about `std::error::Error`? The neat thing is that besides restricting
/// ourselves to only types that are errors, we also get a type that implements `Display` since
/// that is one of `std::error::Error`'s trait bounds. What is a little less known is that if you
/// implement `Display` for a type, you implicitly get `ToString`. It's kind of like how if you
/// implement `From` you automatically get `Into`. As a result calling `to_string()` on this error
/// type gives us what it would look like if it was display, so during our conversion we just call
/// `.into().to_string()` on the input to `From` and store the value to then be dumped to stdout.
/// Then all we have after this is a simple `Debug` implementation for `Terminator` that just dumps
/// that string out as if it was `Display`!
#[cfg(not(feature = "rust2015"))]
impl<T: Into<Box<dyn Error>>> From<T> for Terminator {
    fn from(err: T) -> Self {
        Self { err: err.into().to_string() }
    }
}
#[cfg(feature = "rust2015")]
impl<T: Into<Box<Error>>> From<T> for Terminator {
    fn from(err: T) -> Self {
        Self { err: err.into().to_string() }
    }
}

/// A manually implemented implementation of `Debug` that writes the error out to stderr as if it
/// was `Display`
impl Debug for Terminator {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{}", self.err)
    }
}
