use nvimo::api::opts::*;
use nvimo::vimfn;

#[nvimo::test]
fn stdpath() {
    let _ = vimfn::stdpath(StdPath::Config)
        .expect("calling `stdpath` for `StdPath::Config` failed")
        .next()
        .expect(
            "calling `stdpath` for `StdPath::Config` didn't return any paths",
        );
}
