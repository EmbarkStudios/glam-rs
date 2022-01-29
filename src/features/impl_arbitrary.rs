macro_rules! impl_vec_types {
    ($t:ty, $vec2:ident, $vec3:ident, $vec4:ident) => {
        impl<'a> Arbitrary<'a> for $vec2 {
            fn arbitrary(u: &mut Unstructured<'a>) -> Result<Self> {
                let x = <$t>::arbitrary(u)?;
                let y = <$t>::arbitrary(u)?;
                Ok(Self::new(x, y))
            }
        }

        impl<'a> Arbitrary<'a> for $vec3 {
            fn arbitrary(u: &mut Unstructured<'a>) -> Result<Self> {
                let x = <$t>::arbitrary(u)?;
                let y = <$t>::arbitrary(u)?;
                let z = <$t>::arbitrary(u)?;
                Ok(Self::new(x, y, z))
            }
        }

        impl<'a> Arbitrary<'a> for $vec4 {
            fn arbitrary(u: &mut Unstructured<'a>) -> Result<Self> {
                let x = <$t>::arbitrary(u)?;
                let y = <$t>::arbitrary(u)?;
                let z = <$t>::arbitrary(u)?;
                let w = <$t>::arbitrary(u)?;
                Ok(Self::new(x, y, z, w))
            }
        }

        /*
                #[test]
                fn test_vec2_rand() {
                    use rand::{Rng, SeedableRng};
                    use rand_xoshiro::Xoshiro256Plus;
                    let mut rng1 = Xoshiro256Plus::seed_from_u64(0);
                    let a: ($t, $t) = rng1.gen();
                    let mut rng2 = Xoshiro256Plus::seed_from_u64(0);
                    let b: $vec2 = rng2.gen();
                    assert_eq!(a, b.into());
                }

                #[test]
                fn test_vec3_rand() {
                    use rand::{Rng, SeedableRng};
                    use rand_xoshiro::Xoshiro256Plus;
                    let mut rng1 = Xoshiro256Plus::seed_from_u64(0);
                    let a: ($t, $t, $t) = rng1.gen();
                    let mut rng2 = Xoshiro256Plus::seed_from_u64(0);
                    let b: $vec3 = rng2.gen();
                    assert_eq!(a, b.into());
                }

                #[test]
                fn test_vec4_rand() {
                    use rand::{Rng, SeedableRng};
                    use rand_xoshiro::Xoshiro256Plus;
                    let mut rng1 = Xoshiro256Plus::seed_from_u64(0);
                    let a: ($t, $t, $t, $t) = rng1.gen();
                    let mut rng2 = Xoshiro256Plus::seed_from_u64(0);
                    let b: $vec4 = rng2.gen();
                    assert_eq!(a, b.into());
                }
        */
    };
}

