mod support;

use const_tools::from_indices;

compile_test!(single_closure => {
    const fn double_indices<const N: usize>() -> [usize; N] {
        from_indices!(N, |i| i * 2)
    }

    assert!(matches!(double_indices::<4>(), [0, 2, 4, 6]));
});

compile_test!(single_function => {
    const fn square(index: usize) -> usize {
        index * index
    }

    const fn squares<const N: usize>() -> [usize; N] {
        from_indices!(N, square)
    }

    assert!(matches!(squares::<4>(), [0, 1, 4, 9]));
});

compile_test!(length_expression => {
    const M: usize = 2;
    const K: usize = 2;

    const fn descending() -> [usize; M + K] {
        from_indices!(M + K, |i| M + K - i)
    }

    assert!(matches!(descending(), [4, 3, 2, 1]));
});
