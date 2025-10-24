use const_tools::unzip;

type T13<T> = (T, T, T, T, T, T, T, T, T, T, T, T, T);

const fn test<T, const N: usize>(input: [T13<T>; N]) -> T13<[T; N]> {
    unzip!(let (a, b, c, d, e, f, g, h, i, j, k, l, m) = input)
}

fn main() {}
