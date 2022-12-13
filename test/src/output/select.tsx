import { SetStateAction, useEffect, useRef, useState } from "react";
import { update } from "./components";
import { get_rotation, rotate, useFocus } from "./focus";
import './select.css'

const MAX_LEN = 10;

  export class SelectInput {
      constructor(names: string[], outputs: JSX.Element[], index: number, setChild: (n: SetStateAction<number>)=>void, setPrev?: (n: SetStateAction<number>)=>void){
      this.names = names;
      this.outputs = outputs;
      this.index = index;
      this.setChild = setChild;
      this.setPrev = setPrev;
    }
    names: string[];
    outputs: JSX.Element[];
    id: string = '';
    index: number;
    setChild: (n: SetStateAction<number>)=>void;
    setPrev?: (n: SetStateAction<number>)=>void;
  }

  
export const Select: React.FC<SelectInput> = (props) => {
  function handleSelect(name: string) {
    if (name === '..'){
      if(props.setPrev !== undefined) props.setPrev(-1);
      return;
    }
    props.setChild(props.names.findIndex(x => x === name));
    update();
    rotate(1, true);
  }
  function handleText(text: string){
    setTentative(0);
    setText(text);
  }
  function deleteCurrent(){
    props.setChild(-1);
    update();
  }
  function handleKeyPress(event: React.KeyboardEvent<HTMLInputElement>){
    switch (event.key){
        case "ArrowDown":
            if (tentative == -1){
                setTentative(0);
                break;
            }
            setTentative(Math.min(names.length-1,tentative+1));
            break;
        case "ArrowUp":
            setTentative(Math.max(-1,tentative-1));
            break;
        case "Escape":
            setTentative(-1);
            break;
        case "Enter":
            handleSelect(names[tentative][0]);
    }
  }
  const [tentative, setTentative] = useState(-1);
  const [text, setText] = useState("");
  const [focus,ref] = useFocus();
  let i = 0;
  let width = getTextWidth(text.toLowerCase()) + 30;

  let names = [];


  if (props.index > -1) {
    return <div tabIndex={-1} ref={ref} style={{height: 20}}
    onClick={(e)=>{e.stopPropagation();rotate(focus.current, true);}}
    onKeyDown={
      (e)=>{
        if (ref.current === document.activeElement && e.key == 'Escape'){
          deleteCurrent();
        }
      }
    }>
    {props.outputs[props.index]}
    </div>
  }
  else{
    if (tentative != -1){
        let filters = props.names.map((name: string)=>fuzzy(name,text));
        names = props.names.map((name: string, i: number)=>{return [name,filters[i]]})
        .filter((e)=>e[1]!==0)
        .sort((a:[string,number],b:[string,number])=>{return a[1]-b[1]});
        if(names.length > MAX_LEN){
            names = names.splice(0,MAX_LEN);
        }
    }

  return <div className="select-container" style={{width: width}} onClick={(e)=>{e.stopPropagation();rotate(focus.current, true);}}>
        <input className="select-input" style={{width: width}} ref={ref} defaultValue={text} onChange={(e)=>{handleText(e.target.value)}} onKeyDown={handleKeyPress}/>
        <div className="select-options">
        {
          (focus.current===get_rotation())?
        names.map((e) => 
            <option className="select-option" style={((i===tentative)? {backgroundColor: "rgb(221, 187, 229)"} : {backgroundColor: ""})} key={i++} value={e[0]}>{e[0]}</option>
        ): null}
        { props.setPrev? <option key={i++} value='..'>..</option> : null }
        </div>
    </div>
  }
}

export function getTextWidth(text: string) {
    const font = "normal 14px Courier New";
    const canvas = document.createElement('canvas');
    const context = canvas.getContext('2d');
  
    context.font = font || getComputedStyle(document.body).font;
  
    return context.measureText(text).width;
}
function fuzzy(name: string, input: string){
    let inputIndex = 0;
    let nameIndex = 0;
    const inputLen = input.length;
    const nameLen = name.length;
    let score = 0;
    let longest_streak = 0;
    let current_streak = 0;

    while (inputIndex != inputLen && nameIndex != nameLen) {
        const patternChar = input.charAt(inputIndex).toLowerCase();
        const strChar = name.charAt(nameIndex).toLowerCase();
        if (patternChar == strChar) {
            ++inputIndex;
            score += 5*(1/(nameIndex+1));
            ++current_streak;
        }
        if (current_streak > longest_streak) longest_streak = current_streak;
        current_streak = 0;
        ++nameIndex;
    }

    score += longest_streak;

    if (inputLen == 0) return 1;

    return inputLen != 0 && nameLen != 0 && inputIndex == inputLen
        ? score
        : 0;

}