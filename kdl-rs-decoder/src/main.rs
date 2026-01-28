use anyhow::{Context, Result};
use kdl::{KdlDocument, KdlNode, KdlValue};
use serde_json::{Value as JsonValue, json};
use std::io::{self, Read};

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin()
        .read_to_string(&mut input)
        .context("Failed to read from stdin")?;

    let doc = input.parse()?;
    let json = kdl_document_to_json(&doc);
    println!("{}", serde_json::to_string_pretty(&json)?);
    Ok(())
}

fn kdl_document_to_json(doc: &KdlDocument) -> JsonValue {
    json!(doc.nodes().iter().map(kdl_node_to_json).collect::<Vec<_>>())
}

fn kdl_node_to_json(node: &KdlNode) -> JsonValue {
    let ty = node.ty().map(|id| id.value());
    let name = node.name().value();

    let mut entries = Vec::new();
    for entry in node.entries() {
        entries.push(json!({
            "name": entry.name().map(|name| name.value()),
            "type": entry.ty().map(|id| id.value()),
            "value": kdl_value_to_json(entry.value()),
        }));
    }

    let children = node
        .children()
        .map_or_else(|| json!([]), kdl_document_to_json);

    json!({
        "type": ty,
        "name": name,
        "entries": entries,
        "children": children,
    })
}

fn kdl_value_to_json(value: &KdlValue) -> JsonValue {
    match value {
        KdlValue::String(s) => json!({
            "type": "string",
            "value": s,
        }),
        KdlValue::Integer(x) => json!({
            "type": "number",
            "value": format!("{}.0", x),
        }),
        KdlValue::Float(x) if x.is_infinite() => json!({
            "type": "number",
            "value": format!("{}inf", if *x < 0.0 { "-" } else { "" }),
        }),
        KdlValue::Float(x) if x.is_nan() => json!({
            "type": "number",
            "value": "nan",
        }),
        KdlValue::Float(x) => json!({
            "type": "number",
            "value": format!("{}", x),
        }),
        KdlValue::Bool(x) => json!({
            "type": "boolean",
            "value": if *x { "true" } else { "false" },
        }),
        KdlValue::Null => json!({
            "type": "null",
        }),
    }
}
