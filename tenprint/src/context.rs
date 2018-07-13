use brainfuck::ast::{Block, Node};

use std::io::{self, Read};

type BufSize = i8;

#[derive(Debug, PartialEq)]
pub struct Context {
    lbuf: Vec<BufSize>,
    rbuf: Vec<BufSize>,
    index: i64,
    pub out: String,
    pub tick: u64,
    pub loop_count: u64,
    max_tick: u64,
    max_loop: u64,
}

impl Context {
    pub fn new(max_tick: u64, max_loop: u64) -> Context {
        Context {
            lbuf: Vec::new(),
            rbuf: vec![0],
            index: 0,
            out: String::new(),
            tick: 0,
            loop_count: 0,
            max_tick: max_tick,
            max_loop: max_loop
        }
    }

    fn get(&self) -> Option<&BufSize> {
        if self.index < 0 {
            self.lbuf.get((-self.index - 1) as usize)
        } else {
            self.rbuf.get(self.index as usize)
        }
    }

    fn get_mut(&mut self) -> Option<&mut BufSize> {
        if self.index < 0 {
            self.lbuf.get_mut((-self.index - 1) as usize)
        } else {
            self.rbuf.get_mut(self.index as usize)
        }
    }

    fn loop_cond(&self) -> bool {
        self.get().map(|e| *e != 0).unwrap_or(false)
    }

    fn run_node(&mut self, node: &Node){
        if self.tick > self.max_tick{
            return;
        }
        match *node {
            Node::LShift => {
                self.index -= 1;
                if self.index < 0 {
                    while self.lbuf.len() <= ((-self.index - 1) as usize) {
                        self.lbuf.push(0);
                    }
                }
            }
            Node::RShift => {
                self.index += 1;
                if self.index >= 0 {
                    while self.rbuf.len() <= (self.index as usize) {
                        self.rbuf.push(0);
                    }
                }
            }
            Node::Inc => {
                if let Some(elem) = self.get_mut() {
                    *elem += 1;
                }
            }
            Node::Dec => {
                if let Some(elem) = self.get_mut() {
                    *elem -= 1;
                }
            }
            Node::PutCh => {
                let mut out:Option<char> = None;
                if let Some(elem) = self.get_mut() {
                    //print!("{}", (*elem as u8) as char);
                    out = Some((*elem as u8) as char);
                }
                if let Some(ch) = out{
                    self.out.push(ch);
                }
            }
            Node::GetCh => {
                let mut buffer = [0; 1];
                io::stdin()
                    .read_exact(&mut buffer)
                    .expect("Failed to read from stdin");
                if let Some(elem) = self.get_mut() {
                    *elem = buffer[0] as BufSize;
                }
            }
            Node::Loop(ref block) => {
                self.loop_count = 0;
                while self.loop_cond() {
                    self.run(block);
                    if self.loop_count>self.max_loop{
                        break;
                    }
                    self.loop_count += 1;
                }
            }
        }
        self.tick += 1;
    }

    pub fn run(&mut self, block: &Block){
        for node in block.into_iter() {
            self.run_node(node);
        }
    }
}
