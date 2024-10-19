use core::{
    iter::{DoubleEndedIterator, FusedIterator},
    num::ParseIntError,
    str::Split,
};
use derive_more::{AsRef, Deref, Display};
use pipe_trait::Pipe;

macro_rules! impl_str {
    ($container:ident) => {
        impl<'a> $container<'a> {
            /// Create the wrapper.
            pub fn new(text: &'a str) -> Self {
                $container(text)
            }

            /// Get an immutable reference to the raw string underneath.
            pub fn as_str(&self) -> &'a str {
                &self.0.as_ref()
            }
        }
    };
}

macro_rules! impl_num {
    ($container:ident, $num:ty) => {
        impl_str!($container);
        impl<'a> $container<'a> {
            /// Extract numeric value.
            pub fn parse(&self) -> Result<$num, ParseIntError> {
                self.as_str().parse()
            }
        }
    };
}

macro_rules! def_str_wrappers {
    ($(
        $(#[$attrs:meta])*
        $name:ident;
    )*) => {$(
        $(#[$attrs])*
        #[derive(Debug, Display, Clone, Copy, AsRef, Deref)]
        pub struct $name<'a>(&'a str);
        impl_str!($name);
    )*};
}

macro_rules! def_num_wrappers {
    ($(
        $(#[$attrs:meta])*
        $name:ident = $num:ty;
    )*) => {$(
        $(#[$attrs])*
        #[derive(Debug, Display, Clone, Copy, AsRef, Deref)]
        pub struct $name<'a>(&'a str);
        impl_num!($name, $num);
    )*};
}

macro_rules! def_list_wrappers {
    ($(
        $(#[$container_attrs:meta])*
        $container_name:ident {
            $(#[$iter_attrs:meta])*
            Iter = $iter_name:ident;
            $(#[$item_attrs:meta])*
            Item = $item_name:ident;
        }
    )*) => {$(
        $(#[$container_attrs])*
        #[derive(Debug, Clone, Copy)]
        pub struct $container_name<Text>(Text);

        impl<Text> $container_name<Text> {
            /// Create the wrapper.
            pub fn new(text: Text) -> Self {
                $container_name(text)
            }
        }

        impl<Text> $container_name<Text>
        where
            Text: AsRef<str>,
        {
            /// Convert the wrapper of owned string into a wrapper of [`str`] slice.
            pub fn as_ref(&self) -> $container_name<&'_ str> {
                self.0.as_ref().pipe($container_name)
            }

            /// List the items.
            pub fn iter(&self) -> $iter_name<'_> {
                self.as_ref().into_iter()
            }
        }

        impl<'a> IntoIterator for $container_name<&'a str> {
            type IntoIter = $iter_name<'a>;
            type Item = $item_name<'a>;
            fn into_iter(self) -> Self::IntoIter {
                $iter_name(self.0.split('\n'))
            }
        }

        $(#[$iter_attrs])*
        #[derive(Debug, Clone)]
        pub struct $iter_name<'a>(Split<'a, char>);

        impl<'a> Iterator for $iter_name<'a> {
            type Item = $item_name<'a>;
            fn next(&mut self) -> Option<Self::Item> {
                self.0.next().map($item_name)
            }
            fn size_hint(&self) -> (usize, Option<usize>) {
                self.0.size_hint()
            }
        }

        impl<'a> DoubleEndedIterator for $iter_name<'a> {
            fn next_back(&mut self) -> Option<Self::Item> {
                self.0.next_back().map($item_name)
            }
        }

        impl<'a> FusedIterator for $iter_name<'a> {}

        $(#[$item_attrs])*
        #[derive(Debug, Display, Clone, Copy, AsRef, Deref)]
        pub struct $item_name<'a>(&'a str);
        impl_str!($item_name);
    )*};
}

def_str_wrappers! {
    /// Type of value of `FILENAME`.
    FileName;
    /// Type of value of `NAME`.
    Name;
    /// Type of value of `BASE`.
    Base;
    /// Type of value of `VERSION`.
    Version;
    /// Type of value of `DESC`.
    Description;
    /// Type of value of `MD5SUM`.
    Md5Sum;
    /// Type of value of `SHA256SUM`.
    Sha256Sum;
    /// Type of value of `PGPSIG`.
    PgpSignature;
    /// Type of value of `URL`.
    Url;
    /// Type of value of `PACKAGER`.
    Packager;
}

def_num_wrappers! {
    /// Type of value of `CSIZE` and `ISIZE`.
    Size = u64;
    /// Type of value of `BUILDDATE`.
    Timestamp = u64;
}

def_list_wrappers! {
    /// Type of value of `GROUPS`.
    GroupList {
        /// [Iterator] type of [`GroupList`].
        Iter = GroupIter;
        /// Type of [iterator item](Iterator::Item) of [`GroupList`].
        Item = Group;
    }

    /// Type of value of `LICENSE`.
    LicenseList {
        /// [Iterator] type of [`LicenseList`].
        Iter = LicenseIter;
        /// Type of [iterator item](Iterator::Item) of [`LicenseList`].
        Item = License;
    }

    /// Type of value of `ARCH`.
    ArchList {
        /// [Iterator] type of [`ArchList`].
        Iter = ArchIter;
        /// Type of [iterator item](Iterator::Item) of [`ArchList`].
        Item = Arch;
    }

    /// Type of value of `DEPENDS`, `MAKEDEPENDS`, `CHECKDEPENDS`, `PROVIDES`, `CONFLICTS`, and `REPLACES`.
    DependList {
        /// [Iterator] type of [`DependList`].
        Iter = DependIter;
        /// Type of [iterator item](Iterator::Item) of [`DependList`].
        Item = Depend;
    }

    /// Type of value of `OPTDEPENDS`.
    DependAndReasonList {
        /// [Iterator] type of [`DependAndReasonList`].
        Iter = DependAndReasonIter;
        /// Type of [iterator item](Iterator::Item) of [`DependAndReasonList`].
        Item = DependAndReason;
    }
}

def_str_wrappers! {
    /// Name of a [dependency](Depend). It could either be a [package name](Name) or a soname of a library.
    DependName;
    /// Reason for installing a [dependency](DependAndReason).
    DependReason;
    /// Specification of a [dependency](DependName).
    DependSpec;
}

mod depend;
mod depend_and_reason;
mod depend_name;
mod depend_spec;
mod depend_spec_operator;

pub use depend_spec_operator::DependSpecOperator;
