use polars::prelude::*;
use std::fs::File;
use std::error::Error;

pub fn ler_csv(caminho: &str) -> PolarsResult<DataFrame> {
    CsvReader::from_path("iris_csv")?
            .has_header(true)
            .finish()
}


// pub fn calcular_total_infracoes(df: &DataFrame) -> Result<i64, PolarsError> {
//     let total: i64 = df.column("Quantidade")?.i64()?.sum().unwrap_or(0);
//     Ok(total)
// }

// pub fn filtrar_por_uf(df: &DataFrame, uf: &str) -> Result<DataFrame, PolarsError> {
//     let filtro_uf = df.filter(&df.column("UF")?.utf8()?.eq(uf))?;
//     Ok(filtro_uf)
// }

// pub fn agrupar_por_uf(df: &DataFrame) -> Result<DataFrame, PolarsError> {
//     let grouped_df = df.groupby("UF")?
//         .agg(&[("Quantidade", &["sum", "mean", "min", "max"])])?;
//     Ok(grouped_df)
// }

// pub fn salvar_csv(df: &mut DataFrame, caminho: &str) -> Result<(), Box<dyn Error>> {
//     let mut file = File::create(caminho)?;
//     CsvWriter::new(&mut file).finish(df)?;
//     Ok(())
// }
