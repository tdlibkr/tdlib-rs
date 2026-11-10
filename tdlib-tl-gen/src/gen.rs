use crate::{
    enums::write_enums_mod, functions::write_functions_mod, metadata::Metadata,
    types::write_types_mod,
};
use std::io::{self, Write};
use tdlib_tl_parser::tl::Definition;

////////////////////////////////////////////////////////////////////////////////

/// Генерируем непосредственно rust-код и пишем в процессе сразу в файлик
pub fn generate_rust_code(
    file: &mut impl Write,
    definitions: &[Definition],
    gen_bots_only_api: bool,
) -> io::Result<()> {
    // Напишем префикс в выходной файлик
    write!(
        file,
        "\
         // Copyright 2020 - developers of the `grammers` project.\n\
         // Copyright 2021 - developers of the `tdlib-rs` project.\n\
         //\n\
         // Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or\n\
         // https://www.apache.org/licenses/LICENSE-2.0> or the MIT license\n\
         // <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your\n\
         // option. This file may not be copied, modified, or distributed\n\
         // except according to those terms.\n\
         "
    )?;

    // Формируем метаданные о типах для будущей обработки
    let metadata = Metadata::new(definitions);

    write_types_mod(file, definitions, &metadata, gen_bots_only_api)?;

    write_enums_mod(file, definitions, &metadata, gen_bots_only_api)?;

    write_functions_mod(file, definitions, &metadata, gen_bots_only_api)?;

    Ok(())
}
