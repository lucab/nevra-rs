extern crate nevra;

#[test]
fn parse_nevra() {
    let testcases = vec![
        "n-e:v-r.a",
        "nnn-eee:vvv-rrr.aaa",
        "f-v",
        "foo-v",
        "f-e:v",
        "f-0-1",
        "f-0:1",
    ];
    let versions = vec![
        // (N, E, V, R, A)
        (
            "n",
            Some("e".into()),
            "v",
            Some("r".into()),
            Some("a".into()),
        ),
        (
            "nnn",
            Some("eee".into()),
            "vvv",
            Some("rrr".into()),
            Some("aaa".into()),
        ),
        ("f", None, "v", None, None),
        ("foo", None, "v", None, None),
        ("f", Some("e".into()), "v", None, None),
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
