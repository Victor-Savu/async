pub trait TypeList {
    type Head;
    type Tail: TypeList;
}

impl TypeList for ! {
    type Head = !;
    type Tail = !;
}

pub trait List : TypeList {
    fn split(self) -> (Self::Head, Self::Tail);
}
