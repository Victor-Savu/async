use gen::map::ret::{MapReturn, GenMapReturn};
use gen::Returns;
use gen::comb::join::{Join, GenJoin};


pub type GenChain<F, L> = GenJoin<GenMapReturn<F, L>>;

pub trait Chain
{
    fn chain<L>(self, l: L) -> GenChain<Self, L> where Self: Sized + Returns,  L: FnOnce<(Self::Return,)>
    {
        self.map_return(l).join()
    }
}

impl<F> Chain for F {}

#[cfg(test)]
mod tests {

    use gen::iter::wrap::Wrap;
    use gen::comb::chain::Chain;

    #[test]
    fn chain_integers() {
        let first = (1..9).wrap();
        let second = (1..3).wrap();

        let the_chain = first.chain(|_| second);
        let msg = "This is the end";
        let mut elem = 1;
        let message = each!(the_chain => i in {
            assert_eq!(i, elem);
            elem = if elem == 8 { 1 } else { elem + 1 };
        } then {
            msg
        });
        assert_eq!(elem, 3);
        assert_eq!(message, msg);
    }
}
