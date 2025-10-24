use const_tools::zip;

type T13<T> = (T, T, T, T, T, T, T, T, T, T, T, T, T);

const fn test<T, const N: usize>(
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
    ia12: [T; N],
) -> [T13<T>; N] {
    zip!(
        ia0, ia1, ia2, ia3, ia4, ia5, ia6, ia7, ia8, ia9, ia10, ia11, ia12
    )
}

fn main() {}
