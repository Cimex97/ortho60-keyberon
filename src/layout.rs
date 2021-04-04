use keyberon::action::Action::Trans;
use keyberon::action::{k, l, m};
use keyberon::key_code::KeyCode::*;

// Shift + KeyCode
macro_rules! s {
    ($k:ident) => {
        m(&[LShift, $k])
    };
}

#[rustfmt::skip]
pub static LAYERS: keyberon::layout::Layers = &[
    &[
        &[k(Escape), k(Grave),k(Kb1),k(Kb2), k(Kb3), k(Kb4),  k(Kb5) ], // left
        &[k(Kb6),   k(Kb7),  k(Kb8),  k(Kb9),   k(Kb0),   k(T),    k(PgUp)    ], // right

        &[k(Tab), k(Minus),k(X),k(V), k(L), k(C),  k(W) ], // left
        &[k(K),   k(H),  k(G),  k(F),   k(Q),   k(Y),    k(PgDown)    ], // right

        &[l(2), Trans, k(U), k(I), k(A), k(E), k(O),  k(S) ], // left
        &[k(S),   k(N),  k(R),  k(T),   k(D),   Trans,    l(2)    ], // right

        &[l(1), k(LBracket), k(SColon), k(Quote), k(P), k(Z), k(BSpace) ], // left
        &[Trans, k(B),   k(M),  k(Comma),  k(Dot),  k(J),    l(1)    ], // right

        &[k(LCtrl), Trans, Trans, Trans, k(LGui), k(Space), k(LAlt) ], // left
        &[k(RShift), k(Enter), l(3),  Trans, Trans,  Trans,    k(RCtrl) ], // right


    ], &[
        &[k(Escape), s!(Grave),k(Kb1),k(Kb2), k(Kb3), k(Kb4),  k(Kb5) ], // left
        &[k(Kb6),   k(Kb7),  k(Kb8),  k(Kb9),   k(Kb0),   k(T),    k(PgUp)    ], // right

        &[k(Tab), k(Minus), s!(X),s!(V), s!(L), s!(C),  s!(W) ], // left
        &[s!(K),   s!(H),  s!(G),  s!(F),   s!(Q),   s!(Y),    k(PgDown)    ], // right

        &[l(2), Trans, s!(U), s!(I), s!(A), s!(E), s!(O),  s!(S) ], // left
        &[s!(S),   s!(N),  s!(R),  s!(T),   s!(D),   Trans,    l(2)    ], // right

        &[Trans, s!(LBracket), s!(SColon), s!(Quote), s!(P), s!(Z), k(BSpace) ], // left
        &[Trans, s!(B),   s!(M),  s!(Comma),  s!(Dot),  s!(J),    Trans    ], // right
    ], &[
        &[Trans, Trans, Trans, Trans, Trans, Trans, Trans], //left
        &[Trans, Trans, Trans, Trans, Trans, Trans, Trans], //right

        &[Trans, Trans, m(& [RAlt, Dot] ), m(& [LShift, Slash]), m(& [RAlt, Kb8]), m(& [RAlt, Kb9]),  k(Grave) ], // left
        &[s!(Kb1),   k(NonUsBslash),  s!(NonUsBslash),  s!(Kb0),   s!(Kb6),   m(&[RAlt, Q]),    k(PgDown)    ], // right

        &[Trans, Trans, m(& [RAlt, Minus] ), m(& [LShift, Kb7]), m(& [RAlt, Kb7]), m(& [RAlt, Kb0]),  s!(RBracket) ], // left
        &[s!(Minus),   s!(Kb8),  s!(Kb9),  k(Slash),   s!(Dot),   Trans,    Trans    ], // right

        &[Trans, k(NonUsHash), s!(Kb4), m(& [RAlt, NonUsBslash]), m(& [RAlt, RBracket]), s!(Equal),  k(BSpace) ], // left
        &[Trans,   k(RBracket),  s!(Kb5),  s!(Kb2),   s!(NonUsHash),   s!(Comma),    Trans    ], // right
    ]
];
