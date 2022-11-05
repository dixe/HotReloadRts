extern crate shared;
use nalgebra::vector;
use gl_lib::{gl, na, objects::{plane, mesh, shadow_map, texture_quad, square, gltf_mesh}, shader::{self, Shader}};
use crate::game::*;
use std::collections::HashMap;

pub type MeshIndex = usize;

include!(concat!(env!("OUT_DIR"), "/shaders_gen.rs"));


pub struct RenderData {
    // MESHES LOADED FROM FILES
    meshes: Vec::<mesh::Mesh>,
    mesh_name_to_index: HashMap::<String, MeshIndex>,

    // STATIC MESHES AND MODELS
    pub plane: mesh::Mesh,
    pub square: square::Square,

    //SHADOW MAP STUFF
    pub tex_quad: texture_quad::TextureQuad,
    pub shadow_map: shadow_map::ShadowMap,

    // SHADERS
    pub shaders: Shaders,
}


impl RenderData {

    pub fn new(gl: &gl::Gl) -> Self {
        let base_path: std::path::PathBuf = "E:/repos/HotReloadRts/assets".to_string().into();

        let hashmap = std::collections::HashMap::new();

        let boids_gltf = gltf_mesh::meshes_from_gltf(&"E:/repos/HotReloadRts/assets/boid.glb", &hashmap).unwrap();

        let boid = boids_gltf.get_mesh(gl, "Boid").unwrap();

        let plane = plane::Plane::new(gl);
        let mut mesh_name_to_index : HashMap::<String, usize> = Default::default();

        mesh_name_to_index.insert("Boid".to_string(), 0);

        Self {
            meshes: vec![boid],
            mesh_name_to_index,
            plane,
            square: square::Square::new(gl),
            shadow_map: shadow_map::ShadowMap::new(&gl),
            tex_quad: texture_quad::TextureQuad::new(&gl),
            shaders: Shaders::new(gl, &base_path),
        }
    }


    pub fn get_mesh_index(&self, name: &str) -> MeshIndex {
        *self.mesh_name_to_index.get(name).unwrap()
    }

    /// Add or replace a mesh, return the mesh_index
    pub fn set_mesh(&mut self, name: &str, mesh: mesh::Mesh) -> MeshIndex {
        match self.mesh_name_to_index.get(name) {
            Some(&index) => {
                self.meshes[index] = mesh;
                index
            },
            None => {
                let index = self.meshes.len();
                self.meshes.push(mesh);
                self.mesh_name_to_index.insert(name.to_string(), index);
                index
            }
        }
    }


    /// Replace a mesh, panic if mesh does not exists
    pub fn replace_mesh(&mut self, name: &str, mesh: mesh::Mesh) {
        let index = *self.mesh_name_to_index.get(name).unwrap();
        self.meshes[index] = mesh;
    }

}


pub fn create_shader(gl: &gl::Gl, root_path: &std::path::PathBuf, name: &str) -> Result<shader::BaseShader, failure::Error> {
    let vert_shader_path =  std::path::Path::new(root_path).join(format!("{}.vert", name));
    let vert_source = std::fs::read_to_string(vert_shader_path.clone())
        .expect(&format!("Could not reader vert shader file at: {:?}", vert_shader_path));


    let frag_shader_path = std::path::Path::new(root_path).join(format!("{}.frag", name));
    let frag_source = std::fs::read_to_string(frag_shader_path.clone())
        .expect(&format!("Could not reader frag shader file at: {:?}", frag_shader_path));

    shader::BaseShader::new(gl, &vert_source, &frag_source)
}


pub struct RenderMesh<'a> {
    pub shader: &'a shader::BaseShader,
    pub model_mat: na::Matrix4::<f32>,
    pub mesh: &'a mesh::Mesh,
    pub color: na::Vector3::<f32>,
}


