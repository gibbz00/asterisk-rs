macro_rules! id_newtype {
    (
        $(#[$attr:meta])*
        $name:tt
    ) => {
        $(#[$attr])*
        #[derive(Clone, Debug, PartialEq, Eq, Hash, ::derive_more::Display, ::serde::Serialize, ::serde::Deserialize)]
        #[serde(transparent)]
        pub struct $name(String);

        impl $name {
            pub fn new(inner: impl Into<String>) -> Self {
                Self(inner.into())
            }

            pub fn inner(&self) -> &str {
                &self.0
            }

            pub fn into_inner(self) -> String {
                self.0
            }
        }
    };
}
pub(crate) use id_newtype;
