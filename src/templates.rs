use tera::Tera;
use once_cell::sync::Lazy;

pub static TERA : Lazy<Tera> = Lazy::new(||{
    let mut tera = Tera::default();
    tera.add_raw_template("ast_file", r"
export abstract class AST_node{
    constructor(parent: AST_node | undefined){
        this.parent = parent;
    }
    abstract child: any;
    parent?: AST_node;
}
{% for astNode in astNodes %}
{{astNode}}
{%- endfor %}
").unwrap();
    tera.add_raw_template("component_file", r"
import React, { useState } from 'react';
import { useRef } from 'react';
import { useEffect } from 'react';
import { AST_node } from './ast';
import { 
{%- for name in names -%}
{{name}}_node,
{%- endfor -%}
} from './ast';

import './ast.css'
import { peak_focus, rotate, set_rotation, useFocus, useFocusRoot } from './focus';
import { getTextWidth, Select, SelectInput } from './select';
export let update = ()=>{}


class ComponentInput {
  constructor(parent: AST_node, nth: number[], current?: any){
    this.parent = parent;
    this.setCurrent = (n: any)=>{
      if (current !== undefined){
        n = current;
      }
      if (nth.length===0){
        parent.child=n;
      }
      else{
        let ref = parent.child;
        for (let i in nth.slice(0,-1)){
          ref = parent.child[i]
        }
        ref[nth[nth.length-1]] = n;
      }
    };
    this.current = current;
  }
  parent: AST_node; 
  setCurrent: (n: any)=>void;
  current?: any;
  children?: React.ReactNode;
}

const TextInput: React.FC<ComponentInput> = (props) => {
  const [val, setVal] = useState('');
  props.setCurrent(val);
  let width = getTextWidth(val.toLowerCase()) + 30;
  const [focus,ref] = useFocus();
  return <input className='input' ref={ref} style={% raw %}{{width: width}} {% endraw %}
    onChange={(e) => {props.setCurrent(e.target.value);update();}}
    defaultValue={val}
  />
}
const Default: React.FC<ComponentInput> = (props) => {
  if (props.current){
    props.setCurrent(props.current);
  }
  return <div>
    {props.current}
  </div>
}

class VariaticInput {
  constructor(getNth: (n: number)=>JSX.Element, min: number){
    this.min = min;
    this.getNth = getNth;
  }
  min: number;
  getNth: (n: number)=>JSX.Element;
}

const Variatic: React.FC<VariaticInput> = (props) => {
  const [n, setN] = useState(props.min);
  function doit(amount: number){
    set_rotation(f.current);
    setN(n+amount);
    update();
  }
  function increase(){
    doit(1);
  }
  function decrease(){
    if (n > props.min){
      doit(-1);
    }
  }
  let f = useRef(0);
  useEffect(()=>{
    f.current = peak_focus();
  }, [])
  return <div>
    {Array.from(Array(n).keys()).map(props.getNth)}
    <div>
      <div onClick={increase}>add</div>
      <div onClick={decrease}>remove</div>
    </div>
  </div>
}


{% for component in components %}
{{component}}
{%- endfor %}
    ").unwrap();

    tera.add_raw_template("component", r"
const {{name}}: React.FC<ComponentInput> = (props) => {
  {%- for select in selects%}
  const [select{{select}}, setSelect{{select}}] = useState(-1);
  {%- endfor %}
  let expr: {{astname}}_node = new {{astname}}_node(props.parent);
  props.setCurrent(expr);
  return {{jsx}};
}
").unwrap();

    tera.add_raw_template("top_component", r"
export function {{name}}(props: any){
  {%- for select in selects%}
  const [select{{select}}, setSelect{{select}}] = useState(-1);
  {%- endfor %}
  const focus = useFocusRoot();
  const [, updateState] = React.useState({});
  const [expr, setExpr] = useState(new {{astname}}_node(undefined));
  props.set(expr);
  update = React.useCallback(() => {updateState({}); setExpr(expr)}, []);

  return <div onKeyDown={(e)=>{if (!e.ctrlKey) return;if(e.key === 'ArrowLeft') {rotate(-1, false);} else if (e.key === 'ArrowRight') {rotate(1,false);}}}>
  {{jsx}}
  </div>;
}
").unwrap();

    tera.add_raw_template("AST", r"
export class {{name}}_node extends AST_node {
  constructor(parent: AST_node|undefined){
      super(parent);
  }
  //accept = (v : Visitor) => { v.visit{{name}}_node(this); }
  child: {{type}} = [];
}
").unwrap();

    tera.add_raw_template("Visitor", r"
export abstract class Visitor {
  {% for name in names %}
  abstract visit{{name}}_node (node : {{name}}_node) : void;
  {%- endfor %}
}
").unwrap();

    tera
});
