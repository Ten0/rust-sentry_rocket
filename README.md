# rust-sentry_rocket
[![Crates.io](https://img.shields.io/crates/v/sentry_rocket.svg?style=flat)](https://crates.io/crates/sentry_rocket)

[Sentry.io](https://sentry.io/) client extension for the [Rocket](https://rocket.rs/) webserver.

## Instructions
Use the `sentry_rocket::Error` type as return error type for your handler functions (ex: `Result<String, custom_derive::Error>`)

Don't forget to run the running the usual `let _guard = sentry::init(...);` and it will send any error to sentry. (Panics on 1st error if not done)

Note that you should use the `failure` crate to propagate your errors if you wish to have proper backtrace handling.

Also, using `sentry_rocket::sentry` instead of `sentry` will ensure the versions match so that the initialized sentry module is the same as the one the module uses (panics on 1st error otherwise)
(if doing so, you don't need `sentry` as a dependency for your project, only `sentry_rocket`).

## Example

```rust
#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use sentry_rocket::sentry;

#[get("/")]
fn index() -> &'static str {
	"Hello, world!"
}

#[get("/fail")]
fn fail() -> Result<(), sentry_rocket::Error> {
	something_that_may_fail()?;
	Ok(())
}

fn something_that_may_fail() -> Result<(), failure::Error> {
	let some_result = Err(std::io::Error::new(
		std::io::ErrorKind::Other,
		"Something failed",
	));
	Ok(some_result?)
}

fn main() {
	let _guard = sentry::init("https://a94ae32be2584e0bbd7a4cbb95971fee@sentry.io/1041156");
	std::env::set_var("RUST_BACKTRACE", "1");
	sentry::integrations::panic::register_panic_handler();

	rocket::ignite().mount("/", routes![index, fail]).launch();
}
```
(taken from `examples/basic_webserver.rs`)
