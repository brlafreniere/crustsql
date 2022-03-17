SELECT col_a, col_b FROM tbl_name WHERE conditions 

need a way to break this string into logical parts.

# Ideas

1. have a list of keywords, and then search by keyword, and perhaps even look
for keywords in a certain order.

Primary Commands:
- SELECT, CREATE, UPDATE, DELETE, DROP

The primary commands are positional, they are the first command that can appear
in the query.

If our primary command is SELECT, then the next expected command is going to be
FROM, optionally followed by WHERE.

Primary commands determine what follows.