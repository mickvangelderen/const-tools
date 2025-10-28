use const_tools::map;

const fn wrap_all<T, const N: usize>(value: [T; N]) -> [(T,); N] {
    map!(value, |item| (item,))
}

fn main() {}