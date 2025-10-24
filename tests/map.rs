mod support;

use const_destructure::const_destructure;
use const_tools::map;

compile_test!(single_closure => {
    const fn double<const N: usize>(ia: [i32; N]) -> [i32; N] {
        map!(ia, |x| x * 2)
    }

    assert!(matches!(double([1, 2, 3]), [2, 4, 6]));
});

compile_test!(single_function => {
    const fn square(x: i32) -> i32 {
        x * x
    }

    const fn map_square<const N: usize>(ia: [i32; N]) -> [i32; N] {
        map!(ia, square)
    }

    assert!(matches!(map_square([1, 2, 3]), [1, 4, 9]));
});

compile_test!(zip_closure => {
    const fn element_wise_sum<const N: usize>(ia0: [i32; N], ia1: [i32; N]) -> [i32; N] {
        map!(zip!(ia0, ia1), |(x, y)| x + y)
    }

    assert!(matches!(element_wise_sum([1, 2], [3, 4]), [4, 6]));
});

compile_test!(zip_function => {
    const fn add_pair(pair: (i32, i32)) -> i32 {
        const_destructure!(let (x, y) = pair);
        x + y
    }

    const fn element_wise_sum<const N: usize>(ia0: [i32; N], ia1: [i32; N]) -> [i32; N] {
        map!(zip!(ia0, ia1), add_pair)
    }

    assert!(matches!(element_wise_sum([1, 2], [3, 4]), [4, 6]));
});
