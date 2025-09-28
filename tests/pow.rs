#[test]
fn test_cube_root_of_27() {
    let base = imagnum::create_float("27");
    let num = imagnum::create_float("1");
    let den = imagnum::create_float("3");
    let res = (base.pow(&(num / den).expect("1/3 failed"))).expect("27^(1/3) failed");
    assert_eq!(res, imagnum::create_float("3"), "cube root of 27 should be 3");
}

#[test]
fn test_7div2_equal_3dot5() {
    let a = (imagnum::create_float("7") / imagnum::create_float("2")).expect("7/2 failed");
    let b = imagnum::create_float("3.5");
    assert_eq!(a, b, "7/2 should equal 3.5");

    let c = imagnum::create_int("137").to_float().expect("int to float failed");
    let res1 = (&c / &a).expect("137/(7/2) failed");
    let res2 = (&c / &b).expect("137/3.5 failed");

    assert_eq!(res1, res2, "137/(7/2) should equal 137/3.5");
}

#[test]
fn test_2pow2dot5() {
    let base = imagnum::create_float("2");
    let exponent = imagnum::create_float("2.5");
    let result = base.pow(&exponent).expect("2^2.5 failed");
    let expected_start = "5.656854249492380195206754896838";
    assert!(
        result.to_str().starts_with(expected_start),
        "2^2.5 should start with {}, got {}",
        expected_start,
        result.to_str()
    );
}