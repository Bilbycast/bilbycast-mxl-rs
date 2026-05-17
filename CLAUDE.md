# CLAUDE.md — bilbycast-mxl-rs

## What Is This

Bilbycast wrapper over the EBU / Linux Foundation **Media eXchange Layer (MXL)** Rust crate. Vendors `dmf-mxl/mxl` v1.0.1 as a git submodule, exposes the upstream safe API to bilbycast-edge via a thin `mxl-rs` re-export crate, and is the planned home for bilbycast-specific MXL glue (V210 ↔ planar YUV converters, grain timestamp ↔ MasterClock translation, audio frame slicing per `packet_time_us`).

Same sibling-crate pattern as `bilbycast-libsrt-rs` and `bilbycast-fdk-aac-rs`. Used only when bilbycast-edge is built with `--features mxl` (default off).

## Projects

| Crate | Role |
|-------|------|
| **mxl-rs** | Safe wrapper. Today: `pub use mxl::*` (thin re-export pinned to upstream v1.0.1). Later: bilbycast-specific helpers. The crate bilbycast-edge depends on. |
| **vendor/mxl** | Git submodule of `dmf-mxl/mxl` pinned to tag `v1.0.1` (Apache-2.0, 2026-05-07). Carries the upstream Rust workspace (`mxl-sys`, `mxl`, `gst-mxl-rs`) and the libmxl C++ source under `lib/`. |

## Build & Test

```bash
# After cloning bilbycast-mxl-rs, bring up the vendored upstream submodule
git submodule update --init --recursive

# Build the workspace (requires the full prereq chain — see Prerequisites below)
export CC=clang CXX=clang++
cargo check
cargo build
```

### Prerequisites

The MXL ecosystem ships with a heavier build chain than bilbycast's other C wrappers (vcpkg + GStreamer + several parser/linker tools). The full list, verified by smoke test 2026-05-17:

**Ubuntu / Debian apt packages:**

```bash
sudo apt install -y \
  clang cmake pkg-config \
  ninja-build bison flex lld \
  libgstreamer1.0-dev libgstreamer-plugins-base1.0-dev
```

| Package | Why |
|---|---|
| `clang` | Upstream's only CMake preset is `Linux-Clang-{Debug,Release}` (preset hardcodes Clang + LLD) |
| `cmake` | Meta-build |
| `pkg-config` | Bindgen + CMake `find_package` |
| `ninja-build` | Upstream's `mxl-sys/build.rs` hardcodes `.generator("Ninja")` |
| `bison` | libpcap (transitive via `pcapplusplus` in `vcpkg.json`) needs Bison |
| `flex` | Pairs with bison |
| `lld` | Upstream's preset specifies `CMAKE_LINKER_TYPE: LLD` |
| `libgstreamer1.0-dev` + `-plugins-base1.0-dev` | Upstream's `utils/gst-looping-filesrc/` requires it. Upstream's `mxl-sys/build.rs` forgets to set `BUILD_UTILS=OFF` even though it sets `BUILD_DOCS`, `BUILD_TESTS`, `BUILD_TOOLS` to OFF — known upstream bug |

**vcpkg (user-local, no sudo):**

```bash
git clone https://github.com/microsoft/vcpkg.git ~/vcpkg
~/vcpkg/bootstrap-vcpkg.sh -disableMetrics
```

~500 MB on disk after bootstrap. First `cargo build` triggers vcpkg to fetch + build the 7 transitive C++ deps from upstream's `vcpkg.json` (`catch2`, `stduuid`, `spdlog`, `fmt`, `picojson`, `cli11`, `pcapplusplus`) — ~5–10 min first time, ~30 s on subsequent builds via the vcpkg binary cache.

**Required env vars:**

```bash
export CC=clang CXX=clang++
```

Without these, the `cmake` build-dep crate defaults to system GCC (`/usr/bin/cc`) on Ubuntu and fails at the C++ compiler test under `-fuse-ld=lld`.

### Verified compatible versions (smoke test 2026-05-17)

| Tool | Version |
|---|---|
| clang / clang++ | Ubuntu 21.1.8 |
| cmake | 4.2.3 |
| ninja | 1.13.2 |
| bison | 3.8.2 |
| flex | 2.6.4 |
| ld.lld | Ubuntu LLD 21.1.8 |
| pkg-config | 2.5.1 |
| GStreamer | 1.28.2 |
| vcpkg | 2026-04-08 (e0612b42ce) |

## Architecture

### Why a wrapper crate?

Upstream `dmf-mxl/mxl` already ships a `mxl-sys` raw-FFI crate and a `mxl` safe wrapper. Bilbycast's other C wrappers (`bilbycast-libsrt-rs`, `bilbycast-fdk-aac-rs`) hand-roll both layers because they vendor pure C source. For MXL we accept upstream's bindings and add only a thin namespace boundary plus a home for bilbycast-specific helpers.

### Why feature-gate it off by default?

The build prereq footprint is materially heavier than other bilbycast wrappers. Most bilbycast-edge installs (contribution-grade workflows, small productions) don't need MXL for 12–24 months. Defaulting off keeps `cargo build` lean for the vast majority of users.

### Upstream surface (re-exported via `pub use mxl::*`)

Confirmed at v1.0.1: `MxlApi`, `load_api`, `Error`, `Result`, `FlowReader`, `FlowWriter`, `GrainReader`, `GrainWriter`, `GrainWriteAccess`, `MxlInstance`, `SamplesReader`, `SamplesWriter`, `SamplesWriteAccess`, plus `grain::data::*` and `samples::data::*`.

⚠️ Main-branch upstream re-exports `Rational` and `MXL_DATA_FORMAT_GRAIN_SIZE` at root; the v1.0.1 tag does not. Always consume via the pinned submodule, not main.

## Key Design Constraints

1. **Pinned to v1.0.1.** Always update the `vendor/mxl` submodule via a discrete commit; don't track upstream main.
2. **Feature-gated off in bilbycast-edge.** New `mxl` feature opt-in. Never default-on.
3. **Apache-2.0 throughout.** Upstream is Apache-2.0; our wrapper carries Apache-2.0. Clean against bilbycast-edge's AGPL-3.0-or-later combined work.
4. **Never run libmxl C++ build at runtime.** All C++ work happens at `cargo build` time; the resulting binary links libmxl statically (or as a vendored shared library) into bilbycast-edge.

## Integration with bilbycast-edge

bilbycast-edge depends on this crate as a path dependency, gated by the `mxl` feature:

```toml
[dependencies]
mxl-rs = { path = "../bilbycast-mxl-rs/mxl-rs", optional = true }

[features]
mxl = ["dep:mxl-rs"]
```

The full MXL integration plan lives at `bilbycast-edge/docs/mxl-integration-plan.md`.

## Known upstream bugs encountered

1. **`mxl-sys/build.rs` forgets `BUILD_UTILS=OFF`** — sets BUILD_DOCS/TESTS/TOOLS to OFF but not UTILS, forcing GStreamer-dev as a transitive build prereq even though the utils aren't shipped. File upstream PR or patch in `vendor/mxl/`.

2. **API surface drift between main and v1.0.1** — main re-exports `Rational` and `MXL_DATA_FORMAT_GRAIN_SIZE` at root; v1.0.1 tag does not. Always pin via the submodule.
