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
pub struct Fl_Input {
    _unused: [u8; 0],
}
extern "C" {
    pub fn Fl_Input_new(
        x: ::std::os::raw::c_int,
        y: ::std::os::raw::c_int,
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
        title: *const ::std::os::raw::c_char,
    ) -> *mut Fl_Input;
}
extern "C" {
    pub fn Fl_Input_set_label(arg1: *mut Fl_Input, title: *const ::std::os::raw::c_char);
}
extern "C" {
    pub fn Fl_Input_redraw(arg1: *mut Fl_Input);
}
extern "C" {
    pub fn Fl_Input_show(arg1: *mut Fl_Input);
}
extern "C" {
    pub fn Fl_Input_hide(arg1: *mut Fl_Input);
}
extern "C" {
    pub fn Fl_Input_activate(arg1: *mut Fl_Input);
}
extern "C" {
    pub fn Fl_Input_deactivate(arg1: *mut Fl_Input);
}
extern "C" {
    pub fn Fl_Input_redraw_label(arg1: *mut Fl_Input);
}
extern "C" {
    pub fn Fl_Input_resize(
        arg1: *mut Fl_Input,
        x: ::std::os::raw::c_int,
        y: ::std::os::raw::c_int,
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn Fl_Input_tooltip(arg1: *mut Fl_Input) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn Fl_Input_set_tooltip(arg1: *mut Fl_Input, txt: *const ::std::os::raw::c_char);
}
extern "C" {
    pub fn Fl_Input_get_type(arg1: *mut Fl_Input) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Input_set_type(arg1: *mut Fl_Input, typ: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Input_color(arg1: *mut Fl_Input) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Input_set_color(arg1: *mut Fl_Input, color: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Input_label_color(arg1: *mut Fl_Input) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Input_set_label_color(arg1: *mut Fl_Input, color: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Input_label_font(arg1: *mut Fl_Input) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Input_set_label_font(arg1: *mut Fl_Input, font: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Input_label_size(arg1: *mut Fl_Input) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Input_set_label_size(arg1: *mut Fl_Input, sz: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Input_label_type(arg1: *mut Fl_Input) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Input_set_label_type(arg1: *mut Fl_Input, typ: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Input_box(arg1: *mut Fl_Input) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Input_set_box(arg1: *mut Fl_Input, typ: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Input_changed(arg1: *mut Fl_Input) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Input_set_changed(arg1: *mut Fl_Input);
}
extern "C" {
    pub fn Fl_Input_clear_changed(arg1: *mut Fl_Input);
}
extern "C" {
    pub fn Fl_Input_align(arg1: *mut Fl_Input) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Input_set_align(arg1: *mut Fl_Input, typ: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Input_delete(arg1: *mut Fl_Input);
}
extern "C" {
    pub fn Fl_Input_set_value(
        arg1: *mut Fl_Input,
        arg2: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Input_value(arg1: *mut Fl_Input) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn Fl_Input_maximum_size(arg1: *mut Fl_Input) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Input_set_maximum_size(arg1: *mut Fl_Input, m: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Input_position(arg1: *mut Fl_Input) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Input_set_position(
        arg1: *mut Fl_Input,
        p: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Input_set_mark(
        arg1: *mut Fl_Input,
        m: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Input_mark(arg1: *mut Fl_Input) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Input_replace(
        arg1: *mut Fl_Input,
        b: ::std::os::raw::c_int,
        e: ::std::os::raw::c_int,
        text: *const ::std::os::raw::c_char,
        ilen: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Input_insert(
        arg1: *mut Fl_Input,
        t: *const ::std::os::raw::c_char,
        l: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Input_append(
        arg1: *mut Fl_Input,
        t: *const ::std::os::raw::c_char,
        l: ::std::os::raw::c_int,
        keep_selection: ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Input_copy(
        arg1: *mut Fl_Input,
        clipboard: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Input_undo(arg1: *mut Fl_Input) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Input_copy_cuts(arg1: *mut Fl_Input) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Input_text_font(arg1: *mut Fl_Input) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Input_set_text_font(arg1: *mut Fl_Input, s: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Input_text_color(arg1: *mut Fl_Input) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Input_set_text_color(arg1: *mut Fl_Input, s: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Input_text_size(arg1: *mut Fl_Input) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Input_set_text_size(arg1: *mut Fl_Input, s: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Input_readonly(arg1: *mut Fl_Input) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Input_set_readonly(arg1: *mut Fl_Input, boolean: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Input_wrap(arg1: *mut Fl_Input) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Input_set_wrap(arg1: *mut Fl_Input, boolean: ::std::os::raw::c_int);
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Fl_Int_Input {
    _unused: [u8; 0],
}
extern "C" {
    pub fn Fl_Int_Input_new(
        x: ::std::os::raw::c_int,
        y: ::std::os::raw::c_int,
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
        title: *const ::std::os::raw::c_char,
    ) -> *mut Fl_Int_Input;
}
extern "C" {
    pub fn Fl_Int_Input_set_label(arg1: *mut Fl_Int_Input, title: *const ::std::os::raw::c_char);
}
extern "C" {
    pub fn Fl_Int_Input_redraw(arg1: *mut Fl_Int_Input);
}
extern "C" {
    pub fn Fl_Int_Input_show(arg1: *mut Fl_Int_Input);
}
extern "C" {
    pub fn Fl_Int_Input_hide(arg1: *mut Fl_Int_Input);
}
extern "C" {
    pub fn Fl_Int_Input_activate(arg1: *mut Fl_Int_Input);
}
extern "C" {
    pub fn Fl_Int_Input_deactivate(arg1: *mut Fl_Int_Input);
}
extern "C" {
    pub fn Fl_Int_Input_redraw_label(arg1: *mut Fl_Int_Input);
}
extern "C" {
    pub fn Fl_Int_Input_resize(
        arg1: *mut Fl_Int_Input,
        x: ::std::os::raw::c_int,
        y: ::std::os::raw::c_int,
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn Fl_Int_Input_tooltip(arg1: *mut Fl_Int_Input) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn Fl_Int_Input_set_tooltip(arg1: *mut Fl_Int_Input, txt: *const ::std::os::raw::c_char);
}
extern "C" {
    pub fn Fl_Int_Input_get_type(arg1: *mut Fl_Int_Input) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Int_Input_set_type(arg1: *mut Fl_Int_Input, typ: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Int_Input_color(arg1: *mut Fl_Int_Input) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Int_Input_set_color(arg1: *mut Fl_Int_Input, color: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Int_Input_label_color(arg1: *mut Fl_Int_Input) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Int_Input_set_label_color(arg1: *mut Fl_Int_Input, color: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Int_Input_label_font(arg1: *mut Fl_Int_Input) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Int_Input_set_label_font(arg1: *mut Fl_Int_Input, font: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Int_Input_label_size(arg1: *mut Fl_Int_Input) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Int_Input_set_label_size(arg1: *mut Fl_Int_Input, sz: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Int_Input_label_type(arg1: *mut Fl_Int_Input) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Int_Input_set_label_type(arg1: *mut Fl_Int_Input, typ: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Int_Input_box(arg1: *mut Fl_Int_Input) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Int_Input_set_box(arg1: *mut Fl_Int_Input, typ: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Int_Input_changed(arg1: *mut Fl_Int_Input) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Int_Input_set_changed(arg1: *mut Fl_Int_Input);
}
extern "C" {
    pub fn Fl_Int_Input_clear_changed(arg1: *mut Fl_Int_Input);
}
extern "C" {
    pub fn Fl_Int_Input_align(arg1: *mut Fl_Int_Input) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Int_Input_set_align(arg1: *mut Fl_Int_Input, typ: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Int_Input_delete(arg1: *mut Fl_Int_Input);
}
extern "C" {
    pub fn Fl_Int_Input_set_value(
        arg1: *mut Fl_Int_Input,
        arg2: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Int_Input_value(arg1: *mut Fl_Int_Input) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn Fl_Int_Input_maximum_size(arg1: *mut Fl_Int_Input) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Int_Input_set_maximum_size(arg1: *mut Fl_Int_Input, m: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Int_Input_position(arg1: *mut Fl_Int_Input) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Int_Input_set_position(
        arg1: *mut Fl_Int_Input,
        p: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Int_Input_set_mark(
        arg1: *mut Fl_Int_Input,
        m: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Int_Input_mark(arg1: *mut Fl_Int_Input) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Int_Input_replace(
        arg1: *mut Fl_Int_Input,
        b: ::std::os::raw::c_int,
        e: ::std::os::raw::c_int,
        text: *const ::std::os::raw::c_char,
        ilen: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Int_Input_insert(
        arg1: *mut Fl_Int_Input,
        t: *const ::std::os::raw::c_char,
        l: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Int_Input_append(
        arg1: *mut Fl_Int_Input,
        t: *const ::std::os::raw::c_char,
        l: ::std::os::raw::c_int,
        keep_selection: ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Int_Input_copy(
        arg1: *mut Fl_Int_Input,
        clipboard: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Int_Input_undo(arg1: *mut Fl_Int_Input) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Int_Input_copy_cuts(arg1: *mut Fl_Int_Input) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Int_Input_text_font(arg1: *mut Fl_Int_Input) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Int_Input_set_text_font(arg1: *mut Fl_Int_Input, s: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Int_Input_text_color(arg1: *mut Fl_Int_Input) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Int_Input_set_text_color(arg1: *mut Fl_Int_Input, s: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Int_Input_text_size(arg1: *mut Fl_Int_Input) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Int_Input_set_text_size(arg1: *mut Fl_Int_Input, s: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Int_Input_readonly(arg1: *mut Fl_Int_Input) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Int_Input_set_readonly(arg1: *mut Fl_Int_Input, boolean: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Int_Input_wrap(arg1: *mut Fl_Int_Input) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Int_Input_set_wrap(arg1: *mut Fl_Int_Input, boolean: ::std::os::raw::c_int);
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Fl_Float_Input {
    _unused: [u8; 0],
}
extern "C" {
    pub fn Fl_Float_Input_new(
        x: ::std::os::raw::c_int,
        y: ::std::os::raw::c_int,
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
        title: *const ::std::os::raw::c_char,
    ) -> *mut Fl_Float_Input;
}
extern "C" {
    pub fn Fl_Float_Input_set_label(
        arg1: *mut Fl_Float_Input,
        title: *const ::std::os::raw::c_char,
    );
}
extern "C" {
    pub fn Fl_Float_Input_redraw(arg1: *mut Fl_Float_Input);
}
extern "C" {
    pub fn Fl_Float_Input_show(arg1: *mut Fl_Float_Input);
}
extern "C" {
    pub fn Fl_Float_Input_hide(arg1: *mut Fl_Float_Input);
}
extern "C" {
    pub fn Fl_Float_Input_activate(arg1: *mut Fl_Float_Input);
}
extern "C" {
    pub fn Fl_Float_Input_deactivate(arg1: *mut Fl_Float_Input);
}
extern "C" {
    pub fn Fl_Float_Input_redraw_label(arg1: *mut Fl_Float_Input);
}
extern "C" {
    pub fn Fl_Float_Input_resize(
        arg1: *mut Fl_Float_Input,
        x: ::std::os::raw::c_int,
        y: ::std::os::raw::c_int,
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn Fl_Float_Input_tooltip(arg1: *mut Fl_Float_Input) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn Fl_Float_Input_set_tooltip(
        arg1: *mut Fl_Float_Input,
        txt: *const ::std::os::raw::c_char,
    );
}
extern "C" {
    pub fn Fl_Float_Input_get_type(arg1: *mut Fl_Float_Input) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Float_Input_set_type(arg1: *mut Fl_Float_Input, typ: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Float_Input_color(arg1: *mut Fl_Float_Input) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Float_Input_set_color(arg1: *mut Fl_Float_Input, color: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Float_Input_label_color(arg1: *mut Fl_Float_Input) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Float_Input_set_label_color(arg1: *mut Fl_Float_Input, color: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Float_Input_label_font(arg1: *mut Fl_Float_Input) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Float_Input_set_label_font(arg1: *mut Fl_Float_Input, font: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Float_Input_label_size(arg1: *mut Fl_Float_Input) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Float_Input_set_label_size(arg1: *mut Fl_Float_Input, sz: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Float_Input_label_type(arg1: *mut Fl_Float_Input) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Float_Input_set_label_type(arg1: *mut Fl_Float_Input, typ: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Float_Input_box(arg1: *mut Fl_Float_Input) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Float_Input_set_box(arg1: *mut Fl_Float_Input, typ: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Float_Input_changed(arg1: *mut Fl_Float_Input) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Float_Input_set_changed(arg1: *mut Fl_Float_Input);
}
extern "C" {
    pub fn Fl_Float_Input_clear_changed(arg1: *mut Fl_Float_Input);
}
extern "C" {
    pub fn Fl_Float_Input_align(arg1: *mut Fl_Float_Input) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Float_Input_set_align(arg1: *mut Fl_Float_Input, typ: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Float_Input_delete(arg1: *mut Fl_Float_Input);
}
extern "C" {
    pub fn Fl_Float_Input_set_value(
        arg1: *mut Fl_Float_Input,
        arg2: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Float_Input_value(arg1: *mut Fl_Float_Input) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn Fl_Float_Input_maximum_size(arg1: *mut Fl_Float_Input) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Float_Input_set_maximum_size(arg1: *mut Fl_Float_Input, m: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Float_Input_position(arg1: *mut Fl_Float_Input) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Float_Input_set_position(
        arg1: *mut Fl_Float_Input,
        p: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Float_Input_set_mark(
        arg1: *mut Fl_Float_Input,
        m: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Float_Input_mark(arg1: *mut Fl_Float_Input) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Float_Input_replace(
        arg1: *mut Fl_Float_Input,
        b: ::std::os::raw::c_int,
        e: ::std::os::raw::c_int,
        text: *const ::std::os::raw::c_char,
        ilen: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Float_Input_insert(
        arg1: *mut Fl_Float_Input,
        t: *const ::std::os::raw::c_char,
        l: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Float_Input_append(
        arg1: *mut Fl_Float_Input,
        t: *const ::std::os::raw::c_char,
        l: ::std::os::raw::c_int,
        keep_selection: ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Float_Input_copy(
        arg1: *mut Fl_Float_Input,
        clipboard: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Float_Input_undo(arg1: *mut Fl_Float_Input) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Float_Input_copy_cuts(arg1: *mut Fl_Float_Input) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Float_Input_text_font(arg1: *mut Fl_Float_Input) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Float_Input_set_text_font(arg1: *mut Fl_Float_Input, s: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Float_Input_text_color(arg1: *mut Fl_Float_Input) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Float_Input_set_text_color(arg1: *mut Fl_Float_Input, s: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Float_Input_text_size(arg1: *mut Fl_Float_Input) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Float_Input_set_text_size(arg1: *mut Fl_Float_Input, s: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Float_Input_readonly(arg1: *mut Fl_Float_Input) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Float_Input_set_readonly(arg1: *mut Fl_Float_Input, boolean: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Float_Input_wrap(arg1: *mut Fl_Float_Input) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Float_Input_set_wrap(arg1: *mut Fl_Float_Input, boolean: ::std::os::raw::c_int);
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Fl_Multiline_Input {
    _unused: [u8; 0],
}
extern "C" {
    pub fn Fl_Multiline_Input_new(
        x: ::std::os::raw::c_int,
        y: ::std::os::raw::c_int,
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
        title: *const ::std::os::raw::c_char,
    ) -> *mut Fl_Multiline_Input;
}
extern "C" {
    pub fn Fl_Multiline_Input_set_label(
        arg1: *mut Fl_Multiline_Input,
        title: *const ::std::os::raw::c_char,
    );
}
extern "C" {
    pub fn Fl_Multiline_Input_redraw(arg1: *mut Fl_Multiline_Input);
}
extern "C" {
    pub fn Fl_Multiline_Input_show(arg1: *mut Fl_Multiline_Input);
}
extern "C" {
    pub fn Fl_Multiline_Input_hide(arg1: *mut Fl_Multiline_Input);
}
extern "C" {
    pub fn Fl_Multiline_Input_activate(arg1: *mut Fl_Multiline_Input);
}
extern "C" {
    pub fn Fl_Multiline_Input_deactivate(arg1: *mut Fl_Multiline_Input);
}
extern "C" {
    pub fn Fl_Multiline_Input_redraw_label(arg1: *mut Fl_Multiline_Input);
}
extern "C" {
    pub fn Fl_Multiline_Input_resize(
        arg1: *mut Fl_Multiline_Input,
        x: ::std::os::raw::c_int,
        y: ::std::os::raw::c_int,
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn Fl_Multiline_Input_tooltip(
        arg1: *mut Fl_Multiline_Input,
    ) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn Fl_Multiline_Input_set_tooltip(
        arg1: *mut Fl_Multiline_Input,
        txt: *const ::std::os::raw::c_char,
    );
}
extern "C" {
    pub fn Fl_Multiline_Input_get_type(arg1: *mut Fl_Multiline_Input) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Multiline_Input_set_type(arg1: *mut Fl_Multiline_Input, typ: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Multiline_Input_color(arg1: *mut Fl_Multiline_Input) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Multiline_Input_set_color(
        arg1: *mut Fl_Multiline_Input,
        color: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn Fl_Multiline_Input_label_color(arg1: *mut Fl_Multiline_Input) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Multiline_Input_set_label_color(
        arg1: *mut Fl_Multiline_Input,
        color: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn Fl_Multiline_Input_label_font(arg1: *mut Fl_Multiline_Input) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Multiline_Input_set_label_font(
        arg1: *mut Fl_Multiline_Input,
        font: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn Fl_Multiline_Input_label_size(arg1: *mut Fl_Multiline_Input) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Multiline_Input_set_label_size(
        arg1: *mut Fl_Multiline_Input,
        sz: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn Fl_Multiline_Input_label_type(arg1: *mut Fl_Multiline_Input) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Multiline_Input_set_label_type(
        arg1: *mut Fl_Multiline_Input,
        typ: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn Fl_Multiline_Input_box(arg1: *mut Fl_Multiline_Input) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Multiline_Input_set_box(arg1: *mut Fl_Multiline_Input, typ: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Multiline_Input_changed(arg1: *mut Fl_Multiline_Input) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Multiline_Input_set_changed(arg1: *mut Fl_Multiline_Input);
}
extern "C" {
    pub fn Fl_Multiline_Input_clear_changed(arg1: *mut Fl_Multiline_Input);
}
extern "C" {
    pub fn Fl_Multiline_Input_align(arg1: *mut Fl_Multiline_Input) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Multiline_Input_set_align(arg1: *mut Fl_Multiline_Input, typ: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Multiline_Input_delete(arg1: *mut Fl_Multiline_Input);
}
extern "C" {
    pub fn Fl_Multiline_Input_set_value(
        arg1: *mut Fl_Multiline_Input,
        arg2: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Multiline_Input_value(arg1: *mut Fl_Multiline_Input)
        -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn Fl_Multiline_Input_maximum_size(arg1: *mut Fl_Multiline_Input) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Multiline_Input_set_maximum_size(
        arg1: *mut Fl_Multiline_Input,
        m: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn Fl_Multiline_Input_position(arg1: *mut Fl_Multiline_Input) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Multiline_Input_set_position(
        arg1: *mut Fl_Multiline_Input,
        p: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Multiline_Input_set_mark(
        arg1: *mut Fl_Multiline_Input,
        m: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Multiline_Input_mark(arg1: *mut Fl_Multiline_Input) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Multiline_Input_replace(
        arg1: *mut Fl_Multiline_Input,
        b: ::std::os::raw::c_int,
        e: ::std::os::raw::c_int,
        text: *const ::std::os::raw::c_char,
        ilen: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Multiline_Input_insert(
        arg1: *mut Fl_Multiline_Input,
        t: *const ::std::os::raw::c_char,
        l: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Multiline_Input_append(
        arg1: *mut Fl_Multiline_Input,
        t: *const ::std::os::raw::c_char,
        l: ::std::os::raw::c_int,
        keep_selection: ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Multiline_Input_copy(
        arg1: *mut Fl_Multiline_Input,
        clipboard: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Multiline_Input_undo(arg1: *mut Fl_Multiline_Input) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Multiline_Input_copy_cuts(arg1: *mut Fl_Multiline_Input) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Multiline_Input_text_font(arg1: *mut Fl_Multiline_Input) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Multiline_Input_set_text_font(
        arg1: *mut Fl_Multiline_Input,
        s: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn Fl_Multiline_Input_text_color(arg1: *mut Fl_Multiline_Input) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Multiline_Input_set_text_color(
        arg1: *mut Fl_Multiline_Input,
        s: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn Fl_Multiline_Input_text_size(arg1: *mut Fl_Multiline_Input) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Multiline_Input_set_text_size(
        arg1: *mut Fl_Multiline_Input,
        s: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn Fl_Multiline_Input_readonly(arg1: *mut Fl_Multiline_Input) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Multiline_Input_set_readonly(
        arg1: *mut Fl_Multiline_Input,
        boolean: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn Fl_Multiline_Input_wrap(arg1: *mut Fl_Multiline_Input) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Multiline_Input_set_wrap(
        arg1: *mut Fl_Multiline_Input,
        boolean: ::std::os::raw::c_int,
    );
}
