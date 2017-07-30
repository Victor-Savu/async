pub trait List {
    type Head;
    type Next: List;
}

impl List for ! {
    type Head = !;
    type Next = !;
}
