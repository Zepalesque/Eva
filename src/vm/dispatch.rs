pub macro binary_op {
    ($ctx:ident -> $t1:ty, $t2:ty: $id:ident) => {{
        let reg = $ctx.read_regid();
        let aux = $ctx.reg_as::<$t1>().$id($ctx.reg_as::<$t2>());
        $ctx.get_reg(reg).set(aux)
    }},

    ($ctx:ident -> $ty:ty: $id:ident) => {{
        let reg = $ctx.read_regid();
        let aux = $ctx.reg_as::<$ty>().$id($ctx.reg_as::<$ty>());
        $ctx.get_reg(reg).set(aux)
    }},

    ($ctx:ident -> $t1:ty, $t2:ty: $op:tt) => {{
        let reg = $ctx.read_regid();
        let aux = $ctx.reg_as::<$t1>() $op $ctx.reg_as::<$t2>();
        $ctx.get_reg(reg).set(aux)
    }},
    ($ctx:ident -> $ty:ty: $op:tt) => {{
        let reg = $ctx.read_regid();
        let aux = $ctx.reg_as::<$ty>() $op $ctx.reg_as::<$ty>();
        $ctx.get_reg(reg).set(aux)
    }},
}

pub macro unary_op {
    ($ctx:ident -> $ty:ty: $op:tt) => {{
        let reg = $ctx.read_regid();
        let aux = $op $ctx.reg_as::<$ty>();
        $ctx.get_reg(reg).set(aux)
    }},
}

pub macro conv {
    ($ctx:ident -> $t1:ty as $t2:ty) => {{
        let reg = $ctx.read_regid();
        let aux = $ctx.reg_as::<$t1>() as $t2;
        $ctx.get_reg(reg).set(aux)
    }},
}

pub macro read {
    ($ctx:ident -> $ty:ty) => {{
        let reg = $ctx.read_regid();
        let aux = $ctx.decoder.read::<$ty>();
        $ctx.get_reg(reg).set(aux)
    }},
}

pub macro load {
    ($ctx:ident -> $ty:ty) => {unsafe {
        let reg = $ctx.read_regid();
        let addr = $ctx.reg_as::<usize>();

        let ptr = std::ptr::with_exposed_provenance::<$ty>(addr);
        let val = ptr.read_unaligned();

        $ctx.get_reg(reg).set(val)
    }},
}

pub macro store {
    ($ctx:ident -> $ty:ty) => {unsafe {
        let addr = $ctx.reg_as::<usize>();
        let reg = $ctx.read_regid();
        let val = $ctx.get_reg(reg).get::<$ty>();

        let ptr = std::ptr::with_exposed_provenance::<$ty>(addr) as *mut $ty;

        ptr.write_unaligned(val);
    }},
}