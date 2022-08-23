macro_rules! forth {
    (@binop [$b:expr, $a:expr, $($e:tt)*] $name:tt |$an:ident, $bn:ident|$res:block $($x:tt)*) => {
        {
            let $an = $a;
            let $bn = $b;
            forth!(@t [$res, $($e)*] $($x)*)
        }
    };

    (@binop [$($e:tt)*] $name:ident $func:block $($x:tt)*) => {
        compile_error!("Expected 2 parameters for '" stringify!(name) "'")
    };

    // end of code is top of stack
    (@t [$top:expr, $($e:tt)*]) => {
        $top
    };

    // empty stack is void
    (@t []) => {
        ()
    };

    (@t [$var:ident, $($e:tt)*] @ $($x:tt)*) => {
        forth!(@t [$var, $($e)*] $($x)*)
    };

    (@t [$($e:tt)*] @ $($x:tt)*) => {
        compile_error!("Expected variable name for @")
    };

    (@t [$var:ident, $val:expr, $($e:tt)*] ! $($x:tt)*) => {
        {
            let $var = $val;
            forth!(@t [$($e)*] $($x)*)
        }
    };

    (@t [$($e:tt)*] ! $($x:tt)*) => {
        compile_error!("Expected 2 parameters for '!'")
    };

    (@t [$c:expr, $b:expr, $a:expr, $($e:tt)*] rot $($x:tt)*) => {
        forth!(@t [$a, $c, $b, $($e)*] $($x)*)
    };

    (@t [$($e:tt)*] rot $($x:tt)*) => {
        compile_error!("Expected 3 parameters for 'rot'")
    };

    (@t [$b:expr, $a:expr, $($e:tt)*] swap $($x:tt)*) => {
        forth!(@t [$a, $b, $($e)*] $($x)*)
    };

    (@t [$($e:tt)*] swap $($x:tt)*) => {
        compile_error!("Expected 2 parameters for 'swap'")
    };

    (@t [$($e:tt)*] + $($x:tt)*) => {
        forth!(@binop [$($e)*] + |a, b|{a + b} $($x)*)
    };

    (@t [$($e:tt)*] - $($x:tt)*) => {
        forth!(@binop [$($e)*] - |a, b|{a - b} $($x)*)
    };

    (@t [$($e:tt)*] * $($x:tt)*) => {
        forth!(@binop [$($e)*] * |a, b|{a * b} $($x)*)
    };

    (@t [$($e:tt)*] / $($x:tt)*) => {
        forth!(@binop [$($e)*] / |a, b|{a / b} $($x)*)
    };

    (@t [$($e:tt)*] < $($x:tt)*) => {
        forth!(@binop [$($e)*] < |a, b|{if a < b { 1 } else { 0 }} $($x)*)
    };

    (@t [$($e:tt)*] > $($x:tt)*) => {
        forth!(@binop [$($e)*] > |a, b|{if a > b { 1 } else { 0 }} $($x)*)
    };

    (@t [$($e:tt)*] = $($x:tt)*) => {
        forth!(@binop [$($e)*] = |a, b|{if a == b { 1 } else { 0 }} $($x)*)
    };

    (@t [$top:expr, $next:expr, $($e:tt)*] over $($x:tt)*) => {
        forth!(@t [$next, $top, $next, $($e)*] $($x)*)
    };

    (@t [] over $($x:tt)*) => {
        compile_error!("'over' on empty stack")
    };

    (@t [$top:expr, $next:expr, $($e:tt)*] tuck $($x:tt)*) => {
        forth!(@t [$top, $next, $top, $($e)*] $($x)*)
    };

    (@t [] tuck $($x:tt)*) => {
        compile_error!("'tuck' on empty stack")
    };

    (@t [$top:expr, $($e:tt)*] dup $($x:tt)*) => {
        forth!(@t [$top, $top, $($e)*] $($x)*)
    };

    (@t [] dup $($x:tt)*) => {
        compile_error!("'dup' on empty stack")
    };

    (@t [$top:expr, $($e:tt)*] drop $($x:tt)*) => {
        forth!(@t [$($e)*] $($x)*)
    };

    (@t [] drop $($x:tt)*) => {
        compile_error!("'drop' on empty stack")
    };

    (@if_zero [$($e:tt)*] else $($x:tt)*) => {
        forth!(@t [$($e)*] $($x)*)
    };

    (@if_zero [$($e:tt)*] $top:tt $($x:tt)*) => {
        forth!(@if_zero [$($e)*] $($x)*)
    };

    (@t [$($e:tt)*] then $($x:tt)*) => {
        forth!(@t [$($e)*] $($x)*)
    };

    (@if_nonzero [$($e:tt)*] then $($x:tt)*) => {
        forth!(@t [$($e)*] $($x)*)
    };

    (@if_nonzero [$($e:tt)*] $top:tt $($x:tt)*) => {
        forth!(@if_nonzero [$($e)*] $($x)*)
    };

    (@t [$($e:tt)*] else $($x:tt)*) => {
        forth!(@if_nonzero [$($e)*] $($x)*)
    };

    // nested ifs are not supported, but should be easy to implement with an additional stack
    (@t [$top:expr, $($e:tt)*] if $($x:tt)*) => {
        if $top != 0 {
            forth!(@t [$($e)*] $($x)*)
        } else {
            forth!(@if_zero [$($e:tt)*] $($x)*)
        }
    };

    (@t [] if $($x:tt)*) => {
        compile_error!("'if' on empty stack")
    };

    (@t [$top:expr, $($e:tt)*] . $($x:tt)*) => {
        {
            println!("{}", $top);
            forth!(@t [$($e)*] $($x)*)
        }
    };

    (@t [] . $($x:tt)*) => {
        compile_error!("'.' on empty stack")
    };

    (@t [$($e:tt)*] $num:literal $($x:tt)*) => {
        forth!(@t [$num, $($e)*] $($x)*)
    };

    (@t [$($e:tt)*] $var:ident $($x:tt)*) => {
        forth!(@t [$var, $($e)*] $($x)*)
    };

    ($($x:tt)*) => {
        forth!(@t [] $($x)*)
    };
}

fn main() {
    const TWO: i32 = forth!(5 3 -);
    forth!(
        TWO . // 2
    );

    const HUNDRED: i8 = forth!(
        1 2 dup * + dup + // 10
        dup * // 100
        hundred !
        3 dup swap drop drop
        hundred @
    );

    forth!(
        HUNDRED @ dup . // 100
        50 > if "bigger" else "smaller" then . // bigger
    );
}

