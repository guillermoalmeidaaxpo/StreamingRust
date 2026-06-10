# ANTLR Grammar for StreamingRust

This directory contains the ANTLR4 grammar files for the Outbound API filter expressions.

## How to generate Rust code

To generate the Rust lexer and parser, you need to use the ntlr4-rust target.
You can use a tool like ntlr4 with the Rust target installed.

`ash
antlr4 -Dlanguage=Rust -visitor -o ../src/infrastructure/generated grammar/*.g4
`

Make sure the generated code is properly referenced in src/infrastructure/mod.rs.
