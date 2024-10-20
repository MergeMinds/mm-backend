
/// Trait for database entities.
pub(crate) trait Entity {
    fn table_name() -> &'static str;
}

/// Quickly implement `Entity` trait on a struct.
macro_rules! impl_entity {
    ($sn:ident, $tn:literal) => {
        impl Entity for $sn {
            fn table_name() -> &'static str {
                $tn
            }
        }
    };
}
