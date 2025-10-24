use const_tools::map;

const fn test<const N: usize>(input: [usize; N]) -> [usize; N] {
    map!(input, |_| {
        break;
    })
}

const _: () = {
    test([1]);
};

fn main() {}
