mod support;

use const_tools::zip;

compile_test!(zip2 => {
    const fn zip2<A, B, const N: usize>(a: [A; N], b: [B; N]) -> [(A, B); N] {
        zip!(a, b)
    }

    assert!(matches!(
        zip2([1, 2, 3], ['a', 'b', 'c']),
        [(1, 'a'), (2, 'b'), (3, 'c')]
    ));
});

compile_test!(zip3 => {
    const fn zip3<A, B, C, const N: usize>(a: [A; N], b: [B; N], c: [C; N]) -> [(A, B, C); N] {
        zip!(a, b, c)
    }

    assert!(matches!(
        zip3([1, 2], ['a', 'b'], [true, false]),
        [(1, 'a', true), (2, 'b', false)]
    ));
});

compile_test!(zip12 => {
    type T12<T> = (T, T, T, T, T, T, T, T, T, T, T, T);

    #[allow(clippy::too_many_arguments)]
    const fn zip12<T, const N: usize>(
        ia0: [T; N],
        ia1: [T; N],
        ia2: [T; N],
        ia3: [T; N],
        ia4: [T; N],
        ia5: [T; N],
        ia6: [T; N],
        ia7: [T; N],
        ia8: [T; N],
        ia9: [T; N],
        ia10: [T; N],
        ia11: [T; N],
    ) -> [T12<T>; N] {
        zip!(ia0, ia1, ia2, ia3, ia4, ia5, ia6, ia7, ia8, ia9, ia10, ia11)
    }

    assert!(matches!(
        zip12([1], [1], [1], [1], [1], [1], [1], [1], [1], [1], [1], [1]),
        [(1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1)]
    ));
});
