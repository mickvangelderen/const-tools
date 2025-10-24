use const_tools::map;

const fn test<T, const N: usize>(a: [T; N], b: [T; N]) -> [T; N] {
    map!(zip!(a, b), |(a,)| a)
}

fn main() {}
