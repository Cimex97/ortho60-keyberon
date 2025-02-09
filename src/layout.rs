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
        &[NoOp, k(Escape), k(Grave),k(Kb1),k(Kb2), k(Kb3), k(Kb4),  k(Kb5) ],               // 1
        &[NoOp, k(Tab), k(Minus),k(X),k(V), k(L), k(C),  k(W) ],                            // 2
        &[NoOp, l(MOD3LAYER), NoOp, k(U), k(I), k(A), k(E), k(O),  k(S) ],                  // 3
        &[NoOp, k(LShift), k(LBracket), k(SColon), k(Quote), k(P), k(Y), k(BSpace) ],       // 4
        &[NoOp, k(LCtrl), l(FNLAYER), NoOp, NoOp, k(LGui), k(Space), k(LAlt) ],             // 5
        &[NoOp, k(Kb6),   k(Kb7),  k(Kb8),  k(Kb9),   k(Kb0),   k(T),    k(PgUp)    ],      // 6
        &[NoOp, k(K),   k(H),  k(G),  k(F),   k(Q),   k(Z),    k(PgDown)    ],              // 7
        &[NoOp, k(S),   k(N),  k(R),  k(T),   k(D),   NoOp,    l(MOD3LAYER)    ],           // 8
        &[NoOp, NoOp, k(B),   k(M),  k(Comma),  k(Dot),  k(J),    k(RShift)    ],       // 9
        &[NoOp, k(RShift), k(Enter), l(MOD4LAYER),  NoOp, NoOp,  l(FNLAYER),    k(RCtrl) ], // 10


    ], &[
        //Shift layer
        &[NoOp, k(Escape), s!(Grave),k(Kb1),k(Kb2), k(Kb3), k(Kb4),  k(Kb5) ],         // 1
        &[NoOp, k(Tab), k(Minus), s!(X),s!(V), s!(L), s!(C),  s!(W) ],                 // 2
        &[NoOp, Trans, NoOp, s!(U), s!(I), s!(A), s!(E), s!(O),  s!(S) ],              // 3
        &[NoOp, Trans, s!(LBracket), s!(SColon), s!(Quote), s!(P), s!(Y), k(BSpace) ], // 4
        &[NoOp, k(Kb6),   k(Kb7),  k(Kb8),  k(Kb9),   k(Kb0),   k(T),    k(PgUp)    ], // 5
        &[NoOp, s!(K),   s!(H),  s!(G),  s!(F),   s!(Q),   s!(Z),    k(PgDown)    ],   // 6
        &[NoOp, s!(S),   s!(N),  s!(R),  s!(T),   s!(D),   NoOp,    Trans    ],        // 7
        &[NoOp, NoOp, s!(B),   s!(M),  s!(Comma),  s!(Dot),  s!(J),    Trans    ],     // 8

    ], &[
        // Mod3 Layer
        &[NoOp, Trans, Trans, Trans, Trans, Trans, Trans, Trans],                                                          // 1
        &[NoOp, Trans, Trans, m(& [RAlt, Dot] ), m(& [LShift, Slash]), m(& [RAlt, Kb8]), m(& [RAlt, Kb9]),  k(Grave) ],    // 2
        &[NoOp, Trans, NoOp, m(& [RAlt, Minus] ), m(& [LShift, Kb7]), m(& [RAlt, Kb7]), m(& [RAlt, Kb0]),  s!(RBracket) ], // 3
        &[NoOp, Trans, k(NonUsHash), s!(Kb4), m(& [RAlt, NonUsBslash]), m(& [RAlt, RBracket]), s!(Equal),  k(BSpace) ],    // 4
        &[NoOp, Trans, Trans, Trans, Trans, Trans, Trans, Trans],                                                          // 5
        &[NoOp, Trans, Trans, Trans, Trans, Trans, Trans, Trans],                                                          // 6
        &[NoOp, s!(Kb1),   k(NonUsBslash),  s!(NonUsBslash),  s!(Kb0),   s!(Kb6),   m(&[RAlt, Q]),    k(PgDown)    ],      // 7
        &[NoOp, s!(Minus),   s!(Kb8),  s!(Kb9),  k(Slash),   s!(Dot),   NoOp,    Trans    ],                               // 8
        &[NoOp, NoOp,   k(RBracket),  s!(Kb5),  s!(Kb2),   s!(NonUsHash),   s!(Comma),    Trans    ],                      // 9
        &[NoOp, Trans, Trans, Trans, Trans, Trans, Trans, Trans],                                                          // 10
    ], &[
        //Mod4 Layer
        &[NoOp, Trans, Trans, Trans, Trans, Trans, Trans, Trans],                       // 1
        &[NoOp, Trans, Trans, k(PgUp), k(BSpace), k(Up), k(Delete),  k(Delete) ],       // 2
        &[NoOp, Trans, NoOp, k(Home), k(Left), k(Down), k(Right),  k(End) ],            // 3
        &[NoOp, Trans, k(Escape), k(Tab), k(Insert), k(Enter),  k(Undo), Trans ],       // 4
        &[NoOp, Trans, Trans, Trans, Trans, Trans,  k(Kb0), Trans ],                    // 5
        &[NoOp, Trans, Trans, Trans, Trans, Trans, Trans, Trans],                       // 6
        &[NoOp, m(& [RAlt, Kb1]), k(Kb7), k(Kb8), k(Kb9), k(RBracket), Trans,  Trans ], // 7
        &[NoOp, m(&[ RAlt, Minus]), k(Kb4), k(Kb5), k(Kb6), k(Comma), NoOp,  Trans],    // 8
        &[NoOp, NoOp, s!(Dot), k(Kb1), k(Kb2), k(Kb3),  s!(Comma), Trans ],             // 9
        &[NoOp, Trans, s!(Dot), k(Kb1), k(Kb2), k(Kb3),  s!(Comma), Trans ],            // 10
    ], &[
        //FN Layer
        &[NoOp, k(Escape), Trans, k(F1), k(F2), k(F3),  k(F4), k(F5)], // 1
        &[NoOp, k(F6), k(F7), k(F8), k(F9), k(F10),  k(F11), k(F12)],  // 2

    ]
];
