use super::*;

pub fn complicated_ratios() -> Vec<R> {
    vec![
        R::atio(23, 2, 0.0, 0.04, Pan::Left),
        R::atio(23, 2, -0.1, 0.04, Pan::Left),
        R::atio(19, 2, 0.0, 0.1, Pan::Left),
        R::atio(19, 2, -0.2, 0.1, Pan::Left),
        R::atio(15, 2, 18.0, 0.15, Pan::Left),
        R::atio(15, 2, 0.0, 0.15, Pan::Left),
        R::atio(10, 2, -9.0, 0.15, Pan::Left),
        R::atio(7, 2, 1.0, 1.0, Pan::Left),
        R::atio(7, 2, 0.0, 1.0, Pan::Left),
        R::atio(3, 2, 3.0, 1.0, Pan::Left),
        R::atio(12, 4, 0.0, 1.0, Pan::Left),
        R::atio(15, 8, 0.0, 1.0, Pan::Left),
        R::atio(15, 8, 6.0, 1.0, Pan::Left),
        R::atio(1, 1, 0.0, 1.0, Pan::Left),
        R::atio(1, 1, -2.0, 1.0, Pan::Left),
        R::atio(1, 2, 0.0, 0.5, Pan::Left),
        R::atio(1, 2, 0.5, 0.5, Pan::Left),
        R::atio(1, 4, 1.0, 0.6, Pan::Left),
        R::atio(1, 4, 0.0, 0.6, Pan::Left),
//
        R::atio(21, 2, 0.0, 0.05, Pan::Right),
        R::atio(21, 2, 0.2, 0.05, Pan::Right),
        R::atio(17, 2, 0.0, 0.1, Pan::Right),
        R::atio(17, 2, 0.3, 0.1, Pan::Right),
        R::atio(13, 2, 0.0, 0.15, Pan::Right),
        R::atio(13, 2, -11.0, 0.15, Pan::Right),
        R::atio(11, 2, 5.0, 0.15, Pan::Right),
        R::atio(11, 2, 0.0, 0.15, Pan::Right),
        R::atio(12, 4, 0.0, 1.0, Pan::Right),
        R::atio(9, 4, 0.0, 1.0, Pan::Right),
        R::atio(9, 4, 6.0, 1.0, Pan::Right),
        R::atio(5, 4, 0.0, 1.0, Pan::Right),
        R::atio(7, 3, -3.0, 1.0, Pan::Right),
        R::atio(11, 8, 0.0, 1.0, Pan::Right),
        R::atio(1, 1, -3.0, 1.0, Pan::Right),
        R::atio(1, 2, -0.0, 0.5, Pan::Right),
        R::atio(1, 2, 0.5, 0.5, Pan::Right),
        R::atio(1, 4, 1.25, 0.6, Pan::Right),
        R::atio(1, 4, 0.0, 0.6, Pan::Right),
    ]
}

pub fn simple_ratios() -> Vec<R> {
    vec![
        R::atio(1, 2, 0.0, 0.2, Pan::Right),
        R::atio(1, 2, 3.0, 0.2, Pan::Right),
        R::atio(1, 1, -1.0, 0.5, Pan::Right),
        R::atio(7, 4, 1.0, 0.8, Pan::Right),
        R::atio(7, 4, 0.0, 0.7, Pan::Right),
        R::atio(3, 2, 0.0, 0.4, Pan::Right),
        R::atio(3, 2, 4.0, 0.3, Pan::Right),
        R::atio(3, 2, 4.0, 0.3, Pan::Right),
        R::atio(12, 5, 11.0, 0.2, Pan::Right),
        R::atio(12, 5, 0.0, 0.2, Pan::Right),
        R::atio(15, 4, 6.0, 0.17, Pan::Right),
        R::atio(15, 4, 5.0, 0.15, Pan::Right),
        R::atio(23, 4, 6.0, 0.095, Pan::Right),
        R::atio(23, 4, 5.0, 0.095, Pan::Right),
        R::atio(27, 4, 9.0, 0.055, Pan::Right),
        R::atio(27, 4, 0.0, 0.055, Pan::Right),
        R::atio(31, 4, 0.25, 0.05, Pan::Right),
        R::atio(37, 4, 0.0, 0.05, Pan::Right),
        //
        R::atio(1, 2, 0.0, 0.8, Pan::Left),
        R::atio(1, 2, -3.0, 0.8, Pan::Left),
        R::atio(1, 1, -1.0, 0.5, Pan::Left),
        R::atio(5, 4, 1.0, 0.7, Pan::Left),
        R::atio(5, 4, 0.0, 0.8, Pan::Left),
        R::atio(11, 8, 1.0, 0.4, Pan::Left),
        R::atio(11, 8, -4.0, 0.4, Pan::Left),
        R::atio(13, 4, -13.0, 0.2, Pan::Left),
        R::atio(13, 4, 6.0, 0.2, Pan::Left),
        R::atio(17, 4, 3.0, 0.15, Pan::Left),
        R::atio(17, 4, 4.0, 0.15, Pan::Left),
        R::atio(21, 4, 11.0, 0.095, Pan::Left),
        R::atio(21, 4, 0.0, 0.095, Pan::Left),
        R::atio(25, 4, -7.0, 0.055, Pan::Left),
        R::atio(25, 4, 0.0, 0.055, Pan::Left),
        R::atio(30, 4, 0.25, 0.05, Pan::Left),
        R::atio(30, 4, 0.0, 0.05, Pan::Left),
    ]
}