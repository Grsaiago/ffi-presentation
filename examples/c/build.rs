fn main() {
    cc::Build::new().files(["c_src/my_lib.c"]).compile("mylib");

    println!("cargo::rerun-if-changed=c_src/*.c");
}
