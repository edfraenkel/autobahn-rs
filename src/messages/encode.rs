pub use encoder::Encoder;

pub trait Encode {
    fn encode<E>(&self, encoder: E) -> Result<E::T> where E: Encoder;
}

