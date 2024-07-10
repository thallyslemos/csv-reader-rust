use plotters::{element::Drawable, prelude::*};
use std::collections::HashMap;

pub fn plotar_barras(
    data: &[(String, u32)],
    descricao_map: &HashMap<String, String>,
    titulo: &str,
    filename: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let filename = if filename.ends_with(".png") {
        filename.to_string()
    } else {
        format!("{}.png", filename)
    };
    let root = BitMapBackend::new(&filename, (640, 480)).into_drawing_area();
    root.fill(&WHITE)?;

    // Configura a área do gráfico
    let mut chart = ChartBuilder::on(&root)
        .caption(titulo, ("sans-serif", 40))
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(
            0..data.len(),
            0..data.iter().map(|(_, v)| *v).max().unwrap_or(0) as i32,
        )?;

    // Desenha a malha do gráfico
    chart.configure_mesh().draw()?;

    // Configura e desenha as legendas
    chart
        .configure_series_labels()
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .draw()?;

    // Itera sobre os dados para desenhar cada barra
    for (idx, (estado, quantidade)) in data.iter().enumerate() {
        let descricao = descricao_map.get(estado).unwrap_or(estado);
        let x0 = idx;
        let y0 = 0;
        let x1 = idx + 1;
        let y1 = *quantidade as i32;

        // Desenha a série (barra) e adiciona a legenda
        chart
            .draw_series(std::iter::once(Rectangle::new(
                [(x0, y0), (x1, y1)],
                BLUE.filled(),
            )))
            .unwrap()
            .label(descricao.clone())
            .legend(move |(x, y)| Rectangle::new([(x, y - 5), (x + 10, y + 5)], BLUE.filled()));
    }

    // Apresenta o gráfico finalizado
    root.present()?;

    Ok(())
}
