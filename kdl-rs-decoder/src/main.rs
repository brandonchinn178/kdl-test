use anyhow::{Context, Result};
use kdl::{KdlDocument, KdlEntryFormat, KdlNode, KdlValue};
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
            "value": kdl_value_to_json(entry.value(), entry.format()),
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

fn kdl_value_to_json(value: &KdlValue, format: Option<&KdlEntryFormat>) -> JsonValue {
    match value {
        KdlValue::String(s) => json!({
            "type": "string",
            "value": s,
        }),
        KdlValue::Integer(x) => json!({
            "type": "number",
            "value": format!("{}.0", x),
        }),
        KdlValue::Float(x) => {
            let value = if let Some((n, exp)) =
                format.and_then(|fmt| fmt.value_repr.split_once(&['e', 'E']))
            {
                // Original value was written with scientific notation, which may be
                // too large/small to fit in f64, so manually render it
                let exp = exp
                    .chars()
                    .filter(|c| *c != '_')
                    .collect::<String>()
                    .parse::<i64>()
                    .expect("Could not parse exponent");
                let (i, f) = n.split_once('.').unwrap_or((n, ""));
                match usize::try_from(exp) {
                    // Positive exponent
                    Ok(exp) => {
                        let i = if i == "0" { "" } else { i };
                        let f = format!("{:0<width$}", f, width = exp);
                        if f.len() == exp {
                            format!("{}{}.0", i, f)
                        } else {
                            let (f1, f2) = f.split_at(exp);
                            format!("{}{}.{}", i, f1, f2)
                        }
                    }
                    // Negative exponent
                    _ => {
                        let exp = -exp as usize;
                        let i = format!("{:0>width$}", i, width = exp);
                        let f = if f == "0" { "" } else { f };
                        if i.len() == exp {
                            format!("0.{}{}", i, f)
                        } else {
                            let (i1, i2) = i.split_at(exp);
                            format!("{}.{}{}", i1, i2, f)
                        }
                    }
                }
            } else if x.is_infinite() {
                format!("{}inf", if *x < 0.0 { "-" } else { "" })
            } else if x.is_nan() {
                format!("nan")
            } else if x.fract() == 0.0 {
                format!("{}.0", x)
            } else {
                format!("{}", x)
            };
            json!({
                "type": "number",
                "value": value,
            })
        }
        KdlValue::Bool(x) => json!({
            "type": "boolean",
            "value": if *x { "true" } else { "false" },
        }),
        KdlValue::Null => json!({
            "type": "null",
        }),
    }
}
