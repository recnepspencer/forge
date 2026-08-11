//! Generative-brand introductions that must actually invoke their consumer.

pub(crate) fn brand(consumer: impl for<'id> FnOnce(worth_proof::Brand<'id>)) {
    worth_proof::with_brand(consumer);
}

pub(crate) fn branded(consumer: impl for<'id> FnOnce(worth_proof::Branded<'id, u8>)) {
    worth_proof::with_brand(|brand| consumer(brand.bind(7)));
}
