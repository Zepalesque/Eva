use crate::core::struct_layout::StructLayout;
use crate::core::RegIdRepr;
use crate::emit::encode::Encoder;

fn gen_direct_field_access(encoder: &mut Encoder, layout: StructLayout, field: String, from: (RegIdRepr, RegIdRepr)) {
    let opt = layout.get(field.as_str());

    match opt {
        Some(f) => {
            if f.size <= 8 {
                if f.start_offs < 8 && f.endpoint() >= 8 {
                }
            }
        }
        None => { panic!("Invalid field: {}", field); }
    }

}