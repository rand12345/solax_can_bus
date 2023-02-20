fn main() {
    let dbc_path = "./dbc/solax.dbc";
    let dbc_file = std::fs::read(dbc_path).unwrap();
    println!("cargo:rerun-if-changed={dbc_path}");

    let mut out = std::io::BufWriter::new(std::fs::File::create("src/messages.rs").unwrap());
    dbc_codegen::codegen("solax.dbc", &dbc_file, &mut out, true).unwrap();
}
