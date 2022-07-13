use std::fs::File;
use std::io::{prelude::*, BufReader, BufWriter};
use std::ops::Add;
use std::time::Instant;
mod esri_header;
mod vertex;
mod triangle;

pub fn process_file(input_filename: &str, output_filename: &str,elevation_factor: i32) {
    let input_file = File::open(input_filename).expect("No se ha podido acceder al fichero de entrada");
    let now = Instant::now();
    let header:esri_header::EsriHeader =read_header(&input_file);
    
    println!("{}",header);
    let output_file = File::create(output_filename).expect("no se ha podido escribir la salida");
    // output_file.write(header.to_str().as_bytes()).expect("No se ha podido escribir en la salida");
    
    println!("header took {} millis.", now.elapsed().as_millis());
    
    let input_file = File::open(input_filename).expect("No se ha podido acceder al fichero de entrada");
    let lines = BufReader::new(input_file).lines().skip(6)
    .map(|x| x.unwrap())
    .collect::<Vec<String>>();
    
    println!("Total Lines: {} ",lines.len());
    
    write_vertices(&header,lines,&output_file,elevation_factor);
    write_normals(&output_file);
    write_triangles(&header,&output_file);


}

pub fn read_header (file: &File) -> esri_header::EsriHeader {
    let expected_num_header_lines:u8 = 6;
    // let mut line : String = String::from("");
    let mut header:esri_header::EsriHeader = esri_header::EsriHeader::default();


    let mut iter = BufReader::new(file).lines();

    for _i in 1..expected_num_header_lines+1 {
        let line = iter.next().unwrap().unwrap().to_uppercase();
        println!("line{} ",line);
        //  BufReader::new(file).read_line(&mut line).expect("Error leyendo la cabecera");
        if line.starts_with("NCOLS") {
            let str_value = line.strip_prefix("NCOLS ").unwrap().trim();
            header.ncols = str_value.parse::<i32>().expect("El valor de la cabecera: NCOLS, no se ha podido parsear");
        } else if line.starts_with("NROWS") {
            let str_value = line.strip_prefix("NROWS ").unwrap().trim();
            header.nrows = str_value.parse::<i32>().expect("El valor de la cabecera: NROWS, no se ha podido parsear");
        } else if line.starts_with("XLLCENTER") {
            let str_value = line.strip_prefix("XLLCENTER ").unwrap().trim();
            header.xllcenter = str_value.parse::<i32>().expect("El valor de la cabecera: XLLCENTER, no se ha podido parsear");
        } else if line.starts_with("YLLCENTER") {
            let str_value = line.strip_prefix("YLLCENTER ").unwrap().trim();
            header.yllcenter = str_value.parse::<i32>().expect("El valor de la cabecera: YLLCENTER, no se ha podido parsear");
        } else if line.starts_with("CELLSIZE") {
            let str_value = line.strip_prefix("CELLSIZE ").unwrap().trim();
            header.cellsize = str_value.parse::<i32>().expect("El valor de la cabecera: CELLSIZE, no se ha podido parsear");
        } else if line.starts_with("NODATA_VALUE") {
            let str_value = line.strip_prefix("NODATA_VALUE ").unwrap().trim();
            header.nodatavalue = str_value.parse::<f32>().expect("El valor de la cabecera: NODATA_VALUE, no se ha podido parsear");
        }
    }
    header


}


