use generic_array_struct::generic_array_struct;

#[generic_array_struct(builder pub)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct AtaPdaArgs<T> {
    pub wallet: T,
    pub token_prog: T,
    pub mint: T,
}

pub type AtaPdaSeeds<'a> = AtaPdaArgs<&'a [u8; 32]>;

impl<'a> AtaPdaSeeds<'a> {
    #[inline]
    pub const fn as_seeds(&self) -> &[&'a [u8; 32]; 3] {
        &self.0
    }
}
