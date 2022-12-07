extern crate ebnf;
use tera::{Tera, Context};
use once_cell::sync::Lazy;
use std::cmp;

static TERA : Lazy<Tera> = Lazy::new(||{
    let mut tera = Tera::default();

    tera.add_raw_template("component", r"
const {{name}}: React.FC<ComponenetInput> = (props) => {
    {% for select in selects%}
    const [select{{select}}, setSelect{{select}}] = useState(-1);
    {%- endfor %}
    let expr: {{name}}_node = new {{name}}_node(props.parent);
    props.setCurrent(expr);
    return {{jsx}};
}
").unwrap();

    tera.add_raw_template("AST", r"
export class {{name}}_node extends AST_node {
    consutructor(parent: AST_node|undefinded){
        super(parent);
    }
    accept = (v : Visitor) => { v.visit{{name}}_node(this); }
    child: {{type}} = [];
}
").unwrap();

    tera
});

fn main() {
    let source = r"
    program     ::= expr;
    expr        ::= const | binary;
    binary      ::= expr binaryOp expr;
    binaryOp    ::= '+' | '*' | ('-' | 'n') | '/';
    const       ::= #'[0-9]+';
";

    let result = ebnf::get_grammar(source).unwrap();
    println!("{:#?}",result);
    result.expressions.iter().for_each(|item| {
        println!("{}",expression_to_ast(&item));
    });
    result.expressions.iter().for_each(|item| {
        println!("{}",expression_to_component(&item));
    });

}
fn upper(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}

fn alternation_to_list(node: &ebnf::Node) -> Result<Vec<&Box<ebnf::Node>>,i32>{
    match node {
        ebnf::Node::Symbol(l,_,r) => {
            let mut ret = vec![l];
            let mut lookingAt = r;
            loop {
                match &**lookingAt {
                    ebnf::Node::Symbol(l,_,r) => {
                        ret.push(&l);
                        lookingAt = &r;
                    },
                    _ => {ret.push(lookingAt);break;}
                }
            }
            return Ok(ret)
        },
        _ => return Err(0)
    }
}

fn node_to_ids(node: &ebnf::Node) -> Vec<String>{
    fn node_ids_helper(node: &ebnf::Node, mult_index: &Vec<i32>, last_select: &str, ret: &mut Vec<String>) {
        let id = &(mult_index.iter().map(|i|i.to_string()).collect::<Vec<String>>().join("_") + "_" + last_select);
        match node {
            ebnf::Node::String(s) => {0},
            ebnf::Node::RegexString(s) => {0},
            ebnf::Node::Terminal(s) => {0},
            ebnf::Node::Multiple(nodes) => {
                let _ = nodes.iter().enumerate().map(|(i,n)| node_ids_helper(n,&[&mult_index[..], &[i as i32 + 1]].concat(), last_select,ret));
                0
        },
            ebnf::Node::RegexExt(n, _kind) => {node_ids_helper(n,mult_index,last_select, ret); 0},
            ebnf::Node::Symbol(left, _kind, right) => {
                let v = alternation_to_list(node).unwrap();
                ret.push(id.to_string());
                v.iter().map( |n|{node_ids_helper(n,mult_index,id,ret);0}).collect::<Vec<i32>>();
                0
            },
            ebnf::Node::Group(n) => {node_ids_helper(n,mult_index,last_select,ret);0},
            ebnf::Node::Optional(n) => {node_ids_helper(n,mult_index,last_select,ret);0},
            ebnf::Node::Repeat(n) => {node_ids_helper(n,mult_index,last_select,ret);0},
            ebnf::Node::Unknown => {0},
        };
    }
    let mut ret = vec![];
    node_ids_helper(node, &vec![],"",&mut ret);
    return ret;
}

fn node_to_jsx(node: &ebnf::Node) -> String{
    fn node_to_jsx_helper(node: &ebnf::Node, mult_index: &Vec<i32>, last_select: &str) -> String {
        return match node {
            ebnf::Node::String(s) => {format!("<Default {{...new ComponentInput(expr,{:?},{})}}/>",mult_index,s)},
            ebnf::Node::RegexString(s) => {format!("<TextInput {{...new ComponentInput(expr,{:?})}}/>",mult_index)},
            ebnf::Node::Terminal(s) => {format!("<{} {{...new ComponentInput(expr,{:?})}}/>",upper(s),mult_index)},
            ebnf::Node::Multiple(nodes) => {
                "<div>\n".to_string() + &node_to_jsx_helper(&nodes[0], mult_index, last_select) + &nodes.iter().enumerate()
            .fold("".to_string(),|c:String,(i,n)| c+",\n"+&node_to_jsx_helper(n,&[&mult_index[..], &[i as i32]].concat(),last_select)) + "\n</div>"
        },
            ebnf::Node::RegexExt(n, _kind) => {node_to_string(n) + "[]"},
            ebnf::Node::Symbol(left, _kind, right) => {
                let v = alternation_to_list(node).unwrap();
                let id = mult_index.iter().map(|i|i.to_string()).collect::<Vec<String>>().join("_") + "_" + last_select;
                let mut ending = "".to_string();
                if last_select.len() != 0{
                    ending = format!(",setSelect{}",last_select);
                }
                "<Select\n{...\nnew Select_input([\"".to_string() + &node_to_string(v[0]) +"\"" + 
                &v[1..].iter().fold(
                    "".to_string(),
                    |c,n|c+", \""+&node_to_string(n) + "\"")
                + "],[\n" +
                &node_to_jsx_helper(v[0], mult_index, &id) + "\n" +
                &v[1..].iter().fold(
                    "".to_string(),
                    |c,n|c+","+&node_to_jsx_helper(n,mult_index,&id) + "\n")
                + "]," + &format!("select{x},setSelect{x}",x=id) + &ending + ")\n}\n/>"


            },
            ebnf::Node::Group(n) => {node_to_jsx_helper(n,mult_index,last_select)},
            ebnf::Node::Optional(n) => {node_to_jsx_helper(n,mult_index,last_select)},
            ebnf::Node::Repeat(n) => {node_to_jsx_helper(n,mult_index,last_select)},
            ebnf::Node::Unknown => {"".to_string()},
        }
    }
    return node_to_jsx_helper(node, &vec![], "")
}

fn node_to_string(node: &ebnf::Node) -> String {
    return match node {
        ebnf::Node::String(s) => {s.to_owned()},
        ebnf::Node::RegexString(s) => {s.to_owned()},
        ebnf::Node::Terminal(s) => {"<".to_string() + s + ">"},
        ebnf::Node::Multiple(nodes) => {nodes.iter()
        .fold("".to_string(),|c:String,n| c+","+&node_to_string(n))},
        ebnf::Node::RegexExt(n, _kind) => {node_to_string(n) + "[]"},
        ebnf::Node::Symbol(left, _kind, right) => {node_to_string(left) + "|" + &node_to_string(right)},
        ebnf::Node::Group(n) => {"(".to_string() + &node_to_string(n) + ")"},
        ebnf::Node::Optional(n) => {"".to_string()},
        ebnf::Node::Repeat(n) => {node_to_string(n)},
        ebnf::Node::Unknown => {"".to_string()},
    }
}


fn type_of(node: &ebnf::Node) -> String{
    fn node_to_type(node: &ebnf::Node) -> String {
        return match node {
            ebnf::Node::String(_s) => {"string".to_string()},
            ebnf::Node::RegexString(_s) => {"string".to_string()},
            ebnf::Node::Terminal(s) => {s.clone() + "_node"},
            ebnf::Node::Multiple(nodes) => {"[".to_string() + &node_to_type(&nodes[0]) + &nodes[1..].iter()
            .fold("".to_string(),|c:String,n| c+","+&node_to_type(n)) + "]"},
            ebnf::Node::RegexExt(n, _kind) => {node_to_type(n) + "[]"},
            ebnf::Node::Symbol(left, _kind, right) => {node_to_type(left) + "|" + &node_to_type(right)},
            ebnf::Node::Group(n) => {"(".to_string() + &node_to_type(n) + ")"},
            ebnf::Node::Optional(n) => {"".to_string()},
            ebnf::Node::Repeat(n) => {node_to_type(n)},
            ebnf::Node::Unknown => {"".to_string()},
        }
    }
    return node_to_type(node) + "|" + "[]"
}

fn undefinded(node: &ebnf::Node) -> String{
    let lm = longest_multiple(node);
    if lm == 0 {
        return "undefined".to_string();
    }
    return "[".to_string() + &"undefined,".repeat(cmp::max(lm as i32 - 1,0) as usize) + "undefined]" ;
}

fn longest_multiple(node: &ebnf::Node) -> usize {
    return match node {
        ebnf::Node::String(_s) => {0},
        ebnf::Node::RegexString(_s) => {0},
        ebnf::Node::Terminal(s) => {0},
        ebnf::Node::Multiple(nodes) => {
            if (nodes.len() == 0){
                return 0
            }
            cmp::max(nodes.len(),longest_multiple(&ebnf::Node::Multiple(nodes[1..].iter().cloned().collect())))
        },
        ebnf::Node::RegexExt(n, _kind) => {longest_multiple(n)},
        ebnf::Node::Symbol(left, _kind, right) => {cmp::max(longest_multiple(left), longest_multiple(right))},
        ebnf::Node::Group(n) => {longest_multiple(n)},
        ebnf::Node::Optional(n) => {longest_multiple(n)},
        ebnf::Node::Repeat(n) => {longest_multiple(n)},
        ebnf::Node::Unknown => {0},
    }
}


fn expression_to_ast(expression:&ebnf::Expression) -> String {
    let mut context = Context::new();
    context.insert("name", &expression.lhs);
    context.insert("type", &type_of(&expression.rhs));
    
    return TERA.render("AST",&context).unwrap();
}

fn expression_to_component(expression:&ebnf::Expression) -> String{
    let mut context = Context::new();
    context.insert("name", &expression.lhs);
    context.insert("selects", &node_to_ids(&expression.rhs));
    context.insert("jsx", &node_to_jsx(&expression.rhs));

    return TERA.render("component",&context).unwrap();
}
