#!/bin/python3
from sys import argv, maxsize
SIZE = 8 if maxsize > 2**32 else 4
'''
  pop
  push
  add
  sub
  read
  write
  jz
  dup
  swap
  halt
'''
inst_to_opcode = {
  'push' : 1,
  'pop'  : 2,
  'add'  : 3,
  'sub'  : 4,
  'read' : 5,
  'write': 6,
  'jz'   : 7,
  'dup'  : 8,
  'swap' : 9,
  'halt' : 0,
}

symbol_to_inst = {
  'pop' :'pop'   ,
  '+'   :'add'   ,
  '-'   :'sub'   ,
  ','   :'read'  ,
  '.'   :'write' ,
  'jz'  :'jz'    ,
  'dup' :'dup'   ,
  'swap':'swap'  ,
  'end' :'halt'  ,
}

def parser(tokens):
  parsed = [] # (line , col , ("push", 32)) (line, col, ('pop', -1))
  for line, col, inst in tokens:
    if inst.isnumeric():
      parsed.append((line,col,("push",int(inst))))
    else:
      try:
        parsed.append((line,col,(symbol_to_inst[inst],-1)))
      except KeyError:
        print(f"Unknown Symbol '{inst}' at {line}:{col}")
        exit(1)
  return parsed

# Add compile time checks
def compiler(filename, parsed):
  compiled = []
  for line, col, instruction in parsed:
    (inst,val) = instruction
    compiled.append(inst_to_opcode[inst].to_bytes(1,byteorder='little'))
    if val > 0:
      compiled.append(val.to_bytes(SIZE, byteorder='little'))
  with open(filename, 'wb') as f:
    for i in compiled:
      f.write(i)
    
def tokenize(s):
  tokens = []
  for i, val in enumerate(s.split('\n')):
    comment_removed = val.split('//')[0]
    for j,val in enumerate(comment_removed.split(' ')):
      if val.isspace() or val == '': continue
      tokens.append((i+1,j+1,val))
  return tokens


def main():
  error = "Improper Usage";
  if len(argv) < 2:
    print(error)
    exit(1)
  filename = argv[1]
  with open(filename) as f:
    l = f.read()
  final_filename = '.'.join([filename.split('.')[0],'xx'])
  compiler(final_filename, parser(tokenize(l)))

if __name__ == "__main__":
  main()
