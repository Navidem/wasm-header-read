extern crate parity_wasm;
use std::{env, path, fs, io};
use parity_wasm::elements::*;
fn main() -> Result<(), Box<std::error::Error>>{
    let args: Vec<String> = env::args().collect();
    let path = path::Path::new(&args[1]);
    let file = fs::File::open(path)?;
    let mut file = io::BufReader::new(file);
    let module = Module::deserialize(&mut file)?;

    for sec in module.sections() {
        println!("{:#?}", sec);
    }
    print_reloc(module);
    Ok(())
}
fn print_reloc(module: parity_wasm::elements::Module) {
    let reloc_mod  = module.parse_reloc().unwrap();
        for sec in reloc_mod.sections() {
            match sec {
                Section::Reloc(x) => {
                    println!("printing {:#?}",  x)
                
                }
                _ => ()
            }
    }
    
}