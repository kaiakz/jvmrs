use super::Slot;

pub struct Frame {
    max_local_variables: usize,
    max_operand_stack: usize,
    local_variables: Vec<Slot>,
    operand_stack: Vec<Slot>,
}

impl Frame {

    pub fn new(max_local_variables: usize, max_operand_stack: usize) -> Frame {
        let mut local_variables: Vec<Slot> = Vec::with_capacity(max_local_variables);
        // local_variables.resize(max_local_variables, Slot::Ref(None));

        for _i in 0..max_local_variables {
            local_variables.push(Slot::Ref);
        }

        let operand_stack: Vec<Slot> = Vec::with_capacity(max_operand_stack);
        
        Frame {
            max_local_variables: max_local_variables,
            max_operand_stack: max_operand_stack,
            local_variables: local_variables,
            operand_stack: operand_stack,
        }
    }


    pub fn local_variables_set(&mut self, index: usize, var: Slot) {
        // self.local_variables[index] = var;
        match self.local_variables.get_mut(index) {
            Some(i) => *i = var,
            None => panic!("out of local_variables bound"),
        }
    }

    pub fn local_variables_set_last(&mut self, var: Slot) {
        match self.local_variables.last_mut() {
            Some(last) => *last = var,
            None => panic!("local_variables is empty"),
        }
    }

    /// Clone
    pub fn local_variables_get(&mut self, index: usize) -> Slot {
        match self.local_variables.get(index) {
            Some(var) => var.clone(),
            None => panic!("out of local_variables bound"),
        }
        // self.local_variables[index]
    }

    pub fn operand_stack_push(&mut self, var: Slot) {
        if self.operand_stack.len() >= self.max_operand_stack {
            panic!("operand_stack overflows");
        }
        self.operand_stack.push(var);
    }
    
    pub fn operand_stack_pop(&mut self) -> Slot {
        match self.operand_stack.pop() {
            Some(var) => var,
            None => panic!("operand_stack is empty"),
        }
    }

    /// It will reverse tails
    pub fn operand_stack_extend(&mut self, tail: &mut Vec<Slot>) {
        tail.reverse();
        self.operand_stack.extend(tail.clone());
    }

}