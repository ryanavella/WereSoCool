macro_rules! r {
    ($(($num:expr,$den:expr,$offset:expr,$gain:expr,$pan:expr))*) => {
        vec![$(
            R::atio($num,$den,$offset, $gain * ((-1.0 + $pan) / -2.0), Pan::Left),
            R::atio($num,$den,$offset, $gain * ((1.0 + $pan) / 2.0), Pan::Right),
        )*]
    }
}
//macro_rules! pan_expand {
//    (left) => {Pan::Left};
//    (right) => {Pan::Right};
//    ($other:tt) => {$other};
//}


