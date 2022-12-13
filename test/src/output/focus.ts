import { SetStateAction, useEffect, useRef, useState } from "react";

export class FocusTree {
  constructor(val: number){
    this.val = val;
    this.children = [];
  }
  val: number;
  children: FocusTree[];
  add(n: number, adding: number, counter: number){
    if (counter === n){
      this.children.push(new FocusTree(adding))
      return [counter, true];
    }
    if (this.children.length == 0){
      return [1, false];
    }
    let sum = 1;
    for (let child of this.children){
      let [count, res] = child.add(n, adding, counter + sum);
      if (res == true) return [count, true];
      sum += count;
    }
    return [sum, false];
  }
  get_index(n: number, counter: number){
    if (n == this.val){
      return [counter, true];
    }
    if (this.children.length == 0){
      return [1, false];
    }
    let sum = 1;
    for (let child of this.children){
      let [count, res] = child.get_index(n, counter + sum);
      if (res) return [count, true];
      sum += count;
    }
    return [sum, false];
  }
  get_node(n: number, counter: number){
    if (n == this.val){
      return [counter, true, this];
    }
    if (this.children.length == 0){
      return [1, false, null];
    }
    let sum = 1;
    for (let child of this.children){
      let [count, res] = child.get_node(n, counter + sum);
      if (res) return [count, true, child];
      sum += count;
    }
    return [sum, false, null];
  }
  num_children(){
    if (this.children.length == 0 && this.val != -1){
      return 1;
    }
    let sum = 0;
    for (let child of this.children){
      let count = child.num_children();
      sum += count;
    }
    if (this.val != -1){
      return sum + 1;
    }
    else {
      return sum;
    }
  }
  remove(n: number){
    if (this.children.length == 0){
      return;
    }
    for (let i = 0; i < this.children.length; i++){
      if (this.children[i].val == n){
        this.children.splice(i,1);
        return;
      }
    }
    for (let child of this.children){
      child.remove(n);
    }
  }
  toString(){
    if (this.children.length == 0){
      return this.val.toString() + " ";
    }
    let ret = this.val.toString() + "["
    for (let child of this.children){
      ret += child.toString();
    }
    return ret + "]";
  }
  getMax(){
    if (this.children.length == 0){
      return this.val;
    }
    let max = -1;
    for (let child of this.children){
      let child_max = child.getMax();
      if (child_max > max){
        max = child_max;
      }
    }
    return max;
  }

}

export let rotate = (i: number, force: boolean)=>{return true;}
export let set_rotation = (i:SetStateAction<number>)=>{};
export let set_temp_rotation = (i:SetStateAction<number>)=>{};
export let setup_focus = ()=>{return 0;}
export let get_focus = (i:number)=>{return 0;}
export let get_rotation = ()=>{return 0;}
export let remove_focus = (i:number)=>{}
export let peak_focus = ()=>{return 0;}

export function useFocus(){
  const ref = useRef(null);
  const f = useRef(0);
  const focus = useRef(0)
  useEffect(()=>{
    f.current = setup_focus();
    return ()=>{remove_focus(f.current);}
  }, [])
  useEffect(()=>{
    focus.current = get_focus(f.current);
    if (ref)
    if(focus.current === get_rotation()){
      ref.current.focus();
    }
  })
  return [focus, ref];
}

export function useFocusRoot(){
  let focus = useRef(new FocusTree(-1));
  const [rotation, setRotation] = useState(0);
  const [temp_rotation, setTempRotation] = useState(-2);
  rotate = (i: number, force: boolean) =>{
    // console.log(focus.current.get_node(rotation,-1)[2].num_children());
    setTempRotation(-2);

    if (force){
      setRotation(rotation+i);
    }
    else{
      const len = focus.current.num_children();
      if (len != 0 && (rotation+i) >= (len)) return false;
      else if (rotation+i < 0) return false;
      else {
        setRotation(rotation+i);
      }
    }
    return true;
  }
  set_rotation = setRotation;
  set_temp_rotation = setTempRotation;
  get_rotation = ()=>{
    return rotation;
  };
  get_focus = (n:number)=>{
    let ret = focus.current.get_index(n,-1);
    return ret[0];
  }
  setup_focus = ()=>{
    let rot = rotation - 1;
    if (temp_rotation != -2) {
        rot = temp_rotation;
    }
    const ret_id = focus.current.getMax() + 1;
    focus.current.add(Math.max(rot,-1),ret_id,-1);
    console.log(focus.current.toString(),rot);
    return ret_id;
  };
  remove_focus = (i: number)=>{
    focus.current.remove(i);
  };
  peak_focus = ()=>{
    return focus.current.getMax();
  }
  return focus;
    
}