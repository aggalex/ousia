#[macro_export]
macro_rules! forte {
    { ($($prev:tt)+) $prop:ident : $val:expr $(, $($rest:tt)+)? } => {
        forte! { ($($prev)+.$prop($val)) $($($rest)+)? }
    };
    { ($($prev:tt)+) # $prop:ident : $val:expr $(, $($rest:tt)+)? } => {
        forte! { ($($prev)+.bind().$prop($val)) $($($rest)+)? }
    };
    { ($($prev:tt)+) @ $signal:ident : $([$( $(@$strength:ident)? $var:ident ),+])? $val:expr $(, $($rest:tt)+)? } => {
        forte! { ($($prev)+.connect().$signal($val)) $($($rest)+)? }
    };
    { ($($prev:tt)+) } => { $($prev)+.create() };
}