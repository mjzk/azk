use ark_serialize::{CanonicalDeserialize, CanonicalSerialize, Compress, Validate};

pub fn ark_serialize<S, A: CanonicalSerialize>(a: &A, s: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    s.serialize_bytes(&{
        let mut bytes = vec![];
        let _ = a.serialize_with_mode(&mut bytes, Compress::Yes);
        bytes
    })
}

pub fn ark_deserialize<'de, D, A: CanonicalDeserialize>(data: D) -> Result<A, D::Error>
where
    D: serde::de::Deserializer<'de>,
{
    let s: Vec<u8> = serde::de::Deserialize::deserialize(data)?;
    A::deserialize_with_mode(s.as_slice(), Compress::Yes, Validate::No)
        .map_err(serde::de::Error::custom)
}
