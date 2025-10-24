#[macro_export]
macro_rules! compile_test {
    ($name:ident => $body:block) => {
        const _: () = $body;

        #[test]
        fn $name() {}
    };
}
