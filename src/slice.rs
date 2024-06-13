//fn slicing() {
//    let vertices: &VertexAttributeValues = mesh.attribute(Mesh::ATTRIBUTE_POSITION).unwrap();
//    let indices = match mesh.indices().unwrap() {
//        Indices::U32(indices) => indices.clone(),
//        _ => panic!("Unsupported index format"),
//    };
//
//    let VertexAttributeValues::Float32x3(vertices) = vertices else {
//        panic!()
//    };
//
//    let mut vertices1 = vec![];
//    let mut indices1 = vec![];
//    let mut vertices2 = vec![];
//    let mut indices2 = vec![];
//
//    for i in (0..indices.len()).step_by(3) {
//        let v0 = vertices[indices[i] as usize];
//        let v1 = vertices[indices[i + 1] as usize];
//        let v2 = vertices[indices[i + 2] as usize];
//
//        if v0[1] > 0.0 {
//            vertices1.push(v0);
//            vertices1.push(v1);
//            vertices1.push(v2);
//            indices1.push(indices[i]);
//            indices1.push(indices[i + 1]);
//            indices1.push(indices[i + 2]);
//        } else {
//            vertices2.push(v0);
//            vertices2.push(v1);
//            vertices2.push(v2);
//            indices2.push(indices[i]);
//            indices2.push(indices[i + 1]);
//            indices2.push(indices[i + 2]);
//        }
//    }
//
//    log::info!("{:#?}", mesh);
//
//    let mut mesh1 = Mesh::new(
//        PrimitiveTopology::TriangleList,
//        RenderAssetUsages::MAIN_WORLD,
//    );
//    mesh1.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertices1);
//    mesh1.insert_indices(Indices::U32(indices1));
//
//    let mut mesh2 = Mesh::new(
//        PrimitiveTopology::TriangleList,
//        RenderAssetUsages::MAIN_WORLD,
//    );
//    mesh2.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertices2);
//    mesh2.insert_indices(Indices::U32(indices2));
//
//    commands.spawn(PbrBundle {
//        mesh: meshes.add(mesh1),
//        material: materials.add(Color::rgb_u8(124, 144, 255)),
//        transform: Transform::from_xyz(0.0, 0.5, 0.0),
//        ..Default::default()
//    });
//
//    commands.spawn(PbrBundle {
//        mesh: meshes.add(mesh2),
//        material: materials.add(Color::rgb_u8(124, 144, 255)),
//        transform: Transform::from_xyz(0.0, 0.5, 0.0),
//        ..Default::default()
//    });
//}
