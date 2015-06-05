pub struct CPU2A03 {
    a: u8,
    x: u8,
    y: u8,
    pc: u16,
    cc: u16,
    mem: Vec<u8>,
    c: bool,
    z: bool,
    i: bool,
    d: bool,
    b: bool,
    the_one: bool,
    v: bool,
    s: bool
}

impl CPU2A03 {
    pub fn new() -> CPU2A03 {
        CPU2A03 {
            a:       0,
            x:       0,
            y:       0,
            pc:      0,
            cc:      0,
            mem:     vec![0; 65536],
            c:       false,
            z:       false,
            i:       false,
            d:       false,
            b:       false,
            the_one: true,
            v:       false,
            s:       false
        }
    }
    
    fn cli(&mut self) {
        self.i = false;
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
