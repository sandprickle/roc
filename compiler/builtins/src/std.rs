use roc_collections::all::{default_hasher, MutMap, MutSet};
use roc_module::ident::TagName;
use roc_module::symbol::Symbol;
use roc_region::all::Region;
use roc_types::builtin_aliases::{
    bool_type, dict_type, float_type, i128_type, int_type, list_type, nat_type, num_type,
    ordering_type, result_type, set_type, str_type, str_utf8_byte_problem_type, u16_type, u32_type,
    u64_type, u8_type,
};
use roc_types::solved_types::SolvedType;
use roc_types::subs::VarId;
use roc_types::types::RecordField;
use std::collections::HashMap;

/// Example:
///
///     let_tvars! { a, b, c }
///
/// This is equivalent to:
///
///     let a = VarId::from_u32(1);
///     let b = VarId::from_u32(2);
///     let c = VarId::from_u32(3);
///
/// The idea is that this is less error-prone than assigning hardcoded IDs by hand.
macro_rules! let_tvars {
    ($($name:ident,)+) => { let_tvars!($($name),+) };
    ($($name:ident),*) => {
        let mut _current_tvar = 0;

        $(
            _current_tvar += 1;

            let $name = VarId::from_u32(_current_tvar);
        )*
    };
}

#[derive(Debug, Clone)]
pub struct StdLib {
    pub types: MutMap<Symbol, (SolvedType, Region)>,
    pub applies: MutSet<Symbol>,
}

pub fn standard_stdlib() -> StdLib {
    StdLib {
        types: types(),
        applies: vec![
            Symbol::LIST_LIST,
            Symbol::SET_SET,
            Symbol::DICT_DICT,
            Symbol::STR_STR,
        ]
        .into_iter()
        .collect(),
    }
}

/// Keep this up to date by hand! It's the number of builtin aliases that are imported by default.
const NUM_BUILTIN_IMPORTS: usize = 7;

/// These can be shared between definitions, they will get instantiated when converted to Type
const TVAR1: VarId = VarId::from_u32(1);
const TVAR2: VarId = VarId::from_u32(2);
const TVAR3: VarId = VarId::from_u32(3);
const TVAR4: VarId = VarId::from_u32(4);
const TOP_LEVEL_CLOSURE_VAR: VarId = VarId::from_u32(5);