fn write_vertices(header: &esri_header::EsriHeader,lines: Vec<String>, output_file: &File,elevation_factor: i32) {
    let mut buf_file_output = BufWriter::new(output_file);
    let now = Instant::now();
    let mut line_num = 0;
    let ok_cellsize = elevation_factor * header.cellsize;
    let mut max_elevation = f32::MIN;
    let mut min_elevation = f32::MAX;
    // let mut err_msg = String::from("valor de elevation incorrecto: ");
    
    // First we find the lowest and highest elevation values so we put the lowest in the nodataValues in the second pass
    for line in lines.iter() {
        for elevation_str in line.split(" ") {
            if elevation_str.is_empty() {
                continue;
            }
            let elevation_value = elevation_str.parse::<f32>().expect(elevation_str);

            if elevation_value == header.nodatavalue {
                continue;
            }
            if elevation_value > max_elevation {
                max_elevation = elevation_value;
            } else if elevation_value < min_elevation {
                min_elevation = elevation_value;
            }
        }
    }

    for line in lines.iter() {
        let mut col_num = 0;
        for elevation_str in line.split(" ") {
            if elevation_str.is_empty() {
                continue;
            }

            let elevation_value = if elevation_str.parse::<f32>().expect(elevation_str) == (header.nodatavalue) {
                min_elevation
            } else {
                elevation_str.parse::<f32>().expect(elevation_str)
            };
            
            let current_vertex:vertex::Vertex = vertex::Vertex{x: col_num * ok_cellsize, y: line_num * ok_cellsize, z: elevation_value};
            buf_file_output.write(current_vertex.to_str().as_bytes()).expect("Error al escribir algun vertice");
            col_num +=1;
        }
        line_num+=1;
    }

    let elevation_range = (max_elevation - min_elevation)  / 5.0 ;
    let base_z = if min_elevation > elevation_range {  min_elevation - elevation_range  } else { 0.0 };
   
    // Write base vertices

    //TOP vertex
    for x in 0..header.ncols {
        let current_vertex:vertex::Vertex = vertex::Vertex{x: x * ok_cellsize, y: 0, z: base_z};
        buf_file_output.write(current_vertex.to_str().as_bytes()).expect("Error al escribir algun vertice top");
        // print!("vertexT {} ",current_vertex.to_str());
    }
    
    //RIGHT vertex
    let right_x = (header.ncols -1) * ok_cellsize;
    for y in 0..header.nrows {
        let current_vertex:vertex::Vertex = vertex::Vertex{x: right_x, y: y * ok_cellsize, z: base_z};
        buf_file_output.write(current_vertex.to_str().as_bytes()).expect("Error al escribir algun vertice right");
        // print!("vertexR {} ",current_vertex.to_str());
    }
    
    //BOTTOM VERTEX
    let bottom_y = (header.nrows -1) * ok_cellsize;
    for x in 0..header.ncols{
        let current_vertex:vertex::Vertex = vertex::Vertex{x: x * ok_cellsize, y: bottom_y , z: base_z};
        buf_file_output.write(current_vertex.to_str().as_bytes()).expect("Error al escribir algun vertice bottom");
        // print!("vertexB {} ",current_vertex.to_str());
    }

    //LEFT vertex
    for y in 0..header.nrows{
        let current_vertex:vertex::Vertex = vertex::Vertex{x: 0, y: y * ok_cellsize, z: base_z};
        buf_file_output.write(current_vertex.to_str().as_bytes()).expect("Error al escribir algun vertice left");
        // print!("vertexL {} ",current_vertex.to_str());
    }
        
    
    println!("write_vertices took {} millis.", now.elapsed().as_millis());
}

fn write_normals(mut output_file:&File) {
    output_file.write("\r\nvn  0.0  0.0  1.0 \r\n\r\n".as_bytes()).expect("Error al escribir las normales");
}

