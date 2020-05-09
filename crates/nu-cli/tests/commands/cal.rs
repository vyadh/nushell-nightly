use nu_test_support::{nu, pipeline};

#[test]
fn cal_february_2020_leap_year() {
    let actual = nu!(
        cwd: ".", pipeline(
        r#"
        cal -my 2020 | where month == "february" | to json
        "#
    ));

    let cal_february_json = r#"[{"year":2020,"month":"february","sunday":null,"monday":null,"tuesday":null,"wednesday":null,"thurday":null,"friday":null,"saturday":1},{"year":2020,"month":"february","sunday":2,"monday":3,"tuesday":4,"wednesday":5,"thurday":6,"friday":7,"saturday":8},{"year":2020,"month":"february","sunday":9,"monday":10,"tuesday":11,"wednesday":12,"thurday":13,"friday":14,"saturday":15},{"year":2020,"month":"february","sunday":16,"monday":17,"tuesday":18,"wednesday":19,"thurday":20,"friday":21,"saturday":22},{"year":2020,"month":"february","sunday":23,"monday":24,"tuesday":25,"wednesday":26,"thurday":27,"friday":28,"saturday":29}]"#;

    assert_eq!(actual.out, cal_february_json);
}

#[test]
fn cal_friday_the_thirteenths_in_2015() {
    let actual = nu!(
        cwd: ".", pipeline(
        r#"
        cal -ym 2015 | where friday == 13 | count
        "#
    ));

    assert!(actual.out.contains("3"));
}

#[test]
fn cal_rows_in_2020() {
    let actual = nu!(
        cwd: ".", pipeline(
        r#"
        cal -y 2020 | count
        "#
    ));

    assert!(actual.out.contains("62"));
}
