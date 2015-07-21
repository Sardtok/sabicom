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

    fn adc(&mut self, address: usize) {
    }

    fn and(&mut self, address: usize) {
    }

    fn asl(&mut self, address: usize) {
    }

    fn bcc(&mut self, address: usize) {
    }

    fn bcs(&mut self, address: usize) {
    }

    fn beq(&mut self, address: usize) {
    }

    fn bit(&mut self, address: usize) {
    }

    fn bmi(&mut self, address: usize) {
    }

    fn bne(&mut self, address: usize) {
    }

    fn bpl(&mut self, address: usize) {
    }
    
    fn brk(&mut self) {
        self.flag_i = false;        
    }

    fn bvc(&mut self, address: usize) {
    }

    fn bvs(&mut self, address: usize) {
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

    fn cmp(&mut self, address: usize) {
    }

    fn cpx(&mut self, address: usize) {
    }

    fn cpy(&mut self, address: usize) {
    }

    fn dec(&mut self) {
    }

    fn dex(&mut self) {
    }

    fn dey(&mut self) {
    }

    fn eor(&mut self, address: usize) {
    }

    fn inc(&mut self) {
    }

    fn inx(&mut self) {
    }

    fn iny(&mut self) {
    }

    fn jmp(&mut self, address: usize) {
    }

    fn jsr(&mut self, address: usize) {
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
    
    fn lsr(&mut self, address: usize) {
    }

    fn ora(&mut self, address: usize) {
    }

    fn pha(&mut self) {
    }

    fn php(&mut self) {
    }

    fn pla(&mut self) {
    }

    fn plp(&mut self) {
    }

    fn rol(&mut self, address: usize) {
    }

    fn ror(&mut self, address: usize) {
    }

    fn rti(&mut self) {
    }

    fn rts(&mut self) {
    }

    fn sbc(&mut self, address: usize) {
    }

    fn sec(&mut self) {
    }

    fn sed(&mut self) {
    }

    fn sei(&mut self) {
    }

    fn sta(&mut self, address: usize) {
    }

    fn stx(&mut self, address: usize) {
    }

    fn sty(&mut self, address: usize) {
    }

    fn tax(&mut self) {
    }

    fn tay(&mut self) {
    }

    fn tsx(&mut self) {
    }

    fn txa(&mut self) {
    }

    fn txs(&mut self) {
    }

    fn tya(&mut self) {
    }
}
