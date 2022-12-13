
export abstract class AST_node{
    constructor(parent: AST_node | undefined){
        this.parent = parent;
    }
    abstract child: any;
    parent?: AST_node;
}


export class program_node extends AST_node {
  constructor(parent: AST_node|undefined){
      super(parent);
  }
  //accept = (v : Visitor) => { v.visitprogram_node(this); }
  child: expr_node[]|[] = [];
}


export class expr_node extends AST_node {
  constructor(parent: AST_node|undefined){
      super(parent);
  }
  //accept = (v : Visitor) => { v.visitexpr_node(this); }
  child: const_node|binary_node|[] = [];
}


export class binary_node extends AST_node {
  constructor(parent: AST_node|undefined){
      super(parent);
  }
  //accept = (v : Visitor) => { v.visitbinary_node(this); }
  child: [string,expr_node,binaryOp_node,expr_node,string]|[] = [];
}


export class binaryOp_node extends AST_node {
  constructor(parent: AST_node|undefined){
      super(parent);
  }
  //accept = (v : Visitor) => { v.visitbinaryOp_node(this); }
  child: string|string|string|string|[] = [];
}


export class const_node extends AST_node {
  constructor(parent: AST_node|undefined){
      super(parent);
  }
  //accept = (v : Visitor) => { v.visitconst_node(this); }
  child: string|[] = [];
}

