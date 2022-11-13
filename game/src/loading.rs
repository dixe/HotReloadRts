use std::fs;
use std::path::PathBuf;
use std::io::{self, BufRead};
use gl_lib::{gl, objects::{ gltf_mesh, mesh}, animations::{self, skeleton, types::KeyFrame}};
use std::collections::HashMap;
use crate::render;



pub type ModelsAssets = HashMap::<String, Model>;

pub struct GameAssets {
    pub units: HashMap::<String, UnitAsset>,
    pub models: ModelsAssets
}

#[derive(Debug)]
pub struct UnitAsset {
    pub model_name: String

}

pub struct Model {
    pub mesh: gltf_mesh::GltfMesh, // not mesh::Mesh, since that requried gl, and creates data on the gpu
    pub animations: HashMap::<String, Vec::<KeyFrame>>
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

    let mut res : ModelsAssets = Default::default();

    let paths = fs::read_dir(path).unwrap();

    for entry in paths {
        if let Ok(entry) = entry {
            let file_path: String = entry.path().into_os_string().into_string().unwrap();
            if file_path.ends_with(".glb") {

                let skins = skeleton::load_skins(&file_path).unwrap();
                let mesh_animations = animations::load_animations(&file_path, &skins);


                match gltf_mesh::meshes_from_gltf(&file_path, &skins) {
                    Ok(meshes_gltf) => {

                        for (name, mesh) in &meshes_gltf.meshes {
                            let animations = match mesh_animations.get(name) {
                                Some(anis) => anis.clone(),
                                None => Default::default()
                            };

                            let model = Model {
                                mesh: mesh.clone(),
                                animations
                            };

                            res.insert(name.clone(), model);
                        }
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


fn load_all_units(path: PathBuf) -> Result<HashMap::<String, UnitAsset>, String> {
    let paths = fs::read_dir(path).unwrap();

    let mut res = HashMap::default();

    for file_path in paths {
        let fp = file_path.unwrap();
        let file_name = fp.path().file_stem().unwrap().to_os_string().into_string().unwrap();
        res.insert(file_name, load_unit_file(fp.path()));
    }

    Ok(res)

}

fn load_unit_file(path: PathBuf) -> UnitAsset {

    let file = fs::File::open(&path).unwrap();

    let lines = io::BufReader::new(file).lines();


    let mut res = UnitAsset {
        model_name: "".to_string()
    };

    for line_o in lines {
        if let Ok(line) = line_o {
            if line.starts_with("Model:") {
                res.model_name = line.split(":").last().expect("model: should be followed by model name").trim().to_string();

            }
        }
    }

    // Check that model_name is set??
    if res.model_name == "" {
        panic!("Model name empty for {:#?}", path);
    }

    res

}


pub fn populate_render_data(gl: &gl::Gl, rd: &mut render::RenderData, models: &ModelsAssets) {

    // Setup render data first
    for (name, model) in models {
        let mesh = model.mesh.get_mesh(gl);
        // we also have animations on model
        let animations = &model.animations;

        rd.set_mesh(name, mesh);
    }

}