macro_rules! impl_float_types {
    ($t:ident, $mat2:ident, $mat3:ident, $mat4:ident, $quat:ident, $vec2:ident, $vec3:ident, $vec4:ident) => {
        impl_vec_types!($t, $vec2, $vec3, $vec4);

        pub fn arbitrary_array<'a, T, const N: usize>(u: &mut Unstructured<'a>) -> Result<[T; N]>
        where
            T: Arbitrary<'a> + Default + Copy,
        {
            let mut a = [T::default(); N];
            for v in &mut a {
                *v = T::arbitrary(u)?;
            }
            Ok(a)
        }

        impl<'a> Arbitrary<'a> for $mat2 {
            fn arbitrary(u: &mut Unstructured<'a>) -> Result<Self> {
                Ok(Self::from_cols_array(&arbitrary_array(u)?))
            }
        }

        impl<'a> Arbitrary<'a> for $mat3 {
            fn arbitrary(u: &mut Unstructured<'a>) -> Result<Self> {
                Ok(Self::from_cols_array(&arbitrary_array(u)?))
            }
        }

        impl<'a> Arbitrary<'a> for $mat4 {
            fn arbitrary(u: &mut Unstructured<'a>) -> Result<Self> {
                Ok(Self::from_cols_array(&arbitrary_array(u)?))
            }
        }

        impl<'a> Arbitrary<'a> for $quat {
            fn arbitrary(u: &mut Unstructured<'a>) -> Result<Self> {
                let a = <$t>::arbitrary(u)?;
                let b = <$t>::arbitrary(u)?;
                let c = <$t>::arbitrary(u)?;
                Ok(Self::from_euler(crate::EulerRot::YXZ, a, b, c))
            }
        }

        /*
                #[test]
                fn test_mat2_rand() {
                    use rand::{Rng, SeedableRng};
                    use rand_xoshiro::Xoshiro256Plus;
                    let mut rng1 = Xoshiro256Plus::seed_from_u64(0);
                    let a = $mat2::from_cols_array(&rng1.gen::<[$t; 4]>());
                    let mut rng2 = Xoshiro256Plus::seed_from_u64(0);
                    let b = rng2.gen::<$mat2>();
                    assert_eq!(a, b);
                }

                #[test]
                fn test_mat3_rand() {
                    use rand::{Rng, SeedableRng};
                    use rand_xoshiro::Xoshiro256Plus;
                    let mut rng1 = Xoshiro256Plus::seed_from_u64(0);
                    let a = $mat3::from_cols_array(&rng1.gen::<[$t; 9]>());
                    let mut rng2 = Xoshiro256Plus::seed_from_u64(0);
                    let b = rng2.gen::<$mat3>();
                    assert_eq!(a, b);
                }

                #[test]
                fn test_mat4_rand() {
                    use rand::{Rng, SeedableRng};
                    use rand_xoshiro::Xoshiro256Plus;
                    let mut rng1 = Xoshiro256Plus::seed_from_u64(0);
                    let a = $mat4::from_cols_array(&rng1.gen::<[$t; 16]>());
                    let mut rng2 = Xoshiro256Plus::seed_from_u64(0);
                    let b = rng2.gen::<$mat4>();
                    assert_eq!(a, b);
                }

                #[test]
                fn test_quat_rand() {
                    use rand::{Rng, SeedableRng};
                    use rand_xoshiro::Xoshiro256Plus;
                    let mut rng1 = Xoshiro256Plus::seed_from_u64(0);
                    let a: $quat = rng1.gen();
                    assert!(a.is_normalized());
                    let mut rng2 = Xoshiro256Plus::seed_from_u64(0);
                    let b: $quat = rng2.gen();
                    assert_eq!(a, b);
                }
        */
    };
}

mod f32 {
    use crate::{Mat2, Mat3, Mat4, Quat, Vec2, Vec3, Vec3A, Vec4};
    use arbitrary::{Arbitrary, Result, Unstructured};

    impl_float_types!(f32, Mat2, Mat3, Mat4, Quat, Vec2, Vec3, Vec4);

    impl<'a> Arbitrary<'a> for Vec3A {
        fn arbitrary(u: &mut Unstructured<'a>) -> Result<Self> {
            let x = f32::arbitrary(u)?;
            let y = f32::arbitrary(u)?;
            let z = f32::arbitrary(u)?;
            Ok(Self::new(x, y, z))
        }
    }

    /*

        #[test]
        fn test_vec3a_rand() {
            use rand::{Rng, SeedableRng};
            use rand_xoshiro::Xoshiro256Plus;
            let mut rng1 = Xoshiro256Plus::seed_from_u64(0);
            let a: (f32, f32, f32) = rng1.gen();
            let mut rng2 = Xoshiro256Plus::seed_from_u64(0);
            let b: Vec3A = rng2.gen();
            assert_eq!(a, b.into());
        }
    */
}

mod f64 {
    use crate::{DMat2, DMat3, DMat4, DQuat, DVec2, DVec3, DVec4};
    use arbitrary::{Arbitrary, Result, Unstructured};

    impl_float_types!(f64, DMat2, DMat3, DMat4, DQuat, DVec2, DVec3, DVec4);
}

mod i32 {
    use crate::{IVec2, IVec3, IVec4};
    use arbitrary::{Arbitrary, Result, Unstructured};

    impl_vec_types!(i32, IVec2, IVec3, IVec4);
}

mod u32 {
    use crate::{UVec2, UVec3, UVec4};
    use arbitrary::{Arbitrary, Result, Unstructured};

    impl_vec_types!(u32, UVec2, UVec3, UVec4);
}
