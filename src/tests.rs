macro_rules! tests {
    ( $( ($name: ident, $problem: ident, $data: ident, $want: expr), )+ ) => {
        $(
            #[test]
            fn $name() {
                assert_eq!($want, $problem($data));
            }
        )+
    };
}

pub(crate) use tests;
