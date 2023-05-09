use chrono::{offset::Utc, DateTime};
use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

#[derive(Debug, Serialize, Deserialize)]
pub struct Cidade {
    pub nome: String,
    pub estado: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Endereco {
    pub logradouro: String,
    pub numero: String,
    pub cidade: Cidade,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Cliente {
    pub nome: String,
    pub endereco: Endereco,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Estoque {
    pub quantidade: f64,
    pub unidade: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Produto {
    pub nome: String,
    pub estoque: Estoque,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Faturamentos {
    pub clientes: Vec<Thing>,
    pub produtos: Vec<Thing>,
    pub faturamento: Vec<DateTime<Utc>>,
}
