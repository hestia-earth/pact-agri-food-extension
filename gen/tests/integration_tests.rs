use std::{fs::File, io::Read};

use ileap_extension::{Hoc, ShipmentFootprint, Tad, Toc};
use schemars::schema_for;
use serde_json::to_string_pretty;

fn read_schema(schema_name: &str) -> String {
    let schema_dir = std::path::Path::new("schemas");
    let mut file = File::open(schema_dir.join(schema_name)).unwrap();

    let mut schema = String::new();
    file.read_to_string(&mut schema).unwrap();

    schema
}

#[test]
fn compare_schemas() {
    assert_eq!(
        read_schema("shipment-footprint.json"),
        to_string_pretty(&schema_for!(ShipmentFootprint)).unwrap()
    );
    assert_eq!(
        read_schema("toc.json"),
        to_string_pretty(&schema_for!(Toc)).unwrap()
    );
    assert_eq!(
        read_schema("hoc.json"),
        to_string_pretty(&schema_for!(Hoc)).unwrap()
    );
    assert_eq!(
        read_schema("tad.json"),
        to_string_pretty(&schema_for!(Tad)).unwrap()
    );
}
