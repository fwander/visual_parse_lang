extern crate ebnf;
use tera::Context;
use std::cmp;
use std::fs;
mod templates;

fn main() {
    let source = r"
    program     ::= expr*;
    expr        ::= const | binary;
    binary      ::= '(' expr binaryOp expr ')';
    binaryOp    ::= '+' | '*' | '-' | '/';
    const       ::= #'[0-9]+';
";
//     let source = r"
//     program     ::= expression+;
//     expression  ::= 'a' | (borc '+' expression);
//     borc        ::= 'b' | 'c';
// ";

    let result = ebnf::get_grammar(source).unwrap();
    // println!("{:#?}",result);
    let mut astNodes: Vec<String> = vec![];
    let mut components: Vec<String> = vec![];
    let mut names: Vec<String> = vec![];
    let mut first = true;
    result.expressions.iter().for_each(|item| {
        if first{
            components.push(expression_to_top_component(&item));
            first = false;
        }
        else{
            components.push(expression_to_component(&item));
        }
        astNodes.push(expression_to_ast(&item));
        names.push(item.lhs.to_string());
    });
    let mut componentContext = Context::new();
    componentContext.insert("names", &names);
    componentContext.insert("components", &components);
    fs::write("components.tsx", templates::TERA.render("component_file",&componentContext).unwrap()).expect("can't write to file");

    let mut astContext = Context::new();
    astContext.insert("astNodes", &astNodes);
    fs::write("ast.ts", templates::TERA.render("ast_file",&astContext).unwrap()).expect("can't write to file");

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

fn node_to_ids(node: &ebnf::Node, select: bool) -> Vec<String>{
    fn node_ids_helper(node: &ebnf::Node, ids: &Vec<i32>, ret: &mut Vec<String>, select: bool) {
        let id = &(ids.iter().map(|i|i.to_string()).collect::<Vec<String>>().join("_"));
        match node {
            ebnf::Node::String(s) => {0},
            ebnf::Node::RegexString(s) => {0},
            ebnf::Node::Terminal(s) => {0},
            ebnf::Node::Multiple(nodes) => {
                let _ = nodes.iter().enumerate().map(|(i,n)| node_ids_helper(n,&[&ids[..], &[i as i32]].concat(), ret, select));
                0
        },
            ebnf::Node::RegexExt(n, _kind) => {node_ids_helper(n, ids,ret, select); 0},
            ebnf::Node::Symbol(left, _kind, right) => {
                let v = alternation_to_list(node).unwrap();
                if select {
                    ret.push(id.to_string());
                }
                v.iter().enumerate().map(|(i, n)|{node_ids_helper(n,&[&ids[..], &[i as i32]].concat(), ret, select)}).for_each(drop);
                0
            },
            ebnf::Node::Group(n) => {node_ids_helper(n,ids,ret,select);0},
            ebnf::Node::Optional(n) => {node_ids_helper(n,ids,ret,select);0},
            ebnf::Node::Repeat(n) => {node_ids_helper(n,ids,ret,select);0},
            ebnf::Node::Unknown => {0},
        };
    }
    let mut ret = vec![];
    node_ids_helper(node, &vec![0],&mut ret,select);
    return ret;
}

fn list_to_str(input: &Vec<&str>) -> String {
    format!("[{}]", input.iter().fold(String::new(), |acc, val| acc + val + ", "))
}

fn node_to_jsx(node: &ebnf::Node, name: &str) -> String{
    fn node_to_jsx_helper(node: &ebnf::Node, mult_index: &Vec<&str>, last_select: &str, ids: &Vec<i32>, last_variatic: i32) -> String {
        let id = ids.iter().map(|i|i.to_string()).collect::<Vec<String>>().join("_");
        return match node {
            ebnf::Node::String(s) => {format!("<Default {{...new ComponentInput(expr,{},'{}')}}/>",list_to_str(mult_index),s)},
            ebnf::Node::RegexString(s) => {format!("<TextInput {{...new ComponentInput(expr,{})}}/>",list_to_str(mult_index))},
            ebnf::Node::Terminal(s) => {format!("<{} {{...new ComponentInput(expr,{})}}/>",upper(s),list_to_str(mult_index))},
            ebnf::Node::Multiple(nodes) => {
                "<div className=\"multiple\">\n".to_string() + &nodes.iter().enumerate()
            .fold("".to_string(),|c:String,(i,n)| c+"\n"+&node_to_jsx_helper(n,&[&mult_index[..], &[&i.to_string()]].concat(),last_select,&[&ids[..], &[i as i32]].concat(),last_variatic)) + "\n</div>"
        },
            ebnf::Node::RegexExt(n, _kind) => {
                let mut min = 0;
                if matches!(_kind, ebnf::RegexExtKind::Repeat1){
                    min = 1;
                }
                let mut adding = mult_index.clone();
                let last = 'n'.to_string() + &last_variatic.to_string();
                adding.push(&(last));
                format!("<Variatic {{...new VariaticInput((n{}:number)=>{{\nreturn <div>\n",last_variatic) + &node_to_jsx_helper(n, &adding, last_select, ids, last_variatic+1) + &format!("\n</div>\n}},{})}}\n/>\n",min)
            },
            ebnf::Node::Symbol(left, _kind, right) => {
                let v = alternation_to_list(node).unwrap();
                let mut ending = "".to_string();
                if last_select.len() != 0{
                    ending = format!(",setSelect{}",last_select);
                }
                "<Select\n{...\nnew SelectInput([\"".to_string() + &node_to_string(v[0]) +"\"" + 
                &v[1..].iter().fold(
                    "".to_string(),
                    |c,n|c+", \""+&node_to_string(n) + "\"")
                + "],[\n" +
                &node_to_jsx_helper(v[0], mult_index, &id, &[&ids[..], &[0]].concat(),last_variatic) + "\n" +
                &v[1..].iter().enumerate().fold(
                    "".to_string(),
                    |c,(i,n)|c+","+&node_to_jsx_helper(n,mult_index,&id, &[&ids[..], &[i as i32 + 1]].concat(),last_variatic) + "\n")
                + "]," + &format!("select{x},setSelect{x}",x=id) + &ending + ")\n}\n/>"
            },
            ebnf::Node::Group(n) => {node_to_jsx_helper(n,mult_index,last_select,ids,last_variatic)},
            ebnf::Node::Optional(n) => {node_to_jsx_helper(n,mult_index,last_select,ids,last_variatic)},
            ebnf::Node::Repeat(n) => {node_to_jsx_helper(n,mult_index,last_select,ids,last_variatic)},
            ebnf::Node::Unknown => {"".to_string()},
        }
    }
    return format!("<div className=\"{}\">\n", name) + &node_to_jsx_helper(node, &vec![], "",&vec![0],0) + "\n</div>"
}

fn node_to_string(node: &ebnf::Node) -> String {
    return match node {
        ebnf::Node::String(s) => {s.to_owned()},
        ebnf::Node::RegexString(s) => {s.to_owned()},
        ebnf::Node::Terminal(s) => {"<".to_string() + s + ">"},
        ebnf::Node::Multiple(nodes) => {nodes.iter()
        .fold("".to_string(),|c:String,n| c+" "+&node_to_string(n))},
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
    
    return templates::TERA.render("AST",&context).unwrap();
}

fn expression_to_component(expression:&ebnf::Expression) -> String{
    let mut context = Context::new();
    context.insert("name", &upper(&expression.lhs));
    context.insert("astname", (&expression.lhs));
    context.insert("selects", &node_to_ids(&expression.rhs,true));
    context.insert("jsx", &node_to_jsx(&expression.rhs,&expression.lhs));

    return templates::TERA.render("component",&context).unwrap();
}

fn expression_to_top_component(expression:&ebnf::Expression) -> String{
    let mut context = Context::new();
    context.insert("name", &upper(&expression.lhs));
    context.insert("astname", (&expression.lhs));
    context.insert("selects", &node_to_ids(&expression.rhs,true));
    context.insert("jsx", &node_to_jsx(&expression.rhs,&expression.lhs));

    return templates::TERA.render("top_component",&context).unwrap();
}