pub trait Board {}

type BitBoard = [u16; 40];

impl Board for BitBoard {}
