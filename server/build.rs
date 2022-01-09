fn main() {
    // Recompile if migrations are added
    println!("cargo:rerun-if-changed=migrations");
}
