// For synchronous signing.
use alloy::{
    k256::{
        ecdsa::signature::digest::{FixedOutput, FixedOutputReset, HashMarker, Reset, Update},
        sha2::{
            self,
            digest::{Output, OutputSizeUser},
            Digest,
        },
    },
    primitives::FixedBytes,
};

pub(crate) type Sha256Proxy = ProxyDigest<sha2::Sha256>;

#[derive(Clone)]
pub(crate) enum ProxyDigest<D: Digest> {
    Proxy(Output<D>),
    Digest(D),
}

impl<D: Digest + Clone> From<FixedBytes<32>> for ProxyDigest<D>
where
    Output<D>: Copy,
{
    fn from(src: FixedBytes<32>) -> Self {
        ProxyDigest::Proxy(*Output::<D>::from_slice(src.as_slice()))
    }
}

impl<D: Digest> Default for ProxyDigest<D> {
    fn default() -> Self {
        ProxyDigest::Digest(D::new())
    }
}

impl<D: Digest> Update for ProxyDigest<D> {
    // we update only if we are digest
    fn update(&mut self, data: &[u8]) {
        match self {
            ProxyDigest::Digest(ref mut d) => {
                d.update(data);
            }
            ProxyDigest::Proxy(..) => {
                unreachable!("can not update if we are proxy");
            }
        }
    }
}

impl<D: Digest> HashMarker for ProxyDigest<D> {}

impl<D: Digest> Reset for ProxyDigest<D> {
    // make new one
    fn reset(&mut self) {
        *self = Self::default();
    }
}

impl<D: Digest> OutputSizeUser for ProxyDigest<D> {
    // we default to the output of the original digest
    type OutputSize = <D as OutputSizeUser>::OutputSize;
}

impl<D: Digest> FixedOutput for ProxyDigest<D> {
    fn finalize_into(self, out: &mut Output<Self>) {
        match self {
            ProxyDigest::Digest(d) => {
                *out = d.finalize();
            }
            ProxyDigest::Proxy(p) => {
                *out = p;
            }
        }
    }
}

impl<D: Digest> FixedOutputReset for ProxyDigest<D> {
    fn finalize_into_reset(&mut self, out: &mut Output<Self>) {
        let s = std::mem::take(self);
        Digest::finalize_into(s, out)
    }
}
