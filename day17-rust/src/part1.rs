

const INPUT: &str = include_str!("./input.txt");

pub fn run() {
    let mut program = parse_input(INPUT);
    program.exec();
}


struct Program {
    a: u32,
    b: u32,
    c: u32,
    code: Vec<u8>,
}

impl Program {
    pub fn new(a: u32, b: u32, c: u32, code: Vec<u8>) -> Self {
        Self { a, b, c, code }
    }

    pub fn exec(&mut self) {
        let mut index = 0;
        while index < self.code.len() {
            let operand = self.code[index + 1];
            match self.code[index] {
                OP_ADV => self.a = self.perf_div(operand),
                OP_BDV => self.b = self.perf_div(operand),
                OP_CDV => self.c = self.perf_div(operand),
                OP_BXC => self.perf_bxc(),
                OP_BXL => self.perf_bxl(operand),
                OP_BST => self.perf_bst(operand),
                OP_OUT => self.perf_out(operand),
                OP_JNZ if self.a == 0 => (),
                OP_JNZ => {
                    index = operand as usize;
                    continue;
                },
                _ => unreachable!(),
            }

            index += 2;
        }
    }

    fn perf_out(&self, operand: u8) {
        let combo = self.combo(operand) % 8;
        print!("{},", combo);
    }

    fn perf_bst(&mut self, operand: u8) {
        self.b = self.combo(operand) % 8;
    }

    fn perf_div(&self, operand: u8) -> u32 {
        let den = 2u32.pow(self.combo(operand));
        self.a / den
    }

    fn perf_bxl(&mut self, operand: u8) {
        self.b = self.b ^ operand as u32;
    }

    fn perf_bxc(&mut self) {
        self.b = self.b ^ self.c;
    }

    fn combo(&self, operand: u8) -> u32 {
        match operand {
            0..=3 => operand as u32,
            4 => self.a,
            5 => self.b,
            6 => self.c,
            _ => unreachable!(),
        }
    }
}

fn parse_input(input: &str) -> Program {
    let mut iter = input.split("\r\n\r\n");
    let [a, b, c] = parse_regs(iter.next().unwrap());
    let ins = parse_ins(iter.next().unwrap());

    Program::new(a, b, c, ins)
}

fn parse_regs(input: &str) -> [u32; 3] {
    input.split("\r\n")
        .map(|l| l.split(" ")
            .nth(2).unwrap()
            .parse().unwrap()
        )
        .collect::<Vec<u32>>()
        .try_into().unwrap()
}

fn parse_ins(input: &str) -> Vec<u8> {
    input.split(" ")
        .nth(1).unwrap()
        .split(",")
        .map(|x| x.parse().unwrap())
        .collect()
}

const OP_ADV: u8 = 0;
const OP_BXL: u8 = 1;
const OP_BST: u8 = 2;
const OP_JNZ: u8 = 3;
const OP_BXC: u8 = 4;
const OP_OUT: u8 = 5;
const OP_BDV: u8 = 6;
const OP_CDV: u8 = 7;