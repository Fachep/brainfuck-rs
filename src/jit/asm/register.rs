pub trait RegisterTrait {
    fn reg(&self) -> u8;
    fn width_bit(&self) -> u8;
}

pub enum DataRegisterIdx {
    A,
    B,
    C,
    D
}

pub enum DataRegisterType {
    ByteLow,
    ByteHigh,
    Word,
    DWord,
    QWord,
}

pub struct DataRegister {
    id: DataRegisterIdx,
    type_: DataRegisterType,
}

pub enum PointerRegister {
    Stack,
    Base,
}

pub enum Register {
    Data(DataRegister),
    Pointer(PointerRegister),
}

impl RegisterTrait for DataRegister {
    fn reg(&self) -> u8 {
        match self.type_ {
            DataRegisterType::ByteHigh => match self.id {
                DataRegisterIdx::A => 0b100,
                DataRegisterIdx::B => 0b111,
                DataRegisterIdx::C => 0b101,
                DataRegisterIdx::D => 0b110,
            },
            _ => match self.id {
                DataRegisterIdx::A => 0b000,
                DataRegisterIdx::B => 0b011,
                DataRegisterIdx::C => 0b001,
                DataRegisterIdx::D => 0b010,
            }
        }
    }

    fn width_bit(&self) -> u8 {
        match self.type_ {
            DataRegisterType::ByteLow |
            DataRegisterType::ByteHigh => 0,
            _ => 1,
        }
    }
}

impl RegisterTrait for PointerRegister {
    fn reg(&self) -> u8 {
        match self {
            PointerRegister::Stack => 0b100,
            PointerRegister::Base => 0b101,
        }
    }

    fn width_bit(&self) -> u8 {
        1
    }
}

impl RegisterTrait for Register {
    fn reg(&self) -> u8 {
        match self {
            Register::Data(data) => data.reg(),
            Register::Pointer(pointer) => pointer.reg(),
        }
    }

    fn width_bit(&self) -> u8 {
        match self {
            Register::Data(data) => data.width_bit(),
            Register::Pointer(pointer) => pointer.width_bit(),
        }
    }
}

impl DataRegisterIdx {
    pub fn argument() -> Self {
        Self::C
    }

    pub fn result() -> Self {
        Self::A
    }
}
