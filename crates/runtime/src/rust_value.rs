use crate::Result;

macro_rules! uni {
    ($ret_ty: ty, $($ident: ident),*) => {
        $(
            fn $ident(&self) -> Result<$ret_ty>
                where
                    Self: Sized;
        )*
    };
}

macro_rules! bin {
    ($ret_ty: ty, $($ident: ident),*) => {
        $(
            fn $ident(&self, rhs: &Self) -> Result<$ret_ty>
                where
                    Self: Sized;
        )*
    };
}

pub trait IntOp {
    uni![i32, eqz];
    uni![Self, clz, ctz];

    bin![i32, eq, ne, lt_s, lt_u, gt_s, gt_u, le_s, le_u, ge_s, ge_u];
    bin![
        Self, add, sub, mul, div_s, div_u, rem_s, rem_u, and, or, xor, shl, shr_s, shr_u, rotl,
        rotr
    ];

    fn popcnt(&self) -> Result<Self>
    where
        Self: Sized;
}

pub trait FloatOp {
    uni![Self, abs, neg, ceil, floor, trunc, nearest, sqrt];

    bin![i32, eq, ne, lt, gt, le, ge];
    bin![Self, add, sub, mul, div, min, max, copysign];
}

macro_rules! bool {
    ($v: expr) => {
        if $v {
            1 as i32
        } else {
            0 as i32
        }
    };
}

macro_rules! impl_int_op {
    ($unsigned_ty: ty) => {
        fn eqz(&self) -> Result<i32> {
            Ok(bool!(*self == 0))
        }

        fn clz(&self) -> Result<Self> {
            Ok(self.leading_zeros() as Self)
        }

        fn ctz(&self) -> Result<Self> {
            Ok(self.trailing_zeros() as Self)
        }

        fn popcnt(&self) -> Result<Self> {
            Ok((*self as $unsigned_ty).count_ones() as Self)
        }

        fn eq(&self, rhs: &Self) -> Result<i32> {
            Ok(bool!(*self == *rhs))
        }

        fn ne(&self, rhs: &Self) -> Result<i32> {
            Ok(bool!(*self != *rhs))
        }

        fn lt_s(&self, rhs: &Self) -> Result<i32> {
            Ok(bool!(*self < *rhs))
        }

        fn lt_u(&self, rhs: &Self) -> Result<i32> {
            Ok(bool!((*self as $unsigned_ty) < (*rhs as $unsigned_ty)))
        }

        fn gt_s(&self, rhs: &Self) -> Result<i32> {
            Ok(bool!(*self > *rhs))
        }

        fn gt_u(&self, rhs: &Self) -> Result<i32> {
            Ok(bool!(self.abs() > rhs.abs()))
        }

        fn le_s(&self, rhs: &Self) -> Result<i32> {
            Ok(bool!(*self <= *rhs))
        }

        fn le_u(&self, rhs: &Self) -> Result<i32> {
            Ok(bool!((*self as $unsigned_ty) <= (*rhs as $unsigned_ty)))
        }

        fn ge_s(&self, rhs: &Self) -> Result<i32> {
            Ok(bool!(*self >= *rhs))
        }

        fn ge_u(&self, rhs: &Self) -> Result<i32> {
            Ok(bool!((*self as $unsigned_ty) >= (*rhs as $unsigned_ty)))
        }

        fn add(&self, rhs: &Self) -> Result<Self> {
            Ok(self + rhs)
        }

        fn sub(&self, rhs: &Self) -> Result<Self> {
            Ok(self - rhs)
        }

        fn mul(&self, rhs: &Self) -> Result<Self> {
            Ok(self * rhs)
        }

        fn div_s(&self, rhs: &Self) -> Result<Self> {
            Ok(self / rhs)
        }

        fn div_u(&self, rhs: &Self) -> Result<Self> {
            Ok(((*self as $unsigned_ty) / (*rhs as $unsigned_ty)) as Self)
        }

        fn rem_s(&self, rhs: &Self) -> Result<Self> {
            Ok(self % rhs)
        }

        fn rem_u(&self, rhs: &Self) -> Result<Self> {
            Ok(((*self as $unsigned_ty) % (*rhs as $unsigned_ty)) as Self)
        }

        fn and(&self, rhs: &Self) -> Result<Self> {
            Ok((*self & rhs) as Self)
        }

        fn or(&self, rhs: &Self) -> Result<Self> {
            Ok((*self | rhs) as Self)
        }

        fn xor(&self, rhs: &Self) -> Result<Self> {
            Ok((*self ^ rhs) as Self)
        }

        fn shl(&self, rhs: &Self) -> Result<Self> {
            Ok((*self).wrapping_shl(*rhs as u32))
        }

        fn shr_s(&self, rhs: &Self) -> Result<Self> {
            Ok((*self).wrapping_shr(*rhs as u32))
        }

        fn shr_u(&self, rhs: &Self) -> Result<Self> {
            Ok((*self as $unsigned_ty).wrapping_shr(*rhs as u32) as Self)
        }

        fn rotl(&self, rhs: &Self) -> Result<Self> {
            Ok((*self).rotate_left(*rhs as u32))
        }

        fn rotr(&self, rhs: &Self) -> Result<Self> {
            Ok((*self).rotate_right(*rhs as u32))
        }
    };
}

