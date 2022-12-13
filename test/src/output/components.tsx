
import React, { useState } from 'react';
import { useRef } from 'react';
import { useEffect } from 'react';
import { AST_node } from './ast';
import {program_node,expr_node,binary_node,binaryOp_node,const_node,} from './ast';

import './ast.css'
import { peak_focus, rotate, set_rotation, set_temp_rotation, useFocus, useFocusRoot } from './focus';
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
  return <input className='input' ref={ref} style={{width: width}} 
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
    set_temp_rotation(f.current);
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
    console.log(f.current);
  }, [])
  return <div>
    {Array.from(Array(n).keys()).map(props.getNth)}
    <div>
      <div onClick={increase}>add</div>
      <div onClick={decrease}>remove</div>
    </div>
  </div>
}




export function Program(props: any){
  const focus = useFocusRoot();
  const [, updateState] = React.useState({});
  const [expr, setExpr] = useState(new program_node(undefined));
  props.set(expr);
  update = React.useCallback(() => {updateState({}); setExpr(expr)}, []);

  return <div onKeyDown={(e)=>{if (!e.ctrlKey && !e.shiftKey) return;if(e.key === 'ArrowLeft') {rotate(-1, false);} else if (e.key === 'ArrowRight') {rotate(1,false);}}}>
  <div className="program">
<Variatic {...new VariaticInput((n0:number)=>{
return <div>
<Expr {...new ComponentInput(expr,[n0, ])}/>
</div>
},0)}
/>

</div>
  </div>;
}


const Expr: React.FC<ComponentInput> = (props) => {
  const [select0, setSelect0] = useState(-1);
  let expr: expr_node = new expr_node(props.parent);
  props.setCurrent(expr);
  return <div className="expr">
<Select
{...
new SelectInput(["<const>", "<binary>"],[
<Const {...new ComponentInput(expr,[])}/>
,<Binary {...new ComponentInput(expr,[])}/>
],select0,setSelect0)
}
/>
</div>;
}


const Binary: React.FC<ComponentInput> = (props) => {
  let expr: binary_node = new binary_node(props.parent);
  props.setCurrent(expr);
  return <div className="binary">
<div className="multiple">

<Default {...new ComponentInput(expr,[0, ],'(')}/>
<Expr {...new ComponentInput(expr,[1, ])}/>
<BinaryOp {...new ComponentInput(expr,[2, ])}/>
<Expr {...new ComponentInput(expr,[3, ])}/>
<Default {...new ComponentInput(expr,[4, ],')')}/>
</div>
</div>;
}


const BinaryOp: React.FC<ComponentInput> = (props) => {
  const [select0, setSelect0] = useState(-1);
  let expr: binaryOp_node = new binaryOp_node(props.parent);
  props.setCurrent(expr);
  return <div className="binaryOp">
<Select
{...
new SelectInput(["+", "*", "-", "/"],[
<Default {...new ComponentInput(expr,[],'+')}/>
,<Default {...new ComponentInput(expr,[],'*')}/>
,<Default {...new ComponentInput(expr,[],'-')}/>
,<Default {...new ComponentInput(expr,[],'/')}/>
],select0,setSelect0)
}
/>
</div>;
}


const Const: React.FC<ComponentInput> = (props) => {
  let expr: const_node = new const_node(props.parent);
  props.setCurrent(expr);
  return <div className="const">
<TextInput {...new ComponentInput(expr,[])}/>
</div>;
}

    