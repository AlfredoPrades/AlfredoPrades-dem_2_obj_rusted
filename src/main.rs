mod file_processor;

fn main() {
    //let elevation_filename = "MDT02-ETRS89-HU30-0592-4-COB2.asc_Cropped.dem";
    //file_read_test::file_read(elevation_filename);
    println!("\nBienvenido a DEM to OBJ, este programa convierte mapas de elevacion en formato DEM (ESRI .asc) a .OBJ (solido 3d).\n");

    let wrong_parameters_msg = "Se requieren 3 parametros ruta fichero dem entrada, y ruta fichero stl salida y factor de correccion de altura por defecto 1000";
    let input_dem_filename:String = std::env::args().nth(1).expect(wrong_parameters_msg);
    let output_stl_filename:String = std::env::args().nth(2).expect(wrong_parameters_msg);
    let optional_elevation_factor:String = std::env::args().nth(3).unwrap();

    let mut elevation_factor = 1000;
    if !optional_elevation_factor.is_empty(){
        elevation_factor =  optional_elevation_factor.parse::<i32>().expect("El tercer parametro, (elevation factor [default=1000]) debe ser numerico.");
    }

    println!("Se va a procesar:\x1b[1m {} \x1b[0m\nla salida se copiara a: \x1b[96m {} \x1b[0m",input_dem_filename,output_stl_filename);


    
    file_processor::process_file(input_dem_filename.as_str(), output_stl_filename.as_str(),elevation_factor)
}
