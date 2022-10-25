use std::env;
use std::fs::{self, File};
use std::path::Path;
use std::collections::HashSet;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();

    println!("{:?}", out_dir);

    let mut cur_dir = std::env::current_dir().unwrap();
    cur_dir.push("../assets");



    let paths = fs::read_dir(&cur_dir).unwrap();

    let mut names = HashSet::new();

    for path in paths {
        let mut name = path.unwrap().file_name().into_string().unwrap();

        if name.ends_with(".vert") {
            names.insert(name.replace(".vert", ""));
        }
    }


    fs::write(&Path::new(&out_dir).join("shaders_gen.rs"), shaders_file(names));
}


fn shaders_file(names: HashSet::<String>) -> String {

    let mut code =  "
pub struct Shaders {\n".to_string();;

    for name in &names {
        code += &format!("    pub {name}: shader::BaseShader,\n", name = name);
    }

    code += "}\n\n";


    code += "impl Shaders {

    pub fn new(gl: &gl::Gl, base_path: &std::path::PathBuf) -> Self {
        Self {\n";

    for name in &names {
        code += &format!("      {name}: create_shader(gl, base_path, \"{name}\").unwrap(),\n", name = name);
    }

    code += "
    }
  }";

    code += "\n\n
    pub fn reload(&mut self, gl: &gl::Gl, base_path: &std::path::PathBuf) {
";

    for name in &names {
        code += &format!("\n        match create_shader(gl, base_path, \"{name}\") {{
            Ok(shader) => {{
                   self.{name} = shader;
                println!(\"Reloaded {name}\");
            }},
            Err(err) => {{
                println!(\"{{:?}}\", err);
            }},
        }};", name = name);

    }

code += "
   }
}

";

        code

}
