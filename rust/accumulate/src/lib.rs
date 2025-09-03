/// What should the type of function be?
pub fn map<F, A, B>(input: Vec<A>, mut function: F) -> Vec<B>
where
    F: FnMut(A) -> B,
{
    // todo!("Transform input vector {input:?} using passed function");
    //
    let mut ret = Vec::new();

    for el in input {
        ret.push(function(el));
    }

    ret
}
