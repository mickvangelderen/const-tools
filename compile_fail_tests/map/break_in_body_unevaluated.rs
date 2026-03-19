use const_tools::map;

const fn test<const N: usize>(input: [usize; N]) -> [usize; N] {
    map!(input, |_| {
        break;
    })
}

fn main() {}
