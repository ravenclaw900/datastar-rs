#![doc = include_str!("../README.md")]

#[cfg(feature = "axum")]
mod axum;
pub mod fragments;
pub mod generator;
pub mod response;
pub mod scripts;
pub mod signals;
