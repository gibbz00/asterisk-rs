use serde::Serializer;

// UNTESTED:
pub fn join_serialize<S>(slice: &[impl AsRef<str>], s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let mut joined = slice.iter().map(AsRef::as_ref).fold(String::new(), |mut acc, element| {
        acc.push_str(element);
        acc.push(',');
        acc
    });

    // pop the last comma
    joined.pop();

    s.serialize_str(&joined)
}

#[cfg(test)]
mod tests {
    use serde::Serialize;
    use serde_test::{assert_ser_tokens, Token};

    #[derive(Serialize)]
    pub struct TestParams<'a> {
        #[serde(serialize_with = "super::join_serialize")]
        list: &'a [&'a str],
    }

    #[test]
    fn join_serialize() {
        let input = TestParams { list: &["a"] };

        assert_ser_tokens(&input, &expected_struct("a"));

        let input = TestParams { list: &["a", "b"] };

        assert_ser_tokens(&input, &expected_struct("a,b"));

        fn expected_struct(expected_str: &'static str) -> [Token; 4] {
            [
                Token::Struct {
                    name: "TestParams",
                    len: 1,
                },
                Token::Str("list"),
                Token::Str(expected_str),
                Token::StructEnd,
            ]
        }
    }
}
