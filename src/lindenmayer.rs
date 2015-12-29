
pub trait LindenmayerSystem<Alphabet: Clone> {
    /// Should return the initial Lindenmayer system string (iteration 0).
    fn initial() -> Vec<Alphabet>;

    /// Apply Lindenmayer system rules to a given character.
    ///
    /// This is done to efficiently use `match`ing at compile time, rather than returning a hashmap
    /// and handling it at runtime.
    fn apply_rule(Alphabet) -> Vec<Alphabet>;

    /// Generates a Lindenmayer system string for `iteration`.
    fn generate(iterations: u64) -> Vec<Alphabet> {
        let mut lstr: Vec<Alphabet> = Self::initial();
        let mut i = 0;
        while i < iterations {
            i = i + 1;
            let mut newlstr: Vec<Alphabet> = vec![];

            for l in lstr.iter().cloned() {
                for other in Self::apply_rule(l).iter().cloned() {
                    newlstr.push(other);
                }
            }
            lstr = newlstr;
        }
        lstr
    }
}

#[cfg(test)]
mod test {
    use super::LindenmayerSystem;

    #[derive(Clone, PartialEq, Eq, Debug)]
    enum TestABC {
        A,
        B,
        C,
        Foo,
    }

    struct TestLS;

    impl LindenmayerSystem<TestABC> for TestLS {
        fn initial() -> Vec<TestABC> {
            vec![TestABC::A, TestABC::B, TestABC::C]
        }

        fn apply_rule(l: TestABC) -> Vec<TestABC> {
            match l {
                TestABC::A => vec![TestABC::A, TestABC::Foo, TestABC::B, TestABC::C],
                TestABC::B => vec![TestABC::Foo],
                TestABC::C => vec![TestABC::Foo, TestABC::B],
                TestABC::Foo => vec![TestABC::Foo],
            }
        }
    }

    #[test]
    fn test_ls() {
        assert_eq!(TestLS::generate(0), [TestABC::A, TestABC::B, TestABC::C]);
        assert_eq!(TestLS::generate(1),
                   [TestABC::A,
                    TestABC::Foo,
                    TestABC::B,
                    TestABC::C,
                    TestABC::Foo,
                    TestABC::Foo,
                    TestABC::B]);
        assert_eq!(TestLS::generate(2),
                   [TestABC::A,
                    TestABC::Foo,
                    TestABC::B,
                    TestABC::C,
                    TestABC::Foo,
                    TestABC::Foo,
                    TestABC::Foo,
                    TestABC::B,
                    TestABC::Foo,
                    TestABC::Foo,
                    TestABC::Foo]);
        assert_eq!(TestLS::generate(3),
                   [TestABC::A,
                    TestABC::Foo,
                    TestABC::B,
                    TestABC::C,
                    TestABC::Foo,
                    TestABC::Foo,
                    TestABC::Foo,
                    TestABC::B,
                    TestABC::Foo,
                    TestABC::Foo,
                    TestABC::Foo,
                    TestABC::Foo,
                    TestABC::Foo,
                    TestABC::Foo,
                    TestABC::Foo]);
    }
}
