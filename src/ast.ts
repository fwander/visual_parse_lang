export abstract class AST_node{
    constructor(parent: AST_node | undefined){
        this.parent = parent;
    }
    parent?: AST_node;
}

export class program_node extends AST_node{
    constructor(parent: AST_node | undefined){
        super(parent);
    }
    child: expr_node;
}

export class expr_node extends AST_node{
    constructor(parent: AST_node | undefined){
        super(parent);
    }
    child: const_node | binary_node;
}

export class const_node extends AST_node{
    constructor(parent: AST_node | undefined){
        super(parent);
    }
    child: string;
}

export class binary_node extends AST_node{
    constructor(parent: AST_node | undefined){
        super(parent);
    }
    child: [expr_node,binaryOp_node,expr_node];
}

export class binaryOp_node extends AST_node{
    constructor(parent: AST_node | undefined){
        super(parent);
    }
    child: string|string|string|string;
}