fn write_triangles(header: &esri_header::EsriHeader,output_file: &File) {
    let mut buf_file_output = BufWriter::new(output_file);
    let now = Instant::now();
    for y in 1..(header.nrows ) {
        let offset = (y - 1) * header.ncols;
        // println!("offset: {} y:{} ",offset, y );
        for x in 1..(header.ncols ) {
            // print!(",{} ",x );
            let current_id = offset + x;

            
            /*              (x)   (x+1)            (x)
             *   Triangle1:  V1 -- V2   Triangle2:  V1                      (y)
             *                \    |                 | \
             *                 \   |                 |  \
             *                   V3                 V3-- V2                 (y+1)
            */

            let triangle1: triangle::Triangle = triangle::Triangle { v1: current_id, v2: current_id + 1, v3: current_id + 1 + header.ncols };
            let triangle2: triangle::Triangle = triangle::Triangle { v1: current_id, v2: current_id + 1 + header.ncols, v3: current_id + header.ncols };
            let mut square: String = String::from(triangle1.to_str());
            square = square.add(triangle2.to_str().as_str());
            buf_file_output.write(square.as_bytes()).expect("Error al escribir triangulos");
        }

    }

    //Perimeter WALL

    let offset = header.ncols * header.nrows;
    
    let offset_last_row = offset - header.ncols;
    let offset_bottom  = offset + header.ncols + header.nrows;
    for i_col in 1..header.ncols {
        //TOP WALL
        let triangle1: triangle::Triangle = triangle::Triangle { v1: i_col, v2: i_col + offset, v3: i_col + 1 };
        buf_file_output.write(triangle1.to_str().as_bytes()).expect("Error al escribir triangulos top perimeter");
        let triangle2: triangle::Triangle = triangle::Triangle { v1: i_col + 1 , v2: i_col +offset, v3: i_col + offset +1};
        buf_file_output.write(triangle2.to_str().as_bytes()).expect("Error al escribir triangulos top perimeter");
    
        //BOTTOM WALL
        let triangle1: triangle::Triangle = triangle::Triangle { v1: i_col + offset_last_row + 1 , v2: i_col + offset_bottom, v3:i_col + offset_last_row };
        buf_file_output.write(triangle1.to_str().as_bytes()).expect("Error al escribir triangulos top perimeter");
        let triangle2: triangle::Triangle = triangle::Triangle { v1:  i_col + offset_bottom +1 , v2: i_col + offset_bottom, v3:i_col + offset_last_row + 1  };
        buf_file_output.write(triangle2.to_str().as_bytes()).expect("Error al escribir triangulos top perimeter");

    }

    
    let start_right = offset + header.ncols;
    let start_left = offset + header.ncols * 2 + header.nrows;
    for j_row in 1..header.nrows {
        //RIGHT WALL
        let triangle1: triangle::Triangle = triangle::Triangle { v1: header.ncols* (j_row +1), v2: header.ncols*j_row, v3: start_right + j_row};
        buf_file_output.write(triangle1.to_str().as_bytes()).expect("Error al escribir triangulos top perimeter");
        let triangle2: triangle::Triangle = triangle::Triangle { v1: header.ncols*(j_row+1) , v2: start_right + j_row , v3: start_right + j_row + 1};
        buf_file_output.write(triangle2.to_str().as_bytes()).expect("Error al escribir triangulos top perimeter");

        //LEFT WALL
        let triangle1: triangle::Triangle = triangle::Triangle { v1:start_left + j_row, v2: header.ncols*(j_row-1) +1, v3:  header.ncols * j_row + 1};
        buf_file_output.write(triangle1.to_str().as_bytes()).expect("Error al escribir triangulos top perimeter");
        let triangle2: triangle::Triangle = triangle::Triangle { v1: start_left + j_row + 1 , v2: start_left + j_row , v3:header.ncols*(j_row) +1 };
        buf_file_output.write(triangle2.to_str().as_bytes()).expect("Error al escribir triangulos top perimeter");
        
    }

    //BOTTOM LID

    let triangle1: triangle::Triangle = triangle::Triangle { v1:offset + header.ncols +header.nrows , v2: offset + header.ncols, v3: offset + 1 };
    buf_file_output.write(triangle1.to_str().as_bytes()).expect("Error al escribir triangulos top perimeter");
    let triangle2: triangle::Triangle = triangle::Triangle { v1: offset + header.ncols +header.nrows, v2:  offset + 1, v3: offset + header.ncols*2 +header.nrows*2 };
    buf_file_output.write(triangle2.to_str().as_bytes()).expect("Error al escribir triangulos top perimeter");






    //FALTAN LAS PAREDES Y LA BASE


    println!("write_triangles took {} millis.", now.elapsed().as_millis());
}

