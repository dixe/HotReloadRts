use std::fs;
use std::path::PathBuf;
use std::io::{self, BufRead};
use gl_lib::{gl, na, objects::{plane, mesh, shadow_map, texture_quad, square, gltf_mesh}, shader::{self, Shader}};
use std::collections::HashMap;
use crate::render;


pub struct GameAssets {
    pub units : Vec::<UnitAsset>,
    pub models: ModelsAssets
}

pub struct UnitAsset {
    pub model_name: String
}

pub struct ModelsAssets {
    pub names: HashMap::<String, usize>,
    pub meshes: Vec::<gltf_mesh::GltfMeshes>
}



pub fn load_all_assets(base_path: PathBuf) -> Result<GameAssets, String> {

    // load units from units folder, each file describes a unit, filename is unit name

    let mut units_path = base_path.clone();
    units_path.push("units");

    let models = load_all_glb(base_path);

    let units = load_all_units(units_path)?;
    Ok(GameAssets {
        units: units,
        models: models
    })
}


fn load_all_glb(path: PathBuf) -> ModelsAssets {

    let mut res = ModelsAssets {
        names: Default::default(),
        meshes: vec![]
    };

    let paths = fs::read_dir(path).unwrap();

    for entry in paths {
        if let Ok(entry) = entry {
            let file_path: String = entry.path().into_os_string().into_string().unwrap();
            if file_path.ends_with(".glb") {
                let hashmap = std::collections::HashMap::new();
                match gltf_mesh::meshes_from_gltf(&file_path, &hashmap) {
                    Ok(meshes_gltf) => {

                        let index = res.meshes.len();

                        for (name, _) in &meshes_gltf.meshes {
                            res.names.insert(name.clone(), index);
                        }
                        res.meshes.push(meshes_gltf);
                    }
                    Err(err) => {
                        println!("{:?}", err);
                    }
                }

            }
        }
    }

    res
}


fn load_all_units(path: PathBuf) -> Result<Vec::<UnitAsset>, String> {
    let paths = fs::read_dir(path).unwrap();

    let mut res = vec![];

    for file_path in paths {

        res.push(load_unit_file(file_path.unwrap().path()));
    }

    Ok(res)

}

fn load_unit_file(path: PathBuf) -> UnitAsset {

    let file = fs::File::open(path).unwrap();

    let lines = io::BufReader::new(file).lines();

    let mut res = UnitAsset {
        model_name: "".to_string()
    };

    for line_o in lines {
        if let Ok(line) = line_o {
            if line.starts_with("model:") {
                res.model_name = line.split(":").last().expect("model: should be followed by model name").trim().to_string();

            }
        }
    }

    // Check that model_name is set??
    res

}


pub fn populate_render_data(gl: &gl::Gl, rd: &mut render::RenderData, models: &ModelsAssets) {

    // Setup render data first
    for (name, &index) in &models.names {
        let gltf_mesh = &models.meshes[index];
        let mesh = gltf_mesh.get_mesh(gl, name).unwrap();

        rd.replace_mesh(name, mesh);
    }
}
