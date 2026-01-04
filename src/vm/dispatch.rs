pub macro binary_op {
    ($ctx:ident<$ps:ident> -> $t1:ty, $t2:ty: $id:ident) => {
        $ctx.read_reg().set($ctx.reg_as::<$t1, $ps>().$id($ctx.reg_as::<$t2, $ps>()) );
    },
    ($ctx:ident<$ps:ident> -> $ty:ty: $id:ident) => {
        $ctx.read_reg().set($ctx.reg_as::<$ty, $ps>().$id($ctx.reg_as::<$ty, $ps>()) );
    },

    ($ctx:ident<$ps:ident> -> $t1:ty, $t2:ty: $op:tt) => {
        $ctx.read_reg().set($ctx.reg_as::<$t1, $ps>() $op $ctx.reg_as::<$t2, $ps>());
    },
    ($ctx:ident<$ps:ident> -> $ty:ty: $op:tt) => {
        $ctx.read_reg().set($ctx.reg_as::<$ty, $ps>() $op $ctx.reg_as::<$ty, $ps>());
    },
}

pub macro unary_op {
    ($ctx:ident<$ps:ident> -> $ty:ty: $op:tt) => {
        $ctx.read_reg().set($op $ctx.reg_as::<$ty, $ps>());
    },
}