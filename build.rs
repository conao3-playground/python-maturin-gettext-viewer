fn main() {
    cc::Build::new()
        .file("src/gettext.c")
        .flag("-lgettextpo")
        .compile("gettext");

    println!("cargo:rustc-link-lib=gettextpo");
}
