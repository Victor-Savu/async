use cat::sum::Sum;
use cat::prod::Prod;

pub mod match_transition;

/// Computes the types of possible continuations from the current state of a fsm
///
/// A continuation is a pair-like (product) type of an emitted value and a new state. While the
/// emitted value can be anything, the new state must implement the State trait. A continuation
/// comes about when a state transitions into another as a result of a call to `State::send`.
///
/// The ContinuationSet computes a list of these continuation types as an enumeration.
pub trait ContinuationSet {
    /// The type to be emitted if the current continuation transition is activated
    type Emit;
    /// The type of the new state the fsm transitions into if the current continuation transition is
    /// activated
    type Continue: State;
    /// The type of the continuation resulting from the activation of the current continuation
    /// transition
    type Head: Prod<Left = Self::Emit, Right = Self::Continue>;
    /// The type of ContinuationSet to be used for computing the continuations resulting from the
    /// activation of the subsequent continuation transitions
    type Suspend: ContinuationSet;
    /// The discriminated union type used for holding one of the continuations
    type Output: Sum<Left = Self::Head, Right = <Self::Suspend as ContinuationSet>::Output>;
}

/// The never type can be used to show that there are no ensuing continuations
impl ContinuationSet for ! {
    type Emit = !;
    type Continue = !;
    type Head = !;
    type Suspend = !;
    type Output = !;
}

/// Models a fsm state
///
/// A state in a fsm has the sole characteristic that it can transition either into a value and a
/// child state or into a final exit value. The transition is triggered by the receival of an input
/// value.
pub trait State {
    /// The type of the input value which triggers the transition
    type Input;
    /// Tye type of the exit value that this state may transition into
    type Exit;
    /// This type models two concepts:
    ///  - The ContinuationSet of this state
    ///  - The result of a transition, which is a Sum type between the output of the
    ///  ContinuationSet and the Exit type
    type Transition: ContinuationSet + Sum<Left = <Self::Transition as ContinuationSet>::Output, Right = Self::Exit>;

    /// Implements the state transition
    fn send(self, i: Self::Input) -> Self::Transition;
}

/// The never type trivially represents a state which cannot be reached
impl State for ! {
    type Input = !;
    type Exit = !;
    type Transition = !;

    fn send(self, _: Self::Input) -> Self::Transition {
        unreachable!()
    }
}
