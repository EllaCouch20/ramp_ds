// use bevy::prelude::*;

// // Header - Option< Icon || PFP >, TEXT, Option< Icon || PFP >

// pub enum Interface {
//     Root,
//     Stack,
//     Takeover,
// }

// pub struct Header {
//     title: String,
//     variant: Interface,
//     icon_left: Option<(ImageNode, NavigateTo)>,
//     icon_right: Option<(ImageNode, NavigateTo)>,
//     profile_photo: bool,
// }

// impl Header {
//     pub fn new(
//         title: &str,
//         variant: Interface,
//         icon_left: Option<(ImageNode, NavigateTo)>,
//         icon_right: Option<(ImageNode, NavigateTo)>,
//         profile_photo: bool,
//     ) -> Self {
//         Header {
//             title: title.to_string()
//             variant,
//             icon_left,
//             icon_right,
//             profile_photo,
//         }
//     }

//     pub fn spawn_under<T: Bundle>(
//         self,
//         parent: &mut ChildBuilder,
//         theme: &Res<Theme>
//     ) {

//         font_size = match variant { // RID THIS
//             Interface::Takeover => theme.fonts.size.h3,
//             Interface::Root => theme.fonts.size.h3,
//             Interface::Stack => theme.fonts.size.h4,
//         }

//         let font = theme.fonts.style.heading.clone();

//         parent.spawn(Node{
//             width: EXPAND,
//             align_items: AlignItems::Center,
//             justify_content: JustifyContent::SpaceBetween,
//             flex_direction: FlexDirection::Row,
//             padding: UiRect::all(Val::Px(24.0)),
//             ..default()
//         }).with_children(|parent| {

//             icon_button(parent, self.icon_left); // Optional
//             pfp_button(parent, self.profile_photo); // Optional
            
//             parent.spawn(text(self.title, font, font_size, theme.colors.text_heading));

//             icon_button(parent, self.icon_right); // Optional
//         });
//     }
// }


// Content - Vec[Spawns]

// Bumper - Vec[Buttons]

// Option< Tab Navigator || Navigation Sidebar >

// Interfaces
    // Root
    // Stack
    // Takeover