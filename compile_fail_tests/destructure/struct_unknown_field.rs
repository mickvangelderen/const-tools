use const_tools::destructure;

struct Wrap<T> {
    value: T,
}

impl<T> Wrap<T> {
    const fn test(self) -> T {
        destructure!(let Wrap { value, unknown: _ } = self);
        value
    }
}

fn main() {}
