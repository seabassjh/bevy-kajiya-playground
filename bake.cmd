@echo off

cd bevy-kajiya\kajiya

cargo build --bin bake --release
set BAKE=bevy-kajiya\kajiya\target\release\bake

cd ..\..

%BAKE% --scene "assets/meshes/336_lrm/scene.gltf" --scale 1.0 -o car
%BAKE% --scene "assets/meshes/floor/scene.gltf" --scale 1.0 -o floor
%BAKE% --scene "assets/meshes/ring/scene.gltf" --scale 1.0 -o ring
%BAKE% --scene "assets/meshes/mirror/scene.gltf" --scale 1.0 -o mirror
%BAKE% --scene "assets/meshes/smiley_box/scene.gltf" --scale 0.1 -o smiley_box