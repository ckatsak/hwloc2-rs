extern crate pkg_config;

fn main() {
    // XXX(ckatsak): Modify both `{LD_,}LIBRARY_PATH` env vars (to include the path to a local
    // installation of libhwloc) to allow `cargo run`ning the examples without having to explicitly
    // export the env vars:
    //println!("cargo:rustc-env=LD_LIBRARY_PATH=/home/christos/src/c-hwloc/_build/lib");
    //println!("cargo:rustc-env=LIBRARY_PATH=/home/christos/src/c-hwloc/_build/lib");

    let _probed = pkg_config::Config::new()
        .atleast_version("2.0.0")
        .probe("hwloc");
}
