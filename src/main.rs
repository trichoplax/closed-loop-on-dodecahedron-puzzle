use std::fs;
use std::f64::consts::PI;

const DEGREES: f64 = PI / 180_f64;

fn main() -> std::io::Result<()> {
    let cos72 = (72. * DEGREES).cos();
    let cos54 = (54. * DEGREES).cos();
    let sin72 = (72. * DEGREES).sin();
    let tan54 = (54. * DEGREES).tan();
    let pentagon_edge_length = 150.;
    let pentagon_stroke_width = 15.;
    let pentagon_narrowed_stroke_width = pentagon_stroke_width * 0.9;
    let dodecahedron_net_image_width = pentagon_edge_length * (6. + 7. * cos72);
    let dodecahedron_net_image_height = pentagon_edge_length * (2. * cos54 + 3. * sin72);
    let pentagon_edge_start_x = pentagon_edge_length * cos72;
    let pentagon_narrowed_edge_x_adjustment = (pentagon_stroke_width - pentagon_narrowed_stroke_width) / 2. / tan54;
    let pentagon_narrowed_edge_start_x = pentagon_edge_start_x + pentagon_narrowed_edge_x_adjustment;
    let pentagon_narrowed_edge_length = pentagon_edge_length - 2. * pentagon_narrowed_edge_x_adjustment;
    let pentagon_height = pentagon_edge_length * (cos54 + sin72);
    let pentagon_edge_start_y = pentagon_height - pentagon_stroke_width / 2.;
    let pentagon_centre_x = pentagon_edge_length * (0.5 + cos72);
    let pentagon_centre_y = pentagon_edge_length / (2. * cos54);
    let top_pentagon_x = pentagon_edge_length * (1. + cos72);
    let top_pentagon_centre_x = pentagon_edge_length * (1.5 + 2. * cos72);
    let centre_pentagon_centre_y = pentagon_height + pentagon_edge_length * 0.5 * tan54;
    let dodecahedron_net_centre_x = dodecahedron_net_image_width / 2.;
    let dodecahedron_net_centre_y = dodecahedron_net_image_height / 2.;

    fs::write("dodecahedron-net.svg", format!(r##"<svg width="{dodecahedron_net_image_width}" height="{dodecahedron_net_image_height}" viewBox="0 0 {dodecahedron_net_image_width} {dodecahedron_net_image_height}" xmlns="http://www.w3.org/2000/svg" version="2">
    <defs>
        <path id="pentagon-edge" d="M {pentagon_narrowed_edge_start_x},{pentagon_edge_start_y} h {pentagon_narrowed_edge_length}" fill="none" stroke="grey" stroke-width="{pentagon_narrowed_stroke_width}" />
        <g id="pentagon-tile">
            <use href="#pentagon-edge" stroke="red" />
            <use href="#pentagon-edge" transform="rotate(72, {pentagon_centre_x}, {pentagon_centre_y})" stroke="orange" />
            <use href="#pentagon-edge" transform="rotate(144, {pentagon_centre_x}, {pentagon_centre_y})" stroke="yellow" />
            <use href="#pentagon-edge" transform="rotate(216, {pentagon_centre_x}, {pentagon_centre_y})" stroke="green" />
            <use href="#pentagon-edge" transform="rotate(288, {pentagon_centre_x}, {pentagon_centre_y})" stroke="blue" />
        </g>
        <g id="pentagons-around-pentagon">
            <use href="#pentagon-tile" x="{top_pentagon_x}" y="0" transform="rotate(180, {top_pentagon_centre_x}, {pentagon_height})" />
            <use href="#pentagon-tile" x="{top_pentagon_x}" y="0" />
            <use href="#pentagon-tile" x="{top_pentagon_x}" y="0" transform="rotate(72, {top_pentagon_centre_x}, {centre_pentagon_centre_y})" />
            <use href="#pentagon-tile" x="{top_pentagon_x}" y="0" transform="rotate(144, {top_pentagon_centre_x}, {centre_pentagon_centre_y})" />
            <use href="#pentagon-tile" x="{top_pentagon_x}" y="0" transform="rotate(216, {top_pentagon_centre_x}, {centre_pentagon_centre_y})" />
            <use href="#pentagon-tile" x="{top_pentagon_x}" y="0" transform="rotate(288, {top_pentagon_centre_x}, {centre_pentagon_centre_y})" />
        </g>
        <g id="dodecahedron-net">
            <use href="#pentagons-around-pentagon" />
            <use href="#pentagons-around-pentagon" transform="rotate(180, {dodecahedron_net_centre_x}, {dodecahedron_net_centre_y})" />
        </g>
    </defs>
    <use href="#dodecahedron-net" />
</svg>
"##))?;

    let pentagon_width = pentagon_edge_length * (1. + 2. * cos72);
    let dodecahedron_tiles_image_width = pentagon_width * 4.;
    let dodecahedron_tiles_image_height = pentagon_height * 3.;
    let arc_start_x = pentagon_width / 4.;
    let arc_start_y = pentagon_edge_length * cos54 / 2.;
    let arc_radius = pentagon_edge_length / 2.;
    let arc_offset_x = pentagon_edge_length * cos72 / 2. - arc_start_x;
    let arc_offset_y = pentagon_height / 2.;
    let wide_arc_width = 40;
    let narrow_arc_width = 20;
    let pentagon_double_width = pentagon_width * 2.;
    let pentagon_triple_width = pentagon_width * 3.;
    let pentagon_double_height = pentagon_height * 2.;

    fs::write("dodecahedron-tiles.svg", format!(r##"<svg width="{dodecahedron_tiles_image_width}" height="{dodecahedron_tiles_image_height}" viewBox="0 0 {dodecahedron_tiles_image_width} {dodecahedron_tiles_image_height}" xmlns="http://www.w3.org/2000/svg" version="2">
    <defs>
        <path id="pentagon-edge" d="M {pentagon_edge_start_x},{pentagon_edge_start_y} h {pentagon_edge_length}" fill="none" stroke="grey" stroke-width="{pentagon_stroke_width}" />
        <g id="pentagon-tile">
            <use href="#pentagon-edge" />
            <use href="#pentagon-edge" transform="rotate(72, {pentagon_centre_x}, {pentagon_centre_y})" />
            <use href="#pentagon-edge" transform="rotate(144, {pentagon_centre_x}, {pentagon_centre_y})" />
            <use href="#pentagon-edge" transform="rotate(216, {pentagon_centre_x}, {pentagon_centre_y})" />
            <use href="#pentagon-edge" transform="rotate(288, {pentagon_centre_x}, {pentagon_centre_y})" />
        </g>
        <path id="arc-component" d="M {arc_start_x},{arc_start_y} a {arc_radius} {arc_radius} 108 0 1 {arc_offset_x},{arc_offset_y}" fill="none" />
        <g id="arc">
            <use href="#arc-component" stroke="#24f" stroke-width="{wide_arc_width}" />
            <use href="#arc-component" stroke="#6de" stroke-width="{narrow_arc_width}" />
        </g>
        <g id="pentagon-tile-with-arcs">
            <use href="#pentagon-tile" />
            <use href="#arc" />
            <use href="#arc" transform="rotate(144, {pentagon_centre_x}, {pentagon_centre_y})" />
        </g>
        <g id="tile-row">
            <use href="#pentagon-tile-with-arcs" x="0" y="0" />
            <use href="#pentagon-tile-with-arcs" x="{pentagon_width}" y="0" />
            <use href="#pentagon-tile-with-arcs" x="{pentagon_double_width}" y="0" />
            <use href="#pentagon-tile-with-arcs" x="{pentagon_triple_width}" y="0" />
        </g>
    </defs>
    <use href="#tile-row" x="0" y="0" />
    <use href="#tile-row" x="0" y="{pentagon_height}" />
    <use href="#tile-row" x="0" y="{pentagon_double_height}" />
</svg>
"##))?;

    let small_arc_start_x = pentagon_width / 4.;
    let small_arc_start_y = pentagon_edge_length * cos54 / 2.;
    let small_arc_radius = pentagon_edge_length / 2.;
    let small_arc_offset_x = pentagon_width / 2.;
    let small_arc_offset_y = 0;
    let large_arc_start_x = pentagon_edge_length * cos72 / 2.;
    let large_arc_start_y = pentagon_edge_length * (cos54 + sin72 / 2.);
    let pentagon_diagonal = (pentagon_height * pentagon_height + pentagon_edge_length * pentagon_edge_length / 4.).sqrt();
    let large_arc_radius = pentagon_edge_length / 2. + pentagon_diagonal;
    let large_arc_offset_x = pentagon_edge_length * (1. + cos72);
    let large_arc_offset_y = 0;

    fs::write("dodecahedron-mixed-arc-tiles.svg", format!(r##"<svg width="{dodecahedron_tiles_image_width}" height="{dodecahedron_tiles_image_height}" viewBox="0 0 {dodecahedron_tiles_image_width} {dodecahedron_tiles_image_height}" xmlns="http://www.w3.org/2000/svg" version="2">
    <defs>
        <path id="pentagon-edge" d="M {pentagon_edge_start_x},{pentagon_edge_start_y} h {pentagon_edge_length}" fill="none" stroke="grey" stroke-width="{pentagon_stroke_width}" />
        <g id="pentagon-tile">
            <use href="#pentagon-edge" />
            <use href="#pentagon-edge" transform="rotate(72, {pentagon_centre_x}, {pentagon_centre_y})" />
            <use href="#pentagon-edge" transform="rotate(144, {pentagon_centre_x}, {pentagon_centre_y})" />
            <use href="#pentagon-edge" transform="rotate(216, {pentagon_centre_x}, {pentagon_centre_y})" />
            <use href="#pentagon-edge" transform="rotate(288, {pentagon_centre_x}, {pentagon_centre_y})" />
        </g>
        <path id="small-arc-component" d="M {small_arc_start_x},{small_arc_start_y} a {small_arc_radius} {small_arc_radius} 108 0 0 {small_arc_offset_x},{small_arc_offset_y}" fill="none" />
        <g id="small-arc">
            <use href="#small-arc-component" stroke="#921" stroke-width="{wide_arc_width}" />
            <use href="#small-arc-component" stroke="#db0" stroke-width="{narrow_arc_width}" />
        </g>
        <path id="large-arc-component" d="M {large_arc_start_x},{large_arc_start_y} a {large_arc_radius} {large_arc_radius} 108 0 1 {large_arc_offset_x},{large_arc_offset_y}" fill="none" />
        <g id="large-arc">
            <use href="#large-arc-component" stroke="#921" stroke-width="{wide_arc_width}" />
            <use href="#large-arc-component" stroke="#db0" stroke-width="{narrow_arc_width}" />
        </g>
        <g id="pentagon-tile-with-arcs">
            <use href="#pentagon-tile" />
            <use href="#small-arc" />
            <use href="#large-arc" />
        </g>
        <g id="tile-row">
            <use href="#pentagon-tile-with-arcs" x="0" y="0" />
            <use href="#pentagon-tile-with-arcs" x="{pentagon_width}" y="0" />
            <use href="#pentagon-tile-with-arcs" x="{pentagon_double_width}" y="0" />
            <use href="#pentagon-tile-with-arcs" x="{pentagon_triple_width}" y="0" />
        </g>
    </defs>
    <use href="#tile-row" x="0" y="0" />
    <use href="#tile-row" x="0" y="{pentagon_height}" />
    <use href="#tile-row" x="0" y="{pentagon_double_height}" />
</svg>
"##))?;

    Ok(())
}
