mod crust;

fn main() {
    crust::sql::Parser::parse("SELECT * FROM foobar;");
}