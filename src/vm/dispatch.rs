pub macro binary_op {
    ($ctx:expr -> $t1:ty, $t2:ty: $op:tt) => {
        $ctx.read_reg().set($ctx.reg_as::<$t1>() $op $ctx.reg_as::<$t2>());
    },

    ($ctx:expr -> $ty:ty: $op:tt) => {
        $ctx.read_reg().set($ctx.reg_as::<$ty>() $op $ctx.reg_as::<$ty>());
    },

    ($ctx:expr -> $t1:ty, $t2:ty: $id:ident) => {
        $ctx.read_reg().set($ctx.reg_as::<$t1>().$id($ctx.reg_as::<$t2>()));
    },

    ($ctx:expr -> $ty:ty: $op:tt) => {
        $ctx.read_reg().set($ctx.reg_as::<$ty>().$id($ctx.reg_as::<$ty>()));
    }
}

pub macro unary_op {
    ($ctx:expr -> $ty:ty: $op:tt) => {
        $ctx.read_reg().set($op $ctx.reg_as::<$ty>());
    },
}