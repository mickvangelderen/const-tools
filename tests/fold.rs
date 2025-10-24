mod support;

use const_destructure::const_destructure;
use const_tools::{fold, zip};

compile_test!(single_closure => {
    const fn sum<const N: usize>(ia: [u32; N]) -> u32 {
        fold!(ia, 0, |acc, x| acc + x)
    }

    assert!(matches!(sum([1, 2, 3]), 6));
});

compile_test!(single_closure_block => {
    const fn sum<const N: usize>(ia: [u32; N]) -> u32 {
        fold!(ia, 0, |mut acc, x| {
            acc += x;
            acc
        })
    }

    assert!(matches!(sum([1, 2, 3]), 6));
});

compile_test!(single_function => {
    const fn add(a: u32, b: u32) -> u32 {
        a + b
    }

    const fn sum<const N: usize>(ia: [u32; N]) -> u32 {
        fold!(ia, 0, add)
    }

    assert!(matches!(sum([1, 2, 3]), 6));
});

compile_test!(zip_closure => {
    const fn sum_pairs<const N: usize>(a: [u32; N], b: [u32; N]) -> u32 {
        fold!(zip!(a, b), 0, |acc, (x, y)| acc + x + y)
    }

    assert!(matches!(sum_pairs([1, 2, 3], [4, 5, 6]), 21));
});

compile_test!(zip_function => {
    const fn add_pair(acc: u32, pair: (u32, u32)) -> u32 {
        const_destructure!(let (x, y) = pair);
        acc + x + y
    }

    const fn sum_pairs<const N: usize>(a: [u32; N], b: [u32; N]) -> u32 {
        fold!(zip!(a, b), 0, add_pair)
    }

    assert!(matches!(sum_pairs([1, 2, 3], [4, 5, 6]), 21));
});
