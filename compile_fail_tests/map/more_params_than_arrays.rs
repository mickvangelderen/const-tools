use const_tools::map;

const fn test<T, const N: usize>(input: [T; N]) -> [T; N] {
    map!(zip!(input,), |(x, y)| x)
}

fn main() {}
