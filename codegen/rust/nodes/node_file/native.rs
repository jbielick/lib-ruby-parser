use crate::codegen::rust::nodes::helpers::filename;
use lib_ruby_parser_nodes::template::*;

const TEMPLATE: &str = "// This file is auto-generated by {{ helper generated-by }}

{{ helper imports }}

{{ helper node-comment }}
#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct {{ helper node-rust-camelcase-name }} {
{{ each node-field }}<dnl>
{{ helper node-field-comment }}
    pub {{ helper node-field-rust-field-name }}: {{ helper node-field-native-type }},

{{ end }}<dnl>
}

impl {{ helper node-rust-camelcase-name }} {
    // getters
{{ each node-field }}
    /// Returns `{{ helper node-field-rust-field-name }}` field
    pub fn get_{{ helper node-field-name }}(&self) -> &{{ helper node-field-native-type }} {
        &self.{{ helper node-field-rust-field-name }}
    }
{{ end }}

    // setters
{{ each node-field }}
    /// Sets `{{ helper node-field-rust-field-name }}` field
    pub fn set_{{ helper node-field-name }}(&mut self, {{ helper node-field-rust-field-name }}: {{ helper node-field-native-type }}) {
        self.{{ helper node-field-rust-field-name }} = {{ helper node-field-rust-field-name }};
    }
{{ end }}

    #[allow(dead_code)]
    pub(crate) fn into_internal(self) -> super::Internal{{ helper node-rust-camelcase-name }} {
        let Self { {{ each node-field }}{{ helper node-field-rust-field-name }}, {{ end }} } = self;
        super::Internal{{ helper node-rust-camelcase-name }} { {{ each node-field }}{{ helper node-field-rust-field-name }}, {{ end }} }
    }
}

impl InnerNode for {{ helper node-rust-camelcase-name }} {
    fn expression(&self) -> &Loc {
        &self.expression_l
    }

    fn inspected_children(&self, indent: usize) -> Vec<String> {
        let mut result = InspectVec::new(indent);
{{ each node-field }}<dnl>
        {{ helper inspect-field }}
{{ end }}<dnl>
        result.strings()
    }

    fn str_type(&self) -> &'static str {
        \"{{ helper node-str-type }}\"
    }

    fn print_with_locs(&self) {
        println!(\"{}\", self.inspect(0));
{{ each node-field }}<dnl>
        {{ helper print-field-with-loc }}
{{ end }}<dnl>
    }
}
";

pub(crate) fn codegen(node: &lib_ruby_parser_nodes::Node) {
    let template = NodeTemplateRoot::new(TEMPLATE).unwrap();
    let mut fns = crate::codegen::fns::default_fns!();

    fns.register_helper("imports", local_helpers::imports);
    fns.register_helper("node-field-native-type", local_helpers::native_field_type);
    fns.register_helper("inspect-field", local_helpers::inspect_field);
    fns.register_helper("print-field-with-loc", local_helpers::print_field_with_loc);

    let contents = template.render(node, &fns);

    let dir = filename(node);
    let path = format!("src/nodes/types/{}/native.rs", dir);
    std::fs::write(&path, contents).unwrap();
}

mod local_helpers {
    use lib_ruby_parser_nodes::{Node, NodeWithField};

    pub(crate) fn imports(node: &Node) -> String {
        let mut imports = vec![];
        imports.push("use crate::nodes::InnerNode;");
        imports.push("use crate::nodes::InspectVec;");
        imports.push("use crate::Loc;");

        let has_field = |field_type: lib_ruby_parser_nodes::NodeFieldType| {
            node.fields.any_field_has_type(field_type)
        };

        if has_field(lib_ruby_parser_nodes::NodeFieldType::Node)
            || has_field(lib_ruby_parser_nodes::NodeFieldType::Nodes)
            || has_field(lib_ruby_parser_nodes::NodeFieldType::MaybeNode {
                regexp_options: true,
            })
            || has_field(lib_ruby_parser_nodes::NodeFieldType::MaybeNode {
                regexp_options: false,
            })
        {
            imports.push("use crate::Node;");
        }

        if has_field(lib_ruby_parser_nodes::NodeFieldType::StringValue) {
            imports.push("use crate::Bytes;");
        }

        imports.join("\n")
    }

    pub(crate) fn native_field_type(node_with_field: &NodeWithField) -> String {
        use lib_ruby_parser_nodes::NodeFieldType;
        match node_with_field.field.field_type {
            NodeFieldType::Node => "Box<Node>",
            NodeFieldType::Nodes => "Vec<Node>",
            NodeFieldType::MaybeNode { .. } => "Option<Box<Node>>",
            NodeFieldType::Loc => "Loc",
            NodeFieldType::MaybeLoc => "Option<Loc>",
            NodeFieldType::Str { .. } => "String",
            NodeFieldType::MaybeStr { .. } => "Option<String>",
            NodeFieldType::StringValue => "Bytes",
            NodeFieldType::U8 => "u8",
        }
        .to_string()
    }

    pub(crate) fn inspect_field(node_with_field: &NodeWithField) -> String {
        use lib_ruby_parser_nodes::NodeFieldType::*;

        let method_name = match node_with_field.field.field_type {
            Node => "push_node",
            Nodes => "push_nodes",
            MaybeNode { regexp_options } => {
                if regexp_options {
                    "push_regex_options"
                } else if node_with_field.field.always_print {
                    "push_maybe_node_or_nil"
                } else {
                    "push_maybe_node"
                }
            }
            Loc => return format!(""),
            MaybeLoc => return format!(""),
            Str { raw } => {
                if raw {
                    "push_raw_str"
                } else {
                    "push_str"
                }
            }
            MaybeStr { chars } => {
                if chars {
                    "push_chars"
                } else {
                    "push_maybe_str"
                }
            }
            StringValue => "push_string_value",
            U8 => "push_u8",
        };

        format!(
            "result.{}(self.get_{}());",
            method_name, node_with_field.field.snakecase_name
        )
    }

    pub(crate) fn print_field_with_loc(node_with_field: &NodeWithField) -> String {
        use lib_ruby_parser_nodes::NodeFieldType::*;

        match node_with_field.field.field_type {
            Node => format!(
                "self.get_{field_name}().inner_ref().print_with_locs();",
                field_name = node_with_field.field.snakecase_name
            ),
            Nodes =>
                format!(
                    "for node in self.get_{field_name}().iter() {{ node.inner_ref().print_with_locs(); }}",
                    field_name = node_with_field.field.snakecase_name
                ),
            MaybeNode { .. } => format!(
                "if let Some(node) = self.get_{field_name}().as_ref() {{ node.inner_ref().print_with_locs() }}",
                field_name = node_with_field.field.snakecase_name
            ),
            Loc => format!(
                "self.get_{field_name}().print(\"{printable_field_name}\");",
                field_name = node_with_field.field.snakecase_name,
                printable_field_name = node_with_field.field
                    .snakecase_name
                    .strip_suffix("_l")
                    .expect("expected loc field to end with _l")
            ),
            MaybeLoc => format!(
                "if let Some(loc) = self.get_{field_name}().as_ref() {{ loc.print(\"{printable_field_name}\") }}",
                field_name = node_with_field.field.snakecase_name,
                printable_field_name = node_with_field.field
                    .snakecase_name
                    .strip_suffix("_l")
                    .expect("expected loc field to end with _l"),
            ),
            Str { .. } => format!(""),
            MaybeStr { .. } => format!(""),
            StringValue => format!(""),
            U8 => format!(""),
        }
    }
}
