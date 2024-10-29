 mod roll_l {
use lrlex::{LexerDef, LRNonStreamingLexerDef, Rule, StartState};

#[allow(dead_code)]
pub fn lexerdef() -> LRNonStreamingLexerDef<lrlex::defaults::DefaultLexerTypes> {
let regex_options = ::lrlex::RegexOptions {
            dot_matches_new_line: true,
            multi_line: true,
            octal: true,
            case_insensitive: None,
            unicode: None,
            swap_greed: None,
            ignore_whitespace: None,
            size_limit: None,
            dfa_size_limit: None,
            nest_limit: None,
        };    let start_states: Vec<StartState> = vec![
        StartState::new(0, "INITIAL", false, ::cfgrammar::Span::new(0, 0)),
    ];
    let rules = vec![
        Rule::new(::lrlex::unstable_api::InternalPublicApi, None, Some("DIE".to_string()), ::cfgrammar::Span::new(18, 21), "[0-9]+d[0-9]+".to_string(), [].to_vec(), None, &regex_options).unwrap(),
        Rule::new(::lrlex::unstable_api::InternalPublicApi, Some(4), Some("INTEGER".to_string()), ::cfgrammar::Span::new(31, 38), "[0-9]+".to_string(), [].to_vec(), None, &regex_options).unwrap(),
        Rule::new(::lrlex::unstable_api::InternalPublicApi, Some(0), Some("ADD".to_string()), ::cfgrammar::Span::new(44, 47), "\\+".to_string(), [].to_vec(), None, &regex_options).unwrap(),
        Rule::new(::lrlex::unstable_api::InternalPublicApi, None, Some("SUB".to_string()), ::cfgrammar::Span::new(53, 56), "\\-".to_string(), [].to_vec(), None, &regex_options).unwrap(),
        Rule::new(::lrlex::unstable_api::InternalPublicApi, Some(1), Some("MUL".to_string()), ::cfgrammar::Span::new(62, 65), "\\*".to_string(), [].to_vec(), None, &regex_options).unwrap(),
        Rule::new(::lrlex::unstable_api::InternalPublicApi, Some(2), Some("LPAR".to_string()), ::cfgrammar::Span::new(71, 75), "\\(".to_string(), [].to_vec(), None, &regex_options).unwrap(),
        Rule::new(::lrlex::unstable_api::InternalPublicApi, Some(3), Some("RPAR".to_string()), ::cfgrammar::Span::new(81, 85), "\\)".to_string(), [].to_vec(), None, &regex_options).unwrap(),
        Rule::new(::lrlex::unstable_api::InternalPublicApi, Some(7), None, ::cfgrammar::Span::new(96, 96), "[\\t\\n ]+".to_string(), [].to_vec(), None, &regex_options).unwrap(),
    ];
    LRNonStreamingLexerDef::from_rules(start_states, rules)
}

#[allow(dead_code)]
pub const T_INTEGER: u32 = 4;
#[allow(dead_code)]
pub const T_MUL: u32 = 1;
#[allow(dead_code)]
pub const T_LPAR: u32 = 2;
#[allow(dead_code)]
pub const T_RPAR: u32 = 3;
#[allow(dead_code)]
pub const T_ADD: u32 = 0;
}