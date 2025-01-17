use rand::{prelude::ThreadRng, seq::SliceRandom, Rng};

struct TypeDef {
    name: &'static str,
    generics: u8,
}
impl TypeDef {
    const fn new(name: &'static str, generics: u8) -> Self {
        Self {name, generics}
    }
}

const RECURSION_LIMIT: u8 = 1;
const REF_CHANCE: f64 = 0.25;
const PTR_CHANCE: f64 = 0.125;
const MUT_CHANCE: f64 = 0.25;
const ARRAY_CHANCE: f64 = 0.05;
const TUPLE_CHANCE: f64 = 0.1;
const TYPES: &[TypeDef] = &[
    TypeDef::new("_", 0),
    TypeDef::new("!", 0),
    TypeDef::new("bool", 0),
    TypeDef::new("char", 0),
    TypeDef::new("i8", 0),
    TypeDef::new("i16", 0),
    TypeDef::new("i32", 0),
    TypeDef::new("i64", 0),
    TypeDef::new("isize", 0),
    TypeDef::new("u8", 0),
    TypeDef::new("u16", 0),
    TypeDef::new("u32", 0),
    TypeDef::new("u64", 0),
    TypeDef::new("usize", 0),
    TypeDef::new("f32", 0),
    TypeDef::new("f64", 0),
    TypeDef::new("&str", 0),
    TypeDef::new("String", 0),
    TypeDef::new("()", 0),
    TypeDef::new("Box", 1),
    TypeDef::new("Box<dyn Error>",0),
    TypeDef::new("Vec", 1),
    TypeDef::new("Arc", 1),
    TypeDef::new("HashSet", 1),
    TypeDef::new("Result", 2),
    TypeDef::new("HashMap", 2),
];

pub fn random_type(rng: &mut ThreadRng, mut recursion_depth: u8) -> String {
    let mut rtype = TYPES.choose(rng).unwrap();

    if recursion_depth >= RECURSION_LIMIT {
        while rtype.generics != 0 {
            rtype = TYPES.choose(rng).unwrap();
        }
    }

    let mut ret = String::new();
    let is_array = ARRAY_CHANCE > rng.gen();
    let is_tuple = TUPLE_CHANCE > rng.gen();

    let mut ref_add = |r: &mut String| {
        while REF_CHANCE > rng.gen() {
            if PTR_CHANCE > rng.gen() {
                if MUT_CHANCE > rng.gen() {r.push_str("*mut ")}
                else {r.push_str("*const ")}
            } else {
                if MUT_CHANCE > rng.gen() {r.push_str("&mut ")}
                else {r.push('&')}
            }
        }
    };

    ref_add(&mut ret);
    if is_array {
        recursion_depth +=1 ;
        ret.push('[');
        ref_add(&mut ret);
    }

    if is_tuple {
        recursion_depth +=1 ;
        ret.push('(');
        for _ in 0..rng.gen_range(1..=2) {
            ret.push_str(&random_type(rng, recursion_depth));
            ret.push_str(", ");
        }
    }

    ret.push_str(rtype.name);

    if rtype.generics > 0 {
        ret.push('<');
        for _ in 0..rtype.generics-1 {
            ret.push_str(&random_type(rng, recursion_depth+1));
            ret.push_str(", ");
        }
        ret.push_str(&random_type(rng, recursion_depth+1));
        ret.push('>');
    }

    if is_array {
        ret.push_str(&format!("; {}]", rng.gen::<u8>()));
    }
    if is_tuple {
        ret.push(')');
    }

    ret
}
