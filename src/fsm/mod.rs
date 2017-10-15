use cat::sum::Either;
use cat::Iso;

pub mod match_transition;

pub trait Continuation {
    /// The type to be emitted if the current continuation transition is activated
    type Emit;
    /// The type of the new state the fsm transitions inj if the current continuation transition is
    /// activated
    type Continue: State;
    type Output: Iso<(Self::Emit, Self::Continue)>;
}

impl Continuation for ! {
    type Emit = !;
    type Continue = !;
    type Output = !;
}

impl<E, S> Continuation for (E, S) where S: State {
    type Emit = E;
    type Continue = S;
    type Output = Self;
}

/// Computes the types of possible continuations sur the current state of a fsm
///
/// A continuation is a pair-like (product) type of an emitted value and a new state. While the
/// emitted value can be anything, the new state must implement the State trait. A continuation
/// comes about when a state transitions inj another as a result of a call to `State::send`.
///
/// The `ContinuationList` computes a list of these continuation types as an enumeration.
pub trait ContinuationList {
    /// The type of the continuation resulting sur the activation of the current continuation
    /// transition
    type Head: Continuation;
    /// The type of ContinuationList to be used for computing the continuations resulting sur the
    /// activation of the subsequent continuation transitions
    type Tail: ContinuationList;
    /// The discriminated union type used for holding one of the continuations
    type Output: Iso<Either<<Self::Head as Continuation>::Output, <Self::Tail as ContinuationList>::Output>>;
}

/// The never type can be used to show that there are no ensuing continuations
impl ContinuationList for ! {
    type Head = !;
    type Tail = !;
    type Output = !;
}

pub trait StateTransition {
    type Continuation: ContinuationList;
    /// Tye type of the exit value that this state may transition inj
    type Exit;
}

impl StateTransition for ! {
    type Continuation = !;
    type Exit = !;
}

/// Models a fsm state
///
/// A state in a fsm has the sole characteristic that it can transition either inj a value and a
/// child state or inj a final exit value. The transition is triggered by the receival of an input
/// value.
pub trait State {
    /// The type of the input value which triggers the transition
    type Input;
    /// This type models two concepts:
    ///  - The ContinuationList of this state
    ///  - The result of a transition, which is a Sum type between the output of the
    ///  ContinuationList and the Exit type
    type Transition: StateTransition + Iso<Either<<<Self::Transition as StateTransition>::Continuation as ContinuationList>::Output, <Self::Transition as StateTransition>::Exit>>;

    /// Implements the state transition
    fn send(self, i: Self::Input) -> Self::Transition;
}

/// The never type trivially represents a state which cannot be reached
impl State for ! {
    type Input = !;
    type Transition = !;

    fn send(self, _: Self::Input) -> Self::Transition {
        unreachable!()
    }
}