pub fn types() -> MutMap<Symbol, (SolvedType, Region)> {
    let mut types = HashMap::with_capacity_and_hasher(NUM_BUILTIN_IMPORTS, default_hasher());

    macro_rules! add_type {
        ($symbol:expr, $typ:expr $(,)?) => {{
            debug_assert!(
                !types.contains_key(&$symbol),
                "Duplicate type definition for {:?}",
                $symbol
            );

            // TODO instead of using Region::zero for all of these,
            // instead use the Region where they were defined in their
            // source .roc files! This can give nicer error messages.
            types.insert($symbol, ($typ, Region::zero()));
        }};
    }

    macro_rules! add_top_level_function_type {
        ($symbol:expr, $arguments:expr, $result:expr $(,)?) => {{
            debug_assert!(
                !types.contains_key(&$symbol),
                "Duplicate type definition for {:?}",
                $symbol
            );

            let ext = Box::new(SolvedType::Flex(TOP_LEVEL_CLOSURE_VAR));

            let typ = SolvedType::Func(
                $arguments,
                Box::new(SolvedType::TagUnion(
                    vec![(TagName::Closure($symbol), vec![])],
                    ext,
                )),
                $result,
            );

            // TODO instead of using Region::zero for all of these,
            // instead use the Region where they were defined in their
            // source .roc files! This can give nicer error messages.
            types.insert($symbol, (typ, Region::zero()));
        }};
    }

    // Num module

    // add or (+) : Num a, Num a -> Num a
    add_top_level_function_type!(
        Symbol::NUM_ADD,
        vec![num_type(flex(TVAR1)), num_type(flex(TVAR1))],
        Box::new(num_type(flex(TVAR1))),
    );

    fn overflow() -> SolvedType {
        SolvedType::TagUnion(
            vec![(TagName::Global("Overflow".into()), vec![])],
            Box::new(SolvedType::Wildcard),
        )
    }

    // addChecked : Num a, Num a -> Result (Num a) [ Overflow ]*
    add_top_level_function_type!(
        Symbol::NUM_ADD_CHECKED,
        vec![num_type(flex(TVAR1)), num_type(flex(TVAR1))],
        Box::new(result_type(num_type(flex(TVAR1)), overflow())),
    );

    // addWrap : Int range, Int range -> Int range
    add_top_level_function_type!(
        Symbol::NUM_ADD_WRAP,
        vec![int_type(flex(TVAR1)), int_type(flex(TVAR1))],
        Box::new(int_type(flex(TVAR1))),
    );

    // sub or (-) : Num a, Num a -> Num a
    add_top_level_function_type!(
        Symbol::NUM_SUB,
        vec![num_type(flex(TVAR1)), num_type(flex(TVAR1))],
        Box::new(num_type(flex(TVAR1))),
    );

    // subWrap : Int range, Int range -> Int range
    add_top_level_function_type!(
        Symbol::NUM_SUB_WRAP,
        vec![int_type(flex(TVAR1)), int_type(flex(TVAR1))],
        Box::new(int_type(flex(TVAR1))),
    );

    // subChecked : Num a, Num a -> Result (Num a) [ Overflow ]*
    add_top_level_function_type!(
        Symbol::NUM_SUB_CHECKED,
        vec![num_type(flex(TVAR1)), num_type(flex(TVAR1))],
        Box::new(result_type(num_type(flex(TVAR1)), overflow())),
    );

    // mul or (*) : Num a, Num a -> Num a
    add_top_level_function_type!(
        Symbol::NUM_MUL,
        vec![num_type(flex(TVAR1)), num_type(flex(TVAR1))],
        Box::new(num_type(flex(TVAR1))),
    );

    // mulWrap : Int range, Int range -> Int range
    add_top_level_function_type!(
        Symbol::NUM_MUL_WRAP,
        vec![int_type(flex(TVAR1)), int_type(flex(TVAR1))],
        Box::new(int_type(flex(TVAR1))),
    );

    // mulChecked : Num a, Num a -> Result (Num a) [ Overflow ]*
    add_top_level_function_type!(
        Symbol::NUM_MUL_CHECKED,
        vec![num_type(flex(TVAR1)), num_type(flex(TVAR1))],
        Box::new(result_type(num_type(flex(TVAR1)), overflow())),
    );

    // abs : Num a -> Num a
    add_top_level_function_type!(
        Symbol::NUM_ABS,
        vec![num_type(flex(TVAR1))],
        Box::new(num_type(flex(TVAR1)))
    );

    // neg : Num a -> Num a
    add_top_level_function_type!(
        Symbol::NUM_NEG,
        vec![num_type(flex(TVAR1))],
        Box::new(num_type(flex(TVAR1)))
    );

    // isEq or (==) : a, a -> Bool
    add_top_level_function_type!(
        Symbol::BOOL_EQ,
        vec![flex(TVAR1), flex(TVAR1)],
        Box::new(bool_type())
    );

    // isNeq or (!=) : a, a -> Bool
    add_top_level_function_type!(
        Symbol::BOOL_NEQ,
        vec![flex(TVAR1), flex(TVAR1)],
        Box::new(bool_type())
    );

    // isLt or (<) : Num a, Num a -> Bool
    add_top_level_function_type!(
        Symbol::NUM_LT,
        vec![num_type(flex(TVAR1)), num_type(flex(TVAR1))],
        Box::new(bool_type()),
    );

    // isLte or (<=) : Num a, Num a -> Bool
    add_top_level_function_type!(
        Symbol::NUM_LTE,
        vec![num_type(flex(TVAR1)), num_type(flex(TVAR1))],
        Box::new(bool_type()),
    );

    // isGt or (>) : Num a, Num a -> Bool
    add_top_level_function_type!(
        Symbol::NUM_GT,
        vec![num_type(flex(TVAR1)), num_type(flex(TVAR1))],
        Box::new(bool_type()),
    );

    // isGte or (>=) : Num a, Num a -> Bool
    add_top_level_function_type!(
        Symbol::NUM_GTE,
        vec![num_type(flex(TVAR1)), num_type(flex(TVAR1))],
        Box::new(bool_type()),
    );

    // compare : Num a, Num a -> [ LT, EQ, GT ]
    add_top_level_function_type!(
        Symbol::NUM_COMPARE,
        vec![num_type(flex(TVAR1)), num_type(flex(TVAR1))],
        Box::new(ordering_type()),
    );

    // toFloat : Num * -> Float *
    add_top_level_function_type!(
        Symbol::NUM_TO_FLOAT,
        vec![num_type(flex(TVAR1))],
        Box::new(float_type(flex(TVAR2))),
    );

    // isNegative : Num a -> Bool
    add_top_level_function_type!(
        Symbol::NUM_IS_NEGATIVE,
        vec![num_type(flex(TVAR1))],
        Box::new(bool_type())
    );

    // isPositive : Num a -> Bool
    add_top_level_function_type!(
        Symbol::NUM_IS_POSITIVE,
        vec![num_type(flex(TVAR1))],
        Box::new(bool_type())
    );

    // isZero : Num a -> Bool
    add_top_level_function_type!(
        Symbol::NUM_IS_ZERO,
        vec![num_type(flex(TVAR1))],
        Box::new(bool_type())
    );

    // isEven : Num a -> Bool
    add_top_level_function_type!(
        Symbol::NUM_IS_EVEN,
        vec![num_type(flex(TVAR1))],
        Box::new(bool_type())
    );

    // isOdd : Num a -> Bool
    add_top_level_function_type!(
        Symbol::NUM_IS_ODD,
        vec![num_type(flex(TVAR1))],
        Box::new(bool_type())
    );

    // maxInt : Int range
    add_type!(Symbol::NUM_MAX_INT, int_type(flex(TVAR1)));

    // minInt : Int range
    add_type!(Symbol::NUM_MIN_INT, int_type(flex(TVAR1)));

    // divInt : Int a, Int a -> Result (Int a) [ DivByZero ]*
    let div_by_zero = SolvedType::TagUnion(
        vec![(TagName::Global("DivByZero".into()), vec![])],
        Box::new(SolvedType::Wildcard),
    );

    add_top_level_function_type!(
        Symbol::NUM_DIV_INT,
        vec![int_type(flex(TVAR1)), int_type(flex(TVAR1))],
        Box::new(result_type(int_type(flex(TVAR1)), div_by_zero.clone())),
    );

    // bitwiseAnd : Int a, Int a -> Int a
    add_top_level_function_type!(
        Symbol::NUM_BITWISE_AND,
        vec![int_type(flex(TVAR1)), int_type(flex(TVAR1))],
        Box::new(int_type(flex(TVAR1))),
    );

    // bitwiseXor : Int a, Int a -> Int a
    add_top_level_function_type!(
        Symbol::NUM_BITWISE_XOR,
        vec![int_type(flex(TVAR1)), int_type(flex(TVAR1))],
        Box::new(int_type(flex(TVAR1))),
    );

    // bitwiseOr : Int a, Int a -> Int a
    add_top_level_function_type!(
        Symbol::NUM_BITWISE_OR,
        vec![int_type(flex(TVAR1)), int_type(flex(TVAR1))],
        Box::new(int_type(flex(TVAR1))),
    );

    // shiftLeftBy : Int a, Int a -> Int a
    add_top_level_function_type!(
        Symbol::NUM_SHIFT_LEFT,
        vec![int_type(flex(TVAR1)), int_type(flex(TVAR1))],
        Box::new(int_type(flex(TVAR1))),
    );

    // shiftRightBy : Int a, Int a -> Int a
    add_top_level_function_type!(
        Symbol::NUM_SHIFT_RIGHT,
        vec![int_type(flex(TVAR1)), int_type(flex(TVAR1))],
        Box::new(int_type(flex(TVAR1))),
    );

    // shiftRightZfBy : Int a, Int a -> Int a
    add_top_level_function_type!(
        Symbol::NUM_SHIFT_RIGHT_ZERO_FILL,
        vec![int_type(flex(TVAR1)), int_type(flex(TVAR1))],
        Box::new(int_type(flex(TVAR1))),
    );

    // intCast : Int a -> Int b
    add_top_level_function_type!(
        Symbol::NUM_INT_CAST,
        vec![int_type(flex(TVAR1))],
        Box::new(int_type(flex(TVAR2)))
    );

    // rem : Int a, Int a -> Result (Int a) [ DivByZero ]*
    add_top_level_function_type!(
        Symbol::NUM_REM,
        vec![int_type(flex(TVAR1)), int_type(flex(TVAR1))],
        Box::new(result_type(int_type(flex(TVAR1)), div_by_zero.clone())),
    );

    // mod : Int a, Int a -> Result (Int a) [ DivByZero ]*
    add_top_level_function_type!(
        Symbol::NUM_MOD_INT,
        vec![int_type(flex(TVAR1)), int_type(flex(TVAR1))],
        Box::new(result_type(int_type(flex(TVAR1)), div_by_zero.clone())),
    );

    // isMultipleOf : Int a, Int a -> Bool
    add_top_level_function_type!(
        Symbol::NUM_IS_MULTIPLE_OF,
        vec![int_type(flex(TVAR1)), int_type(flex(TVAR1))],
        Box::new(bool_type()),
    );

    // maxI128 : I128
    add_type!(Symbol::NUM_MAX_I128, i128_type());

    // Float module

    // div : Float a, Float a -> Float a
    add_top_level_function_type!(
        Symbol::NUM_DIV_FLOAT,
        vec![float_type(flex(TVAR1)), float_type(flex(TVAR1))],
        Box::new(result_type(float_type(flex(TVAR1)), div_by_zero.clone())),
    );

    // mod : Float a, Float a -> Result (Float a) [ DivByZero ]*
    add_top_level_function_type!(
        Symbol::NUM_MOD_FLOAT,
        vec![float_type(flex(TVAR1)), float_type(flex(TVAR1))],
        Box::new(result_type(float_type(flex(TVAR1)), div_by_zero)),
    );

    // sqrt : Float a -> Float a
    let sqrt_of_negative = SolvedType::TagUnion(
        vec![(TagName::Global("SqrtOfNegative".into()), vec![])],
        Box::new(SolvedType::Wildcard),
    );

    add_top_level_function_type!(
        Symbol::NUM_SQRT,
        vec![float_type(flex(TVAR1))],
        Box::new(result_type(float_type(flex(TVAR1)), sqrt_of_negative)),
    );

    // log : Float a -> Float a
    let log_needs_positive = SolvedType::TagUnion(
        vec![(TagName::Global("LogNeedsPositive".into()), vec![])],
        Box::new(SolvedType::Wildcard),
    );

    add_top_level_function_type!(
        Symbol::NUM_LOG,
        vec![float_type(flex(TVAR1))],
        Box::new(result_type(float_type(flex(TVAR1)), log_needs_positive)),
    );

    // round : Float a -> Int b
    add_top_level_function_type!(
        Symbol::NUM_ROUND,
        vec![float_type(flex(TVAR1))],
        Box::new(int_type(flex(TVAR2))),
    );

    // sin : Float a -> Float a
    add_top_level_function_type!(
        Symbol::NUM_SIN,
        vec![float_type(flex(TVAR1))],
        Box::new(float_type(flex(TVAR1))),
    );

    // cos : Float a -> Float a
    add_top_level_function_type!(
        Symbol::NUM_COS,
        vec![float_type(flex(TVAR1))],
        Box::new(float_type(flex(TVAR1))),
    );

    // tan : Float a -> Float a
    add_top_level_function_type!(
        Symbol::NUM_TAN,
        vec![float_type(flex(TVAR1))],
        Box::new(float_type(flex(TVAR1))),
    );

    // maxFloat : Float a
    add_type!(Symbol::NUM_MAX_FLOAT, float_type(flex(TVAR1)));

    // minFloat : Float a
    add_type!(Symbol::NUM_MIN_FLOAT, float_type(flex(TVAR1)));

    // pow : Float a, Float a -> Float a
    add_top_level_function_type!(
        Symbol::NUM_POW,
        vec![float_type(flex(TVAR1)), float_type(flex(TVAR1))],
        Box::new(float_type(flex(TVAR1))),
    );

    // ceiling : Float a -> Int b
    add_top_level_function_type!(
        Symbol::NUM_CEILING,
        vec![float_type(flex(TVAR1))],
        Box::new(int_type(flex(TVAR2))),
    );

    // powInt : Int a, Int a -> Int a
    add_top_level_function_type!(
        Symbol::NUM_POW_INT,
        vec![int_type(flex(TVAR1)), int_type(flex(TVAR1))],
        Box::new(int_type(flex(TVAR1))),
    );

    // floor : Float a -> Int b
    add_top_level_function_type!(
        Symbol::NUM_FLOOR,
        vec![float_type(flex(TVAR1))],
        Box::new(int_type(flex(TVAR2))),
    );

    // atan : Float a -> Float a
    add_top_level_function_type!(
        Symbol::NUM_ATAN,
        vec![float_type(flex(TVAR1))],
        Box::new(float_type(flex(TVAR1))),
    );

    // acos : Float a -> Float a
    add_top_level_function_type!(
        Symbol::NUM_ACOS,
        vec![float_type(flex(TVAR1))],
        Box::new(float_type(flex(TVAR1))),
    );

    // asin : Float a -> Float a
    add_top_level_function_type!(
        Symbol::NUM_ASIN,
        vec![float_type(flex(TVAR1))],
        Box::new(float_type(flex(TVAR1))),
    );

    // castToNat : Num a -> Nat
    add_top_level_function_type!(
        Symbol::NUM_CAST_TO_NAT,
        vec![int_type(flex(TVAR1))],
        Box::new(nat_type()),
    );

    // bytesToU16 : List U8, Nat -> Result U16 [ OutOfBounds ]
    {
        let position_out_of_bounds = SolvedType::TagUnion(
            vec![(TagName::Global("OutOfBounds".into()), vec![])],
            Box::new(SolvedType::Wildcard),
        );
        add_top_level_function_type!(
            Symbol::NUM_BYTES_TO_U16,
            vec![list_type(u8_type()), nat_type()],
            Box::new(result_type(u16_type(), position_out_of_bounds)),
        );
    }

    // bytesToU32 : List U8, Nat -> Result U32 [ OutOfBounds ]
    {
        let position_out_of_bounds = SolvedType::TagUnion(
            vec![(TagName::Global("OutOfBounds".into()), vec![])],
            Box::new(SolvedType::Wildcard),
        );
        add_top_level_function_type!(
            Symbol::NUM_BYTES_TO_U32,
            vec![list_type(u8_type()), nat_type()],
            Box::new(result_type(u32_type(), position_out_of_bounds)),
        );
    }

    // Bool module

    // and : Bool, Bool -> Bool
    add_top_level_function_type!(
        Symbol::BOOL_AND,
        vec![bool_type(), bool_type()],
        Box::new(bool_type())
    );

    // or : Bool, Bool -> Bool
    add_top_level_function_type!(
        Symbol::BOOL_OR,
        vec![bool_type(), bool_type()],
        Box::new(bool_type())
    );

    // xor : Bool, Bool -> Bool
    add_top_level_function_type!(
        Symbol::BOOL_XOR,
        vec![bool_type(), bool_type()],
        Box::new(bool_type())
    );

    // not : Bool -> Bool
    add_top_level_function_type!(Symbol::BOOL_NOT, vec![bool_type()], Box::new(bool_type()));

    // Str module

    // Str.split : Str, Str -> List Str
    add_top_level_function_type!(
        Symbol::STR_SPLIT,
        vec![str_type(), str_type()],
        Box::new(list_type(str_type())),
    );

    // Str.concat : Str, Str -> Str
    add_top_level_function_type!(
        Symbol::STR_CONCAT,
        vec![str_type(), str_type()],
        Box::new(str_type()),
    );

    // Str.joinWith : List Str, Str -> Str
    add_top_level_function_type!(
        Symbol::STR_JOIN_WITH,
        vec![list_type(str_type()), str_type()],
        Box::new(str_type()),
    );

    // isEmpty : Str -> Bool
    add_top_level_function_type!(
        Symbol::STR_IS_EMPTY,
        vec![str_type()],
        Box::new(bool_type())
    );

    // startsWith : Str, Str -> Bool
    add_top_level_function_type!(
        Symbol::STR_STARTS_WITH,
        vec![str_type(), str_type()],
        Box::new(bool_type())
    );

    // startsWithCodePt : Str, U32 -> Bool
    add_top_level_function_type!(
        Symbol::STR_STARTS_WITH_CODE_PT,
        vec![str_type(), u32_type()],
        Box::new(bool_type())
    );

    // endsWith : Str, Str -> Bool
    add_top_level_function_type!(
        Symbol::STR_ENDS_WITH,
        vec![str_type(), str_type()],
        Box::new(bool_type())
    );

    // countGraphemes : Str -> Nat
    add_top_level_function_type!(
        Symbol::STR_COUNT_GRAPHEMES,
        vec![str_type()],
        Box::new(nat_type())
    );

    // fromInt : Int a -> Str
    add_top_level_function_type!(
        Symbol::STR_FROM_INT,
        vec![int_type(flex(TVAR1))],
        Box::new(str_type())
    );

    // fromUtf8 : List U8 -> Result Str [ BadUtf8 Utf8Problem ]*
    {
        let bad_utf8 = SolvedType::TagUnion(
            vec![(
                TagName::Global("BadUtf8".into()),
                vec![str_utf8_byte_problem_type(), nat_type()],
            )],
            Box::new(SolvedType::Wildcard),
        );

        add_top_level_function_type!(
            Symbol::STR_FROM_UTF8,
            vec![list_type(u8_type())],
            Box::new(result_type(str_type(), bad_utf8)),
        );
    }

    // fromUtf8Range : List U8 -> Result Str [ BadUtf8 Utf8Problem, OutOfBounds ]*
    {
        let bad_utf8 = SolvedType::TagUnion(
            vec![
                (
                    TagName::Global("BadUtf8".into()),
                    vec![str_utf8_byte_problem_type(), nat_type()],
                ),
                (TagName::Global("OutOfBounds".into()), vec![]),
            ],
            Box::new(SolvedType::Wildcard),
        );

        add_top_level_function_type!(
            Symbol::STR_FROM_UTF8_RANGE,
            vec![
                list_type(u8_type()),
                SolvedType::Record {
                    fields: vec![
                        ("start".into(), RecordField::Required(nat_type())),
                        ("count".into(), RecordField::Required(nat_type())),
                    ],
                    ext: Box::new(SolvedType::EmptyRecord),
                }
            ],
            Box::new(result_type(str_type(), bad_utf8)),
        );
    }

    // toUtf8 : Str -> List U8
    add_top_level_function_type!(
        Symbol::STR_TO_UTF8,
        vec![str_type()],
        Box::new(list_type(u8_type()))
    );

    // fromFloat : Float a -> Str
    add_top_level_function_type!(
        Symbol::STR_FROM_FLOAT,
        vec![float_type(flex(TVAR1))],
        Box::new(str_type())
    );

    // List module

    // get : List elem, Nat -> Result elem [ OutOfBounds ]*
    let index_out_of_bounds = SolvedType::TagUnion(
        vec![(TagName::Global("OutOfBounds".into()), vec![])],
        Box::new(SolvedType::Wildcard),
    );

    add_top_level_function_type!(
        Symbol::LIST_GET,
        vec![list_type(flex(TVAR1)), nat_type()],
        Box::new(result_type(flex(TVAR1), index_out_of_bounds)),
    );

    // first : List elem -> Result elem [ ListWasEmpty ]*
    let list_was_empty = SolvedType::TagUnion(
        vec![(TagName::Global("ListWasEmpty".into()), vec![])],
        Box::new(SolvedType::Wildcard),
    );

    add_top_level_function_type!(
        Symbol::LIST_FIRST,
        vec![list_type(flex(TVAR1))],
        Box::new(result_type(flex(TVAR1), list_was_empty.clone())),
    );

    // last : List elem -> Result elem [ ListWasEmpty ]*
    add_top_level_function_type!(
        Symbol::LIST_LAST,
        vec![list_type(flex(TVAR1))],
        Box::new(result_type(flex(TVAR1), list_was_empty)),
    );

    // set : List elem, Nat, elem -> List elem
    add_top_level_function_type!(
        Symbol::LIST_SET,
        vec![list_type(flex(TVAR1)), nat_type(), flex(TVAR1)],
        Box::new(list_type(flex(TVAR1))),
    );

    // concat : List elem, List elem -> List elem
    add_top_level_function_type!(
        Symbol::LIST_CONCAT,
        vec![list_type(flex(TVAR1)), list_type(flex(TVAR1))],
        Box::new(list_type(flex(TVAR1))),
    );

    // contains : List elem, elem -> Bool
    add_top_level_function_type!(
        Symbol::LIST_CONTAINS,
        vec![list_type(flex(TVAR1)), flex(TVAR1)],
        Box::new(bool_type()),
    );

    // sum :  List (Num a) -> Num a
    add_top_level_function_type!(
        Symbol::LIST_SUM,
        vec![list_type(num_type(flex(TVAR1)))],
        Box::new(num_type(flex(TVAR1))),
    );

    // product :  List (Num a) -> Num a
    add_top_level_function_type!(
        Symbol::LIST_PRODUCT,
        vec![list_type(num_type(flex(TVAR1)))],
        Box::new(num_type(flex(TVAR1))),
    );

    // walk : List elem, (elem -> accum -> accum), accum -> accum
    add_top_level_function_type!(
        Symbol::LIST_WALK,
        vec![
            list_type(flex(TVAR1)),
            closure(vec![flex(TVAR1), flex(TVAR2)], TVAR3, Box::new(flex(TVAR2))),
            flex(TVAR2),
        ],
        Box::new(flex(TVAR2)),
    );

    // walkBackwards : List elem, (elem -> accum -> accum), accum -> accum
    add_top_level_function_type!(
        Symbol::LIST_WALK_BACKWARDS,
        vec![
            list_type(flex(TVAR1)),
            closure(vec![flex(TVAR1), flex(TVAR2)], TVAR3, Box::new(flex(TVAR2))),
            flex(TVAR2),
        ],
        Box::new(flex(TVAR2)),
    );

    fn until_type(content: SolvedType) -> SolvedType {
        // [ LT, EQ, GT ]
        SolvedType::TagUnion(
            vec![
                (TagName::Global("Continue".into()), vec![content.clone()]),
                (TagName::Global("Stop".into()), vec![content]),
            ],
            Box::new(SolvedType::EmptyTagUnion),
        )
    }

    // walkUntil : List elem, (elem -> accum -> [ Continue accum, Stop accum ]), accum -> accum
    add_top_level_function_type!(
        Symbol::LIST_WALK_UNTIL,
        vec![
            list_type(flex(TVAR1)),
            closure(
                vec![flex(TVAR1), flex(TVAR2)],
                TVAR3,
                Box::new(until_type(flex(TVAR2))),
            ),
            flex(TVAR2),
        ],
        Box::new(flex(TVAR2)),
    );

    // keepIf : List elem, (elem -> Bool) -> List elem
    add_top_level_function_type!(
        Symbol::LIST_KEEP_IF,
        vec![
            list_type(flex(TVAR1)),
            closure(vec![flex(TVAR1)], TVAR2, Box::new(bool_type())),
        ],
        Box::new(list_type(flex(TVAR1))),
    );

    // keepOks : List before, (before -> Result after *) -> List after
    {
        let_tvars! { star, cvar, before, after};
        add_top_level_function_type!(
            Symbol::LIST_KEEP_OKS,
            vec![
                list_type(flex(before)),
                closure(
                    vec![flex(before)],
                    cvar,
                    Box::new(result_type(flex(after), flex(star))),
                ),
            ],
            Box::new(list_type(flex(after))),
        )
    };

    // keepErrs: List before, (before -> Result * after) -> List after
    {
        let_tvars! { star, cvar, before, after};

        add_top_level_function_type!(
            Symbol::LIST_KEEP_ERRS,
            vec![
                list_type(flex(before)),
                closure(
                    vec![flex(before)],
                    cvar,
                    Box::new(result_type(flex(star), flex(after))),
                ),
            ],
            Box::new(list_type(flex(after))),
        )
    };

    // range : Int a, Int a -> List (Int a)
    add_top_level_function_type!(
        Symbol::LIST_RANGE,
        vec![int_type(flex(TVAR1)), int_type(flex(TVAR1))],
        Box::new(list_type(int_type(flex(TVAR1)))),
    );

    // map : List before, (before -> after) -> List after
    add_top_level_function_type!(
        Symbol::LIST_MAP,
        vec![
            list_type(flex(TVAR1)),
            closure(vec![flex(TVAR1)], TVAR3, Box::new(flex(TVAR2))),
        ],
        Box::new(list_type(flex(TVAR2))),
    );

    // mapWithIndex : List before, (Nat, before -> after) -> List after
    {
        let_tvars! { cvar, before, after};
        add_top_level_function_type!(
            Symbol::LIST_MAP_WITH_INDEX,
            vec![
                list_type(flex(before)),
                closure(vec![nat_type(), flex(before)], cvar, Box::new(flex(after))),
            ],
            Box::new(list_type(flex(after))),
        )
    };

    // map2 : List a, List b, (a, b -> c) -> List c
    {
        let_tvars! {a, b, c, cvar};
        add_top_level_function_type!(
            Symbol::LIST_MAP2,
            vec![
                list_type(flex(a)),
                list_type(flex(b)),
                closure(vec![flex(a), flex(b)], cvar, Box::new(flex(c))),
            ],
            Box::new(list_type(flex(c))),
        )
    };

    {
        let_tvars! {a, b, c, d, cvar};

        // map3 : List a, List b, List c, (a, b, c -> d) -> List d
        add_top_level_function_type!(
            Symbol::LIST_MAP3,
            vec![
                list_type(flex(a)),
                list_type(flex(b)),
                list_type(flex(c)),
                closure(vec![flex(a), flex(b), flex(c)], cvar, Box::new(flex(d))),
            ],
            Box::new(list_type(flex(d))),
        )
    };

    // append : List elem, elem -> List elem
    add_top_level_function_type!(
        Symbol::LIST_APPEND,
        vec![list_type(flex(TVAR1)), flex(TVAR1)],
        Box::new(list_type(flex(TVAR1))),
    );

    // drop : List elem, Nat -> List elem
    add_top_level_function_type!(
        Symbol::LIST_DROP,
        vec![list_type(flex(TVAR1)), nat_type()],
        Box::new(list_type(flex(TVAR1))),
    );

    // swap : List elem, Nat, Nat -> List elem
    add_top_level_function_type!(
        Symbol::LIST_SWAP,
        vec![list_type(flex(TVAR1)), nat_type(), nat_type()],
        Box::new(list_type(flex(TVAR1))),
    );

    // prepend : List elem, elem -> List elem
    add_top_level_function_type!(
        Symbol::LIST_PREPEND,
        vec![list_type(flex(TVAR1)), flex(TVAR1)],
        Box::new(list_type(flex(TVAR1))),
    );

    // join : List (List elem) -> List elem
    add_top_level_function_type!(
        Symbol::LIST_JOIN,
        vec![list_type(list_type(flex(TVAR1)))],
        Box::new(list_type(flex(TVAR1))),
    );

    // single : a -> List a
    add_top_level_function_type!(
        Symbol::LIST_SINGLE,
        vec![flex(TVAR1)],
        Box::new(list_type(flex(TVAR1)))
    );

    // repeat : Nat, elem -> List elem
    add_top_level_function_type!(
        Symbol::LIST_REPEAT,
        vec![nat_type(), flex(TVAR1)],
        Box::new(list_type(flex(TVAR1))),
    );

    // reverse : List elem -> List elem
    add_top_level_function_type!(
        Symbol::LIST_REVERSE,
        vec![list_type(flex(TVAR1))],
        Box::new(list_type(flex(TVAR1))),
    );

    // len : List * -> Nat
    add_top_level_function_type!(
        Symbol::LIST_LEN,
        vec![list_type(flex(TVAR1))],
        Box::new(nat_type())
    );

    // isEmpty : List * -> Bool
    add_top_level_function_type!(
        Symbol::LIST_IS_EMPTY,
        vec![list_type(flex(TVAR1))],
        Box::new(bool_type())
    );

    // sortWith : List a, (a, a -> Ordering) -> List a
    add_top_level_function_type!(
        Symbol::LIST_SORT_WITH,
        vec![
            list_type(flex(TVAR1)),
            closure(
                vec![flex(TVAR1), flex(TVAR1)],
                TVAR2,
                Box::new(ordering_type()),
            ),
        ],
        Box::new(list_type(flex(TVAR1))),
    );

    // Dict module

    // Dict.hashTestOnly : Nat, v -> Nat
    add_top_level_function_type!(
        Symbol::DICT_TEST_HASH,
        vec![u64_type(), flex(TVAR2)],
        Box::new(nat_type())
    );

    // len : Dict * * -> Nat
    add_top_level_function_type!(
        Symbol::DICT_LEN,
        vec![dict_type(flex(TVAR1), flex(TVAR2))],
        Box::new(nat_type()),
    );

    // empty : Dict * *
    add_type!(Symbol::DICT_EMPTY, dict_type(flex(TVAR1), flex(TVAR2)));

    // single : k, v -> Dict k v
    add_top_level_function_type!(
        Symbol::DICT_SINGLE,
        vec![flex(TVAR1), flex(TVAR2)],
        Box::new(dict_type(flex(TVAR1), flex(TVAR2))),
    );

    // get : Dict k v, k -> Result v [ KeyNotFound ]*
    let key_not_found = SolvedType::TagUnion(
        vec![(TagName::Global("KeyNotFound".into()), vec![])],
        Box::new(SolvedType::Wildcard),
    );

    add_top_level_function_type!(
        Symbol::DICT_GET,
        vec![dict_type(flex(TVAR1), flex(TVAR2)), flex(TVAR1)],
        Box::new(result_type(flex(TVAR2), key_not_found)),
    );

    // Dict.insert : Dict k v, k, v -> Dict k v
    add_top_level_function_type!(
        Symbol::DICT_INSERT,
        vec![
            dict_type(flex(TVAR1), flex(TVAR2)),
            flex(TVAR1),
            flex(TVAR2),
        ],
        Box::new(dict_type(flex(TVAR1), flex(TVAR2))),
    );

    // Dict.remove : Dict k v, k -> Dict k v
    add_top_level_function_type!(
        Symbol::DICT_REMOVE,
        vec![dict_type(flex(TVAR1), flex(TVAR2)), flex(TVAR1)],
        Box::new(dict_type(flex(TVAR1), flex(TVAR2))),
    );

    // Dict.contains : Dict k v, k -> Bool
    add_top_level_function_type!(
        Symbol::DICT_CONTAINS,
        vec![dict_type(flex(TVAR1), flex(TVAR2)), flex(TVAR1)],
        Box::new(bool_type()),
    );

    // Dict.keys : Dict k v -> List k
    add_top_level_function_type!(
        Symbol::DICT_KEYS,
        vec![dict_type(flex(TVAR1), flex(TVAR2))],
        Box::new(list_type(flex(TVAR1))),
    );

    // Dict.values : Dict k v -> List v
    add_top_level_function_type!(
        Symbol::DICT_VALUES,
        vec![dict_type(flex(TVAR1), flex(TVAR2))],
        Box::new(list_type(flex(TVAR2))),
    );

    // Dict.union : Dict k v, Dict k v -> Dict k v
    add_top_level_function_type!(
        Symbol::DICT_UNION,
        vec![
            dict_type(flex(TVAR1), flex(TVAR2)),
            dict_type(flex(TVAR1), flex(TVAR2)),
        ],
        Box::new(dict_type(flex(TVAR1), flex(TVAR2))),
    );

    // Dict.intersection : Dict k v, Dict k v -> Dict k v
    add_top_level_function_type!(
        Symbol::DICT_INTERSECTION,
        vec![
            dict_type(flex(TVAR1), flex(TVAR2)),
            dict_type(flex(TVAR1), flex(TVAR2)),
        ],
        Box::new(dict_type(flex(TVAR1), flex(TVAR2))),
    );

    // Dict.difference : Dict k v, Dict k v -> Dict k v
    add_top_level_function_type!(
        Symbol::DICT_DIFFERENCE,
        vec![
            dict_type(flex(TVAR1), flex(TVAR2)),
            dict_type(flex(TVAR1), flex(TVAR2)),
        ],
        Box::new(dict_type(flex(TVAR1), flex(TVAR2))),
    );

    // Dict.walk : Dict k v, (k, v, accum -> accum), accum -> accum
    add_top_level_function_type!(
        Symbol::DICT_WALK,
        vec![
            dict_type(flex(TVAR1), flex(TVAR2)),
            closure(
                vec![flex(TVAR1), flex(TVAR2), flex(TVAR3)],
                TVAR4,
                Box::new(flex(TVAR3)),
            ),
            flex(TVAR3),
        ],
        Box::new(flex(TVAR3)),
    );

    // Set module

    // empty : Set a
    add_type!(Symbol::SET_EMPTY, set_type(flex(TVAR1)));

    // single : a -> Set a
    add_top_level_function_type!(
        Symbol::SET_SINGLE,
        vec![flex(TVAR1)],
        Box::new(set_type(flex(TVAR1)))
    );

    // len : Set * -> Nat
    add_top_level_function_type!(
        Symbol::SET_LEN,
        vec![set_type(flex(TVAR1))],
        Box::new(nat_type())
    );

    // toList : Set a -> List a
    add_top_level_function_type!(
        Symbol::SET_TO_LIST,
        vec![set_type(flex(TVAR1))],
        Box::new(list_type(flex(TVAR1))),
    );

    // fromList : List a -> Set a
    add_top_level_function_type!(
        Symbol::SET_FROM_LIST,
        vec![list_type(flex(TVAR1))],
        Box::new(set_type(flex(TVAR1))),
    );

    // union : Set a, Set a -> Set a
    add_top_level_function_type!(
        Symbol::SET_UNION,
        vec![set_type(flex(TVAR1)), set_type(flex(TVAR1))],
        Box::new(set_type(flex(TVAR1))),
    );

    // difference : Set a, Set a -> Set a
    add_top_level_function_type!(
        Symbol::SET_DIFFERENCE,
        vec![set_type(flex(TVAR1)), set_type(flex(TVAR1))],
        Box::new(set_type(flex(TVAR1))),
    );

    // intersection : Set a, Set a -> Set a
    add_top_level_function_type!(
        Symbol::SET_INTERSECTION,
        vec![set_type(flex(TVAR1)), set_type(flex(TVAR1))],
        Box::new(set_type(flex(TVAR1))),
    );

    // Set.walk : Set a, (a, b -> b), b -> b
    add_top_level_function_type!(
        Symbol::SET_WALK,
        vec![
            set_type(flex(TVAR1)),
            closure(vec![flex(TVAR1), flex(TVAR2)], TVAR3, Box::new(flex(TVAR2))),
            flex(TVAR2),
        ],
        Box::new(flex(TVAR2)),
    );

    add_top_level_function_type!(
        Symbol::SET_INSERT,
        vec![set_type(flex(TVAR1)), flex(TVAR1)],
        Box::new(set_type(flex(TVAR1))),
    );

    add_top_level_function_type!(
        Symbol::SET_REMOVE,
        vec![set_type(flex(TVAR1)), flex(TVAR1)],
        Box::new(set_type(flex(TVAR1))),
    );

    add_top_level_function_type!(
        Symbol::SET_CONTAINS,
        vec![set_type(flex(TVAR1)), flex(TVAR1)],
        Box::new(bool_type()),
    );

    // Result module

    // map : Result a err, (a -> b) -> Result b err
    add_top_level_function_type!(
        Symbol::RESULT_MAP,
        vec![
            result_type(flex(TVAR1), flex(TVAR3)),
            closure(vec![flex(TVAR1)], TVAR4, Box::new(flex(TVAR2))),
        ],
        Box::new(result_type(flex(TVAR2), flex(TVAR3))),
    );

    // mapErr : Result a x, (x -> y) -> Result a x
    add_top_level_function_type!(
        Symbol::RESULT_MAP_ERR,
        vec![
            result_type(flex(TVAR1), flex(TVAR3)),
            closure(vec![flex(TVAR3)], TVAR4, Box::new(flex(TVAR2))),
        ],
        Box::new(result_type(flex(TVAR1), flex(TVAR2))),
    );

    // after : Result a err, (a -> Result b err) -> Result b err
    add_top_level_function_type!(
        Symbol::RESULT_AFTER,
        vec![
            result_type(flex(TVAR1), flex(TVAR3)),
            closure(
                vec![flex(TVAR1)],
                TVAR4,
                Box::new(result_type(flex(TVAR2), flex(TVAR3))),
            ),
        ],
        Box::new(result_type(flex(TVAR2), flex(TVAR3))),
    );

    // withDefault : Result a x, a -> a
    add_top_level_function_type!(
        Symbol::RESULT_WITH_DEFAULT,
        vec![result_type(flex(TVAR1), flex(TVAR3)), flex(TVAR1)],
        Box::new(flex(TVAR1)),
    );

    types
}

#[inline(always)]
fn flex(tvar: VarId) -> SolvedType {
    SolvedType::Flex(tvar)
}

#[inline(always)]
fn closure(arguments: Vec<SolvedType>, closure_var: VarId, ret: Box<SolvedType>) -> SolvedType {
    SolvedType::Func(arguments, Box::new(SolvedType::Flex(closure_var)), ret)
}
