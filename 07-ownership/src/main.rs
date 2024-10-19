mod ownership;
mod ref_and_borrow;
mod slices;

fn main() {
    ownership::ownership_fn();
    ref_and_borrow::ref_and_borrow();
    slices::slicing();
}
