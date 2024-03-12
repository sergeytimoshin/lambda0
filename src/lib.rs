#[cfg(test)]
mod tests {
    use lambdaworks_math::{
        cyclic_group::IsGroup,
        elliptic_curve::{
            short_weierstrass::{
                curves::bls12_381::{compression::BLS12381FieldElement, curve::BLS12381Curve},
                point::{Endianness, PointFormat, ShortWeierstrassProjectivePoint},
            },
            traits::IsEllipticCurve,
        },
        traits::ByteConversion,
    };

    #[test]
    fn sk2pk() {
        let secret_key = "0x6C616D6264617370";
        let element: BLS12381FieldElement = BLS12381FieldElement::from_hex(secret_key).unwrap();
        let generator: ShortWeierstrassProjectivePoint<BLS12381Curve> = BLS12381Curve::generator();

        let x = *element.value();
        let public_key_vec = generator
            .operate_with_self(x)
            .serialize(PointFormat::Uncompressed, Endianness::BigEndian);

        let public_key = BLS12381FieldElement::from_bytes_be(&public_key_vec).unwrap();
        let public_key_hex = public_key.to_hex();

        println!("Generator: {:?}", generator);
        println!("Element: {:?}", element);
        println!("Public Key: {:?}", public_key_hex);
    }
}
