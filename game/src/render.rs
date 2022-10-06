extern crate shared;

use nalgebra::vector;
use libloading;
use crate::state::*;
use gl_lib::{gl, na, objects::{plane, mesh, shadow_map, texture_quad, square, gltf_mesh}, shader::{self, Shader}, camera};
use gl_lib::controller;
use gl_lib::sdl2::keyboard::Keycode;
use crate::game::*;


pub struct RenderData {

    // MESHESH
    pub boid: mesh::Mesh,
    pub plane: mesh::Mesh,

    pub square: square::Square,

    //SHADOW MAP STUFF
    pub tex_quad: texture_quad::TextureQuad,
    pub shadow_map: shadow_map::ShadowMap,

    // SHADERS
    pub mesh_shader: shader::BaseShader,
    pub texture_shader: shader::BaseShader,
    pub select_box_shader: shader::BaseShader,

}

impl RenderData {

    pub fn new(gl: &gl::Gl) -> Self {
        let base_path: std::path::PathBuf = "E:/repos/HerdGame/assets".to_string().into();

        let hashmap = std::collections::HashMap::new();
        let boids_gltf = gltf_mesh::meshes_from_gltf(&"E:/repos/HerdGame/assets/boid.glb", &hashmap).unwrap();
        let boid = boids_gltf.get_mesh(gl, "Boid").unwrap();

        let plane = plane::Plane::new(gl);


        Self {
            plane,
            boid,
            square: square::Square::new(gl),
            shadow_map: shadow_map::ShadowMap::new(&gl),
            tex_quad: texture_quad::TextureQuad::new(&gl),
            mesh_shader: create_shader(gl, &base_path, "mesh").unwrap(),
            texture_shader: create_shader(gl, &base_path, "texture").unwrap(),
            select_box_shader: create_shader(gl, &base_path, "select_box").unwrap(),
        }
    }
}


pub fn create_shader(gl: &gl::Gl, root_path: &std::path::PathBuf, name: &str) -> Result<shader::BaseShader, failure::Error> {
    let vert_shader_path =  std::path::Path::new(root_path).join(format!("{}_shader.vert", name));
    let vert_source = std::fs::read_to_string(vert_shader_path.clone()).expect(&format!("Could not reader vert shader file at: {:?}", vert_shader_path));


    let frag_shader_path = std::path::Path::new(root_path).join(format!("{}_shader.frag", name));
    let frag_source = std::fs::read_to_string(frag_shader_path.clone()).expect(&format!("Could not reader frag shader file at: {:?}", frag_shader_path));

    shader::BaseShader::new(gl, &vert_source, &frag_source)
}


pub struct RenderMesh<'a> {
    pub shader: &'a shader::BaseShader,
    pub model_mat: na::Matrix4::<f32>,
    pub mesh: &'a mesh::Mesh,
    pub color: na::Vector3::<f32>,
}


pub fn render(gl: &gl::Gl, game: &Game) {

    // Setup render meshes
    let mut render_objs = vec![];
    for i in 0..game.state.positions.len() {

        let mut model_mat = na::Matrix4::identity();
        model_mat = model_mat.append_nonuniform_scaling(&na::Vector3::new(0.2, 0.2, 0.2));
        let rotation = na::geometry::Rotation::from_euler_angles(0.0, 0.0, game.state.z_rotations[i]);

        model_mat = rotation.to_homogeneous()* model_mat;

        // cal rotation of boid, based on dir
        model_mat = model_mat.append_translation(&game.state.positions[i]);
        render_objs.push(RenderMesh {
            shader: &game.render_data.mesh_shader,
            model_mat,
            mesh: &game.render_data.boid,
            color: na::Vector3::new(1.0, 0.0, 0.0),
        });
    }

    // grund plane mesh
    let plane_scale = 100.0;
    let mut model_mat = na::Matrix4::identity();
    model_mat = model_mat.prepend_nonuniform_scaling(&vector![plane_scale, plane_scale, 1.0]);
    render_objs.push(RenderMesh {
            shader: &game.render_data.mesh_shader,
            model_mat,
            mesh: &game.render_data.plane,
            color: vector![23.0/255.0, 145.0/255.0, 40.0/255.0]
    });




    unsafe {
        gl.Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
    }




    // SHADOW MAP AND ALL ENTITIES

    let light_pos = na::Vector3::new(0.0, 0.0, 30.0);

    shadow_map_render(gl, &game.render_data.shadow_map, light_pos, &render_objs);

    let light_space_mat = game.render_data.shadow_map.light_space_mat(light_pos);

     // bind the shadow map texture to sampler 1
    unsafe {
        gl.ActiveTexture(gl::TEXTURE0);
        gl.BindTexture(gl::TEXTURE_2D, game.render_data.shadow_map.depth_map);
    }


    let view =  game.camera.view();
    let projection = game.camera.projection();

    for ro in render_objs {
        ro.shader.set_used();
        ro.shader.set_vec3(gl, "lightColor", na::Vector3::new(1.0, 1.0, 1.0));
        ro.shader.set_vec3(gl, "lightPos", light_pos);
        ro.shader.set_mat4(gl, "view", view);
        ro.shader.set_mat4(gl, "projection", projection);
        ro.shader.set_vec3(gl, "color", ro.color);
        ro.shader.set_mat4(gl, "model", ro.model_mat);
        ro.shader.set_mat4(gl, "lightSpaceMat", light_space_mat);

        ro.mesh.render(gl);
    }



    // SELECT BOX

    if let Some(sb) = game.select_box {
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
        game.render_data.select_box_shader.set_used();


        unsafe {
            gl.Clear(gl::DEPTH_BUFFER_BIT);
            gl.BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
            gl.Enable( gl::BLEND );
        }

        game.render_data.square.render(gl);
    }
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
