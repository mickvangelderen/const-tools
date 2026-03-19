use const_tools::destructure;

const fn into_inner<T>(value: (T,)) -> T {
    destructure!(let (inner,) = value);
    inner
}

fn main() {
    assert!(matches!(into_inner((123,)), 123));
}
