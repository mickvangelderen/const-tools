mod support;

use const_destructure::const_destructure;
use const_tools::{scan, unzip, zip};

// TODO: It's hard to write all the combinations of inputs by hand and maintain them, so it would be better to try and
// generate tests for all fused operations and their nested fused operations with their parameter variations (like for
// example the map/scan function |...| (...), |...| { ... (...) }, |...| { ... }, fn, and so forth).

compile_test!(unzip2 => {
    const fn unzip2<A, B, const N: usize>(ia: [(A, B); N]) -> ([A; N], [B; N]) {
        unzip!(let (a, b) = ia);
        (a, b)
    }

    assert!(matches!(
        unzip2([(1, 'a'), (2, 'b'), (3, 'c')]),
        ([1, 2, 3], ['a', 'b', 'c'])
    ));
});

compile_test!(unzip3 => {
    const fn unzip3<A, B, C, const N: usize>(ia: [(A, B, C); N]) -> ([A; N], [B; N], [C; N]) {
        unzip!(let (a, b, c) = ia);
        (a, b, c)
    }

    assert!(matches!(
        unzip3([(1, 'a', true), (2, 'b', false)]),
        ([1, 2], ['a', 'b'], [true, false])
    ));
});

compile_test!(unzip12 => {
    type T12<T> = (T, T, T, T, T, T, T, T, T, T, T, T);

    const fn unzip12<T, const N: usize>(input: [T12<T>; N]) -> T12<[T; N]> {
        unzip!(let (o0, o1, o2, o3, o4, o5, o6, o7, o8, o9, o10, o11) = input);
        (o0, o1, o2, o3, o4, o5, o6, o7, o8, o9, o10, o11)
    }

    assert!(matches!(
        unzip12([(1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1)]),
        ([1], [1], [1], [1], [1], [1], [1], [1], [1], [1], [1], [1])
    ));
});

compile_test!(max_input_output => {
    type T12<T> = (T, T, T, T, T, T, T, T, T, T, T, T);

    #[allow(clippy::too_many_arguments)]
    const fn max_input_output<T, const N: usize>(input: T12<[T; N]>) -> T12<[T; N]> {
        const_destructure!(let (ia0, ia1, ia2, ia3, ia4, ia5, ia6, ia7, ia8, ia9, ia10, ia11) = input);
        unzip!(let (o0, o1, o2, o3, o4, o5, o6, o7, o8, o9, o10, o11) = zip!(ia0, ia1, ia2, ia3, ia4, ia5, ia6, ia7, ia8, ia9, ia10, ia11));
        (o0, o1, o2, o3, o4, o5, o6, o7, o8, o9, o10, o11)
    }

    assert!(matches!(
        max_input_output(([1], [1], [1], [1], [1], [1], [1], [1], [1], [1], [1], [1])),
        ([1], [1], [1], [1], [1], [1], [1], [1], [1], [1], [1], [1])
    ));
});

compile_test!(map_zip_closure => {
    const fn product_and_sum<const N: usize>(
        ia0: [u32; N],
        ia1: [u32; N],
    ) -> ([u32; N], [u32; N]) {
        unzip!(let (products, sums) = map!(zip!(ia0, ia1), |(x, y)| (x * y, x + y)));
        (products, sums)
    }

    assert!(matches!(
        product_and_sum([1, 2], [3, 4]),
        ([3, 8], [4, 6])
    ));
});

compile_test!(map_zip_closure_block_tuple => {
    const fn product_and_sum<const N: usize>(
        ia0: [u32; N],
        ia1: [u32; N],
    ) -> ([u32; N], [u32; N]) {
        unzip!(let (products, sums) = map!(zip!(ia0, ia1), |(x, y)| {
            let product = x * y;
            let sum = x + y;
            (product, sum)
        }));
        (products, sums)
    }

    assert!(matches!(
        product_and_sum([1, 2], [3, 4]),
        ([3, 8], [4, 6])
    ));
});

compile_test!(map_zip_closure_block_opaque => {
    const fn product_and_sum<const N: usize>(
        ia0: [u32; N],
        ia1: [u32; N],
    ) -> ([u32; N], [u32; N]) {
        unzip!(let (products, sums) = map!(zip!(ia0, ia1), |(x, y)| {
            let product = x * y;
            let sum = x + y;
            let result = (product, sum);
            #[allow(clippy::let_and_return)]
            result
        }));
        (products, sums)
    }

    assert!(matches!(
        product_and_sum([1, 2], [3, 4]),
        ([3, 8], [4, 6])
    ));
});

compile_test!(map_zip_function => {
    const fn product_and_sum_fn(pair: (u32, u32)) -> (u32, u32) {
        const_destructure!(let (x, y) = pair);
        (x * y, x + y)
    }

    const fn product_and_sum<const N: usize>(
        ia0: [u32; N],
        ia1: [u32; N],
    ) -> ([u32; N], [u32; N]) {
        unzip!(let (products, sums) = map!(zip!(ia0, ia1), product_and_sum_fn));
        (products, sums)
    }

    assert!(matches!(
        product_and_sum([1, 2], [3, 4]),
        ([3, 8], [4, 6])
    ));
});

compile_test!(map_closure => {
    const fn double_and_increment<const N: usize>(ia: [u32; N]) -> ([u32; N], [u32; N]) {
        unzip!(let (doubled, incremented) = map!(ia, |x| (x * 2, x + 1)));
        (doubled, incremented)
    }

    assert!(matches!(
        double_and_increment([1, 2, 3]),
        ([2, 4, 6], [2, 3, 4])
    ));
});

