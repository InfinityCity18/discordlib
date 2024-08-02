pub trait IntoBox<T> {
    fn intobox(self) -> T;
}

macro_rules! error_template {
    ($name:ident) => {
        pub use crate::error::IntoBox;
        use std::error::Error;
        use std::fmt;

        #[derive(Debug)]
        pub struct $name<'a>(Box<dyn Error + Sync + Send + 'a>);

        impl fmt::Display for $name<'_> {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "{} ( {} )", stringify!($name), self.0)
            }
        }

        impl Error for $name<'_> {}

        impl<'a, T, E> IntoBox<Result<T, $name<'a>>> for Result<T, E>
        where
            E: Error + Send + Sync + 'a,
        {
            fn intobox(self) -> Result<T, $name<'a>> {
                match self {
                    Ok(ok) => Ok(ok),
                    Err(err) => Err($name(Box::new(err))),
                }
            }
        }
    };
}

pub(crate) use error_template;
