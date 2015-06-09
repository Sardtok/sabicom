pub struct CPU2A03 {
    a: u8,
    x: u8,
    y: u8,
    sp: u8,
    pc: u16,
    cc: u16,
    mem: Vec<u8>,
    flag_c: bool,
    flag_z: bool,
    flag_i: bool,
    flag_d: bool,
    flag_b: bool,
    flag_1: bool,
    flag_v: bool,
    flag_s: bool
}

impl CPU2A03 {
    pub fn new() -> CPU2A03 {
        CPU2A03 {
            a:       0,
            x:       0,
            y:       0,
            sp:      0,
            pc:      0,
            cc:      0,
            mem:     vec![0; 65536],
            flag_c:  false,
            flag_z:  false,
            flag_i:  false,
            flag_d:  false,
            flag_b:  false,
            flag_1:  true,
            flag_v:  false,
            flag_s:  false
        }
    }

    fn brk(&mut self) {
        self.flag_i = true;
    }

    fn clc(&mut self) {
        self.flag_c = false;
        self.cc += 1;
    }

    fn cld(&mut self) {
        self.flag_d = false;
        self.cc += 1;
    }
    
    fn cli(&mut self) {
        self.flag_i = false;
        self.cc += 1;
    }

    fn clv(&mut self) {
        self.flag_v = false;
        self.cc += 1;
    }

    fn lda(&mut self, address: usize) {
        self.a = self.mem[address];
        self.cc += 1;
    }

    fn ldx(&mut self, address: usize) {
        self.x = self.mem[address];
        self.cc += 1;
    }

    fn ldy(&mut self, address: usize) {
        self.y = self.mem[address];
        self.cc += 1;
    }
}
