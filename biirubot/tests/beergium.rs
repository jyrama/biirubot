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
    let available = products.filter_map(|x| match x.as_node().select_first("span.sold_out") {
        Ok(_) => None,
        _ => Some(x),
    });

    assert_eq!(available.count(), 14);
}

#[test]
fn beergium_evil_twin_fructus_danica() {
    let source = std::fs::read_to_string("tests/beergium_evil_twin.html").unwrap();
    let doc = kuchiki::parse_html().one(source);

    let products = doc.select("li.ajax_block_product").unwrap();
    let fructus_danica: Vec<_> = products
        .filter_map(|x| {
            match x.as_node().select_first("a[title*=\"fructus danica\" i]") {
                Ok(_) => Some(x),
                Err(_) => None,
            }
                
        })
        .collect();

    assert_eq!(fructus_danica.len(), 1);
    let fructus_danica = fructus_danica.first().unwrap();

    let title = fructus_danica.as_node().select_first("a.product-name").unwrap().text_contents();

    let anchor = fructus_danica
        .as_node()
        .select_first("a.product-name")
        .unwrap();
    let anchor_attrs = anchor.attributes.borrow();
    let link = anchor_attrs.get("href").unwrap();

    let manufacturer = fructus_danica
        .as_node()
        .select_first("p.pro_list_manufacturer")
        .unwrap()
        .text_contents();

    let price = fructus_danica
        .as_node()
        .select_first("span[itemprop=\"price\"]")
        .unwrap();
    let price = price.attributes.borrow();
    let price = price.get("content").unwrap();

    assert_eq!(title, "Evil Twin NYC Fructus Danica 3 Mulberry, Raspberry, Calamansi CANS 47cl");
    assert_eq!(link, "https://www.beergium.com/en/evil-twin-brewing/4570-evil-twin-nyc-fructus-danica-3-mulberry-raspberry-calamansi-cans-47cl.html");
    assert_eq!(manufacturer, "Evil Twin");
    assert_eq!(price, "13.99");
}
