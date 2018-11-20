extern crate nevra;
#[macro_use]
extern crate proptest;

use proptest::prelude::*;

#[test]
fn parse_nevra() {
    let testcases = vec![
        "n-0:v-r.a",
        "nnn-000:vvv-rrr.aaa",
        "f-v",
        "foo-v",
        "f-0:v",
        "f-0-1",
        "f-0:1",
    ];
    let versions = vec![
        // (N, E, V, R, A)
        (
            "n",
            Some("0".into()),
            "v",
            Some("r".into()),
            Some("a".into()),
        ),
        (
            "nnn",
            Some("000".into()),
            "vvv",
            Some("rrr".into()),
            Some("aaa".into()),
        ),
        ("f", None, "v", None, None),
        ("foo", None, "v", None, None),
        ("f", Some("0".into()), "v", None, None),
        ("f", None, "0", Some("1".into()), None),
        ("f", Some("0".into()), "1", None, None),
    ];

    for (t, ver) in testcases.iter().zip(versions.iter()) {
        let parsed = nevra::PackageVersion::parse(t).expect(t);
        assert_eq!(parsed.name(), ver.0, "{} -> {:?}", t, parsed);
        assert_eq!(parsed.epoch(), &ver.1, "{} -> {:?}", t, parsed);
        assert_eq!(parsed.version(), ver.2, "{} -> {:?}", t, parsed);
        assert_eq!(parsed.release(), &ver.3, "{} -> {:?}", t, parsed);
        assert_eq!(parsed.architecture(), &ver.4, "{} -> {:?}", t, parsed);
        assert_eq!(parsed.to_string(), *t, "{} -> {:?}", t, parsed);
    }
}

proptest! {
    #![proptest_config(ProptestConfig{
        cases: 4,
        max_global_rejects: 1000000,
        ..Default::default()
    })]
    #[test]
    fn testprop_roundtrip_string(ref input in any::<String>()){
        let decoded = nevra::PackageVersion::parse(input);
        prop_assume!(decoded.is_ok());
        prop_assert_eq!(input, &decoded.unwrap().to_string());
    }
}
