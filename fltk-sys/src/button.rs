/* automatically generated by rust-bindgen */

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Fl_Widget {
    _unused: [u8; 0],
}
pub type Fl_Callback = ::std::option::Option<
    unsafe extern "C" fn(arg1: *mut Fl_Widget, arg2: *mut ::std::os::raw::c_void),
>;
extern "C" {
    pub fn Fl_Widget_callback_with_captures(
        arg1: *mut Fl_Widget,
        cb: Fl_Callback,
        arg2: *mut ::std::os::raw::c_void,
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Fl_Button {
    _unused: [u8; 0],
}
extern "C" {
    pub fn Fl_Button_new(
        x: ::std::os::raw::c_int,
        y: ::std::os::raw::c_int,
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
        title: *const ::std::os::raw::c_char,
    ) -> *mut Fl_Button;
}
extern "C" {
    pub fn Fl_Button_set_label(arg1: *mut Fl_Button, title: *const ::std::os::raw::c_char);
}
extern "C" {
    pub fn Fl_Button_redraw(arg1: *mut Fl_Button);
}
extern "C" {
    pub fn Fl_Button_show(arg1: *mut Fl_Button);
}
extern "C" {
    pub fn Fl_Button_hide(arg1: *mut Fl_Button);
}
extern "C" {
    pub fn Fl_Button_activate(arg1: *mut Fl_Button);
}
extern "C" {
    pub fn Fl_Button_deactivate(arg1: *mut Fl_Button);
}
extern "C" {
    pub fn Fl_Button_redraw_label(arg1: *mut Fl_Button);
}
extern "C" {
    pub fn Fl_Button_resize(
        arg1: *mut Fl_Button,
        x: ::std::os::raw::c_int,
        y: ::std::os::raw::c_int,
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn Fl_Button_tooltip(arg1: *mut Fl_Button) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn Fl_Button_set_tooltip(arg1: *mut Fl_Button, txt: *const ::std::os::raw::c_char);
}
extern "C" {
    pub fn Fl_Button_get_type(arg1: *mut Fl_Button) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Button_set_type(arg1: *mut Fl_Button, typ: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Button_color(arg1: *mut Fl_Button) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Button_set_color(arg1: *mut Fl_Button, color: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Button_label_color(arg1: *mut Fl_Button) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Button_set_label_color(arg1: *mut Fl_Button, color: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Button_label_font(arg1: *mut Fl_Button) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Button_set_label_font(arg1: *mut Fl_Button, font: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Button_label_size(arg1: *mut Fl_Button) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Button_set_label_size(arg1: *mut Fl_Button, sz: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Button_label_type(arg1: *mut Fl_Button) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Button_set_label_type(arg1: *mut Fl_Button, typ: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Button_box(arg1: *mut Fl_Button) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Button_set_box(arg1: *mut Fl_Button, typ: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Button_changed(arg1: *mut Fl_Button) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Button_set_changed(arg1: *mut Fl_Button);
}
extern "C" {
    pub fn Fl_Button_clear_changed(arg1: *mut Fl_Button);
}
extern "C" {
    pub fn Fl_Button_align(arg1: *mut Fl_Button) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Button_set_align(arg1: *mut Fl_Button, typ: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Button_delete(arg1: *mut Fl_Button);
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Fl_Check_Button {
    _unused: [u8; 0],
}
extern "C" {
    pub fn Fl_Check_Button_new(
        x: ::std::os::raw::c_int,
        y: ::std::os::raw::c_int,
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
        title: *const ::std::os::raw::c_char,
    ) -> *mut Fl_Check_Button;
}
extern "C" {
    pub fn Fl_Check_Button_set_label(
        arg1: *mut Fl_Check_Button,
        title: *const ::std::os::raw::c_char,
    );
}
extern "C" {
    pub fn Fl_Check_Button_redraw(arg1: *mut Fl_Check_Button);
}
extern "C" {
    pub fn Fl_Check_Button_show(arg1: *mut Fl_Check_Button);
}
extern "C" {
    pub fn Fl_Check_Button_hide(arg1: *mut Fl_Check_Button);
}
extern "C" {
    pub fn Fl_Check_Button_activate(arg1: *mut Fl_Check_Button);
}
extern "C" {
    pub fn Fl_Check_Button_deactivate(arg1: *mut Fl_Check_Button);
}
extern "C" {
    pub fn Fl_Check_Button_redraw_label(arg1: *mut Fl_Check_Button);
}
extern "C" {
    pub fn Fl_Check_Button_resize(
        arg1: *mut Fl_Check_Button,
        x: ::std::os::raw::c_int,
        y: ::std::os::raw::c_int,
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn Fl_Check_Button_tooltip(arg1: *mut Fl_Check_Button) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn Fl_Check_Button_set_tooltip(
        arg1: *mut Fl_Check_Button,
        txt: *const ::std::os::raw::c_char,
    );
}
extern "C" {
    pub fn Fl_Check_Button_get_type(arg1: *mut Fl_Check_Button) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Check_Button_set_type(arg1: *mut Fl_Check_Button, typ: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Check_Button_color(arg1: *mut Fl_Check_Button) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Check_Button_set_color(arg1: *mut Fl_Check_Button, color: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Check_Button_label_color(arg1: *mut Fl_Check_Button) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Check_Button_set_label_color(
        arg1: *mut Fl_Check_Button,
        color: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn Fl_Check_Button_label_font(arg1: *mut Fl_Check_Button) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Check_Button_set_label_font(arg1: *mut Fl_Check_Button, font: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Check_Button_label_size(arg1: *mut Fl_Check_Button) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Check_Button_set_label_size(arg1: *mut Fl_Check_Button, sz: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Check_Button_label_type(arg1: *mut Fl_Check_Button) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Check_Button_set_label_type(arg1: *mut Fl_Check_Button, typ: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Check_Button_box(arg1: *mut Fl_Check_Button) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Check_Button_set_box(arg1: *mut Fl_Check_Button, typ: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Check_Button_changed(arg1: *mut Fl_Check_Button) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Check_Button_set_changed(arg1: *mut Fl_Check_Button);
}
extern "C" {
    pub fn Fl_Check_Button_clear_changed(arg1: *mut Fl_Check_Button);
}
extern "C" {
    pub fn Fl_Check_Button_align(arg1: *mut Fl_Check_Button) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Check_Button_set_align(arg1: *mut Fl_Check_Button, typ: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Check_Button_delete(arg1: *mut Fl_Check_Button);
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Fl_Radio_Button {
    _unused: [u8; 0],
}
extern "C" {
    pub fn Fl_Radio_Button_new(
        x: ::std::os::raw::c_int,
        y: ::std::os::raw::c_int,
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
        title: *const ::std::os::raw::c_char,
    ) -> *mut Fl_Radio_Button;
}
extern "C" {
    pub fn Fl_Radio_Button_set_label(
        arg1: *mut Fl_Radio_Button,
        title: *const ::std::os::raw::c_char,
    );
}
extern "C" {
    pub fn Fl_Radio_Button_redraw(arg1: *mut Fl_Radio_Button);
}
extern "C" {
    pub fn Fl_Radio_Button_show(arg1: *mut Fl_Radio_Button);
}
extern "C" {
    pub fn Fl_Radio_Button_hide(arg1: *mut Fl_Radio_Button);
}
extern "C" {
    pub fn Fl_Radio_Button_activate(arg1: *mut Fl_Radio_Button);
}
extern "C" {
    pub fn Fl_Radio_Button_deactivate(arg1: *mut Fl_Radio_Button);
}
extern "C" {
    pub fn Fl_Radio_Button_redraw_label(arg1: *mut Fl_Radio_Button);
}
extern "C" {
    pub fn Fl_Radio_Button_resize(
        arg1: *mut Fl_Radio_Button,
        x: ::std::os::raw::c_int,
        y: ::std::os::raw::c_int,
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn Fl_Radio_Button_tooltip(arg1: *mut Fl_Radio_Button) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn Fl_Radio_Button_set_tooltip(
        arg1: *mut Fl_Radio_Button,
        txt: *const ::std::os::raw::c_char,
    );
}
extern "C" {
    pub fn Fl_Radio_Button_get_type(arg1: *mut Fl_Radio_Button) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Radio_Button_set_type(arg1: *mut Fl_Radio_Button, typ: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Radio_Button_color(arg1: *mut Fl_Radio_Button) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Radio_Button_set_color(arg1: *mut Fl_Radio_Button, color: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Radio_Button_label_color(arg1: *mut Fl_Radio_Button) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Radio_Button_set_label_color(
        arg1: *mut Fl_Radio_Button,
        color: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn Fl_Radio_Button_label_font(arg1: *mut Fl_Radio_Button) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Radio_Button_set_label_font(arg1: *mut Fl_Radio_Button, font: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Radio_Button_label_size(arg1: *mut Fl_Radio_Button) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Radio_Button_set_label_size(arg1: *mut Fl_Radio_Button, sz: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Radio_Button_label_type(arg1: *mut Fl_Radio_Button) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Radio_Button_set_label_type(arg1: *mut Fl_Radio_Button, typ: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Radio_Button_box(arg1: *mut Fl_Radio_Button) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Radio_Button_set_box(arg1: *mut Fl_Radio_Button, typ: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Radio_Button_changed(arg1: *mut Fl_Radio_Button) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Radio_Button_set_changed(arg1: *mut Fl_Radio_Button);
}
extern "C" {
    pub fn Fl_Radio_Button_clear_changed(arg1: *mut Fl_Radio_Button);
}
extern "C" {
    pub fn Fl_Radio_Button_align(arg1: *mut Fl_Radio_Button) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Radio_Button_set_align(arg1: *mut Fl_Radio_Button, typ: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Radio_Button_delete(arg1: *mut Fl_Radio_Button);
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Fl_Toggle_Button {
    _unused: [u8; 0],
}
extern "C" {
    pub fn Fl_Toggle_Button_new(
        x: ::std::os::raw::c_int,
        y: ::std::os::raw::c_int,
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
        title: *const ::std::os::raw::c_char,
    ) -> *mut Fl_Toggle_Button;
}
extern "C" {
    pub fn Fl_Toggle_Button_set_label(
        arg1: *mut Fl_Toggle_Button,
        title: *const ::std::os::raw::c_char,
    );
}
extern "C" {
    pub fn Fl_Toggle_Button_redraw(arg1: *mut Fl_Toggle_Button);
}
extern "C" {
    pub fn Fl_Toggle_Button_show(arg1: *mut Fl_Toggle_Button);
}
extern "C" {
    pub fn Fl_Toggle_Button_hide(arg1: *mut Fl_Toggle_Button);
}
extern "C" {
    pub fn Fl_Toggle_Button_activate(arg1: *mut Fl_Toggle_Button);
}
extern "C" {
    pub fn Fl_Toggle_Button_deactivate(arg1: *mut Fl_Toggle_Button);
}
extern "C" {
    pub fn Fl_Toggle_Button_redraw_label(arg1: *mut Fl_Toggle_Button);
}
extern "C" {
    pub fn Fl_Toggle_Button_resize(
        arg1: *mut Fl_Toggle_Button,
        x: ::std::os::raw::c_int,
        y: ::std::os::raw::c_int,
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn Fl_Toggle_Button_tooltip(arg1: *mut Fl_Toggle_Button) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn Fl_Toggle_Button_set_tooltip(
        arg1: *mut Fl_Toggle_Button,
        txt: *const ::std::os::raw::c_char,
    );
}
extern "C" {
    pub fn Fl_Toggle_Button_get_type(arg1: *mut Fl_Toggle_Button) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Toggle_Button_set_type(arg1: *mut Fl_Toggle_Button, typ: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Toggle_Button_color(arg1: *mut Fl_Toggle_Button) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Toggle_Button_set_color(arg1: *mut Fl_Toggle_Button, color: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Toggle_Button_label_color(arg1: *mut Fl_Toggle_Button) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Toggle_Button_set_label_color(
        arg1: *mut Fl_Toggle_Button,
        color: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn Fl_Toggle_Button_label_font(arg1: *mut Fl_Toggle_Button) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Toggle_Button_set_label_font(
        arg1: *mut Fl_Toggle_Button,
        font: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn Fl_Toggle_Button_label_size(arg1: *mut Fl_Toggle_Button) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Toggle_Button_set_label_size(arg1: *mut Fl_Toggle_Button, sz: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Toggle_Button_label_type(arg1: *mut Fl_Toggle_Button) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Toggle_Button_set_label_type(arg1: *mut Fl_Toggle_Button, typ: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Toggle_Button_box(arg1: *mut Fl_Toggle_Button) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Toggle_Button_set_box(arg1: *mut Fl_Toggle_Button, typ: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Toggle_Button_changed(arg1: *mut Fl_Toggle_Button) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Toggle_Button_set_changed(arg1: *mut Fl_Toggle_Button);
}
extern "C" {
    pub fn Fl_Toggle_Button_clear_changed(arg1: *mut Fl_Toggle_Button);
}
extern "C" {
    pub fn Fl_Toggle_Button_align(arg1: *mut Fl_Toggle_Button) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Toggle_Button_set_align(arg1: *mut Fl_Toggle_Button, typ: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Toggle_Button_delete(arg1: *mut Fl_Toggle_Button);
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Fl_Round_Button {
    _unused: [u8; 0],
}
extern "C" {
    pub fn Fl_Round_Button_new(
        x: ::std::os::raw::c_int,
        y: ::std::os::raw::c_int,
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
        title: *const ::std::os::raw::c_char,
    ) -> *mut Fl_Round_Button;
}
extern "C" {
    pub fn Fl_Round_Button_set_label(
        arg1: *mut Fl_Round_Button,
        title: *const ::std::os::raw::c_char,
    );
}
extern "C" {
    pub fn Fl_Round_Button_redraw(arg1: *mut Fl_Round_Button);
}
extern "C" {
    pub fn Fl_Round_Button_show(arg1: *mut Fl_Round_Button);
}
extern "C" {
    pub fn Fl_Round_Button_hide(arg1: *mut Fl_Round_Button);
}
extern "C" {
    pub fn Fl_Round_Button_activate(arg1: *mut Fl_Round_Button);
}
extern "C" {
    pub fn Fl_Round_Button_deactivate(arg1: *mut Fl_Round_Button);
}
extern "C" {
    pub fn Fl_Round_Button_redraw_label(arg1: *mut Fl_Round_Button);
}
extern "C" {
    pub fn Fl_Round_Button_resize(
        arg1: *mut Fl_Round_Button,
        x: ::std::os::raw::c_int,
        y: ::std::os::raw::c_int,
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn Fl_Round_Button_tooltip(arg1: *mut Fl_Round_Button) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn Fl_Round_Button_set_tooltip(
        arg1: *mut Fl_Round_Button,
        txt: *const ::std::os::raw::c_char,
    );
}
extern "C" {
    pub fn Fl_Round_Button_get_type(arg1: *mut Fl_Round_Button) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Round_Button_set_type(arg1: *mut Fl_Round_Button, typ: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Round_Button_color(arg1: *mut Fl_Round_Button) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Round_Button_set_color(arg1: *mut Fl_Round_Button, color: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Round_Button_label_color(arg1: *mut Fl_Round_Button) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Round_Button_set_label_color(
        arg1: *mut Fl_Round_Button,
        color: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn Fl_Round_Button_label_font(arg1: *mut Fl_Round_Button) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Round_Button_set_label_font(arg1: *mut Fl_Round_Button, font: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Round_Button_label_size(arg1: *mut Fl_Round_Button) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Round_Button_set_label_size(arg1: *mut Fl_Round_Button, sz: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Round_Button_label_type(arg1: *mut Fl_Round_Button) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Round_Button_set_label_type(arg1: *mut Fl_Round_Button, typ: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Round_Button_box(arg1: *mut Fl_Round_Button) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Round_Button_set_box(arg1: *mut Fl_Round_Button, typ: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Round_Button_changed(arg1: *mut Fl_Round_Button) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Round_Button_set_changed(arg1: *mut Fl_Round_Button);
}
extern "C" {
    pub fn Fl_Round_Button_clear_changed(arg1: *mut Fl_Round_Button);
}
extern "C" {
    pub fn Fl_Round_Button_align(arg1: *mut Fl_Round_Button) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Round_Button_set_align(arg1: *mut Fl_Round_Button, typ: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Round_Button_delete(arg1: *mut Fl_Round_Button);
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Fl_Light_Button {
    _unused: [u8; 0],
}
extern "C" {
    pub fn Fl_Light_Button_new(
        x: ::std::os::raw::c_int,
        y: ::std::os::raw::c_int,
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
        title: *const ::std::os::raw::c_char,
    ) -> *mut Fl_Light_Button;
}
extern "C" {
    pub fn Fl_Light_Button_set_label(
        arg1: *mut Fl_Light_Button,
        title: *const ::std::os::raw::c_char,
    );
}
extern "C" {
    pub fn Fl_Light_Button_redraw(arg1: *mut Fl_Light_Button);
}
extern "C" {
    pub fn Fl_Light_Button_show(arg1: *mut Fl_Light_Button);
}
extern "C" {
    pub fn Fl_Light_Button_hide(arg1: *mut Fl_Light_Button);
}
extern "C" {
    pub fn Fl_Light_Button_activate(arg1: *mut Fl_Light_Button);
}
extern "C" {
    pub fn Fl_Light_Button_deactivate(arg1: *mut Fl_Light_Button);
}
extern "C" {
    pub fn Fl_Light_Button_redraw_label(arg1: *mut Fl_Light_Button);
}
extern "C" {
    pub fn Fl_Light_Button_resize(
        arg1: *mut Fl_Light_Button,
        x: ::std::os::raw::c_int,
        y: ::std::os::raw::c_int,
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn Fl_Light_Button_tooltip(arg1: *mut Fl_Light_Button) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn Fl_Light_Button_set_tooltip(
        arg1: *mut Fl_Light_Button,
        txt: *const ::std::os::raw::c_char,
    );
}
extern "C" {
    pub fn Fl_Light_Button_get_type(arg1: *mut Fl_Light_Button) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Light_Button_set_type(arg1: *mut Fl_Light_Button, typ: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Light_Button_color(arg1: *mut Fl_Light_Button) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Light_Button_set_color(arg1: *mut Fl_Light_Button, color: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Light_Button_label_color(arg1: *mut Fl_Light_Button) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Light_Button_set_label_color(
        arg1: *mut Fl_Light_Button,
        color: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn Fl_Light_Button_label_font(arg1: *mut Fl_Light_Button) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Light_Button_set_label_font(arg1: *mut Fl_Light_Button, font: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Light_Button_label_size(arg1: *mut Fl_Light_Button) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Light_Button_set_label_size(arg1: *mut Fl_Light_Button, sz: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Light_Button_label_type(arg1: *mut Fl_Light_Button) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Light_Button_set_label_type(arg1: *mut Fl_Light_Button, typ: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Light_Button_box(arg1: *mut Fl_Light_Button) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Light_Button_set_box(arg1: *mut Fl_Light_Button, typ: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Light_Button_changed(arg1: *mut Fl_Light_Button) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Light_Button_set_changed(arg1: *mut Fl_Light_Button);
}
extern "C" {
    pub fn Fl_Light_Button_clear_changed(arg1: *mut Fl_Light_Button);
}
extern "C" {
    pub fn Fl_Light_Button_align(arg1: *mut Fl_Light_Button) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Light_Button_set_align(arg1: *mut Fl_Light_Button, typ: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Light_Button_delete(arg1: *mut Fl_Light_Button);
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Fl_Repeat_Button {
    _unused: [u8; 0],
}
extern "C" {
    pub fn Fl_Repeat_Button_new(
        x: ::std::os::raw::c_int,
        y: ::std::os::raw::c_int,
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
        title: *const ::std::os::raw::c_char,
    ) -> *mut Fl_Repeat_Button;
}
extern "C" {
    pub fn Fl_Repeat_Button_set_label(
        arg1: *mut Fl_Repeat_Button,
        title: *const ::std::os::raw::c_char,
    );
}
extern "C" {
    pub fn Fl_Repeat_Button_redraw(arg1: *mut Fl_Repeat_Button);
}
extern "C" {
    pub fn Fl_Repeat_Button_show(arg1: *mut Fl_Repeat_Button);
}
extern "C" {
    pub fn Fl_Repeat_Button_hide(arg1: *mut Fl_Repeat_Button);
}
extern "C" {
    pub fn Fl_Repeat_Button_activate(arg1: *mut Fl_Repeat_Button);
}
extern "C" {
    pub fn Fl_Repeat_Button_deactivate(arg1: *mut Fl_Repeat_Button);
}
extern "C" {
    pub fn Fl_Repeat_Button_redraw_label(arg1: *mut Fl_Repeat_Button);
}
extern "C" {
    pub fn Fl_Repeat_Button_resize(
        arg1: *mut Fl_Repeat_Button,
        x: ::std::os::raw::c_int,
        y: ::std::os::raw::c_int,
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn Fl_Repeat_Button_tooltip(arg1: *mut Fl_Repeat_Button) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn Fl_Repeat_Button_set_tooltip(
        arg1: *mut Fl_Repeat_Button,
        txt: *const ::std::os::raw::c_char,
    );
}
extern "C" {
    pub fn Fl_Repeat_Button_get_type(arg1: *mut Fl_Repeat_Button) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Repeat_Button_set_type(arg1: *mut Fl_Repeat_Button, typ: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Repeat_Button_color(arg1: *mut Fl_Repeat_Button) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Repeat_Button_set_color(arg1: *mut Fl_Repeat_Button, color: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Repeat_Button_label_color(arg1: *mut Fl_Repeat_Button) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Repeat_Button_set_label_color(
        arg1: *mut Fl_Repeat_Button,
        color: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn Fl_Repeat_Button_label_font(arg1: *mut Fl_Repeat_Button) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Repeat_Button_set_label_font(
        arg1: *mut Fl_Repeat_Button,
        font: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn Fl_Repeat_Button_label_size(arg1: *mut Fl_Repeat_Button) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Repeat_Button_set_label_size(arg1: *mut Fl_Repeat_Button, sz: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Repeat_Button_label_type(arg1: *mut Fl_Repeat_Button) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Repeat_Button_set_label_type(arg1: *mut Fl_Repeat_Button, typ: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Repeat_Button_box(arg1: *mut Fl_Repeat_Button) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Repeat_Button_set_box(arg1: *mut Fl_Repeat_Button, typ: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Repeat_Button_changed(arg1: *mut Fl_Repeat_Button) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Repeat_Button_set_changed(arg1: *mut Fl_Repeat_Button);
}
extern "C" {
    pub fn Fl_Repeat_Button_clear_changed(arg1: *mut Fl_Repeat_Button);
}
extern "C" {
    pub fn Fl_Repeat_Button_align(arg1: *mut Fl_Repeat_Button) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Repeat_Button_set_align(arg1: *mut Fl_Repeat_Button, typ: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Repeat_Button_delete(arg1: *mut Fl_Repeat_Button);
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Fl_Return_Button {
    _unused: [u8; 0],
}
extern "C" {
    pub fn Fl_Return_Button_new(
        x: ::std::os::raw::c_int,
        y: ::std::os::raw::c_int,
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
        title: *const ::std::os::raw::c_char,
    ) -> *mut Fl_Return_Button;
}
extern "C" {
    pub fn Fl_Return_Button_set_label(
        arg1: *mut Fl_Return_Button,
        title: *const ::std::os::raw::c_char,
    );
}
extern "C" {
    pub fn Fl_Return_Button_redraw(arg1: *mut Fl_Return_Button);
}
extern "C" {
    pub fn Fl_Return_Button_show(arg1: *mut Fl_Return_Button);
}
extern "C" {
    pub fn Fl_Return_Button_hide(arg1: *mut Fl_Return_Button);
}
extern "C" {
    pub fn Fl_Return_Button_activate(arg1: *mut Fl_Return_Button);
}
extern "C" {
    pub fn Fl_Return_Button_deactivate(arg1: *mut Fl_Return_Button);
}
extern "C" {
    pub fn Fl_Return_Button_redraw_label(arg1: *mut Fl_Return_Button);
}
extern "C" {
    pub fn Fl_Return_Button_resize(
        arg1: *mut Fl_Return_Button,
        x: ::std::os::raw::c_int,
        y: ::std::os::raw::c_int,
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn Fl_Return_Button_tooltip(arg1: *mut Fl_Return_Button) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn Fl_Return_Button_set_tooltip(
        arg1: *mut Fl_Return_Button,
        txt: *const ::std::os::raw::c_char,
    );
}
extern "C" {
    pub fn Fl_Return_Button_get_type(arg1: *mut Fl_Return_Button) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Return_Button_set_type(arg1: *mut Fl_Return_Button, typ: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Return_Button_color(arg1: *mut Fl_Return_Button) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Return_Button_set_color(arg1: *mut Fl_Return_Button, color: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Return_Button_label_color(arg1: *mut Fl_Return_Button) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Return_Button_set_label_color(
        arg1: *mut Fl_Return_Button,
        color: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn Fl_Return_Button_label_font(arg1: *mut Fl_Return_Button) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Return_Button_set_label_font(
        arg1: *mut Fl_Return_Button,
        font: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn Fl_Return_Button_label_size(arg1: *mut Fl_Return_Button) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Return_Button_set_label_size(arg1: *mut Fl_Return_Button, sz: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Return_Button_label_type(arg1: *mut Fl_Return_Button) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Return_Button_set_label_type(arg1: *mut Fl_Return_Button, typ: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Return_Button_box(arg1: *mut Fl_Return_Button) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Return_Button_set_box(arg1: *mut Fl_Return_Button, typ: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Return_Button_changed(arg1: *mut Fl_Return_Button) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Return_Button_set_changed(arg1: *mut Fl_Return_Button);
}
extern "C" {
    pub fn Fl_Return_Button_clear_changed(arg1: *mut Fl_Return_Button);
}
extern "C" {
    pub fn Fl_Return_Button_align(arg1: *mut Fl_Return_Button) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Return_Button_set_align(arg1: *mut Fl_Return_Button, typ: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Return_Button_delete(arg1: *mut Fl_Return_Button);
}
