// Copyright 2020 - developers of the `grammers` project.
// Copyright 2022 - developers of the `tdlib-rs` project.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

////////////////////////////////////////////////////////////////////////////////

use crate::rustifier;
use std::collections::{HashMap, HashSet};
use tdlib_tl_parser::tl::{Category, Definition, Type};

////////////////////////////////////////////////////////////////////////////////

/// Дополнительные метаданные о различных типах.
///
/// Additional metadata required by several parts of the generation.
pub(crate) struct Metadata<'a> {
    /// Список типов, которые у нас являются рекурсивными
    recursing_defs: HashSet<&'a String>,

    /// TODO: Список типов с дефолтными реализациями?
    default_impl_defs: HashSet<&'a String>,

    /// TODO: Список типов и их подтипов?
    defs_with_type: HashMap<&'a String, Vec<&'a Definition>>,
}

impl<'a> Metadata<'a> {
    pub fn new(definitions: &'a [Definition]) -> Self {
        // Создаем буфер для будущего заполнения
        let mut metadata = Self {
            recursing_defs: HashSet::new(),
            default_impl_defs: HashSet::new(),
            defs_with_type: HashMap::new(),
        };

        // Отдельно только различные описания типов
        let type_definitions = definitions
            .iter()
            .filter(|d| {
                // println!("Definition type test, name {}, type: {}", d.name, d.ty.name);
                d.category == Category::Types
            })
            .collect::<Vec<_>>();
        // dbg!(&type_definitions);

        // Только описания типов, но в виде хешмапы теперь,
        // в формате `имя: объявление`
        let type_definition_map = type_definitions
            .iter()
            .map(|d| (&d.name, d))
            .collect::<HashMap<_, _>>();
        // dbg!(&type_definition_map);

        // Заполняем в метаданных то же самое описание типов
        // с указанием списка итемов, которые есть внутри
        type_definitions.iter().for_each(|d| {
            metadata
                .defs_with_type
                .entry(&d.ty.name)
                .or_default()
                .push(d);
        });
        // dbg!(&metadata.defs_with_type);

        // TODO: Перебираем теперь все типы и определяем,
        // являются ли они рекурсивными сами для себя?
        type_definitions.iter().for_each(|d| {
            // println!("Definition test: {}", d.name);
            if def_self_references(d, d, &metadata.defs_with_type, &mut HashSet::new()) {
                metadata.recursing_defs.insert(&d.name);
            }

            // TODO: Дополнительно укажем еще этот тип явно,
            // так как алгоритм определения рекурсии что-то не хочет никак определять
            // что рекурсия есть на более глубоком уровне.
            if d.name == "richTextButton" {
                metadata.recursing_defs.insert(&d.name);
            }
        });
        // dbg!(&metadata.recursing_defs);

        type_definitions.iter().for_each(|d| {
            if def_contains_only_bare_types(d, &type_definition_map) {
                metadata.default_impl_defs.insert(&d.name);
            }
        });

        metadata
    }

    /// Returns `true` if any of the parameters of `Definition` eventually
    /// contains the same type as the `Definition` itself (meaning it recurses).
    pub fn is_recursive_def(&self, def: &Definition) -> bool {
        self.recursing_defs.contains(&def.name)
    }

    /// Returns `true` if the `Definition` can implement the trait `Default`
    pub fn can_def_implement_default(&self, def: &Definition) -> bool {
        self.default_impl_defs.contains(&def.name)
    }

    pub fn defs_with_type(&self, ty: &'a Type) -> &Vec<&Definition> {
        &self.defs_with_type[&ty.name]
    }
}

////////////////////////////////////////////////////////////////////////////////

/// Проверка, что какой-то тип является рекурсивным и содержит сам себя
/// Параметры:
/// - `root` корневой итем для которого проверка рекурсии
/// - `check` дочерний проверяемый итем
/// - `defs_with_type` вроде бы как список типов и их дочерние типы?
/// - `visited` хешсет для фильтраци, что уже итем такой был проверен
fn def_self_references<'a>(
    root: &Definition,
    check: &'a Definition,
    defs_with_type: &'a HashMap<&String, Vec<&Definition>>,
    visited: &mut HashSet<&'a String>,
) -> bool {
    // Помечаем проверяемый итем, нужно
    // здесь сохранить именно то же самое объявление,
    // а не только его имя.
    visited.insert(&check.name);
    /* visited.insert(std::ptr::from_ref(check)); */
    /* if std::ptr::from_ref(root) != std::ptr::from_ref(check) {
    visited.insert(check.ty.name.clone());
    } */
    /* visited.insert(check.ty.name.clone()); */
    /* if depth > 5 {
        return false;
    } */

    // Перебираем для типа список дочерних типов
    for param in check.params.iter() {
        // У дочернего типа такое же имя как у корневого проверяемого?
        // Если такое же, то это значит, что итем рекурсивный?
        if param.ty.name == root.ty.name {
            return true;
        }

        // Получаем теперь список всех подтипов для данного дочернего итема
        if let Some(defs) = defs_with_type.get(&param.ty.name) {
            // Перебираем дочерние переменные в структуре
            for def in defs {
                // Такой итем уже у нас проверялся?
                if visited.contains(&def.name) {
                    continue;
                }
                /* if visited.contains(def.ty.name.as_str()) {
                    println!("Continue type: {}", def.ty.name);
                    continue;
                } */

                // Проверяем рекурсивно итемы дочерние
                if def_self_references(root, def, defs_with_type, visited) {
                    return true;
                }
            }
        }
    }

    false
}

////////////////////////////////////////////////////////////////////////////////

fn def_contains_only_bare_types<'a>(
    check: &'a Definition,
    definition_map: &'a HashMap<&String, &&Definition>,
) -> bool {
    for param in check.params.iter() {
        if !rustifier::parameters::is_builtin_type(param) && !param.ty.bare {
            return false;
        }

        if let Some(def) = definition_map.get(&param.ty.name) {
            if !def_contains_only_bare_types(def, definition_map) {
                return false;
            }
        }
    }

    true
}
