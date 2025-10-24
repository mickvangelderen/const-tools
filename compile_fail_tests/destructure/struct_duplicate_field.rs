use const_tools::destructure;

struct Wrap<T> {
    value: T,
}

impl<T> Wrap<T> {
    const fn test(self) -> T {
        destructure!(let Wrap { value: v1, value: v2 } = self);
        v1
    }
}

fn main() {}
