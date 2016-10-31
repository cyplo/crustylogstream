# crustylogstream
Structured log streaming parser library for Rust.

Given a stream of text it should give a stream of tokens, accordingly to the line specification given.

Should be able to use a syntax similar to goaccess.

Example:
    
    [%d:%t %^] %h "%v" "%r" %s %b "%R" "%u" %T

where `%d` is date format, e.g. `%d/%b/%Y` and `%t%` is time format, eg `%H:%M:%S`
