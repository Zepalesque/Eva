pub macro binary_op {
    ($ctx:ident<$ps:ident> -> $t1:ty, $t2:ty: $id:ident) => {{
        let aux = $ctx.reg_as::<$t1, $ps>().$id($ctx.reg_as::<$t2, $ps>());
        $ctx.read_reg::<$ps>().set(aux)
    }},
    ($ctx:ident<$ps:ident> -> $ty:ty: $id:ident) => {{
        let aux = $ctx.reg_as::<$ty, $ps>().$id($ctx.reg_as::<$ty, $ps>());
        $ctx.read_reg::<$ps>().set(aux)
    }},

    ($ctx:ident<$ps:ident> -> $t1:ty, $t2:ty: $op:tt) => {{
        let aux = $ctx.reg_as::<$t1, $ps>() $op $ctx.reg_as::<$t2, $ps>();
        $ctx.read_reg::<$ps>().set(aux)
    }},
    ($ctx:ident<$ps:ident> -> $ty:ty: $op:tt) => {{
        let aux = $ctx.reg_as::<$ty, $ps>() $op $ctx.reg_as::<$ty, $ps>();
        $ctx.read_reg::<$ps>().set(aux)
    }},
}

pub macro unary_op {
    ($ctx:ident<$ps:ident> -> $ty:ty: $op:tt) => {{
        let aux = $op $ctx.reg_as::<$ty, $ps>();
        $ctx.read_reg::<$ps>().set(aux)
    }},
}

pub macro conv {
    ($ctx:ident<$ps:ident> -> $t1:ty as $t2:ty) => {{
        let aux = $ctx.reg_as::<$t1, $ps>() as $t2;
        $ctx.read_reg::<$ps>().set(aux)
    }},
}

pub macro read {
    ($ctx:ident<$ps:ident> -> $ty:ty) => {{
        let aux = $ctx.decoder.read::<$ty, $ps>();
        $ctx.read_reg::<$ps>().set(aux)
    }},
}