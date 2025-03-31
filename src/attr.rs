use std::ops::BitOr;

#[derive(Copy, Clone, PartialEq)]
pub struct Attr(u32);

impl Attr {
    pub const NORMAL: Self = Self(0);        //Normal display (no highlight)
    pub const STANDOUT: Self = Self(1 << 8);      //Best highlighting mode of the terminal.
    pub const UNDERLINE: Self = Self(1 << 9);     //Underlining
    pub const REVERSE: Self = Self(1 << 10);       //Reverse video
    pub const BLINK: Self = Self(1 << 11);         //Blinking
    pub const DIM: Self = Self(1 << 12);           //Half bright
    pub const BOLD: Self = Self(1 << 13);          //Extra bright or bold
    pub const PROTECT: Self = Self(1 << 16);       //Protected mode
    pub const INVIS: Self = Self(1 << 15);         //Invisible or blank mode
    pub const ALTCHARSET: Self = Self(1 << 14);    //Alternate character set
    //A_CHARTEXT      Bit-mask to extract a character
    pub const HORIZONTAL: Self = Self(1 << 17);
    pub const LEFT: Self = Self(1 << 18);
    pub const LOW: Self = Self(1 << 19);
    pub const RIGHT: Self = Self(1 << 20);
    pub const TOP: Self = Self(1 << 21);
    pub const VERTICAL: Self = Self(1 << 22);
    pub const ITALIC: Self = Self(1 << 23);
    
    pub const fn color_pair(n: u8) -> Self   //Color-pair number n
    {
        Self(n as u32)
    }
}

impl BitOr for Attr {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}