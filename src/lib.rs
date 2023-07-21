//! A wrapper over [`gstd`](https://docs.gear.rs/gstd/) that provides a fluent interface to interact
//! with the Gear Protocol.
//!
//! This library can be used instead of the standard library when writing Gear programs. Compared to
//! the [`gstd`](https://docs.gear.rs/gstd/) crate, this library provides higher-level functions
//! that allow you to write Gear programs code in a more convenient way.
//!
//! ### Example
//!
//! Consider an example where the program responds 30 blocks after sending the message.
//!
//! ```rust
//! # const _: &'static str = stringify! {
//! #![no_std]
//! # };
//!
//! use gstd_fluent::{
//!     self as builder,
//!     gstd::{exec, msg, prelude::*},
//! };
//!
//! #[no_mangle]
//! extern "C" fn handle() {
//!     let block_height = exec::block_height();
//!     let payload = format!("You sent me a message in block {block_height}");
//!
//!     // same as `msg::send_delayed(msg::source(), payload, 30, 0)`
//!     builder::send(msg::source(), payload.clone())
//!         .with_delay(30)
//!         // ^ you can customize the message here
//!         //   e.g. specify delay, gas limit, reserved gas
//!         .execute()
//!         // ^ when you call `execute()` it calls the desired gstd function
//!         .expect("Unable to reply");
//!
//!     // for example, you want to specify an explicit gas limit and value:
//!     builder::send(msg::source(), payload.clone())
//!         .with_delay(30)
//!         // add two new lines:
//!         .with_gas_limit(1_000_000)
//!         .with_value(1_000) // value=0 by default
//!         // ^ specify an explicit gas limit and value
//!         .execute()
//!         // ^ same as `msg::send_with_gas_delayed(msg::source(), payload, 1_000_000, 1_000, 30)`
//!         .expect("Unable to reply");
//!
//!     // instead of replacing `msg::send_delayed` with `msg::send_with_gas_delayed`
//!     // you can just add two newlines
//! }
//! ```
//!
//! ### Builder syntax
//!
//! ```ignore
//! # const _: &'static str = stringify! {
//! #![no_std]
//! # };
//!
//! use gstd_fluent::{
//!     self as builder,
//!     gstd::{self, prelude::*, ActorId, CodeId, ReservationId},
//! };
//!
//! // this is pseudo-code to show all possible methods
//! fn how_to_use_send() {
//!     // supported functions:
//!     // - `builder::send(...)`
//!     // - `builder::send_bytes(...)`
//!     // - `builder::send_input(...)`
//!     builder::send(ActorId::zero(), String::from("payload"))
//!         // possible methods (stage 1):
//!             .with_value(1) // value=0 by default
//!             .with_delay(2) // delay in block count
//!             // you can use `.with_gas_limit(...)` or `.with_gas_from_reservation(...)`
//!             // both are not supported yet
//!             .with_gas_limit(3) //explicit gas limit
//!             .with_gas_from_reservation(ReservationId::from([0; 32])) // gas from reservation
//!         // possible methods (stage 2, called after stage 1):
//!             // you can use `.for_reply()` or `.for_reply_as::<T>()`
//!             // both are not supported
//!             .for_reply() // wait for reply (output - bytes)
//!             .for_reply_as::<T>() // wait for reply (output - T)
//!                 .with_reply_deposit(1) // reply_deposit=0 by default
//!         .execute()
//!         // ^ don't forget about `.execute()`
//!         .expect("failed to send msg");
//! }
//!
//! // equivalent to `msg::send_with_gas_for_reply_as`
//! async fn how_to_use_send_with_async() {
//!     #[derive(Decode)]
//!     #[codec(crate = gstd::codec)]
//!     struct Output {
//!         a: i32,
//!         b: Option<bool>,
//!     }
//!
//!     let output = builder::send(ActorId::zero(), String::from("input"))
//!         .with_value(42)
//!         .with_gas_limit(1_000_000)
//!         .for_reply_as::<Output>()
//!         .execute()
//!         .expect("failed to send msg")
//!         .await
//!         .expect("failed to get output");
//! }
//!
//! // this is pseudo-code to show all possible methods
//! fn how_to_use_reply() {
//!     // supported functions:
//!     // - `builder::reply(...)`
//!     // - `builder::reply_bytes(...)`
//!     // - `builder::reply_input(...)`
//!     builder::reply(String::from("payload"))
//!         // possible methods:
//!             .with_value(1) // value=0 by default
//!             // you can use `.with_gas_limit(...)` or `.with_gas_from_reservation(...)`
//!             // both are not supported yet
//!             .with_gas_limit(3) //explicit gas limit
//!             .with_gas_from_reservation(ReservationId::from([0; 32])) // gas from reservation
//!         .execute()
//!         // ^ don't forget about `.execute()`
//!         .expect("failed to reply");
//! }
//!
//! // this is pseudo-code to show all possible methods
//! fn how_to_use_create_program() {
//!     builder::create_program(CodeId::default(), b"payload")
//!         // possible methods (stage 1):
//!             .with_value(1)
//!             .with_delay(2)
//!             .with_gas_limit(3)
//!         .execute()
//!         // possible methods (stage 2, called after stage 1):
//!             // you can use `.for_reply()` or `.for_reply_as::<T>()`
//!             // both are not supported
//!             .for_reply() // wait for reply (output - bytes)
//!             .for_reply_as::<T>() // wait for reply (output - T)
//!                 .with_reply_deposit(1) // reply_deposit=0 by default
//!         // ^ don't forget about `.execute()`
//!         .expect("failed to create program");
//! }
//! ```

#![no_std]

pub extern crate gstd;
pub use functions::*;

mod common;
mod functions;

pub mod generated;
