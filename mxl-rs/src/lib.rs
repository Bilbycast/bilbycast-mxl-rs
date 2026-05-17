//! Bilbycast wrapper over the upstream Media eXchange Layer Rust crate.
//!
//! This crate is a thin re-export of the dmf-mxl/mxl `mxl` safe wrapper pinned to
//! the v1.0.1 release tag. It exists to give bilbycast a stable, namespaced entry
//! point and a home for bilbycast-side glue (V210 ↔ planar YUV conversion, grain
//! timestamp ↔ MasterClock translation, audio frame slicing) as those land in
//! later milestones.
//!
//! See `bilbycast-mxl-rs/CLAUDE.md` for the build prereq footprint (clang, cmake,
//! ninja, bison, flex, lld, GStreamer dev, vcpkg, `CC=clang CXX=clang++`).

pub use mxl::*;
