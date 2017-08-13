use meta::matches::Match;

pub trait TypeList {
    type Head;
    type Tail: TypeList;
}

impl TypeList for ! {
    type Head = !;
    type Tail = !;
}

impl TypeList for () {
    type Head = ();
    type Tail = ();
}

impl<A> TypeList for (A,) {
    type Head = A;
    type Tail = !;
}

impl<A, B> TypeList for (A, B) where B: TypeList {
    type Head = A;
    type Tail = B;
}

impl<A, B> TypeList for Match<A, B> where B: TypeList {
    type Head = A;
    type Tail = B;
}
