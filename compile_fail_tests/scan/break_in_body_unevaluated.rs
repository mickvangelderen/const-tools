use const_tools::scan;

const fn test<const N: usize>(input: [usize; N]) -> [usize; N] {
    scan!(input, 0, |_, _| {
        break;
    })
}

fn main() {}
