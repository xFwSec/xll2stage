fn main() {
    println!(r"cargo:rustc-link-arg=/MANIFEST:NO");
    println!(r"cargo:rustc-link-arg=/NODEFAULTLIB");
}
