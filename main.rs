#[derive(Copy)]
#[derive(Clone)]
#[derive(Debug)]
enum Inst{
  push(usize),
  pop,
  add,
  sub,
  read,
  write,
  jz,
  dup,
  swap,
  halt
}

#[derive(Debug)]
enum State{
  Okay,
  Stack_underflow,
  Stack_overflow,
  Unsolved_error,
  Correct_jump,
  Incorrect_jump,
  Wrong_input,
  Correct_input,

}

struct VM{
  stack_pointer : usize,
  stack : [usize;1024],
  inst_pointer : usize,
  inst: [Inst;1024],
}

impl VM{
  fn create_vm_with_inst(inst : [Inst;1024]) -> VM{
    return VM {
      stack_pointer : 0,
      stack : [0;1024],
      inst_pointer : 0,
      inst
    };
  }
}

// Functions
fn add(stack : &mut [usize;1024], stack_pointer: &mut usize , inst_pointer: &mut usize){
  *stack_pointer-=1;
  let x = stack[*stack_pointer];
  *stack_pointer-=1;
  let y = stack[*stack_pointer];
  stack[*stack_pointer] = x+y;
  *stack_pointer+=1;
  *inst_pointer+=1;
}

fn sub(stack : &mut [usize;1024], stack_pointer: &mut usize , inst_pointer: &mut usize){
  *stack_pointer-=1;
  let x = stack[*stack_pointer];
  *stack_pointer-=1;
  let y = stack[*stack_pointer];
  stack[*stack_pointer] = y-x;
  *stack_pointer+=1;
  *inst_pointer+=1;
}

fn push(stack : &mut [usize;1024], stack_pointer: &mut usize , inst_pointer: &mut usize, val :usize){
  stack[*stack_pointer] = val;
  *stack_pointer+=1;
  *inst_pointer+=1;
}

fn pop(stack : &mut [usize;1024], stack_pointer: &mut usize , inst_pointer: &mut usize){
  *stack_pointer-=1;
  *inst_pointer+=1;
}

fn read(stack : &mut [usize;1024], stack_pointer: &mut usize , inst_pointer: &mut usize) -> State{
  let mut x = String::new();
  match std::io::stdin()
    .read_line(&mut x) {
      Err(..) => {
        return State::Wrong_input;
      }
      Ok(..) =>{
        match x.trim().parse::<usize>() {
          Ok(number) => {
            stack[*stack_pointer] = number;
            *stack_pointer += 1;
            *inst_pointer += 1;
            return State::Correct_input;
          }
          Err(..) => {
            return State::Wrong_input;
          }
        }
      }
    }
}


fn write(stack : &mut [usize;1024], stack_pointer: &mut usize , inst_pointer: &mut usize){
  *stack_pointer-=1;
  println!("{}",stack[*stack_pointer]);
  *inst_pointer += 1;
}

fn jz(stack : &mut [usize;1024], stack_pointer: &mut usize , inst_pointer: &mut usize) -> State {
  *stack_pointer-=1;
  let address = stack[*stack_pointer];
  *stack_pointer-=1;
  let current = stack[*stack_pointer];
  if current == 0 {
    if address <= 255 {
      *inst_pointer = address;
      return State::Correct_jump;
    }
    else {
      return State::Incorrect_jump;
    }
  }
  else {
    *inst_pointer+=1;
    return State::Correct_jump;
  }
}

fn dup(stack : &mut [usize;1024], stack_pointer: &mut usize , inst_pointer: &mut usize){
  let x = stack[*stack_pointer-1];
  stack[*stack_pointer] = x;
  *stack_pointer+=1;
  *inst_pointer+=1;
}

fn swap(stack : &mut [usize;1024], stack_pointer: &mut usize , inst_pointer: &mut usize){
  let x = stack[*stack_pointer-1];
  let y = stack[*stack_pointer-2];
  stack[*stack_pointer-2] = x;
  stack[*stack_pointer-1] = y;
  *inst_pointer+=1;
}

