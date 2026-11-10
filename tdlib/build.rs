// Copyright 2021 - developers of the `tdlib-rs` project.
// Copyright 2020 - developers of the `grammers` project.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.
use std::{
    env,
    fs::File,
    io::{BufWriter, Write},
    path::Path,
};
use tdlib_tl_gen::generate_rust_code;
use tdlib_tl_parser::{parse_tl_file, tl::Definition};

////////////////////////////////////////////////////////////////////////////////

fn main() -> std::io::Result<()> {
    // Выключение линкуемых библиотек для избежания ошибок документирования
    #[cfg(not(feature = "dox"))]
    {
        // Стандартная реализация подключения линкуемых библиотек
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

    // Пробуем подгрузить список из файлика
    let definitions = load_tl("tl/api.tl")?;

    // Создаем файлик для записи выходного rust-кода
    let mut file = {
        // Получаем из окружения выходную директорию
        let out_dir = env::var("OUT_DIR").unwrap();

        // Путь к конечному файлику
        let path = Path::new(&out_dir).join("generated.rs");

        // Создаем новый файлик, подтирая старое содержимое
        let file = File::create(path)?;

        // обертка для буферизации
        BufWriter::new(file)
    };

    // Генерируем растовский код
    generate_rust_code(&mut file, &definitions, cfg!(feature = "bots-only-api"))?;

    // Сбрасываем на диск данные принудительно
    file.flush()?;

    Ok(())
}

////////////////////////////////////////////////////////////////////////////////

/// Загружаем описание протокола из файлика.
/// Ошибки парсинга будут здесь кидаться в `stderr`,
/// а выдаваться будут лишь валидные результаты.
///
/// Load the type language definitions from a certain file.
/// Parse errors will be printed to `stderr`, and only the
/// valid results will be returned.
fn load_tl(file_path: &str) -> Result<Vec<Definition>, std::io::Error> {
    /* // Пробуем открыть файлик
    let mut file = File::open(file)?;

    // Создаем буфер для результата чтения файлика
    let mut contents = String::new();

    // Вычитываем все содержимое файлика в строку
    file.read_to_string(&mut contents)?;

    // Закрываем сразу же файлик
    drop(file); */

    // Пробуем вычитать содержимое файлика в строку с оптимизациями
    let file_content = std::fs::read_to_string(file_path)?;

    // Пробуем распарсить содержимое файлика теперь
    let results = parse_tl_file(file_content)
        .filter_map(|d| {
            // Успешно ли распарсили элемент?
            match d {
                // Все хорошо - отдаем дальше
                Ok(d) => Some(d),
                // Ошибка вылезла
                Err(e) => {
                    // Пишем ошибку, но дальше пропускаем
                    eprintln!("TL: parse error: {:?}", e);
                    None
                }
            }
        })
        .collect();
    // dbg!(&results);

    Ok(results)
}
