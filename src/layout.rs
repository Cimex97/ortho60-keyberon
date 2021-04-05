use keyberon::action::Action::Trans;
use keyberon::action::Action::MultipleActions;
use keyberon::action::Action::NoOp;
use keyberon::action::{k, l, m};
use keyberon::key_code::KeyCode::*;

static SHIFTLAYER: usize = 1;
static MOD3LAYER: usize = 2;
static MOD4LAYER: usize = 3;
static FNLAYER: usize = 4;

// Shift + KeyCode
macro_rules! s {
    ($k:ident) => {
        m(&[LShift, $k])
    };
}

#[rustfmt::skip]
pub static LAYERS: keyberon::layout::Layers = &[
    &[
        //Default layer
        &[k(Escape), k(Grave),k(Kb1),k(Kb2), k(Kb3), k(Kb4),  k(Kb5) ], // left
        &[k(Kb6),   k(Kb7),  k(Kb8),  k(Kb9),   k(Kb0),   k(T),    k(PgUp)    ], // right

        &[k(Tab), k(Minus),k(X),k(V), k(L), k(C),  k(W) ], // left
        &[k(K),   k(H),  k(G),  k(F),   k(Q),   k(Z),    k(PgDown)    ], // right

        &[l(MOD3LAYER), NoOp, k(U), k(I), k(A), k(E), k(O),  k(S) ], // left
        &[k(S),   k(N),  k(R),  k(T),   k(D),   NoOp,    l(MOD3LAYER)    ], // right

        &[k(LShift), k(LBracket), k(SColon), k(Quote), k(P), k(Y), k(BSpace) ], // left
        &[NoOp, k(B),   k(M),  k(Comma),  k(Dot),  k(J),    l(SHIFTLAYER)    ], // right

        &[k(LCtrl), l(FNLAYER), NoOp, NoOp, k(LGui), k(Space), k(LAlt) ], // left
        &[k(RShift), k(Enter), l(MOD4LAYER),  NoOp, NoOp,  l(FNLAYER),    k(RCtrl) ], // right


    ], &[
        //Shift layer
        &[k(Escape), s!(Grave),k(Kb1),k(Kb2), k(Kb3), k(Kb4),  k(Kb5) ], // left
        &[k(Kb6),   k(Kb7),  k(Kb8),  k(Kb9),   k(Kb0),   k(T),    k(PgUp)    ], // right

        &[k(Tab), k(Minus), s!(X),s!(V), s!(L), s!(C),  s!(W) ], // left
        &[s!(K),   s!(H),  s!(G),  s!(F),   s!(Q),   s!(Z),    k(PgDown)    ], // right

        &[Trans, NoOp, s!(U), s!(I), s!(A), s!(E), s!(O),  s!(S) ], // left
        &[s!(S),   s!(N),  s!(R),  s!(T),   s!(D),   NoOp,    Trans    ], // right

        &[Trans, s!(LBracket), s!(SColon), s!(Quote), s!(P), s!(Y), k(BSpace) ], // left
        &[NoOp, s!(B),   s!(M),  s!(Comma),  s!(Dot),  s!(J),    Trans    ], // right

    ], &[
        // Mod3 Layer
        &[Trans, Trans, Trans, Trans, Trans, Trans, Trans], //left
        &[Trans, Trans, Trans, Trans, Trans, Trans, Trans], //right

        &[Trans, Trans, m(& [RAlt, Dot] ), m(& [LShift, Slash]), m(& [RAlt, Kb8]), m(& [RAlt, Kb9]),  k(Grave) ], // left
        &[s!(Kb1),   k(NonUsBslash),  s!(NonUsBslash),  s!(Kb0),   s!(Kb6),   m(&[RAlt, Q]),    k(PgDown)    ], // right

        &[Trans, NoOp, m(& [RAlt, Minus] ), m(& [LShift, Kb7]), m(& [RAlt, Kb7]), m(& [RAlt, Kb0]),  s!(RBracket) ], // left
        &[s!(Minus),   s!(Kb8),  s!(Kb9),  k(Slash),   s!(Dot),   NoOp,    Trans    ], // right

        &[Trans, k(NonUsHash), s!(Kb4), m(& [RAlt, NonUsBslash]), m(& [RAlt, RBracket]), s!(Equal),  k(BSpace) ], // left
        &[NoOp,   k(RBracket),  s!(Kb5),  s!(Kb2),   s!(NonUsHash),   s!(Comma),    Trans    ], // right
    ], &[
        //Mod4 Layer
        &[Trans, Trans, Trans, Trans, Trans, Trans, Trans], //left
        &[Trans, Trans, Trans, Trans, Trans, Trans, Trans], //right

        &[Trans, Trans, k(PgUp), k(BSpace), k(Up), k(Delete),  k(Delete) ], // left
        &[m(& [RAlt, Kb1]), k(Kb7), k(Kb8), k(Kb9), k(RBracket), Trans,  Trans ], // left

        &[Trans, NoOp, k(Home), k(Left), k(Down), k(Right),  k(End) ], // left
        &[m(&[ RAlt, Minus]), k(Kb4), k(Kb5), k(Kb6), k(Comma), NoOp,  Trans], // left

        &[Trans, k(Escape), k(Tab), k(Insert), k(Enter),  k(Undo), Trans ], // left
        &[NoOp, s!(Dot), k(Kb1), k(Kb2), k(Kb3),  s!(Comma), Trans ], // left

        &[Trans, Trans, Trans, Trans, Trans,  k(Kb0), Trans ], // left
        &[Trans, s!(Dot), k(Kb1), k(Kb2), k(Kb3),  s!(Comma), Trans ], // left
    ], &[
        //FN Layer
        &[k(Escape), Trans, k(F1), k(F2), k(F3),  k(F4), k(F5)], // left
        &[k(F6), k(F7), k(F8), k(F9), k(F10),  k(F11), k(F12)], // left

    ]
];
