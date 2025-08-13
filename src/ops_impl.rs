use super::NoCopy;
use core::ops::*;

macro_rules! impl_ops {
    ($Add:ident $add:ident $AddAssign:ident $add_assign:ident) => {
        impl<T: $Add<U>, U> $Add<U> for NoCopy<T> {
            type Output = T::Output;

            #[track_caller]
            fn $add(self, rhs: U) -> Self::Output {
                self.0.$add(rhs)
            }
        }

        impl<T: $AddAssign<U>, U> $AddAssign<U> for NoCopy<T> {
            #[track_caller]
            fn $add_assign(&mut self, rhs: U) {
                self.0.$add_assign(rhs);
            }
        }
    };
}

impl_ops!(Add       add     AddAssign       add_assign);
impl_ops!(Sub       sub     SubAssign       sub_assign);
impl_ops!(Mul       mul     MulAssign       mul_assign);
impl_ops!(Div       div     DivAssign       div_assign);
impl_ops!(Rem       rem     RemAssign       rem_assign);
impl_ops!(Shl       shl     ShlAssign       shl_assign);
impl_ops!(Shr       shr     ShrAssign       shr_assign);
impl_ops!(BitAnd    bitand  BitAndAssign    bitand_assign);
impl_ops!(BitOr     bitor   BitOrAssign     bitor_assign);
impl_ops!(BitXor    bitxor  BitXorAssign    bitxor_assign);

impl<T: Index<U>, U> Index<U> for NoCopy<T> {
    type Output = T::Output;

    #[track_caller]
    fn index(&self, index: U) -> &Self::Output {
        self.0.index(index)
    }
}

impl<T: IndexMut<U>, U> IndexMut<U> for NoCopy<T> {
    #[track_caller]
    fn index_mut(&mut self, index: U) -> &mut Self::Output {
        self.0.index_mut(index)
    }
}

impl<T: Neg> Neg for NoCopy<T> {
    type Output = T::Output;

    #[track_caller]
    fn neg(self) -> Self::Output {
        self.0.neg()
    }
}

impl<T: Not> Not for NoCopy<T> {
    type Output = T::Output;

    #[track_caller]
    fn not(self) -> Self::Output {
        self.0.not()
    }
}

impl<T: RangeBounds<R>, R> RangeBounds<R> for NoCopy<T> {
    #[track_caller]
    fn contains<U>(&self, item: &U) -> bool
    where R: PartialOrd<U>,
          U: ?Sized + PartialOrd<R>,
    {
        self.0.contains(item)
    }

    #[track_caller]
    fn end_bound(&self) -> Bound<&R> {
        self.0.end_bound()
    }

    #[track_caller]
    fn start_bound(&self) -> Bound<&R> {
        self.0.start_bound()
    }
}
