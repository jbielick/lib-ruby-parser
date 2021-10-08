use lib_ruby_parser_nodes::template::*;

const TEMPLATE: &str = "#ifndef LIB_RUBY_PARSER_EXTERNAL_C_NODES_H
#define LIB_RUBY_PARSER_EXTERNAL_C_NODES_H

// This file is autogenerated by {{ helper generated-by }}

struct LIB_RUBY_PARSER_Node;
typedef struct LIB_RUBY_PARSER_Node LIB_RUBY_PARSER_Node;
struct LIB_RUBY_PARSER_Node_BLOB;
typedef struct LIB_RUBY_PARSER_Node_BLOB LIB_RUBY_PARSER_Node_BLOB;
typedef struct LIB_RUBY_PARSER_NodeList
{
    LIB_RUBY_PARSER_Node *ptr;
    uint64_t len;
    uint64_t capacity;
} LIB_RUBY_PARSER_NodeList;

typedef LIB_RUBY_PARSER_Node* LIB_RUBY_PARSER_NodePtr;
typedef LIB_RUBY_PARSER_Node* LIB_RUBY_PARSER_MaybeNodePtr;

{{ each node }}<dnl>
typedef struct LIB_RUBY_PARSER_{{ helper node-camelcase-name }}
{
{{ each node-field }}<dnl>
    {{ helper node-field-c-field-type }} {{ helper node-field-c-name }};
{{ end }}
} LIB_RUBY_PARSER_{{ helper node-camelcase-name }};
{{ end }}<dnl>

struct LIB_RUBY_PARSER_Node {
    enum {
{{ each node }}<dnl>
        {{ helper node-c-enum-variant-name }},
{{ end }}<dnl>
    } tag;

    union {
{{ each node }}<dnl>
        LIB_RUBY_PARSER_{{ helper node-camelcase-name }} {{ helper node-c-union-member-name }};
{{ end }}<dnl>
    } as;
};

{{ each node }}<dnl>
void LIB_RUBY_PARSER_drop_node_{{ helper node-lower-name }}(LIB_RUBY_PARSER_{{ helper node-camelcase-name }} *variant);
{{ end }}<dnl>

void LIB_RUBY_PARSER_drop_node(LIB_RUBY_PARSER_Node *node);
void LIB_RUBY_PARSER_drop_maybe_node_ptr(LIB_RUBY_PARSER_Node **node);
void LIB_RUBY_PARSER_drop_node_ptr(LIB_RUBY_PARSER_Node **node);
void LIB_RUBY_PARSER_drop_node_list(LIB_RUBY_PARSER_NodeList *node_list);

#endif // LIB_RUBY_PARSER_EXTERNAL_C_NODES_H
";

pub(crate) fn codegen() {
    let template = TemplateRoot::new(TEMPLATE).unwrap();
    let fns = crate::codegen::fns::default_fns!();

    let contents = template.render(ALL_DATA, &fns);
    std::fs::write("external/c/nodes.h", contents).unwrap();
}
