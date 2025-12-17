use crate::error;

#[doc = "`CryptoHash`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"string\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
#[serde(transparent)]
pub struct CryptoHash(pub [u8; 32]);
impl ::std::ops::Deref for CryptoHash {
    type Target = [u8; 32];
    fn deref(&self) -> &[u8; 32] {
        &self.0
    }
}
impl ::std::convert::From<&CryptoHash> for CryptoHash {
    fn from(value: &CryptoHash) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for CryptoHash {
    type Err = self::error::ConversionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let bytes = bs58::decode(s).into_vec()?;
        Self::try_from(bytes)
    }
}
impl TryFrom<&[u8]> for CryptoHash {
    type Error = self::error::ConversionError;

    fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
        if bytes.len() != 32 {
            return Err("length for CryptoHash is not 32".into());
        }
        let mut buf = [0; 32];
        buf.copy_from_slice(bytes);
        Ok(Self(buf))
    }
}

impl TryFrom<Vec<u8>> for CryptoHash {
    type Error = self::error::ConversionError;

    fn try_from(v: Vec<u8>) -> Result<Self, Self::Error> {
        <Self as TryFrom<&[u8]>>::try_from(v.as_ref())
    }
}
impl std::fmt::Display for CryptoHash {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::std::fmt::Display::fmt(&bs58::encode(self.0).into_string(), f)
    }
}
