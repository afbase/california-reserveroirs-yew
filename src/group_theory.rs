// use svg::{Group, node::Element};
// use yew::virtual_dom::VTag;

// struct LabeledElement {
//     pub element: Element,
//     pub labeled: bool
// }

// impl LabeledElement {
//     fn new(element: Element) -> LabeledElement {
//         LabeledElement {
//             element,
//             labeled: false
//         }
//     }
// }

// trait ToVTag {
//     fn to_vtag(self) -> VTag;
// }

// impl ToVTag for Group {
//     /// https://github.com/yewstack/yew/issues/2699#issuecomment-1135510535
//     fn to_vtag(group: Group) -> VTag {
//         let mut svg_group_children: Vec<LabeledElement> = Vec::new();
//         let mut vtag_children: Vec<VTag> = Vec::new();
//         let mut svg_vtag = VTag::new("svg");
//         let group_element: Element = group.into();
//         let labeled_group_element = LabeledElement::new(group_element);
//         svg_group_children.push(labeled_svg_vtag);
//         while svg_group_children.len() > 0 {
//             let v = svg_group_children.pop();
//             if !v.labeled {
//                 v.labeled = true;
//                 let children = v.element.get_inner().get_children();
//                 for some_child in children.iter() {
//                     let boxed_child: Box<dyn Node> = some_child.unwrap();
//                     let child: dyn Node = Box::into_raw(boxed_child);

//                 }
//             }

//         }
//         let mut outer_group = VTag::new("g");
//         outer_group.add_attribute("class", "g-chart");
//         svg_vtag.add_children(outer_group);
//         let mut temp = self.get_inner();
//         ///procedure DFS_iterative(G, v) is
//         // let S be a stack
//         // S.push(v)
//         // while S is not empty do
//         //     v = S.pop()
//         //     if v is not labeled as discovered then
//         //         label v as discovered
//         //         for all edges from v to w in G.adjacentEdges(v) do 
//         //             S.push(w)
//         while temp {
//             let children = temp.get_children();
//             let 
//         }
//     }
// }