// Copyright 2020 IOTA Stiftung
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

pub const XCHACHA20POLY1305_KEY_SIZE: usize = 32;
pub const XCHACHA20POLY1305_NONCE_SIZE: usize = 24;
pub const XCHACHA20POLY1305_TAG_SIZE: usize = 16;

use chacha20poly1305::aead::{AeadMutInPlace, NewAead};

pub fn xchacha20poly1305_encrypt(
    ciphertext: &mut [u8],
    tag: &mut [u8; XCHACHA20POLY1305_TAG_SIZE],
    plain: &[u8],
    key: &[u8; XCHACHA20POLY1305_KEY_SIZE],
    nonce: &[u8; XCHACHA20POLY1305_NONCE_SIZE],
    associated_data: &[u8],
) -> () {
    ciphertext.copy_from_slice(plain);

    let k = chacha20poly1305::Key::from_slice(key);
    let n = chacha20poly1305::XNonce::from_slice(nonce);
    let mut c = chacha20poly1305::XChaCha20Poly1305::new(k);
    let t = c.encrypt_in_place_detached(n, associated_data, ciphertext).expect("TODO: map errors");
    tag.copy_from_slice(t.as_slice());
}