use serde::Deserialize;
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use csv::ReaderBuilder;

#[derive(Debug, Deserialize)]
pub struct Infracao {
    pub UF: String,
    pub Cod_Infracao: String,
    pub Quantidade: u32,
}

#[derive(Debug, Deserialize)]
pub struct DescricaoInfracao {
    pub codigo: String,
    pub descricao: String,
}

pub fn ler_csv_infracoes(caminhos: Vec<&str>) -> Result<Vec<Infracao>, Box<dyn Error>> {
    let mut infracoes: Vec<Infracao> = Vec::new();
    for caminho in caminhos {
        let file = File::open(caminho)?;
        let mut rdr = ReaderBuilder::new().delimiter(b';').from_reader(file);
        for result in rdr.deserialize() {
            let record: Infracao = result?;
            infracoes.push(record);
        }
    }
    Ok(infracoes)
}

pub fn ler_csv_descricao(caminho: &str) -> Result<HashMap<String, String>, Box<dyn Error>> {
    let file = File::open(caminho)?;
    let mut rdr = ReaderBuilder::new().delimiter(b';').from_reader(file);
    let mut descricao_map = HashMap::new();
    for result in rdr.deserialize() {
        let record: DescricaoInfracao = result?;
        descricao_map.insert(record.codigo, record.descricao);
    }
    Ok(descricao_map)
}
