//! Integration contracts for schema catalog helpers backing
//! `list_schema_metadata` / `get_schema_metadata`.

#![allow(missing_docs)]

use spectra_backend::{schema_metadata_detail, schema_metadata_list};

#[test]
fn schema_metadata_list_returns_valid_items_happy_path() {
    let items = schema_metadata_list();
    for item in &items {
        assert!(
            !item.table_or_metric.trim().is_empty(),
            "catalog entry must have a name"
        );
        assert!(
            item.logging_kind == "event" || item.logging_kind == "metric",
            "unexpected logging_kind {:?}",
            item.logging_kind
        );
        assert!(item.can_query, "catalog items are queryable by default");
    }
}

#[test]
fn schema_metadata_detail_unknown_name_is_none_sad() {
    assert!(schema_metadata_detail("__spectra_uf_app_no_such_schema__").is_none());
}

#[test]
fn schema_metadata_detail_matches_list_entry_happy_path() {
    let items = schema_metadata_list();
    let Some(first) = items.first() else {
        // No inventory schemas in this workspace — list/detail empty contracts still hold.
        return;
    };
    let detail = schema_metadata_detail(&first.table_or_metric)
        .expect("listed schema must resolve to detail");
    assert_eq!(detail.table_or_metric, first.table_or_metric);
    assert_eq!(detail.logging_kind, first.logging_kind);
    assert!(detail.can_query);
}
