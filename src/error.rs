macro_rules! error_template {
    ($name:ident) => {
        use std::error::Error;
        use std::fmt;

        #[derive(Debug)]
        pub struct $name(Box<dyn Error + Send + Sync>);

        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "{} ( {} )", "$name", self.0)
            }
        }

        impl Error for $name {}

        impl<T> From<Box<T>> for $name
        where
            T: Error + Send + Sync + 'static,
        {
            fn from(value: Box<T>) -> Self {
                $name(value)
            }
        }
    };
}

pub(crate) use error_template;

pub trait BoxErr<T, E> {
    fn bx(self) -> Result<T, Box<E>>;
}

impl<T, E> BoxErr<T, E> for Result<T, E> {
    fn bx(self: Result<T, E>) -> Result<T, Box<E>> {
        match self {
            Ok(ok) => Ok(ok),
            Err(err) => Err(Box::new(err)),
        }
    }
}
