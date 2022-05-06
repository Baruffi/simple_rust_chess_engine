#[derive(PartialEq, PartialOrd, Clone, Copy)]
pub enum Sign {
    None,
    Positive,
    Negative = -1,
}

impl From<isize> for Sign {
    fn from(i: isize) -> Self {
        match i.signum() {
            1 => Self::Positive,
            -1 => Self::Negative,
            _ => Self::None,
        }
    }
}

impl std::ops::Neg for Sign {
    type Output = Self;

    fn neg(self) -> Self::Output {
        match self {
            Sign::None => Sign::None,
            Sign::Positive => Sign::Negative,
            Sign::Negative => Sign::Positive,
        }
    }
}

impl std::ops::Mul<isize> for Sign {
    type Output = isize;

    fn mul(self, rhs: isize) -> Self::Output {
        match self {
            Sign::None => 0,
            Sign::Positive => rhs,
            Sign::Negative => -rhs,
        }
    }
}

impl std::ops::Add<isize> for Sign {
    type Output = isize;

    fn add(self, rhs: isize) -> Self::Output {
        match self {
            Sign::None => rhs,
            Sign::Positive => rhs + 1,
            Sign::Negative => rhs - 1,
        }
    }
}

impl std::ops::Sub<isize> for Sign {
    type Output = isize;

    fn sub(self, rhs: isize) -> Self::Output {
        match self {
            Sign::None => rhs,
            Sign::Positive => rhs - 1,
            Sign::Negative => rhs + 1,
        }
    }
}

impl std::ops::Mul<Sign> for isize {
    type Output = isize;

    fn mul(self, rhs: Sign) -> Self::Output {
        match rhs {
            Sign::None => 0,
            Sign::Positive => self,
            Sign::Negative => -self,
        }
    }
}

impl std::ops::Add<Sign> for isize {
    type Output = isize;

    fn add(self, rhs: Sign) -> Self::Output {
        match rhs {
            Sign::None => self,
            Sign::Positive => self + 1,
            Sign::Negative => self - 1,
        }
    }
}

impl std::ops::Sub<Sign> for isize {
    type Output = isize;

    fn sub(self, rhs: Sign) -> Self::Output {
        match rhs {
            Sign::None => self,
            Sign::Positive => self - 1,
            Sign::Negative => self + 1,
        }
    }
}