fn run (vm :&mut VM) -> State{
  loop{
    match vm.inst[vm.inst_pointer] {
      Inst::add =>{
        if vm.stack_pointer < 2 {
          return State::Stack_underflow;
        }
        add(&mut vm.stack, &mut vm.stack_pointer, &mut vm.inst_pointer);
      },
      Inst::sub =>{
        if vm.stack_pointer < 2 {
          return State::Stack_underflow;
        }
        sub(&mut vm.stack, &mut vm.stack_pointer, &mut vm.inst_pointer);
      },
      Inst::pop =>{
        if vm.stack_pointer < 1 {
          return State::Stack_underflow;
        }
        pop(&mut vm.stack, &mut vm.stack_pointer, &mut vm.inst_pointer);
      },
      Inst::push(val) => {
        if vm.stack_pointer > 1023{
          return State::Stack_overflow;
        }
        push(&mut vm.stack, &mut vm.stack_pointer, &mut vm.inst_pointer, val);
      },
      Inst::jz => {
        if vm.stack_pointer < 1 {
          return State::Stack_underflow;
        }
        let reply = jz(&mut vm.stack, &mut vm.stack_pointer, &mut vm.inst_pointer);
        match reply{
          State::Incorrect_jump => {
            return State::Incorrect_jump;
          }
          _ =>{}
        }
      },
      Inst::read => {
        if vm.stack_pointer > 1023{
          return State::Stack_overflow;
        }

        match read(&mut vm.stack, &mut vm.stack_pointer, &mut vm.inst_pointer) {
          State::Wrong_input => {
            return State::Wrong_input;
          }
          _=>{}
        }
      },
      Inst::write => {
        if vm.stack_pointer < 1 {
          return State::Stack_underflow;
        }
        write(&mut vm.stack, &mut vm.stack_pointer, &mut vm.inst_pointer);
      },
      Inst::halt => {
        return State::Okay;
      },
      Inst::dup =>{
        if vm.stack_pointer < 1 {
          return State::Stack_underflow;
        }
        if vm.stack_pointer > 1023{
          return State::Stack_overflow;
        }
        dup(&mut vm.stack, &mut vm.stack_pointer, &mut vm.inst_pointer);
      },
      Inst::swap => {
        if vm.stack_pointer < 2 {
          return State::Stack_underflow;
        }
        swap(&mut vm.stack, &mut vm.stack_pointer, &mut vm.inst_pointer);
      },

    }
  }
}

use std::io;
use std::io::prelude::*;
use std::fs::File;

enum Compilation_status {
  Okay,
  Inst_out_of_memory,
  Unknown_instruction,
}


fn from_vec_to_inst(vec :&Vec<u8>, inst : &mut [Inst;1024]) -> Compilation_status {
  let SIZE = usize::BITS/8;
  let mut i = 0;
  let mut inst_counter = 0;
  let mut unknown = false;
  while i < vec.len() && !unknown {
    if inst_counter >= 1024{
      return Compilation_status::Inst_out_of_memory;
    }
    inst[inst_counter] = match vec[i]{
      0 => Inst::halt,
      1 => {
        let mut num:usize= 0;
        for j in 0..SIZE {
          i+=1;
          num |= (vec[i] as usize)<<(8*j);
        }
        Inst::push(num)
      }
      2 => Inst::pop,
      3 => Inst::add,
      4 => Inst::sub,
      5 => Inst::read,
      6 => Inst::write,
      7 => Inst::jz,
      8 => Inst::dup,
      9 => Inst::swap,
      _ => {unknown = true; Inst::halt},
    };
    i+=1;
    inst_counter+=1;
  }
    if unknown { return Compilation_status::Unknown_instruction;}
    Compilation_status::Okay
}

use std::env;

fn main(){
  let args: Vec<String> = env::args().collect();
  let mut inst : [Inst;1024] = [Inst::halt;1024];

  if args.len() < 2{
    println!("Usage, ./main [FILE]");
    return;
  }
  let mut file = File::open(&args[1]).unwrap();

  let mut content:Vec<u8> = Vec::new();
  file.read_to_end(&mut content)
    .expect("Could not read file");

  match from_vec_to_inst(&content, &mut inst) {
    Compilation_status::Okay => {},
    Compilation_status::Inst_out_of_memory =>{
      println!("Instruction is out of memory");
      return;
    }
    Compilation_status::Unknown_instruction =>{
      println!("Instruction is out of memory");
      return;
    }
  }
  let mut vm: VM = VM::create_vm_with_inst(inst);
  
  let output = run(&mut vm);
  println!("Output : {:?}",output);
  let final_stack_value = if vm.stack_pointer  >= 1 
  {vm.stack[vm.stack_pointer-1]}
    else {vm.stack[0]};
  println!("Final Stack Value : {final_stack_value}");
}

