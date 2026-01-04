lexer grammar EsmLexer;
options { language=CSharp; }

Semi: ';';
Colon: ':';

LCurly: '{';
RCurly: '}';

// basic operations
Nop: 'nop'; // do nothing, no args
Halt: 'halt'; // TODO: how are args formatted?

// Debug operations
Dev: 'dev'; // debug op prefix
Breakpoint: 'brkpt'; // dev.brkpt

Ldc: 'ldc'; // load a constant to a register, requires a type, should be unsigned if a number
Str: 'str'; // store
// primitives
I8: 'i8'; // 8-bit buffer
I16: 'i16'; // 16-bit buffer
I32: 'i32'; // 32-bit buffer
I64: 'i64'; // 64-bit buffer
Ptr:'ptr'; // ptr-size buffer

Signext: 'sgxt';
Zeroext: 'zrxt';
Trunc: 'trnc';
As: 'as';

// register ops
IAdd: 'iadd';
FAdd: 'fadd';
ISub: 'isub';
FSub: 'fsub';
IMul: 'imul';
FMul: 'fmul';
IDiv: 'idiv';
SDiv: 'sdiv';
FDiv: 'fdiv';
IMod: 'imod';
SMod: 'smod';
FMod: 'fmod';
And: 'and';
Or : 'or' ;
Xor: 'xor';
Not: 'not';
Lsh: 'lsh';
Rsh: 'rsh';
Srsh: 'srsh'; // >> (unsigned)
Lrot: 'lrot'; // <<<
Rrot: 'rrot'; // >>>, not signed right shift as that is +>>

// Memory operations
Mem: 'mem'; // prefix
Alloc: 'alloc';
Zalloc: 'zalloc'; // aka calloc
Realloc: 'realloc';
Rezalloc: 'rezalloc';
Clear: 'clear';
Free: 'free';
Copy: 'copy';

// Jump operations
Jmp: 'jmp'; // prefix
Abs: 'abs';
Rel: 'rel';
Tbl: 'tbl';
Ind: 'ind';

Branch: 'brnch';


// Conditionals
If: 'if';
Bang: '!';
Zero: 'zero';



// Constants
DecIntLiteral: DECIMAL('_'* DECIMAL)* /*INT_SUFFIX?*/ ;
HexIntLiteral: '0'[Xx]('_'* HEX)+ /*INT_SUFFIX?*/ ;
BinIntLiteral: '0'[Bb]('_'* BINARY)+ /*INT_SUFFIX?*/ ;
OctIntLiteral: '0'[Oo]('_'* OCTAL)+ /*INT_SUFFIX?*/ ;

LabelId: '#' ID_UNPREFIXED;

Identifier: '`'? ID_UNPREFIXED;

CharLiteral: '\'' STR_CHAR '\'';
StrLiteral: '"' STR_CHAR* '"';

fragment STR_CHAR: (~[\n\r\\"] | ESC_SEQ);
fragment ESC_SEQ: '\\'(["\\0abfnrtv]|HEX_ESC);
fragment HEX_ESC: 'x' HEX (HEX (HEX HEX?)?)?;


//fragment INT_SUFFIX: [ui]'8';

fragment ID_UNPREFIXED: ID_START ID_CHAR*;

fragment ID_START :           LETTER | '_';
fragment ID_CHAR  : DECIMAL | LETTER | '_';

fragment LETTER  : [A-Za-z]   ;
fragment DECIMAL : [0-9]      ;
fragment HEX     : [0-9A-Fa-f];
fragment BINARY   : [01]      ;
fragment OCTAL   : [0-7]      ;

LineComment  : '//' ~[\n]*? (EOF | '\n') -> skip;

BlockCommentStart: '/*' -> skip, pushMode(BLOCK_COMMENT_MODE);
Whitespace: [ \t\r\n]+ -> skip;

mode BLOCK_COMMENT_MODE;

BlockCommentRecStart: '/*' -> skip, pushMode(BLOCK_COMMENT_MODE);
BlockCommentEnd: '*/' -> skip, popMode;

CommentContents:  .+? -> skip;