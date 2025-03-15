use turing_machine::*;
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_halted() {
        let states = [
            //pub fn new(sym: char, goto: Move, next: usize)
            Transition(|c: char| {
                if c == '1' {
                    return MachineState::new('1', Move::Left, 2);
                }
                MachineState::new('1', Move::Halt, 2)
            }),
        ];
        let mut machine = Tape::new('0', 0);
        let _ = machine.start(&states);
        //println!("{machine}");
        //assert_eq!(1, 2);
        assert_eq!(machine.is_halted(), true);
    }
    #[test]
    fn test_reverse_binary() {
        let mut tape = Tape::new('_', 0);
        let num = 5554;
        for (i, bit) in to_bits(num).iter().enumerate() {
            *tape.get_mut(i) = *bit;
        }
        let states = [
            //Step 1
            Transition(|c: char| {
                let state = match c {
                    '_' => MachineState::new('_', Move::Halt, 0),
                    '0' => MachineState::new('1', Move::Right, 0),
                    '1' => MachineState::new('0', Move::Right, 0),
                    _ => MachineState::new('E', Move::Halt, 0),
                };
                state
            }),
        ];
        match tape.start(&states) {
            Ok(()) => println!("{tape}"),
            Err(e) => println!("{:?}", e),
        }
        let body_1 = tape.body.clone();

        for (i, bit) in to_bits(!num).iter().enumerate() {
            *tape.get_mut(i) = *bit;
        }
        let body_2 = tape.body.clone();
        assert_eq!(body_1, body_2);
    }
    #[test]
    fn test_comparation() {
        let states = [
            //Step-0: Convert 1 into X and move right and goto step 1. If symbol is 0 ignore it and move right and goto step-5.
            Transition(|c| {
                //println!("In state 0");
                if c == '1' {
                    return MachineState::new('X', Move::Right, 1);
                }
                MachineState::new('0', Move::Right, 5)
            }),
            //Step-1: Keep ignoring 1 and move towards right. Ignore 0 move right and goto step-2.
            Transition(|c| {
                //println!("In state 1");
                if c == '1' {
                    return MachineState::new('1', Move::Right, 1);
                }
                MachineState::new('0', Move::Right, 2)
            }),
            //Step-2: Keep ignoring X and move towards right. Ignore 1 move left and goto step-3. If B is found ignore it and move left and goto step-7.
            Transition(|c| {
                //println!("In state 2");
                if c == 'X' {
                    return MachineState::new('X', Move::Right, 2);
                } else if c == '1' {
                    return MachineState::new('X', Move::Left, 3);
                }
                MachineState::new('_', Move::Left, 7)
            }),
            //Step-3: Keep ignoring X and move towards left. Ignore 0 move left and goto step-4.
            Transition(|c| {
                //println!("In state 3");
                if c == 'X' {
                    return MachineState::new('X', Move::Left, 3);
                }
                MachineState::new('0', Move::Left, 4)
            }),
            //Step-4: Keep ignoring 1 and move towards left. Ignore X move right and goto step-0.
            Transition(|c| {
                //println!("In state 4");
                if c == '1' {
                    return MachineState::new('1', Move::Left, 4);
                }
                MachineState::new('X', Move::Right, 0)
            }),
            //Step-5: Keep ignoring X and move towards right. If B is found ignore it and move left and goto step-8. If 1 is found ignore it and move right and goto step-6.
            Transition(|c| {
                //println!("In state 5");
                if c == 'X' {
                    return MachineState::new('X', Move::Right, 5);
                } else if c == '_' {
                    return MachineState::new('_', Move::Left, 8);
                }
                MachineState::new('1', Move::Right, 6)
            }),
            //Step-6: Stop the Machine (A < B)
            Transition(|_c| {
                //println!("In state 6");
                //println!("A less than B");
                return MachineState::new('_', Move::Halt, 6);
            }),
            //Step-7: Stop the Machine (A > B)
            Transition(|_c| {
                //println!("In state 7");
                //println!("A greater than B");
                return MachineState::new('_', Move::Halt, 7);
            }),
            //Step-8: Stop the Machine (A = B)
            Transition(|_c| {
                //println!("In state 8");
                //println!("A equal B");
                return MachineState::new('_', Move::Halt, 8);
            }),
        ];
        let mut tape = Tape::new('_', 1);
        //2 and 3 [110111]
        //5 & 5 [11111011111]
        //10 & 5 [1111111111011111]
        let chunks = [
            "110111".chars().collect::<Vec<char>>(),
            "1111111111011111".chars().collect::<Vec<char>>(),
            "11111011111".chars().collect::<Vec<char>>(),
        ];

        for (i, chunk) in chunks.iter().enumerate() {
            tape.body[1..1 + chunk.len()].clone_from_slice(chunk);
            let _ = tape.start(&states);
            assert_eq!(i + 6, tape.get_final_pos());
            //reset tape
            tape = Tape::new('_', 1);
        }
    }
}
