mod csv_reader;
mod operations;
mod plot;


use std::error::Error;
use structopt::StructOpt;
use csv_reader::{ler_csv_infracoes, ler_csv_descricao};
use operations::{exibir_infracoes, agrupar_somar_quantidades, exibir_somas, extrair_codigos_unicos, exibir_codigos_unicos, agrupar_por_estado, exibir_somas_por_estado};
use plot::plotar_barras;

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

    /// Operação a ser realizada: exibir, agrupar_por_tipo, agrupar_por_uf, codigos_unicos
    #[structopt(short, long)]
    operacao: String,

    /// Caminho do arquivo de saída para o gráfico
    #[structopt(short, long)]
    grafico: Option<String>,
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
            // if let Some(grafico_path) = args.grafico {
            //     plotar_barras(&agrupadas, &descricao_map, "Infrações por Tipo", &grafico_path)?;
            // }

        }
        "agrupar_por_uf" => {
            let agrupadas_por_estado = agrupar_por_estado(&infracoes);
            exibir_somas_por_estado(&agrupadas_por_estado); 
            if let Some(grafico_path) = args.grafico {
                let agrupadas_por_estado_vec: Vec<_> = agrupadas_por_estado.into_iter().collect();
                plotar_barras(&agrupadas_por_estado_vec, &descricao_map, "Infrações por Estado", &grafico_path)?;
            }      
        }
        "codigos_unicos" => {
            let codigos_unicos = extrair_codigos_unicos(&infracoes);
            exibir_codigos_unicos(&codigos_unicos);
        }
        _ => println!("Operação não reconhecida. Use 'exibir', 'agrupar_por_tipo', 'agrupar_por_uf' ou 'codigos_unicos'."),
    }

    Ok(())
}
