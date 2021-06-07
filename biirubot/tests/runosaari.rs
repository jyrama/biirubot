use kuchiki::traits::*;

#[test]
fn runosaari_find_time_and_place_text() {
    let source = std::fs::read_to_string("tests/runosaari.html").unwrap();
    let doc = kuchiki::parse_html().one(source);

    let mut time_and_place = String::new();
    for cssm in doc.select(".time-and-place").unwrap() {
        time_and_place = cssm.text_contents();
    }

    assert_eq!(time_and_place, "~ 22.-24.7. Livonsaari & Velkuanmaa ~");
}

#[test]
fn runosaari_find_source_href() {
    let source = std::fs::read_to_string("tests/runosaari.html").unwrap();
    let doc = kuchiki::parse_html().one(source);

    let anchor = doc.select_first(".credits > a").unwrap();
    let anchor_attr = anchor.attributes.borrow();
    let github = anchor_attr.get("href").unwrap();

    assert_eq!(github, "https://github.com/codevictory/runosaari.net");
}