macro_rules! impl_float_op {
    () => {
        fn abs(&self) -> Result<Self> {
            Ok((*self).abs())
        }

        fn neg(&self) -> Result<Self> {
            Ok(-(*self))
        }

        fn ceil(&self) -> Result<Self> {
            Ok((*self).ceil())
        }

        fn floor(&self) -> Result<Self> {
            Ok((*self).floor())
        }

        fn trunc(&self) -> Result<Self> {
            Ok((*self).trunc())
        }

        fn nearest(&self) -> Result<Self> {
            Ok((*self).round())
        }

        fn sqrt(&self) -> Result<Self> {
            Ok((*self).sqrt())
        }

        fn eq(&self, rhs: &Self) -> Result<i32> {
            Ok(bool!(*self == *rhs))
        }

        fn ne(&self, rhs: &Self) -> Result<i32> {
            Ok(bool!(*self != *rhs))
        }

        fn lt(&self, rhs: &Self) -> Result<i32> {
            Ok(bool!(*self < *rhs))
        }

        fn gt(&self, rhs: &Self) -> Result<i32> {
            Ok(bool!(*self > *rhs))
        }

        fn le(&self, rhs: &Self) -> Result<i32> {
            Ok(bool!(*self <= *rhs))
        }

        fn ge(&self, rhs: &Self) -> Result<i32> {
            Ok(bool!(*self >= *rhs))
        }

        fn add(&self, rhs: &Self) -> Result<Self> {
            Ok(self + rhs)
        }

        fn sub(&self, rhs: &Self) -> Result<Self> {
            Ok(self - rhs)
        }

        fn mul(&self, rhs: &Self) -> Result<Self> {
            Ok(self * rhs)
        }

        fn div(&self, rhs: &Self) -> Result<Self> {
            Ok(self / rhs)
        }

        fn min(&self, rhs: &Self) -> Result<Self> {
            Ok((*self).min(*rhs))
        }

        fn max(&self, rhs: &Self) -> Result<Self> {
            Ok((*self).max(*rhs))
        }

        fn copysign(&self, rhs: &Self) -> Result<Self> {
            Ok((*self).copysign(*rhs))
        }
    };
}

impl IntOp for i32 {
    impl_int_op!(u32);
}

impl IntOp for i64 {
    impl_int_op!(u64);
}

impl FloatOp for f32 {
    impl_float_op!();
}

impl FloatOp for f64 {
    impl_float_op!();
}
