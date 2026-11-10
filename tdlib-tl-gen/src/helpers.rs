use tdlib_tl_parser::tl::Type;

////////////////////////////////////////////////////////////////////////////////

/// Don't generate types for definitions of this type,
/// since they are "core" types and treated differently.
const SPECIAL_CASED_TYPES: [&str; 6] = ["Bool", "Bytes", "Int32", "Int53", "Int64", "Ok"];

////////////////////////////////////////////////////////////////////////////////

pub(crate) fn ignore_type(ty: &Type) -> bool {
    SPECIAL_CASED_TYPES.iter().any(|&x| x == ty.name)
}
