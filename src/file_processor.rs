use std::cmp;
use std::fs::File;
use std::io::{prelude::*, BufWriter};
use std::ops::Add;
use std::time::Instant;
mod esri_header;
mod vertex;
mod triangle;




pub fn process_file(input_filename: &str, output_filename: &str,elevation_factor: i32) {
    let mut input_file = File::open(input_filename).expect("Unable to find the input file");
    let mut input_string = String::new();
    input_file.read_to_string(&mut input_string).expect("Error while reading file");
    
    let output_string:String =  process_string(input_string, elevation_factor); 

    let output_file = File::create(output_filename).expect("Unable to create to the output file");
    let mut buf_file_output = BufWriter::new(output_file);
    buf_file_output.write(output_string.as_bytes()).expect("Unable to write to the output file");
    
}


pub fn process_string(input_string: String, elevation_factor: i32) -> String {
    let lines = input_string.lines().collect::<Vec<&str>>();
    let output_string_vec:Vec<String> = process_string_vec(lines, elevation_factor);
    let output_string = output_string_vec.join("");
    
    output_string
}


fn process_string_vec(lines: Vec<&str>, elevation_factor: i32) -> Vec<String>{
    let now = Instant::now();
    println!("Total Lines: {} ",lines.len());
    let result =read_header(lines);

    let header:esri_header::EsriHeader =result.0;
    let lines_ = result.1;

    
    //println!("{}",header);
    println!("Header took {} millis.", now.elapsed().as_millis());
    let mut output_vec: Vec<String> = Vec::new(); 

    
    
    output_vec = write_vertices(&header,lines_,output_vec,elevation_factor);
    output_vec = write_normals(output_vec);
    output_vec = write_triangles(&header,output_vec);

    output_vec

}


pub fn read_header(input_string_vec: Vec<&str>) -> (esri_header::EsriHeader, Vec<&str>) {
    let expected_num_header_lines:u8 = 6;
    // let mut line : String = String::from("");
    let mut header:esri_header::EsriHeader = esri_header::EsriHeader::default();

    let mut iter = input_string_vec.iter();

    for _i in 1..expected_num_header_lines+1 {
        let line = iter.next().unwrap().to_uppercase();
        //  println!("line{} ",line);
        //  BufReader::new(file).read_line(&mut line).expect("Error leyendo la cabecera");
        if line.starts_with("NCOLS") {
            let str_value = line.strip_prefix("NCOLS ").unwrap().trim();
            header.ncols = str_value.parse::<i32>().expect("Header field: : NCOLS, could not be parsed");
        } else if line.starts_with("NROWS") {
            let str_value = line.strip_prefix("NROWS ").unwrap().trim();
            header.nrows = str_value.parse::<i32>().expect("Header field: : NROWS, could not be parsed");
        } else if line.starts_with("XLLCENTER") {
            let str_value = line.strip_prefix("XLLCENTER ").unwrap().trim();
            header.xllcenter = str_value.parse::<i32>().expect("Header field: : XLLCENTER, could not be parsed");
        } else if line.starts_with("YLLCENTER") {
            let str_value = line.strip_prefix("YLLCENTER ").unwrap().trim();
            header.yllcenter = str_value.parse::<i32>().expect("Header field: : YLLCENTER, could not be parsed");
        } else if line.starts_with("CELLSIZE") {
            let str_value = line.strip_prefix("CELLSIZE ").unwrap().trim();
            header.cellsize = str_value.parse::<i32>().expect("Header field: : CELLSIZE, could not be parsed");
        } else if line.starts_with("NODATA_VALUE") {
            let str_value = line.strip_prefix("NODATA_VALUE ").unwrap().trim();
            header.nodatavalue = str_value.parse::<f32>().expect("Header field: : NODATA_VALUE, could not be parsed");
        }
    }
    (header, input_string_vec)


}


