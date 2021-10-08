use lib_ruby_parser_nodes::template::*;

const TEMPLATE: &str = "// This file is auto-generated by {{ helper generated-by }}

use super::DiagnosticMessage;

impl DiagnosticMessage {
{{ each message }}<dnl>
    /// Returns true if `self` is {{ helper message-camelcase-name }}
    pub fn is_{{ helper message-lower-name }}(&self) -> bool {
        matches!(self, Self::{{ helper message-camelcase-name }} { .. })
    }
{{ end }}<dnl>
}
";

pub(crate) fn codegen() {
    let template = TemplateRoot::new(TEMPLATE).unwrap();
    let fns = crate::codegen::fns::default_fns!();

    let contents = template.render(ALL_DATA, &fns);
    std::fs::write("src/error/message/native/predicates.rs", contents).unwrap();
}
