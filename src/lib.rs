#![doc = include_str!("../README.md")]
#![warn(missing_docs)]
#![cfg_attr(docsrs, feature(doc_cfg))]

macro_rules! define_feature_module {
    ($feat: literal, $name: ident) => {
        #[cfg(feature = $feat)]
        #[doc = concat!("Adds functionality from the `", $feat, "` feature.")]
        // #[cfg_attr(docsrs, doc(cfg(feature = $feat)))]
        pub mod $name ;
    };
    {$($feat: literal => $name: ident),*} => {
        $(define_feature_module!($feat, $name);)*
    };
}

define_feature_module! {
    "plotters" => plot,
    "indicatif" => pb
}