compile_test!(map_closure_block_tuple => {
    const fn double_and_increment<const N: usize>(ia: [u32; N]) -> ([u32; N], [u32; N]) {
        unzip!(let (doubled, incremented) = map!(ia, |x| {
            let doubled = x * 2;
            let incremented = x + 1;
            (doubled, incremented)
        }));
        (doubled, incremented)
    }

    assert!(matches!(
        double_and_increment([1, 2, 3]),
        ([2, 4, 6], [2, 3, 4])
    ));
});

compile_test!(map_closure_block_opaque => {
    const fn double_and_increment<const N: usize>(ia: [u32; N]) -> ([u32; N], [u32; N]) {
        unzip!(let (doubled, incremented) = map!(ia, |x| {
            let doubled = x * 2;
            let incremented = x + 1;
            let result = (doubled, incremented);
            #[allow(clippy::let_and_return)]
            result
        }));
        (doubled, incremented)
    }

    assert!(matches!(
        double_and_increment([1, 2, 3]),
        ([2, 4, 6], [2, 3, 4])
    ));
});

compile_test!(map_function => {
    const fn transform(x: u32) -> (u32, u32) {
        (x * 2, x + 1)
    }

    const fn double_and_increment<const N: usize>(ia: [u32; N]) -> ([u32; N], [u32; N]) {
        unzip!(let (doubled, incremented) = map!(ia, transform));
        (doubled, incremented)
    }

    assert!(matches!(
        double_and_increment([1, 2, 3]),
        ([2, 4, 6], [2, 3, 4])
    ));
});

compile_test!(scan_closure => {
    const fn running_sum_and_product<const N: usize>(ia: [u32; N]) -> ([u32; N], [u32; N]) {
        unzip!(let (sums, products) = scan!(ia, (0u32, 1u32), |acc, x| {
            const_destructure!(let (sum, prod) = *acc);
            let new_sum = sum + x;
            let new_prod = prod * x;
            *acc = (new_sum, new_prod);
            (new_sum, new_prod)
        }));
        (sums, products)
    }

    assert!(matches!(
        running_sum_and_product([2, 3, 4]),
        ([2, 5, 9], [2, 6, 24])
    ));
});

compile_test!(scan_closure_block_tuple => {
    const fn running_sum_and_product<const N: usize>(ia: [u32; N]) -> ([u32; N], [u32; N]) {
        unzip!(let (sums, products) = scan!(ia, (0u32, 1u32), |acc, x| {
            const_destructure!(let (sum, prod) = *acc);
            let new_sum = sum + x;
            let new_prod = prod * x;
            *acc = (new_sum, new_prod);
            {
                (new_sum, new_prod)
            }
        }));
        (sums, products)
    }

    assert!(matches!(
        running_sum_and_product([2, 3, 4]),
        ([2, 5, 9], [2, 6, 24])
    ));
});

compile_test!(scan_closure_block_opaque => {
    const fn running_sum_and_product<const N: usize>(ia: [u32; N]) -> ([u32; N], [u32; N]) {
        unzip!(let (sums, products) = scan!(ia, (0u32, 1u32), |acc, x| {
            acc.0 += x;
            acc.1 *= x;
            *acc
        }));
        (sums, products)
    }

    assert!(matches!(
        running_sum_and_product([2, 3, 4]),
        ([2, 5, 9], [2, 6, 24])
    ));
});

compile_test!(scan_zip_closure => {
    const fn running_sum_and_product<const N: usize>(
        a: [u32; N],
        b: [u32; N],
    ) -> ([u32; N], [u32; N]) {
        unzip!(let (sums, products) = scan!(zip!(a, b), (0u32, 1u32), |acc, (x, y)| {
            const_destructure!(let (sum, prod) = *acc);
            let new_sum = sum + x + y;
            let new_prod = prod * x * y;
            *acc = (new_sum, new_prod);
            (new_sum, new_prod)
        }));
        (sums, products)
    }

    assert!(matches!(
        running_sum_and_product([1, 2], [3, 4]),
        ([4, 10], [3, 24])
    ));
});

compile_test!(scan_zip_closure_block_tuple => {
    const fn running_sum_and_product<const N: usize>(
        a: [u32; N],
        b: [u32; N],
    ) -> ([u32; N], [u32; N]) {
        unzip!(let (sums, products) = scan!(zip!(a, b), (0u32, 1u32), |acc, (x, y)| {
            acc.0 += x + y;
            acc.1 *= x * y;
            const_destructure!(let (sum, prod) = *acc);
            (sum, prod)
        }));
        (sums, products)
    }

    assert!(matches!(
        running_sum_and_product([1, 2], [3, 4]),
        ([4, 10], [3, 24])
    ));
});

compile_test!(scan_zip_closure_block_opaque => {
    const fn running_sum_and_product<const N: usize>(
        a: [u32; N],
        b: [u32; N],
    ) -> ([u32; N], [u32; N]) {
        unzip!(let (sums, products) = scan!(zip!(a, b), (0u32, 1u32), |acc, (x, y)| {
            acc.0 += x + y;
            acc.1 *= x * y;
            *acc
        }));
        (sums, products)
    }

    assert!(matches!(
        running_sum_and_product([1, 2], [3, 4]),
        ([4, 10], [3, 24])
    ));
});
