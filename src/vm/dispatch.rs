pub macro binary_op {
    ($ctx:ident -> $t1:ty, $t2:ty: $id:ident) => {{
        let aux = $ctx.reg_as::<$t1>().$id($ctx.reg_as::<$t2>());
        $ctx.read_reg().set(aux)
    }},
    ($ctx:ident -> $ty:ty: $id:ident) => {{
        let aux = $ctx.reg_as::<$ty>().$id($ctx.reg_as::<$ty>());
        $ctx.read_reg().set(aux)
    }},

    ($ctx:ident -> $t1:ty, $t2:ty: $op:tt) => {{
        let aux = $ctx.reg_as::<$t1>() $op $ctx.reg_as::<$t2>();
        $ctx.read_reg().set(aux)
    }},
    ($ctx:ident -> $ty:ty: $op:tt) => {{
        let aux = $ctx.reg_as::<$ty>() $op $ctx.reg_as::<$ty>();
        $ctx.read_reg().set(aux)
    }},
}

pub macro unary_op {
    ($ctx:ident -> $ty:ty: $op:tt) => {{
        let aux = $op $ctx.reg_as::<$ty>();
        $ctx.read_reg().set(aux)
    }},
}

pub macro conv {
    ($ctx:ident -> $t1:ty as $t2:ty) => {{
        let aux = $ctx.reg_as::<$t1>() as $t2;
        $ctx.read_reg().set(aux)
    }},
}

pub macro read {
    ($ctx:ident -> $ty:ty) => {{
        let aux = $ctx.decoder.read::<$ty>();
        $ctx.read_reg().set(aux)
    }},
}