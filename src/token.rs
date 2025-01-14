//! cryss で使用されるトークン

/// トークン
#[derive(Debug, PartialEq)]
pub enum Token {
    /// 識別子．
    Identifier(String),
    /// `$` で始まる
    Parameter(String),
    Number(f64),
    String(String),
    KeywordLet,
    KeywordBreak,
    KeywordContinue,
    /// キーワード `if`
    KeywordIf,
    /// キーワード `else`
    KeywordElse,
    /// キーワード `while`
    KeywordWhile,
    /// キーワード `for`
    /// `for` 文の syntax は考え中
    KeywordFor,
    KeywordReturn,
    KeywordDef,
    /// `+`: 足し算
    Plus,
    /// `-`: （ 2 項）引き算，（単項）負号
    Hyphen,
    /// `->`: 右矢印
    HyphenGreater,
    /// `*`: 掛け算
    Asterisk,
    /// `/`: 割り算，（単項）逆数
    Slash,
    /// `%`: 割った余り
    Percent,
    /// `^`: 累乗
    Circumflex,
    /// `=`: 代入
    Equal,
    /// `=>`: 右代入
    EqualGreater,
    /// `==`: 等しい
    DoubleEqual,
    /// `!`: 論理否定
    Exclamation,
    /// `!=`: 等しくない
    ExclamationEqual,
    /// `<`: より小さい
    Less,
    /// `<<`: 左シフト（ Sound ）
    DoubleLess,
    /// `>`: より大きい
    Greater,
    /// `>>`: 右シフト（ Sound ）
    DoubleGreater,
    /// `&&`: 論理積
    DoubleAmpersand,
    /// `|`
    Bar,
    /// `||`: 論理和
    DoubleBar,
    /// `:`
    Colon,
    /// `;`
    Semicolon,
    /// `,`
    Comma,
    /// `?`: 出力
    Question,
    /// `(`
    OpeningParenthesis,
    /// `)`
    ClosingParenthesis,
    /// `[`
    OpeningBracket,
    /// `]`
    ClosingBracket,
    /// `{`
    OpeningBrace,
    /// `}`
    ClosingBrace,
}
