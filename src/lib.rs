#[derive(Clone, Copy)]
pub enum Move {
    Left,
    Right,
    Halt,
}
#[derive(Debug)]
pub enum MachineError {
    OutOfBound(String),
}
pub struct MachineState {
    write_sym: char,
    goto: Move,
    next: usize,
}
pub struct Transition(pub fn(char) -> MachineState);
impl MachineState {
    pub fn new(sym: char, goto: Move, next: usize) -> Self {
        Self {
            write_sym: sym,
            goto,
            next,
        }
    }
    pub fn write(&self) -> char {
        self.write_sym
    }
    pub fn goto(&self) -> Move {
        self.goto
    }
    pub fn next(&self) -> usize {
        self.next
    }
}
pub struct Tape {
    pub body: [char; 256],
    pos: Option<usize>,
    state_pos: usize,
    halted: bool,
    len: usize,
}
impl Tape {
    pub fn new(c: char, pos: usize) -> Self {
        Self {
            body: [c; 256],
            pos: Some(pos),
            state_pos: 0,
            halted: false,
            len: 0,
        }
    }
    pub fn get_mut(&mut self, i: usize) -> &mut char {
        &mut self.body[i]
    }
    pub fn get_final_pos(&self) -> usize {
        self.state_pos
    }
    pub fn iter(&self) -> std::slice::Iter<'_, char> {
        self.body.iter()
    }

    fn exec_state(&mut self, state: &Transition) -> Result<(), MachineError> {
        let pos = match self.pos {
            Some(p) => p,
            None => return Err(MachineError::OutOfBound(String::from("Position is None."))),
        };
        let machine_state = state.0(self.body[pos]);
        self.body[pos] = machine_state.write();
        match machine_state.goto() {
            Move::Left if pos <= 0 => {
                return Err(MachineError::OutOfBound(String::from(
                    "You can not got left.",
                )))
            }
            Move::Left => self.pos = Some(pos - 1),
            Move::Right if pos > self.body.len() - 1 => {
                return Err(MachineError::OutOfBound(String::from(
                    "You cannot go right.",
                )))
            }
            Move::Right => self.pos = Some(pos + 1),
            Move::Halt => self.halted = true,
        }
        self.len += 1;
        self.state_pos = machine_state.next();
        Ok(())
    }
    pub fn start(&mut self, states: &[Transition]) -> Result<(), MachineError> {
        //start at state 0
        //println!("{self}");
        while !self.halted {
            self.exec_state(&states[self.state_pos])?;
            //println!("{:?}", self);
        }
        //println!("STATE: {}", self.state_pos);
        Ok(())
    }
    pub fn is_halted(&self) -> bool {
        self.halted
    }
}

impl std::fmt::Debug for Tape {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.body)
    }
}
impl std::fmt::Display for Tape {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (i, c) in self.iter().enumerate() {
            if i >= self.len - 1 {
                break;
            }
            write!(f, " [{}] ", c)?;
        }
        Ok(())
    }
}

//Some utility functions
pub fn to_bits(num: i32) -> [char; 32] {
    let mut bits = ['0'; 32];
    for i in 0..32 {
        if num >> i & 1 == 1 {
            bits[bits.len() - 1 - i] = '1'
        }
    }
    return bits;
}
