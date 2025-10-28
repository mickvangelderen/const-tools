const fn into_inner<T>(wrap: (T,)) -> T {
    let (value,) = wrap;
    value
}

fn main() {}