pub fn render(gl: &gl::Gl, game: &Game) {


    // set shader uniforms
    let light_pos = na::Vector3::new(0.0, 0.0, 30.0);


    let light_space_mat = game.render_data.shadow_map.light_space_mat(light_pos);
    let ms = &game.render_data.shaders.mesh_shader;


    let view =  game.camera.view();
    let projection = game.camera.projection();


    ms.set_used();

    ms.set_vec3(gl, "lightColor", na::Vector3::new(1.0, 1.0, 1.0));
    ms.set_vec3(gl, "lightPos", light_pos);
    ms.set_mat4(gl, "view", view);
    ms.set_mat4(gl, "projection", projection);
    ms.set_mat4(gl, "lightSpaceMat", light_space_mat);


    let ss = &game.render_data.shaders.spell_shader;

    ss.set_mat4(gl, "view", view);
    ss.set_mat4(gl, "projection", projection);



    render_entities(gl, game);

    render_select_box(gl, game);

    render_health_bars(gl, game);

    render_mouse(gl, game);
}


fn render_entities(gl: &gl::Gl, game: &Game) {
    // Setup render meshes
    let mut render_objs = vec![];

    unsafe {
        gl.Clear(gl::DEPTH_BUFFER_BIT);
        gl.BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
        gl.Enable( gl::BLEND );
    }




    for i in 0..game.state.entities.positions.len() {

        let mut model_mat = na::Matrix4::identity();
        model_mat = model_mat.append_nonuniform_scaling(&na::Vector3::new(0.2, 0.2, 0.2));
        let rotation = game.state.entities.z_rotations[i];

        model_mat = rotation.to_homogeneous()* model_mat;

        model_mat = model_mat.append_translation(&game.state.entities.positions[i]);


        let mut color = match game.state.entities.team[i] {
            1 => vector![1.0, 0.0, 0.0],
            2 => vector![0.0, 0.0, 1.0],
            _ => vector![1.0, 1.0, 1.0]
        };


        if game.state.selected.contains(&i) {
            color.z = 1.0;
        }

        let mesh_index = game.state.entities.mesh_index[i];
        render_objs.push(RenderMesh {
            shader: &game.render_data.shaders.mesh_shader,
            model_mat,
            mesh: &game.render_data.meshes[mesh_index],
            color: color
        });
    }

    // grund plane mesh
    let plane_scale = 100.0;
    let mut model_mat = na::Matrix4::identity();
    model_mat = model_mat.prepend_nonuniform_scaling(&vector![plane_scale, plane_scale, 1.0]);
    render_objs.push(RenderMesh {
            shader: &game.render_data.shaders.mesh_shader,
            model_mat,
            mesh: &game.render_data.plane,
            color: vector![150.0/255.0, 74.0/255.0, 39.0/255.0]
    });


    // Select Position
    let mut model_mat = na::Matrix4::identity();
    model_mat = model_mat.prepend_nonuniform_scaling(&vector![0.2, 0.2, 1.0]);
    model_mat = model_mat.append_translation(&game.state.select_pos);
    render_objs.push(RenderMesh {
            shader: &game.render_data.shaders.mesh_shader,
            model_mat,
            mesh: &game.render_data.plane,
            color: vector![0.0, 0.0, 0.0]
    });


    unsafe {
        gl.Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
    }

    // SHADOW MAP AND ALL ENTITIES
    let light_pos = na::Vector3::new(0.0, 0.0, 30.0);

    shadow_map_render(gl, &game.render_data.shadow_map, light_pos, &render_objs);

     // bind the shadow map texture to sampler 1
    unsafe {
        gl.ActiveTexture(gl::TEXTURE0);
        gl.BindTexture(gl::TEXTURE_2D, game.render_data.shadow_map.depth_map);
    }


    // SPELLS
    let mut z_offset = 0.001;
    for i in 0..game.state.active_aoe_spells.len() {
        let pos = game.state.active_aoe_spells[i].pos;
        let r = game.state.active_aoe_spells[i].radius;
        let mut model_mat = na::Matrix4::identity();
        model_mat = model_mat.prepend_nonuniform_scaling(&vector![r * 2.0, r * 2.0, 1.0]);
        model_mat = model_mat.append_translation(&(pos + vector![0.0, 0.0, z_offset]));
        render_objs.push(RenderMesh {
            shader: &game.render_data.shaders.spell_shader,
            model_mat,
            mesh: &game.render_data.plane,
            color: vector![7.0/255.0, 171.0/255.0, 40.0/255.0]
        });
        z_offset += 0.001;
    }

    for ro in render_objs {
        ro.shader.set_used();
        ro.shader.set_vec3(gl, "color", ro.color);
        ro.shader.set_mat4(gl, "model", ro.model_mat);
        ro.mesh.render(gl);
    }

}

