use kuchiki::traits::*;

#[test]
fn beergium_evil_twin_title() {
    let source = std::fs::read_to_string("tests/beergium_evil_twin.html").unwrap();
    let doc = kuchiki::parse_html().one(source);

    let heading = doc.select_first(".page-heading").unwrap().text_contents();
    let evil_twin = heading.split("manufacturer").last().unwrap().trim();

    assert_eq!(evil_twin, "Evil Twin");
}

#[test]
fn beergium_evil_twin_count() {
    let source = std::fs::read_to_string("tests/beergium_evil_twin.html").unwrap();
    let doc = kuchiki::parse_html().one(source);

    let items = doc.select("li.ajax_block_product").unwrap();

    assert_eq!(items.count(), 20);
}

#[test]
fn beergium_evil_twin_sold_outs() {
    let source = std::fs::read_to_string("tests/beergium_evil_twin.html").unwrap();
    let doc = kuchiki::parse_html().one(source);

    let sold_outs1 = doc.select("li.ajax_block_product span.sold_out").unwrap();
    let sold_outs2 = doc.select("span.sold_out").unwrap();

    assert_eq!(sold_outs1.count(), 6);
    assert_eq!(sold_outs2.count(), 6);
}

#[test]
fn beergium_evil_twin_available() {
    let source = std::fs::read_to_string("tests/beergium_evil_twin.html").unwrap();
    let doc = kuchiki::parse_html().one(source);

    let products = doc.select("li.ajax_block_product").unwrap();
    let available = products.filter_map(|x| x.as_node().select_first("span.sold_out").err());

    assert_eq!(available.count(), 14);
}