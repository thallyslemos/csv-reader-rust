use std::collections::{HashMap, HashSet};
use crate::csv_reader::{Infracao};

pub fn exibir_infracoes(infracoes: &[Infracao]) {
    println!("Infrações lidas:");
    for infracao in infracoes {
        println!("{:?}", infracao);
    }
}

pub fn agrupar_somar_quantidades(infracoes: &[Infracao]) -> HashMap<String, u32> {
    let mut agrupadas: HashMap<String, u32> = HashMap::new();
    for infracao in infracoes {
        let counter = agrupadas.entry(infracao.Cod_Infracao.clone()).or_insert(0);
        *counter += infracao.Quantidade;
    }
    agrupadas
}

pub fn exibir_somas(agrupadas: &HashMap<String, u32>, descricao_map: &HashMap<String, String>) {
    println!("\nSoma das infrações por tipo:");
    for (cod_infracao, quantidade) in agrupadas {
        if let Some(descricao) = descricao_map.get(cod_infracao) {
            println!("{}: {} - {}", cod_infracao, descricao, quantidade);
        } else {
            println!("{}: Descrição não encontrada - {}", cod_infracao, quantidade);
        }
    }
}

pub fn extrair_codigos_unicos(infracoes: &[Infracao]) -> Vec<String> {
    let mut codigos_unicos: HashSet<String> = HashSet::new();
    for infracao in infracoes {
        codigos_unicos.insert(infracao.Cod_Infracao.clone());
    }
    codigos_unicos.into_iter().collect()
}

pub fn exibir_codigos_unicos(codigos: &[String]) {
    println!("Lista de Códigos de Infração Únicos:");
    for codigo in codigos {
        println!("{}", codigo);
    }
}

pub fn agrupar_por_estado(infracoes: &[Infracao]) -> HashMap<String, u32> {
    let mut agrupadas_por_estado: HashMap<String, u32> = HashMap::new();
    for infracao in infracoes {
        let counter = agrupadas_por_estado.entry(infracao.UF.clone()).or_insert(0);
        *counter += infracao.Quantidade;
    }
    agrupadas_por_estado
}

pub fn exibir_somas_por_estado(agrupadas_por_estado: &HashMap<String, u32>) {
    println!("\nSoma das infrações por estado:");
    for (estado, quantidade) in agrupadas_por_estado {
        println!("{}: {}", estado, quantidade);
    }
}