fn write_vertices(header: &esri_header::EsriHeader,lines: Vec<&str>, mut output_vec: Vec<String>,elevation_factor: i32) -> Vec<String>{
    let now = Instant::now();
    let mut line_num = 0;
    let ok_cellsize = cmp::max(( header.cellsize*1000) /elevation_factor, 1); //Penyagolosa 2 ok   
    // println!("elevation_factor: {} ",elevation_factor);
    // println!("header.cellsize: {} ",header.cellsize);
    // println!("ok_cellsize: {} ",ok_cellsize);
    let mut max_elevation = f32::MIN;
    let mut min_elevation = f32::MAX;
    // let mut err_msg = String::from("valor de elevation incorrecto: ");
    
    // First we find the lowest and highest elevation values so we put the lowest in the nodataValues in the second pass
    for line in lines.iter().skip(6) {
        for elevation_str in line.split(" ") {
            if elevation_str.is_empty() {
                continue;
            }
            //println!("Parse: f32 {}", elevation_str);
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

    for line in lines.iter().skip(6) {
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
            output_vec.push(current_vertex.to_str());
            //buf_file_output.write(current_vertex.to_str().as_bytes()).expect("Error al escribir algun vertice");
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
        //buf_file_output.write(current_vertex.to_str().as_bytes()).expect("Error al escribir algun vertice top");
        output_vec.push(current_vertex.to_str());
        // print!("vertexT {} ",current_vertex.to_str());
    }
    
    //RIGHT vertex
    let right_x = (header.ncols -1) * ok_cellsize;
    for y in 0..header.nrows {
        let current_vertex:vertex::Vertex = vertex::Vertex{x: right_x, y: y * ok_cellsize, z: base_z};
        //buf_file_output.write(current_vertex.to_str().as_bytes()).expect("Error al escribir algun vertice right");
        output_vec.push(current_vertex.to_str());
        // print!("vertexR {} ",current_vertex.to_str());
    }
    
    //BOTTOM VERTEX
    let bottom_y = (header.nrows -1) * ok_cellsize;
    for x in 0..header.ncols{
        let current_vertex:vertex::Vertex = vertex::Vertex{x: x * ok_cellsize, y: bottom_y , z: base_z};
        //buf_file_output.write(current_vertex.to_str().as_bytes()).expect("Error al escribir algun vertice bottom");
        output_vec.push(current_vertex.to_str());
        // print!("vertexB {} ",current_vertex.to_str());
    }

    //LEFT vertex
    for y in 0..header.nrows{
        let current_vertex:vertex::Vertex = vertex::Vertex{x: 0, y: y * ok_cellsize, z: base_z};
        //buf_file_output.write(current_vertex.to_str().as_bytes()).expect("Error al escribir algun vertice left");
        output_vec.push(current_vertex.to_str());
        // print!("vertexL {} ",current_vertex.to_str());
    }
        
    
    println!("Write_vertices took {} millis.", now.elapsed().as_millis());
    output_vec
}

fn write_normals(mut output_vec:Vec<String>) ->Vec<String>{
    //output_file.write("\r\nvn  0.0  0.0  1.0 \r\n\r\n".as_bytes()).expect("Error al escribir las normales");
    output_vec.push("\r\nvn  0.0  0.0  1.0 \r\n\r\n".to_string());
    output_vec
}

fn write_triangles(header: &esri_header::EsriHeader,mut output_vec: Vec<String>) ->Vec<String> {
    //let mut buf_file_output = BufWriter::new(output_file);
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
            //buf_file_output.write(square.as_bytes()).expect("Error al escribir triangulos");
            output_vec.push(square.to_string());
        }

    }

    //Perimeter WALL

    let offset = header.ncols * header.nrows;
    
    let offset_last_row = offset - header.ncols;
    let offset_bottom  = offset + header.ncols + header.nrows;
    for i_col in 1..header.ncols {
        //TOP WALL
        let triangle1: triangle::Triangle = triangle::Triangle { v1: i_col, v2: i_col + offset, v3: i_col + 1 };
        //buf_file_output.write(triangle1.to_str().as_bytes()).expect("Error al escribir triangulos top perimeter");
        output_vec.push(triangle1.to_str().to_string());
        let triangle2: triangle::Triangle = triangle::Triangle { v1: i_col + 1 , v2: i_col +offset, v3: i_col + offset +1};
        // buf_file_output.write(triangle2.to_str().as_bytes()).expect("Error al escribir triangulos top perimeter");
        output_vec.push(triangle2.to_str().to_string());
    
        //BOTTOM WALL
        let triangle1: triangle::Triangle = triangle::Triangle { v1: i_col + offset_last_row + 1 , v2: i_col + offset_bottom, v3:i_col + offset_last_row };
        // buf_file_output.write(triangle1.to_str().as_bytes()).expect("Error al escribir triangulos top perimeter");
        output_vec.push(triangle1.to_str().to_string());
        let triangle2: triangle::Triangle = triangle::Triangle { v1:  i_col + offset_bottom +1 , v2: i_col + offset_bottom, v3:i_col + offset_last_row + 1  };
        // buf_file_output.write(triangle2.to_str().as_bytes()).expect("Error al escribir triangulos top perimeter");
        output_vec.push(triangle2.to_str().to_string());

    }

    
    let start_right = offset + header.ncols;
    let start_left = offset + header.ncols * 2 + header.nrows;
    for j_row in 1..header.nrows {
        //RIGHT WALL
        let triangle1: triangle::Triangle = triangle::Triangle { v1: header.ncols* (j_row +1), v2: header.ncols*j_row, v3: start_right + j_row};
        // buf_file_output.write(triangle1.to_str().as_bytes()).expect("Error al escribir triangulos top perimeter");
        output_vec.push(triangle1.to_str().to_string());
        let triangle2: triangle::Triangle = triangle::Triangle { v1: header.ncols*(j_row+1) , v2: start_right + j_row , v3: start_right + j_row + 1};
        // buf_file_output.write(triangle2.to_str().as_bytes()).expect("Error al escribir triangulos top perimeter");
        output_vec.push(triangle2.to_str().to_string());

        //LEFT WALL
        let triangle1: triangle::Triangle = triangle::Triangle { v1:start_left + j_row, v2: header.ncols*(j_row-1) +1, v3:  header.ncols * j_row + 1};
        // buf_file_output.write(triangle1.to_str().as_bytes()).expect("Error al escribir triangulos top perimeter");
        output_vec.push(triangle1.to_str().to_string());
        let triangle2: triangle::Triangle = triangle::Triangle { v1: start_left + j_row + 1 , v2: start_left + j_row , v3:header.ncols*j_row +1 };
        // buf_file_output.write(triangle2.to_str().as_bytes()).expect("Error al escribir triangulos top perimeter");
        output_vec.push(triangle2.to_str().to_string());
        
    }

    //BOTTOM LID

    let triangle1: triangle::Triangle = triangle::Triangle { v1:offset + header.ncols +header.nrows , v2: offset + header.ncols, v3: offset + 1 };
    // buf_file_output.write(triangle1.to_str().as_bytes()).expect("Error al escribir triangulos top perimeter");
    output_vec.push(triangle1.to_str().to_string());
    let triangle2: triangle::Triangle = triangle::Triangle { v1: offset + header.ncols +header.nrows, v2:  offset + 1, v3: offset + header.ncols*2 +header.nrows*2 };
    // buf_file_output.write(triangle2.to_str().as_bytes()).expect("Error al escribir triangulos top perimeter");
    output_vec.push(triangle2.to_str().to_string());

    println!("Write triangles took {} millis.", now.elapsed().as_millis());

    output_vec
}

