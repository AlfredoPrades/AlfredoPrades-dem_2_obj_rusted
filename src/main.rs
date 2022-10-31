mod file_processor;

fn main() {
    //println!("\nBienvenido a DEM to OBJ, este programa convierte mapas de elevacion en formato DEM (ESRI .asc) a .OBJ (solido 3d).\n");
    println!("\nWelcome to DEM to OBJ, this program converts DEM (ESRI .asc) elevation maps to .OBJ (solido 3d) which .\n");
    println!("");
    println!("Example command: cargo run .\\test\\Penyagolosa2m.asc .\\test\\Penyagolosa2m.obj 1000");
    println!("Example command: dem_2_obj.exe  .\\test\\Penyagolosa2m.asc .\\test\\Penyagolosa2m.obj");

    let wrong_parameters_msg = "2 parameters are required:  path to the dem input file, and path to the obj outputfile \n
                                      1 optional parameter: elevation modifier [default (no correction) is 1000,] lower values means more \"spiky\" ";
    let input_dem_filename:String = std::env::args().nth(1).expect(wrong_parameters_msg);
    let output_stl_filename:String = std::env::args().nth(2).expect(wrong_parameters_msg);
    let mut optional_elevation_factor:String = String::new();
    if std::env::args().len() > 3 {
        optional_elevation_factor = std::env::args().nth(3).unwrap();  
    }

    let mut elevation_factor = 1000;
    if !optional_elevation_factor.is_empty(){
        elevation_factor =  optional_elevation_factor.parse::<i32>().expect("Third parameters must be numeric. (elevation modifier [no Correction=1000])");
    }

    println!("Input File:\x1b[1m {} \x1b[0m\noutput file: \x1b[96m {} \x1b[0m",input_dem_filename,output_stl_filename);

    file_processor::process_file(input_dem_filename.as_str(), output_stl_filename.as_str(),elevation_factor)
}

