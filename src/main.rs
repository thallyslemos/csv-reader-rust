use csv::ReaderBuilder;
use serde::Deserialize;
use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fs::File;
use structopt::StructOpt;

/// Estrutura que representa uma infração
#[derive(Debug, Deserialize)]
struct Infracao {
    UF: String,
    Cod_Infracao: String,
    Quantidade: u32,
}

/// Estrutura que representa a descrição de uma infração
#[derive(Debug, Deserialize)]
struct DescricaoInfracao {
    codigo: String,
    descricao: String,
}

/// Estrutura para armazenar os argumentos de linha de comando
#[derive(StructOpt, Debug)]
#[structopt(name = "infracoes")]
struct Cli {
    /// Caminhos para os arquivos CSV de infrações
    #[structopt(short, long)]
    infracoes: Vec<String>,

    /// Caminho para o arquivo CSV com as descrições das infrações
    #[structopt(short, long)]
    descricao: String,

    /// Operação a ser realizada: exibir, agrupar, codigos_unicos
    #[structopt(short, long)]
    operacao: String,
}

/// Função para ler os arquivos CSV de infrações
fn ler_csv_infracoes(caminhos: Vec<&str>) -> Result<Vec<Infracao>, Box<dyn Error>> {
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

/// Função para ler o arquivo CSV com as descrições das infrações
fn ler_csv_descricao(caminho: &str) -> Result<HashMap<String, String>, Box<dyn Error>> {
    let file = File::open(caminho)?;
    let mut rdr = ReaderBuilder::new().delimiter(b';').from_reader(file);
    let mut descricao_map = HashMap::new();
    for result in rdr.deserialize() {
        let record: DescricaoInfracao = result?;
        descricao_map.insert(record.codigo, record.descricao);
    }
    Ok(descricao_map)
}

/// Função para exibir as infrações lidas
fn exibir_infracoes(infracoes: &[Infracao]) {
    println!("Infrações lidas:");
    for infracao in infracoes {
        println!("{:?}", infracao);
    }
}

/// Função para agrupar e somar as quantidades por tipo de infração
fn agrupar_somar_quantidades(infracoes: &[Infracao]) -> HashMap<String, u32> {
    let mut agrupadas: HashMap<String, u32> = HashMap::new();
    for infracao in infracoes {
        let counter = agrupadas.entry(infracao.Cod_Infracao.clone()).or_insert(0);
        *counter += infracao.Quantidade;
    }
    agrupadas
}

/// Função para exibir as somas das infrações por tipo, utilizando as descrições dos códigos
fn exibir_somas(agrupadas: &HashMap<String, u32>, descricao_map: &HashMap<String, String>) {
    println!("\nSoma das infrações por tipo:");
    for (cod_infracao, quantidade) in agrupadas {
        if let Some(descricao) = descricao_map.get(cod_infracao) {
            println!("{}: {} - {}", cod_infracao, descricao, quantidade);
        } else {
            println!("{}: Descrição não encontrada - {}", cod_infracao, quantidade);
        }
    }
}

/// Função para extrair os códigos de infração únicos
fn extrair_codigos_unicos(infracoes: &[Infracao]) -> Vec<String> {
    let mut codigos_unicos: HashSet<String> = HashSet::new();
    for infracao in infracoes {
        codigos_unicos.insert(infracao.Cod_Infracao.clone());
    }
    codigos_unicos.into_iter().collect()
}

/// Função para exibir a lista de códigos de infração únicos
fn exibir_codigos_unicos(codigos: &[String]) {
    println!("Lista de Códigos de Infração Únicos:");
    for codigo in codigos {
        println!("{}", codigo);
    }
}

// Nova função para agrupar o total de infrações por estado.
fn agrupar_por_estado(infracoes: &[Infracao]) -> HashMap<String, u32> {
    let mut agrupadas_por_estado: HashMap<String, u32> = HashMap::new();
    for infracao in infracoes {
        let counter = agrupadas_por_estado.entry(infracao.UF.clone()).or_insert(0);
        *counter += infracao.Quantidade;
    }
    agrupadas_por_estado
}

// Função para exibir a soma das infrações por estado.
fn exibir_somas_por_estado(agrupadas_por_estado: &HashMap<String, u32>) {
    println!("\nSoma das infrações por estado:");
    for (estado, quantidade) in agrupadas_por_estado {
        println!("{}: {}", estado, quantidade);
    }
}

/// Função principal
fn main() -> Result<(), Box<dyn Error>> {
    let args = Cli::from_args();

    // Converte os caminhos das infrações para uma fatia de strings
    let caminhos_infracoes: Vec<&str> = args.infracoes.iter().map(AsRef::as_ref).collect();
    let caminho_descricao = &args.descricao;

    let infracoes = ler_csv_infracoes(caminhos_infracoes)?;
    let descricao_map = ler_csv_descricao(caminho_descricao)?;

    // Executa a operação especificada pelo usuário
    match args.operacao.as_str() {
        "exibir" => exibir_infracoes(&infracoes),
        "agrupar_por_tipo" => {
            let agrupadas = agrupar_somar_quantidades(&infracoes);
            exibir_somas(&agrupadas, &descricao_map);
        }
        "agrupar_por_uf" => {
            let agrupadas_por_estado = agrupar_por_estado(&infracoes);
            exibir_somas_por_estado(&agrupadas_por_estado);        
        }
        "codigos_unicos" => {
            let codigos_unicos = extrair_codigos_unicos(&infracoes);
            exibir_codigos_unicos(&codigos_unicos);
        }
        _ => println!("Operação não reconhecida. Use 'exibir', 'agrupar' ou 'codigos_unicos'."),
    }

    Ok(())
}
