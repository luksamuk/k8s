use serde::Deserialize;
use surrealdb::{engine::remote::ws::Ws, opt::auth::Root, sql::Thing, Surreal};

mod model;

use model::*;

#[derive(Clone, Debug, Deserialize)]
struct Record {
    #[allow(dead_code)]
    pub id: Thing,
}

#[tokio::main]
async fn main() -> surrealdb::Result<()> {
    // Connect to database
    let db = Surreal::new::<Ws>("rpi4.local:30101").await?;
    db.signin(Root {
        username: "root",
        password: "root",
    })
    .await?;

    db.use_ns("test").use_db("test").await?;

    // Create a client
    let cliente: Record = db
        .create("cliente")
        .content(Cliente {
            nome: "Fulano de Tal".to_string(),
            endereco: Endereco {
                logradouro: "Rua dos Bobos".to_string(),
                numero: "0".to_string(),
                cidade: Cidade {
                    nome: "Diamantina".to_string(),
                    estado: "Minas Gerais".to_string(),
                },
            },
        })
        .await?;

    // Create a product
    let produto: Record = db
        .create("produto")
        .content(Produto {
            nome: "Leite Longa Vida".to_string(),
            estoque: Estoque {
                quantidade: 50.0,
                unidade: "CX".to_string(),
            },
        })
        .await?;

    // Sell that product three times to the client
    for _ in 0..3 {
        let qtd = 1.0;
        // Each transaction is a new relation between
        // client and product through the verb "comprar",
        // which stores a record named "compra" containing
        // the moment of the transaction ("faturamento")
        // and the amount of stuff bought ("quantidade").
        //
        // Furthermore, the amount bought is also deduced
        // from stock by updating the product.
        db.query(
            "RELATE $client->comprar->$product
             SET
                 compra.faturamento = time::now(),
                 compra.quantidade = $qtd",
        )
        .query(
            "UPDATE $product
             SET
                 estoque.quantidade -= $qtd",
        )
        .bind(("client", cliente.id.clone()))
        .bind(("product", produto.id.clone()))
        .bind(("qtd", qtd))
        .await?;
    }

    // Special query for all buys involving a certain product
    // which we shall recognize by id. We'll fetch the clients
    // involved, the product involved, and the date of buy.
    //
    // Each field is an ordered array for the query.
    let mut response = db
        .query(
            "SELECT
                <-comprar<-cliente as clientes,
                <-comprar<-cliente->comprar->produto as produtos,
                <-comprar.compra.faturamento as faturamento
             FROM produto
             WHERE id = $produto",
        )
        .bind(("produto", produto.id))
        .await?;

    // We take the response for the first (and only) query and
    // deserialize it
    let objs: Option<Faturamentos> = response.take(0)?;

    if let Some(objs) = objs {
        // Now all we have to do is fetch client and product
        // info and organize all that we have...
        let mut fats = vec![];
        for i in 0..objs.faturamento.len() {
            let cliente: Option<Cliente> = db.select(objs.clientes[i].clone()).await?;
            let produto: Option<Produto> = db.select(objs.produtos[i].clone()).await?;
            fats.push((cliente.unwrap(), produto.unwrap(), objs.faturamento[i]));
        }

        println!("Lista de faturamentos:");
        for fat in fats {
            println!(
                "{:#?}\n{:#?}\nData de Faturamento: {}\n",
                fat.0, fat.1, fat.2
            );
        }
    }

    Ok(())
}
