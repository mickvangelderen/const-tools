use const_tools::fold;

const fn test<const N: usize>(input: [usize; N]) -> usize {
    fold!(input, 0, |_, _| {
        break;
    })
}

const _: () = {
    test([1]);
};

fn main() {}
