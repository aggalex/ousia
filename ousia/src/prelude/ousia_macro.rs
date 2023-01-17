#[macro_export]
macro_rules! ousia {
    { ($($prev:tt)+) $prop:ident : $val:expr $(, $($rest:tt)+)? } => {
        ousia! { ($($prev)+.$prop($val)) $($($rest)+)? }
    };
    { ($($prev:tt)+) # $prop:ident : $val:expr $(, $($rest:tt)+)? } => {
        ousia! { ($($prev)+.bind().$prop($val)) $($($rest)+)? }
    };
    { ($($prev:tt)+) @ $signal:ident : $([$( $(@$strength:ident)? $var:ident ),+])? $val:expr $(, $($rest:tt)+)? } => {
        ousia! { ($($prev)+.connect().$signal($val)) $($($rest)+)? }
    };
    { ($($prev:tt)+) } => { $($prev)+.create() };
}