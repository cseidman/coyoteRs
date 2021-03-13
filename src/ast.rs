/* AST Definition*/

use crate::value::* ;
use crate::value::Value::* ;
use crate::ast::node::N_CONST;<>
use crate::ast::ast_node::Constant;

type MemoryLocation = usize ;
type CodeLocation = usize ;

/**
p_node contains a binary representation of the
data which can be emitted directly to either
a constants pool, value type, or compound object (such as a function)
*/

enum node {
    N_VALUE(Value),
    N_BINOP(Binop)
}


pub struct ast_tree {
    node_ptr: usize,          // The current position we're in if we're traversing
    ast_nodes: Vec<node>  // Vector of nodes
}

impl ast_tree {
    // ast::tree.new()
    pub fn new() -> ast_tree {
        return ast_tree{node_ptr: 0, ast_nodes: Vec::<node>::new() } ;
    }

    pub fn push(&mut self, n: node) {
        self.ast_nodes.push(n) ;
        self.node_ptr+=1 ;
    }
    /**
    Check the type of the values in the preceding nodes
    */
    pub fn get_ast_value_type(&self, number_of_nodes_to_seek: i32 , current_position: usize) -> Vec<ValueType> {
        let mut ptr:usize = 0 ;
        let mut vtypes:Vec<ValueType> = Vec::new() ;
        loop {
            ptr += 1 ;
            match &self.ast_nodes[current_position-1]  {
                Constant  => vtypes.push(self.ast_nodes[current_position-1].)
                _ => panic!() ;
            }
        }
    }

}

/*
This tell us what the node contains. Depending on the value type, the node in the tree
will look and behave differently

N_CONST: These are simply value types which we then use as constants
N_BINOP: *,+,-,/, and so on.

 */

macro_rules! AST_CONSTANT {
    {$op:tt} => {
        self.push($valType!((lval $op rval)))
    }
}

/* =============== Unit tests ================= */

#[cfg(test)]
mod tests {

    use crate::ast::* ;
    #[test]
    fn new_tree() {
        let mut tree = ast_tree::new() ;
        assert_eq!(tree.node_ptr,0) ;

        // Add a number node
        tree.push(N_CONST(INTEGER(42))) ;
        tree.push(N_CONST(INTEGER(43))) ;
        tree.push(N_CONST(STRING("HEY"))) ;

        assert_eq!(tree.ast_nodes[0],N_CONST(INTEGER(42)), "Test INTEGER node == 42");
        assert_eq!(tree.ast_nodes[1],N_CONST(INTEGER(43)), "Test INTEGER node == 43");
        assert_eq!(tree.ast_nodes[2],N_CONST(STRING("HEY")), "Test STRING node == 'HEY'");

    }
}