fn render_select_box(gl: &gl::Gl, game: &Game) {

    if let Some(sb) = game.state.select_box {
        let start = sb.start;
        let cur = sb.current;

        let mut left = start.x.min(cur.x) as f32;
        let mut right = start.x.max(cur.x) as f32;
        let mut top = start.y.min(cur.y) as f32;
        let mut bottom = start.y.max(cur.y) as f32;

        // calc clip space [-1,1] coors from screen space coords

        left = (-0.5 + (left as f32) / 1200.0) * 2.0;
        right = (-0.5 + (right as f32) / 1200.0) * 2.0;
        top =  (-0.5 + (top as f32) / 700.0) * -2.0;
        bottom =  (-0.5 + (bottom as f32) / 700.0) * -2.0;


        // update out square data
        game.render_data.square.sub_data(gl, left, right, top, bottom);

        // render square
        game.render_data.shaders.select_box_shader.set_used();


        unsafe {
            gl.Clear(gl::DEPTH_BUFFER_BIT);
            gl.BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
            gl.Enable( gl::BLEND );
        }

        game.render_data.square.render(gl);

    }
}

fn render_health_bars(gl: &gl::Gl, game: &Game) {

    // HEALTH BARS
    for entity_id in &game.state.entities.ids {

        let idx = game.state.entities.id_to_index.get(entity_id).unwrap();
        let screen_pos = game.camera.world_pos_to_screen(game.state.entities.positions[*idx]);

        let mut health = 1.0;
        if let Some(dmg) = game.state.entities.damage.get(entity_id) {
            health = dmg.health;
        }


        unsafe {
            gl.Clear(gl::DEPTH_BUFFER_BIT);
            gl.BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
            gl.Enable( gl::BLEND );
        }


        let cs = game.camera.to_clip_space(screen_pos, 30.0, 10.0);

        game.render_data.shaders.hp_shader.set_used();

        game.render_data.shaders.hp_shader.set_f32(gl, "health", health);
        game.render_data.shaders.hp_shader.set_f32(gl, "left", cs.left);
        game.render_data.shaders.hp_shader.set_f32(gl, "right", cs.right);
        game.render_data.shaders.hp_shader.set_f32(gl, "screen_w", game.camera.width);

        game.render_data.square.sub_data(gl, cs.left, cs.right, cs.top, cs.bottom);
        game.render_data.square.render(gl);

    }

}


fn render_mouse(gl: &gl::Gl, game: &Game) {
    unsafe {
        gl.Clear(gl::DEPTH_BUFFER_BIT);
        gl.BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
        gl.Enable( gl::BLEND );
    }

    let cursor_w = 16.0;
    let cursor_h = 24.0;

    // calc clip space [-1,1] coors from screen space coords
    let left = (-0.5 + (game.state.mouse_pos.x) / 1200.0) * 2.0;
    let right = (-0.5 + (game.state.mouse_pos.x + cursor_w) / 1200.0) * 2.0;
    let top =  (-0.5 + (game.state.mouse_pos.y) / 700.0) * -2.0;
    let bottom =  (-0.5 + (game.state.mouse_pos.y + cursor_h) / 700.0) * -2.0;

    game.render_data.shaders.select_box_shader.set_used();

    //println!("Mouse at {:?}", (left, right, top, bottom));
    game.render_data.square.sub_data(gl, left, right, top, bottom);
    game.render_data.square.render(gl);

}



fn shadow_map_render(gl: &gl::Gl, shadow_map: &shadow_map::ShadowMap, light_pos: na::Vector3::<f32>, render_objs: &[RenderMesh]) {

    shadow_map.pre_render(gl, light_pos);

    unsafe {
        gl.Enable(gl::CULL_FACE);
        gl.CullFace(gl::FRONT);
    }

    for ro in render_objs {
        shadow_map.shader.set_mat4(gl, "model", ro.model_mat);
        ro.mesh.render(gl);
    }

    unsafe {
        gl.CullFace(gl::BACK);
    }

    // TODO: Fix to use viewport
    shadow_map.post_render(gl, 1200, 700);
}
