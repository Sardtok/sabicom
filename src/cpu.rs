pub struct CPU2A03 {
    a: u8,
    x: u8,
    y: u8,
    sp: u8,
    pc: usize,
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

#[derive(Debug,Copy,Clone)]
enum Operand {
    Value(u8),
    Address(usize),
    A,
    X,
    Y,
    None,
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

    fn get_address_from_memory(&mut self, location: usize) -> usize {
        let hi: usize = self.mem[location] as usize;
        let lo: usize = self.mem[location + 1] as usize;
        (hi << 8) | lo
    }

    fn get_address_from_code(&mut self) -> usize {
        let hi: usize = self.mem[self.pc] as usize;
        let lo: usize = self.mem[self.pc + 1] as usize;
        self.pc += 2;
        (hi << 8) | lo
    }

    fn get_address(&mut self, op: Operand) -> usize {
        match op {
            Operand::Address(address) => address,
            _ => panic!("Not an address: {:?}", op)
        }
    }

    fn get_next_byte(&mut self) -> u8 {
        let value: u8 = self.mem[self.pc];
        self.pc += 1;
        value
    }

    fn get_value(&mut self, op: Operand) -> u8 {
        match op {
            Operand::Address(addr) => self.mem[addr],
            Operand::Value(val) => val,
            Operand::A => self.a,
            Operand::X => self.x,
            Operand::Y => self.y,
            Operand::None => 0
        }
    }

    fn set_value(&mut self, op: Operand, value: u8) {
        match op {
            Operand::Address(addr) => self.mem[addr] = value,
            Operand::A => self.a = value,
            Operand::X => self.x = value,
            Operand::Y => self.y = value,
            _ => ()
        }
    }

    fn get_instruction_operand(&mut self) -> Operand {
        // Addressing modes:
        // Bits 2 (4), 3 (8), 4 (16).
        // The choice of addressing mode also depends on bits 0 (1) and 1 (2),
        // which determines which set of operations are used.
        //
        //     00 01 10
        // 000 IM IX IM Varies
        // 001 ZP ZP ZP
        // 010    IM AC Varies
        // 011 AB AB AB
        // 100    IY
        // 101 ZX ZX ZX
        // 110    AY
        // 111 AX AX AX

        let instruction = self.get_next_byte();
        self.pc += 1;
        match (instruction >> 3) & 7 {
            0 =>
                if instruction & 1 == 0 {
                    Operand::Value(self.mem[self.pc])
                } else {
                    Operand::Address((self.get_next_byte() + self.x) as usize) // Zero page pre-indexed X
                },
            1 => Operand::Address(self.get_next_byte() as usize), // Zero page
            2 =>
                if instruction & 1 == 0 {
                    Operand::A
                } else {
                    Operand::Value(self.get_next_byte())
                },
            3 => Operand::Address(self.get_address_from_code()),
            4 => { // Zero page post-indexed Y
                let location = self.get_next_byte() as usize;
                Operand::Address((self.get_address_from_memory(location) + self.y as usize) & 255)
            },
            5 => { // Zero page indexed X
                let location = (self.get_next_byte() + self.x) as usize;
                Operand::Address(self.get_address_from_memory(location))
            },
            6 => Operand::Address(self.get_address_from_code() + self.y as usize), // Absolute indexed Y
            7 => Operand::Address(self.get_address_from_code() + self.x as usize), // Absolute indexed X
            _ => Operand::None // Impossible
        }
    }

    fn push(&mut self, value: u8) {
        self.mem[0x100 + self.sp as usize] = value;
        self.sp += 1
    }

    fn pull(&mut self) -> u8 {
        self.sp -= 1;
        self.mem[0x100 + self.sp as usize]
    }

    fn get_status(&self) -> u8 {
        (self.flag_c as u8)
            | (self.flag_z as u8) << 1
            | (self.flag_i as u8) << 2
            | (self.flag_d as u8) << 3
            | (self.flag_b as u8) << 4
            | (self.flag_1 as u8) << 5
            | (self.flag_v as u8) << 6
            | (self.flag_s as u8) << 7
    }

    fn set_status(&mut self, value: u8) {
        self.flag_c = value &   1 != 0;
        self.flag_z = value &   2 != 0;
        self.flag_i = value &   4 != 0;
        self.flag_d = value &   8 != 0;
        self.flag_b = value &  16 != 0;
        self.flag_1 = value &  32 != 0;
        self.flag_v = value &  64 != 0;
        self.flag_s = value & 128 != 0;
    }

    fn set_sign(&mut self, value: u8) {
        self.flag_s = value & 0x80 != 0
    }

    fn set_carry(&mut self, carry: bool) {
        self.flag_c = carry
    }

    fn set_overflow(&mut self, overflow: bool) {
        self.flag_v = overflow
    }

    fn set_zero(&mut self, zero: u8) {
        self.flag_z = zero == 0
    }

    fn set_interrupt_disable(&mut self, disable: bool) {
        self.flag_i = disable
    }

    fn set_decimal_mode(&mut self, decimal_mode: bool) {
        self.flag_d = decimal_mode
    }

    // Common code between different comparison instructions
    fn compare(&mut self, op: Operand, register: u8) {
        let value = self.get_value(op);
        let res = register - value;
        self.set_carry(value > register);
        self.set_sign(res);
        self.set_zero(res)
    }

    // OPCODES: 61 65 69 6D 71 75 79 7D
    fn adc(&mut self, op: Operand) {
        let value: u8 = self.get_value(op);
        let acc: u8 = self.a;
        let res: u8 = value + acc + if self.flag_c { 1 } else { 0 };
        self.set_sign(res);
        self.set_zero(res);
        self.set_overflow(((value ^ acc) & 0x80) == 0
                          && ((res ^ acc) & 0x80) != 0);
        self.set_carry(res < value);
        self.a = res
    }

    // OPCODES: 21 25 29 2D 31 35 39 3D
    fn and(&mut self, op: Operand) {
        let res = self.a & self.get_value(op);
        self.set_sign(res);
        self.set_zero(res);
        self.a = res
    }

    // OPCODES: 06 0A 0E 16 1E
    fn asl(&mut self, op: Operand) {
        let res = self.get_value(op) << 1;
        self.set_sign(res);
        self.set_zero(res);
        self.set_value(op, res)
    }

    // OPCODES: 90
    fn bcc(&mut self, op: Operand) {
        if !self.flag_c {
            self.pc += self.get_address(op)
        }
    }

    // OPCODES: B0
    fn bcs(&mut self, op: Operand) {
        if self.flag_c {
            self.pc += self.get_address(op)
        }
    }

    // OPCODES: F0
    fn beq(&mut self, op: Operand) {
        if self.flag_z {
            self.pc += self.get_address(op)
        }
    }

    // OPCODES: 24 2C
    fn bit(&mut self, op: Operand) {
        let value = self.get_value(op);
        let acc = self.a;
        self.set_sign(value);
        self.set_overflow(value & 0x40 != 0);
        self.set_zero(value & acc)
    }

    // OPCODES: 30
    fn bmi(&mut self, op: Operand) {
        if self.flag_s {
            self.pc += self.get_address(op)
        }
    }

    // OPCODES: D0
    fn bne(&mut self, op: Operand) {
        if !self.flag_z {
            self.pc += self.get_address(op)
        }
    }

    // OPCODES: 10
    fn bpl(&mut self, op: Operand) {
        if !self.flag_s {
            self.pc += self.get_address(op)
        }
    }

    // OPCODES: 00
    fn brk(&mut self, op: Operand) {
        let pc = self.pc + 1;
        self.push((pc >> 8) as u8);
        self.push(pc as u8);

        self.flag_b = true;
        let status = self.get_status();
        self.push(status);

        self.set_interrupt_disable(true);
        self.pc = (self.mem[0xffff] as usize) << 8 | self.mem[0xfffe] as usize
    }

    // OPCODES: 50
    fn bvc(&mut self, op: Operand) {
        if !self.flag_v {
            self.pc += self.get_address(op)
        }
    }

    // OPCODES: 70
    fn bvs(&mut self, op: Operand) {
        if self.flag_v {
            self.pc += self.get_address(op)
        }
    }

    // OPCODES: 18
    fn clc(&mut self, op: Operand) {
        self.set_carry(false)
    }

    // OPCODES: D8
    fn cld(&mut self, op: Operand) {
        self.set_decimal_mode(false)
    }

    // OPCODES: 58
    fn cli(&mut self, op: Operand) {
        self.set_interrupt_disable(false)
    }

    // OPCODES: B8
    fn clv(&mut self, op: Operand) {
        self.set_overflow(false)
    }

    // OPCODES: C1 C5 C9 CD D1 D5 D9 DD
    fn cmp(&mut self, op: Operand) {
        let value = self.a;
        self.compare(op, value)
    }

    // OPCODES: E0 E4 EC
    fn cpx(&mut self, op: Operand) {
        let value = self.x;
        self.compare(op, value)
    }

    // OPCODES: C0 C4 CC
    fn cpy(&mut self, op: Operand) {
        let value = self.y;
        self.compare(op, value)
    }

    // OPCODES: C6 CE D6 DE
    fn dec(&mut self, op: Operand) {
        let address = self.get_address(op);
        let res = self.mem[address] - 1;
        self.set_sign(res);
        self.set_zero(res);
        self.mem[address] = res
    }

    // OPCODES: CA
    fn dex(&mut self, op: Operand) {
        let res = self.x - 1;
        self.set_sign(res);
        self.set_zero(res);
        self.x = res
    }

    // OPCODES: 88
    fn dey(&mut self, op: Operand) {
        let res = self.y - 1;
        self.set_sign(res);
        self.set_zero(res);
        self.y = res
    }

    // OPCODES: 41 45 49 4D 51 55 59 5D
    fn eor(&mut self, op: Operand) {
        let res = self.a ^ self.get_value(op);
        self.set_sign(res);
        self.set_zero(res);
        self.a = res
    }

    // OPCODES: E6 EE F6 FE
    fn inc(&mut self, op: Operand) {
        let address = self.get_address(op);
        let res = self.mem[address] + 1;
        self.set_sign(res);
        self.set_zero(res);
        self.mem[address] = res
    }

    // OPCODES: E8
    fn inx(&mut self, op: Operand) {
        let res = self.x + 1;
        self.set_sign(res);
        self.set_zero(res);
        self.x = res
    }

    // OPCODES: C8
    fn iny(&mut self, op: Operand) {
        let res = self.y + 1;
        self.set_sign(res);
        self.set_zero(res);
        self.y = res
    }

    // OPCODES: 6C 4C
    fn jmp(&mut self, op: Operand) {
        self.pc = self.get_address(op)
    }

    // OPCODES: 20
    fn jsr(&mut self, op: Operand) {
        let pc = self.pc - 1;
        self.push((pc >> 8) as u8);
        self.push(pc as u8);
        self.pc = self.get_address(op);
    }

    // OPCODES: A1 A5 A9 AD B1 B5 B9 BD
    fn lda(&mut self, op: Operand) {
        let address = self.get_address(op);
        self.a = self.mem[address];
        self.cc += 1;
    }

    // OPCODES: A2 A6 AE B6 BE
    fn ldx(&mut self, op: Operand) {
        let address = self.get_address(op);
        self.x = self.mem[address];
        self.cc += 1;
    }

    // OPCODES: A0 A4 AC B4 BC
    fn ldy(&mut self, op: Operand) {
        let address = self.get_address(op);
        self.y = self.mem[address];
        self.cc += 1;
    }

    // OPCODES: 4A 46 4E 56 5E
    fn lsr(&mut self, op: Operand) {
        let res = self.get_value(op) >> 1;
        self.set_sign(res);
        self.set_zero(res);
        self.set_value(op, res)
    }

    // OPCODES: 01 05 09 0D 11 15 19 1D
    fn ora(&mut self, op: Operand) {
        let res = self.a | self.get_value(op);
        self.set_sign(res);
        self.set_zero(res);
        self.a = res
    }

    // OPCODES: 48
    fn pha(&mut self, op: Operand) {
        let value = self.a;
        self.push(value)
    }

    // OPCODES: 08
    fn php(&mut self, op: Operand) {
        let value = self.get_status();
        self.push(value)
    }

    // OPCODES: 68
    fn pla(&mut self, op: Operand) {
        let res = self.pull();
        self.set_sign(res);
        self.set_zero(res);
        self.a = res
    }

    // OPCODES: 28
    fn plp(&mut self, op: Operand) {
        let value = self.pull();
        self.set_status(value)
    }

    // OPCODES: 26 2E 36 3E
    fn rol_mem(&mut self, op: Operand) {
        let address = self.get_address(op);
        let res = (self.mem[address] << 1) | if self.flag_c { 1 } else { 0 };
        let carry = self.mem[address] & 0x80 != 0;
        self.set_carry(carry);
        self.set_sign(res);
        self.set_zero(res);
        self.mem[address] = res
    }

    // OPCODES: 2A
    fn rol_acc(&mut self, op: Operand) {
        let res = (self.a << 1) | if self.flag_c { 1 } else { 0 };
        let carry = self.a & 0x80 != 0;
        self.set_carry(carry);
        self.set_sign(res);
        self.set_zero(res);
        self.a = res
    }

    // OPCODES: 66 6E 76 7E
    fn ror_mem(&mut self, op: Operand) {
        let address = self.get_address(op);
        let res = (self.mem[address] >> 1) | if self.flag_c { 0x80 } else { 0 };
        let carry = self.mem[address] & 1 != 0;
        self.set_carry(carry);
        self.set_sign(res);
        self.set_zero(res);
        self.mem[address] = res
    }

    // OPCODES: 6A
    fn ror_acc(&mut self, op: Operand) {
        let res = (self.a >> 1) | if self.flag_c { 0x80 } else { 0 };
        let carry = self.a & 1 != 0;
        self.set_carry(carry);
        self.set_sign(res);
        self.set_zero(res);
        self.a = res
    }

    // OPCODES: 4D
    fn rti(&mut self, op: Operand) {
        let status = self.pull();
        self.set_status(status);
        self.pc = (self.pull() as usize) | ((self.pull() as usize) << 8)
    }

    // OPCODES: 60
    fn rts(&mut self, op: Operand) {
        self.pc = (self.pull() as usize) | ((self.pull() as usize) << 8) + 1
    }

    // OPCODES: E1 E5 E9 ED F1 F5 F9 FD
    fn sbc(&mut self, op: Operand) {
        let value = self.get_value(op);
        let acc: u8 = self.a;
        let carry: u8 = if self.flag_c { 0 } else { 1 };
        let res: u8 = acc - value - carry;
        self.set_sign(res);
        self.set_zero(res);
        self.set_overflow(((value ^ acc) & 0x80) == 0
                          && ((res ^ acc) & 0x80) != 0);
        self.set_carry(value > acc - carry);
        self.a = res
    }

    // OPCODES: 38
    fn sec(&mut self, op: Operand) {
        self.set_carry(true)
    }

    // OPCODES: F8
    fn sed(&mut self, op: Operand) {
        self.set_decimal_mode(true)
    }

    // OPCODES: 78
    fn sei(&mut self, op: Operand) {
        self.set_interrupt_disable(true)
    }

    // OPCODES: 81 85 89 8D 91 95 99 9D
    fn sta(&mut self, op: Operand) {
        let value = self.a;
        self.set_value(op, value)
    }

    // OPCODES: 86 8E 96
    fn stx(&mut self, op: Operand) {
        let value = self.x;
        self.set_value(op, value)
    }

    // OPCODES: 84 8C 94
    fn sty(&mut self, op: Operand) {
        let value = self.y;
        self.set_value(op, value)
    }

    // OPCODES: AA
    fn tax(&mut self, op: Operand) {
        let src = self.a;
        self.set_zero(src);
        self.set_sign(src);
        self.x = src
    }

    // OPCODES: A8
    fn tay(&mut self, op: Operand) {
        let src = self.a;
        self.set_zero(src);
        self.set_sign(src);
        self.y = src
    }

    // OPCODES: BA
    fn tsx(&mut self, op: Operand) {
        let src = self.sp;
        self.set_zero(src);
        self.set_sign(src);
        self.x = src
    }

    // OPCODES: 8A
    fn txa(&mut self, op: Operand) {
        let src = self.x;
        self.set_zero(src);
        self.set_sign(src);
        self.a = src
    }

    // OPCODES: 9A
    fn txs(&mut self, op: Operand) {
        let src = self.x;
        self.set_zero(src);
        self.set_sign(src);
        self.sp = src
    }

    // OPCODES: 98
    fn tya(&mut self, op: Operand) {
        let src = self.y;
        self.set_zero(src);
        self.set_sign(src);
        self.a = src
    }
}
