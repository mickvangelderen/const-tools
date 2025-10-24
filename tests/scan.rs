mod support;

use const_tools::destructure;
use const_tools::scan;

compile_test!(single_closure => {
    const fn cumulative_sum<const N: usize>(ia: [u32; N]) -> [u32; N] {
        scan!(ia, 0, |sum, ii| {
            *sum += ii;
            *sum
        })
    }

    assert!(matches!(cumulative_sum([1, 2, 3]), [1, 3, 6]));
});

compile_test!(single_function => {
    const fn accumulate(acc: &mut u32, x: u32) -> u32 {
        *acc += x;
        *acc
    }

    const fn cumulative_sum<const N: usize>(ia: [u32; N]) -> [u32; N] {
        scan!(ia, 0, accumulate)
    }

    assert!(matches!(cumulative_sum([1, 2, 3]), [1, 3, 6]));
});

compile_test!(zip_closure => {
    const fn cumulative_element_wise_sum<const N: usize>(
        a: [u32; N],
        b: [u32; N],
    ) -> [u32; N] {
        scan!(zip!(a, b), 0, |sum, (x, y)| {
            *sum += x + y;
            *sum
        })
    }

    assert!(matches!(
        cumulative_element_wise_sum([1, 2, 3], [4, 5, 6]),
        [5, 12, 21]
    ));
});

compile_test!(zip_function => {
    const fn accumulate_pair(acc: &mut u32, pair: (u32, u32)) -> u32 {
        destructure!(let (x, y) = pair);
        *acc += x + y;
        *acc
    }

    const fn cumulative_element_wise_sum<const N: usize>(
        a: [u32; N],
        b: [u32; N],
    ) -> [u32; N] {
        scan!(zip!(a, b), 0, accumulate_pair)
    }

    assert!(matches!(
        cumulative_element_wise_sum([1, 2, 3], [4, 5, 6]),
        [5, 12, 21]
    ));
});
