# bilbycast-mxl-rs

Bilbycast wrapper over the EBU / Linux Foundation **Media eXchange Layer (MXL)** Rust crate (`dmf-mxl/mxl` v1.0.2, Apache-2.0). Used only when bilbycast-edge is built with `--features mxl` (default off).

Same sibling-crate pattern as `bilbycast-libsrt-rs` and `bilbycast-fdk-aac-rs`.

## Quickstart

```bash
# 1. Clone with submodules (vendored upstream MXL pinned to v1.0.2)
git clone --recursive <repo-url> bilbycast-mxl-rs

# Or, if you already cloned without --recursive:
cd bilbycast-mxl-rs
git submodule update --init --recursive

# 2. Install build prereqs (one-time per host)
sudo apt install -y \
  clang cmake pkg-config \
  ninja-build bison flex lld \
  libgstreamer1.0-dev libgstreamer-plugins-base1.0-dev

git clone https://github.com/microsoft/vcpkg.git ~/vcpkg
~/vcpkg/bootstrap-vcpkg.sh -disableMetrics

# 3. Build
export CC=clang CXX=clang++
cargo build
```

First build is ~5–10 min because vcpkg fetches + builds the upstream C++ transitive deps from cold. Subsequent builds hit the vcpkg binary cache and finish in ~30 s.

See [`CLAUDE.md`](CLAUDE.md) for the full prereq table, the architecture, and the known upstream bugs we work around.

## License

Apache-2.0. Upstream `dmf-mxl/mxl` is Apache-2.0. Wrapper code in this repository is Apache-2.0.
