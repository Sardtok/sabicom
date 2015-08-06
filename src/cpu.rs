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

    fn get_address_from_instruction(&mut self) -> usize {
        // Addressing modes:
        // Indirect Y:  ODD  + 1
        // Indirect X:  EVEN + 1
        // Absolute:    EVEN + 0,C,D,E
        // Absolute X:  ODD  + 0,C,D,E
        // Absolute Y:  ODD  + 9,E
        // Accumulator: EVEN + A
        // Immediate:   EVEN + 0,2,9
        // Implied:     MISC + 0,8,A,D
        // Indirect:    6C
        // Relative:    ODD  + 0
        // Zero Page:   EVEN + 4,5,6
        // Zero Page X: ODD  + 4,5,6
        // Zero Page Y: ODD  + 6

        // Low nibble 1: ADC AND CMP EOR LDA ORA SBC STA
        // Low nibble 5: ADC AND CMP EOR LDA ORA SBC STA
        // Low nibble 9: ADC AND CMP EOR LDA ORA SBC STA
        // Low nibble D: ADC AND CMP EOR LDA ORA SBC STA

        // Low nibble 0: BCC BCS BEQ BMI BNE BPL BRK BVC BVS CPX CPY JSR LDY RTI RTS
        // Low nibble 2: LDX
        // Low nibble 4: BIT CPX CPY LDY STY
        // Low nibble 6: ASL DEC INC LDX LSR ROL ROR STX
        // Low nibble 8: CLC CLD CLI CLV DEY INX INY PHA PHP PLA PLP SEC SED SEI TAY TYA
        // Low nibble A: ASL DEX LSR NOP ROL ROR TAX TSX TXA TXS
        // Low nibble C: BIT CPX CPY JMP LDY STY
        // Low nibble E: ASL DEC INC LDX LSR ROL ROR STX
        return 0
    }

    fn set_sign(&mut self, sign: bool) {
        self.flag_s = sign
    }
    
    fn set_carry(&mut self, carry: bool) {
        self.flag_c = carry
    }
    
    fn set_overflow(&mut self, overflow: bool) {
        self.flag_v = overflow
    }
    
    fn set_zero(&mut self, zero: bool) {
        self.flag_z = zero
    }
    
    fn set_interruptible(&mut self, interruptible: bool) {
        self.flag_i = interruptible
    }

    fn set_decimal_mode(&mut self, decimal_mode: bool) {
        self.flag_d = decimal_mode
    }
    
    // OPCODES: 61 65 69 6D 71 75 79 7D
    fn adc(&mut self, value: u8) {
        let acc: u8 = self.a;
        let res: u8 = value + acc + if self.flag_c { 1 } else { 0 };
        self.set_sign((res & 0xFF) != 0);
        self.set_overflow(((value ^ acc) & 0x80) == 0
                          && ((res ^ acc) & 0x80) != 0);
        self.set_carry(res < value);
        self.a = res
    }

    // OPCODES: 21 25 29 2D 31 35 39 3D
    fn and(&mut self, address: usize) {
    }

    // OPCODES: 06 0A 0E 16 1E
    fn asl(&mut self, address: usize) {
    }

    // OPCODES: 90
    fn bcc(&mut self, address: usize) {
    }

    // OPCODES: B0
    fn bcs(&mut self, address: usize) {
    }

    // OPCODES: F0
    fn beq(&mut self, address: usize) {
    }

    // OPCODES: 24 2C
    fn bit(&mut self, address: usize) {
    }

    // OPCODES: 30
    fn bmi(&mut self, address: usize) {
    }

    // OPCODES: D0
    fn bne(&mut self, address: usize) {
    }

    // OPCODES: 10
    fn bpl(&mut self, address: usize) {
    }

    // OPCODES: 00
    fn brk(&mut self) {
        self.flag_i = false;        
    }

    // OPCODES: 50
    fn bvc(&mut self, address: usize) {
    }

    // OPCODES: 70
    fn bvs(&mut self, address: usize) {
    }

    // OPCODES: 18
    fn clc(&mut self) {
        self.flag_c = false;
        self.cc += 1;
    }

    // OPCODES: D8
    fn cld(&mut self) {
        self.flag_d = false;
        self.cc += 1;
    }
    
    // OPCODES: 58
    fn cli(&mut self) {
        self.flag_i = false;
        self.cc += 1;
    }

    // OPCODES: B8
    fn clv(&mut self) {
        self.flag_v = false;
        self.cc += 1;
    }

    // OPCODES: C1 C5 C9 CD D1 D5 D9 DD
    fn cmp(&mut self, address: usize) {
    }

    // OPCODES: E0 E4 EC
    fn cpx(&mut self, address: usize) {
    }

    // OPCODES: C0 C4 CC
    fn cpy(&mut self, address: usize) {
    }

    // OPCODES: C6 CE D6 DE
    fn dec(&mut self) {
    }

    // OPCODES: CA
    fn dex(&mut self) {
    }

    // OPCODES: 88
    fn dey(&mut self) {
    }

    // OPCODES: 41 45 49 4D 51 55 59 5D
    fn eor(&mut self, address: usize) {
    }

    // OPCODES: E6 EE F6 FE
    fn inc(&mut self) {
    }

    // OPCODES: E8
    fn inx(&mut self) {
    }

    // OPCODES: C8
    fn iny(&mut self) {
    }

    // OPCODES: 6C 4C
    fn jmp(&mut self, address: usize) {
    }

    // OPCODES: 20
    fn jsr(&mut self, address: usize) {
    }

    // OPCODES: A1 A5 A9 AD B1 B5 B9 BD
    fn lda(&mut self, address: usize) {
        self.a = self.mem[address];
        self.cc += 1;
    }

    // OPCODES: A2 A6 AE B6 BE
    fn ldx(&mut self, address: usize) {
        self.x = self.mem[address];
        self.cc += 1;
    }

    // OPCODES: A0 A4 AC B4 BC
    fn ldy(&mut self, address: usize) {
        self.y = self.mem[address];
        self.cc += 1;
    }

    // OPCODES: 46 4A 4E 56 5E
    fn lsr(&mut self, address: usize) {
    }

    // OPCODES: 01 05 09 0D 11 15 19 1D
    fn ora(&mut self, address: usize) {
    }

    // OPCODES: 48
    fn pha(&mut self) {
    }

    // OPCODES: 08
    fn php(&mut self) {
    }

    // OPCODES: 68
    fn pla(&mut self) {
    }

    // OPCODES: 28
    fn plp(&mut self) {
    }

    // OPCODES: 26 2A 2E 36 3E
    fn rol(&mut self, address: usize) {
    }

    // OPCODES: 66 6A 6E 76 7E
    fn ror(&mut self, address: usize) {
    }

    // OPCODES: 4D
    fn rti(&mut self) {
    }

    // OPCODES: 60
    fn rts(&mut self) {
    }

    // OPCODES: E1 E5 E9 ED F1 F5 F9 FD
    fn sbc(&mut self, address: usize) {
    }

    // OPCODES: 38
    fn sec(&mut self) {
    }

    // OPCODES: F8
    fn sed(&mut self) {
    }

    // OPCODES: 78
    fn sei(&mut self) {
    }

    // OPCODES: 81 85 89 8D 91 95 99 9D
    fn sta(&mut self, address: usize) {
    }

    // OPCODES: 86 8E 96
    fn stx(&mut self, address: usize) {
    }

    // OPCODES: 84 8C 94
    fn sty(&mut self, address: usize) {
    }

    // OPCODES: AA
    fn tax(&mut self) {
    }

    // OPCODES: A8
    fn tay(&mut self) {
    }

    // OPCODES: BA
    fn tsx(&mut self) {
    }

    // OPCODES: 8A
    fn txa(&mut self) {
    }

    // OPCODES: 9A
    fn txs(&mut self) {
    }

    // OPCODES: 98
    fn tya(&mut self) {
    }
}
