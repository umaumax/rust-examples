fn main() {
    #[cfg(feature = "hoge")]
    {
        println!("local hoge feature");
        features_ex_lib::hoge();
    }

    #[cfg(feature = "features-ex-lib/hoge")] // you can't detect other crate feature
    {
        println!("features-ex-lib/hoge");
        features_ex_lib::hoge();
    }

    #[cfg(feature = "fuga")]
    {
        println!("local fuga feature");
        features_ex_lib::fuga();
    }

    #[cfg(feature = "features-ex-lib/fuga")] // you can't detect other crate feature
    {
        println!("features-ex-lib/fuga");
        features_ex_lib::fuga();
    }

    #[cfg(feature = "piyo")]
    {
        println!("local piyo feature");
        features_ex_lib::piyo();
    }

    #[cfg(feature = "features-ex-lib/piyo")] // you can't detect other crate feature
    {
        println!("features-ex-lib/piyo");
        features_ex_lib::piyo();
    }

    #[cfg(feature = "enable-lib-piyo")]
    {
        println!("enable-lib-piyo");
        features_ex_lib::piyo();
    }
}
