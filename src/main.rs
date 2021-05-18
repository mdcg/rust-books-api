// O atributo macro_use tem dois propósitos. Primeiro, ele pode ser usado para fazer
// com que o escopo da macro de um módulo não termine quando o módulo for fechado,
// aplicando-o a um módulo:

//          #[macro_use]
//          mod inner {
//              macro_rules! m {
//                  () => {};
//              }
//          }
//          m!();

// Em segundo lugar, ele pode ser usado para importar macros de outra crate,
// anexando-a a uma declaração de crate externa que aparece no módulo raiz da crate.
// As macros importadas dessa forma são importadas para o prelúdio macro_use, não
// textualmente, o que significa que podem ser sombreadas por qualquer outro nome.
// Embora as macros importadas por #[macro_use] possam ser usadas antes da instrução
// de importação, em caso de conflito, a última macro importada vence.
// Opcionalmente, uma lista de macros a serem importadas pode ser especificada usando
// a sintaxe MetaListIdents; isso não é suportado quando #[macro_use] é aplicado
// a um módulo.

//          #[macro_use(lazy_static)] // Or #[macro_use] to import all macros.
//          extern crate lazy_static;
//          lazy_static!{}
//          self::lazy_static!{} // Error: lazy_static is not defined in `self`

// Referência: https://doc.rust-lang.org/reference/macros-by-example.html#the-macro_use-attribute

#[macro_use]
extern crate diesel;
extern crate dotenv;

use dotenv::dotenv;
use std::env;
use diesel::prelude::*;
use diesel::pg::PgConnection;

mod schema;
mod models;

fn main() {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("set DATABASE_URL");
    let conn = PgConnection::establish(&database_url).unwrap();

    let book = models::NewBook {
        title: String::from("Gravity's Rainbow"),
        author: String::from("Thomas Pynchon"),
        published: true,
    };

    if models::Book::insert(book, &conn) {
        println!("success");
    } else {
        println!("failed");
    }
}
