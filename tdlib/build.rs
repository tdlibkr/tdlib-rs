// Copyright 2021 - developers of the `tdlib-rs` project.
// Copyright 2020 - developers of the `grammers` project.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.
use std::env;
use std::fs::File;
use std::io::{self, BufWriter, Read, Write};
use std::path::Path;
use tdlib_tl_gen::generate_rust_code;
use tdlib_tl_parser::parse_tl_file;
use tdlib_tl_parser::tl::Definition;

/// Load the type language definitions from a certain file.
/// Parse errors will be printed to `stderr`, and only the
/// valid results will be returned.
fn load_tl(file: &str) -> io::Result<Vec<Definition>> {
    let mut file = File::open(file)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(parse_tl_file(contents)
        .filter_map(|d| match d {
            Ok(d) => Some(d),
            Err(e) => {
                eprintln!("TL: parse error: {:?}", e);
                None
            }
        })
        .collect())
}

fn main() -> std::io::Result<()> {
    // Prevent linking libraries to avoid documentation failure
    #[cfg(not(feature = "dox"))]
    {
        system_deps::Config::new().probe().unwrap();
        
        /* let dependencies = system_deps::Config::new().probe().unwrap();

        // Документации:
        // - https://doc.rust-lang.org/cargo/reference/build-scripts.html#rustc-link-lib

        // Способы запуска сборки для подробного просмотра значений
        // - cargo build -v
        // - cargo build -vv

        // Отладочный вывод:
        // for library_path in dependencies.all_link_paths() {
        //     println!("cargo::warning='Library path: {}'", library_path.display());
        // }
        // for library_name in dependencies.all_libs() {
        //     println!("cargo::warning='Library name: {}'", library_name);
        // }

        // // Явное указание полного пути к библиотекам для линковки
        // for library_path in dependencies.all_link_paths() {
        //     for library_name in dependencies.all_libs() {
        //         println!(
        //             "cargo::rustc-link-lib={}/{}",
        //             library_path.display(),
        //             library_name
        //         );
        //     }
        // }

        // Отдельно указание с версией сборки
        for (_, info) in dependencies.iter() {
            for path in info.link_paths.iter() {
                println!("cargo::rustc-link-search={}", path.display());
            }
            println!("cargo::rustc-link-lib={}.{}", info.name, info.version);
        } */
    }

    let definitions = load_tl("tl/api.tl")?;

    let mut file = BufWriter::new(File::create(
        Path::new(&env::var("OUT_DIR").unwrap()).join("generated.rs"),
    )?);

    generate_rust_code(&mut file, &definitions, cfg!(feature = "bots-only-api"))?;

    file.flush()?;
    Ok(())
}
