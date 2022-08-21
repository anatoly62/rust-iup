
extern crate libc;
mod iup;
use std::ptr;
use std::collections::HashMap;
use std::cell::RefCell;
use std::ffi::CString;
use iup::{
    IupSetGlobal,Ihandle,IupSetCallback,Icallback,IupSetHandle,IupGetHandle,IupShow,IupShowXY,IupPopup,IupDestroy,
    IupOpen,IupControlsOpen,IupMainLoop,IupExitLoop,IupHide,IupClose,IupGetDialogChild, IupSetAttributeHandle,IupSetFocus,
    IupGetAttribute,IupGetInt,IupGetIntInt,IupSetAttributeId2,IupSetInt,IupSetAttributes, IupSetStrAttribute,IupSetAttribute,
    IupCboxv,IupGridBoxv,IupHboxv,IupVboxv,IupZboxv,IupFill,IupRadio,IupFrame,IupExpander,IupSbox,IupSplit,IupScrollBox,IupTabsv,
    IupMessage,IupDialog,IupButton,IupFlatButton,IupLabel,IupMatrix,IupText,IupList,IupDatePick,IupToggle,IupVal,IupProgressBar,IupCalendar,IupSeparator,
    IupColorDlg,IupFileDlg,IupFontDlg,IupMessageDlg,IupProgressDlg,IupMenuv,IupSubmenu,IupItem,IupImage,IupImageLibOpen,IupLoadImage
};
const CURSOR_COLOR:&str="145 201 247";
pub type IUPPtr = *mut Ihandle;
pub type CBPtr = *const u32;

thread_local! {
    pub static BUTTONS_MAP__: RefCell<HashMap<IUPPtr,Box< dyn Fn()>>> = RefCell::new(HashMap::new());
    pub static COMBOS_MAP__: RefCell<HashMap<IUPPtr,Box< dyn Fn()>>> = RefCell::new(HashMap::new());
    pub static DATES_MAP__: RefCell<HashMap<IUPPtr,Box< dyn Fn()>>> = RefCell::new(HashMap::new());
    pub static FLAT_BUTTONS_MAP__: RefCell<HashMap<IUPPtr,Box< dyn Fn()>>> = RefCell::new(HashMap::new());
    pub static MENU_ITEMS_MAP__: RefCell<HashMap<IUPPtr,Box< dyn Fn()>>> = RefCell::new(HashMap::new());
    pub static TEXTS_MAP__: RefCell<HashMap<IUPPtr,Box< dyn Fn()>>> = RefCell::new(HashMap::new());
    pub static TEXTS_MAPK__: RefCell<HashMap<IUPPtr,Box< dyn Fn(i32)>>> = RefCell::new(HashMap::new());
    pub static TABLES_MAP__: RefCell<HashMap<IUPPtr,Box< dyn Fn(i32)>>> = RefCell::new(HashMap::new());
    pub static TABLES_MAP2__: RefCell<HashMap<IUPPtr,Box< dyn Fn(i32)>>> = RefCell::new(HashMap::new());
    pub static TABLES_MAPM__: RefCell<HashMap<IUPPtr,Box< dyn Fn(i32)>>> = RefCell::new(HashMap::new());
}

#[macro_export]
macro_rules! vec_ptr {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x.ptr());
            )*
            temp_vec
        }
    };
}

#[macro_export]
macro_rules! call_back {($n:ident,$b:block) => (extern fn $n(w:IUPPtr,_:CBPtr,_:CBPtr,_:CBPtr)->i32 {$b;0});}

pub fn p32_to_str(val:CBPtr) ->String{
    let a=val as * const u8;
    let mut v=Vec::new();
    let mut i=0;
    unsafe {
        while *a.offset(i)!=0  {
            v.push(*a.offset(i) as u8);
            i+=1;
        }
        String::from_utf8(v).unwrap()
    }
}
pub fn p8_to_str(val: * const i8)->String{
    let a=val as * const u8;
    let mut v=Vec::new();
    let mut i=0;
    unsafe {
        while *a.offset(i)!=0  {
            v.push(*a.offset(i) as u8);
            i+=1;
        }
        String::from_utf8(v).unwrap()
    }
}
#[derive(Copy,Clone)]
pub struct Document <'a>{
    s:&'a str ,
}
impl<'a> Document<'a> {
    pub fn elem<T: Into<String>>(self,nm: T, )->IUPPtr {
        let name = CString::new(nm.into()).unwrap();
        unsafe { IupGetHandle(name.as_ptr()) }
    }        
}

pub fn load(w:IUPPtr,s:&str){}

pub fn nil()->IUPPtr{ptr::null_mut()}
pub fn show(w:IUPPtr){unsafe {IupShow(w)};}
pub fn show_xy(w:IUPPtr,x:i32,y:i32){unsafe {IupShowXY(w,x,y)};}
pub fn popup(w:IUPPtr,x:i32,y:i32)->i32{unsafe {IupPopup(w,x,y)}}
pub fn hide(w:IUPPtr)->i32{unsafe{IupHide(w)}}
pub fn close(w:IUPPtr){unsafe{IupDestroy(w)}}
pub fn exit_loop(){unsafe{IupExitLoop()}}

pub fn init_gui<'a>()->Document <'a>{
    let mode = CString::new("UTF8MODE".to_string()).unwrap();
    let val = CString::new("YES".to_string()).unwrap();
    unsafe {
        IupOpen(ptr::null(), ptr::null());
        IupSetGlobal( mode.as_ptr(), val.as_ptr());
		IupControlsOpen();
    }
    Document {s:""}
}
pub fn set_global(a:&str,v:&str){
    unsafe {
        let a = CString::new(a.to_string()).unwrap();
        let v = CString::new(v.to_string()).unwrap();
        IupSetGlobal(a.as_ptr(), v.as_ptr());
        IupControlsOpen();
    }
}

pub fn loop_gui(){
    unsafe {
        IupMainLoop();
        IupClose() }
}
// pub fn message<T: Into<String>, M: Into<String>>(t: T, m: M) {
//     let title = CString::new(t.into()).unwrap();
//     let message = CString::new(m.into()).unwrap();
//     unsafe { IupMessage(title.as_ptr(), message.as_ptr()); }
// }

pub fn message(t: &str, m: &str) {
    let title = CString::new(t.to_string()).unwrap();
    let message = CString::new(m.to_string()).unwrap();
    unsafe { IupMessage(title.as_ptr(), message.as_ptr()); }
}

pub fn get_attr_str<T: Into<String>>(w:IUPPtr,a: T)->String {
    let attr = CString::new(a.into()).unwrap();
    unsafe {p8_to_str(IupGetAttribute(w, attr.as_ptr()))}
}
pub fn get_attr_int<T: Into<String>>(w:IUPPtr, a: T) -> i32 {
    let a = CString::new(a.into()).unwrap();
    unsafe {IupGetInt(w, a.as_ptr()) }
}
pub fn get_attr_int2<T: Into<String>>(w:IUPPtr, a: T) -> (i32, i32) {
    let mut f = 0;
    let mut s = 0;
    let a = CString::new(a.into()).unwrap();
    unsafe {IupGetIntInt(w, a.as_ptr(), &mut f, &mut s);}
    (f, s)
}
pub fn set_attr<T: Into<String>,V:Into<String>>(w:IUPPtr, a: T, v: V) {
    let a = CString::new(a.into()).unwrap();
    let v = CString::new(v.into()).unwrap();
    unsafe {IupSetAttribute(w,a.as_ptr(), v.as_ptr());}
}
pub fn set_attr_handle<T: Into<String>>(w:IUPPtr, a: T, v: IUPPtr) {
    let a = CString::new(a.into()).unwrap();
    unsafe {IupSetAttributeHandle(w,a.as_ptr(), v);}
}
pub fn set_attrs<T: Into<String>>(w:IUPPtr, a: T) {
    let attr = CString::new(a.into()).unwrap();
    unsafe {IupSetAttributes(w,attr.as_ptr());}
}
pub fn set_attr_str<T: Into<String>,V:Into<String>>(w:IUPPtr, a: T, v: V) {
    let a = CString::new(a.into()).unwrap();
    let v = CString::new(v.into()).unwrap();
    unsafe {IupSetStrAttribute(w,a.as_ptr(), v.as_ptr());}
}
pub fn set_attr_int<T: Into<String>>(w:IUPPtr, a: T, v:i32 ) {
    let a = CString::new(a.into()).unwrap();
    unsafe {IupSetInt(w,a.as_ptr(), v);}
}
pub fn get_handle<T: Into<String>>(nm: T, )->IUPPtr {
    let name = CString::new(nm.into()).unwrap();
    unsafe { IupGetHandle(name.as_ptr()) }
}
pub fn set_handle<T: Into<String>>(nm: T,w:IUPPtr)->IUPPtr {
    let name = CString::new(nm.into()).unwrap();
    unsafe { IupSetHandle(name.as_ptr(), w) }
}
pub fn load_image<T: Into<String>>( nm: T)->IUPPtr {
    let name = CString::new(nm.into()).unwrap();
    unsafe { IupLoadImage(name.as_ptr()) }
}
pub fn child_by_name<T: Into<String>>(w:IUPPtr,nm:T )->IUPPtr {
    let name = CString::new(nm.into()).unwrap();
    unsafe { IupGetDialogChild(w,name.as_ptr()) }
}
pub fn call_back(w:IUPPtr,s:&str,f: Icallback){
    unsafe {IupSetCallback(w, CString::new(s.to_string()).unwrap().as_ptr(),f)};
}

//Predefined Dialogs
#[derive(Copy,Clone)]
pub struct FileDlg{w:IUPPtr }
impl FileDlg{
    pub fn new()->FileDlg{FileDlg{ w:{unsafe{IupFileDlg()}}}}
    pub fn ptr(self)->IUPPtr{ self.w}
    pub fn get_file(self)->String{get_attr_str(self.w,"VALUE")}
    pub fn mode(self,s:&str)->Self{
        set_attr_str(self.w, "DIALOGTYPE", s);
        self
    }
    pub fn filter(self,s:&str)->Self{
        set_attr_str(self.w, "EXTFILTER", s);
        self
    }
    pub fn start_dir(self,s:&str)->Self{
        set_attr_str(self.w, "DIRECTORY", s);
        self
    }
    pub fn hiden(self,v:bool)->Self{
        set_attr_str(self.w, "SHOWHIDDEN", if v==true {"YES"} else {"NO "});
        self
    }
    pub fn multi(self,v:bool)->Self{
        set_attr_str(self.w, "MULTIPLEFILES", if v==true {"YES"} else {"NO "});
        self
    }
    pub fn preview(self,v:bool)->Self{
        set_attr_str(self.w, "SHOWPREVIEW", if v==true {"YES"} else {"NO "});
        self
    }
    pub fn on_process(self,f: Icallback)->Self{
        call_back(self.w,"FILE_CB",f);
        self
    }
}

#[derive(Copy,Clone)]
pub struct FontDlg{w:IUPPtr }
impl FontDlg{
    pub fn new()->FontDlg{FontDlg{ w:{unsafe{IupFontDlg()}}}}
    pub fn ptr(self)->IUPPtr{ self.w}
    pub fn get_font(self)->String{get_attr_str(self.w,"VALUE")}
}

#[derive(Copy,Clone)]
pub struct ColorDlg{w:IUPPtr }
impl ColorDlg{
    pub fn new()->ColorDlg{ColorDlg{ w:{unsafe{IupColorDlg()}}}}
    pub fn ptr(self)->IUPPtr{ self.w}
    pub fn get_color(self)->String{get_attr_str(self.w,"VALUE")}
}

#[derive(Copy,Clone)]
pub struct MessageDlg{w:IUPPtr }
impl MessageDlg {
    pub fn new() -> MessageDlg { MessageDlg { w: { unsafe { IupMessageDlg() } } } }
    pub fn ptr(self)->IUPPtr{ self.w}
    pub fn mode(self,s:&str)->Self{
        set_attr_str(self.w, "DIALOGTYPE", s);
        self
    }
    pub fn buttons(self,s:&str)->Self{
        set_attr_str(self.w, "BUTTONS", s);
        self
    }
    pub fn title(self,s:&str)->Self{
        set_attr_str(self.w, "TITLE", s);
        self
    }
    pub fn text(self,s:&str)->Self{
        set_attr_str(self.w, "VALUE", s);
        self
    }
    pub fn result(self)->String{ get_attr_str(self.w, "BUTTONRESPONSE") }
}

#[derive(Copy,Clone)]
pub struct ProgressDlg{w:IUPPtr }
impl ProgressDlg {
    pub fn new() -> ProgressDlg { ProgressDlg { w: { unsafe { IupProgressDlg() } } } }
    pub fn ptr(self) -> IUPPtr { self.w }
    pub fn on_cancel(self,f: Icallback)->Self{
        call_back(self.w,"CANCEL_CB",f);
        self
    }
    pub fn get_count(self)->i32{get_attr_int(self.w, "COUNT")}
    pub fn set_count(self,n:i32)->Self{
        set_attr_int(self.w, "COUNT", n);
        self
    }
    pub fn get_total(self)->i32{get_attr_int(self.w, "TOTALCOUNT")}
    pub fn set_total(self,n:i32)->Self{
        set_attr_int(self.w, "TOTALCOUNT", n);
        self
    }
    pub fn title(self,s:&str)->Self{
        set_attr_str(self.w, "TITLE", s);
        self
    }
    pub fn inc(self,n:i32)->Self{
        set_attr_int(self.w, "INC", n);
        self
    }
}
//Image
pub  struct Image{pub w: IUPPtr}
impl Image {
    pub fn new(width:i32,height:i32,pixels: *const u8 ) -> Image { Image { w: { unsafe { IupImage(width,height,pixels)}}}}
    pub fn from(p: IUPPtr) -> Image { Image { w: p }}
    fn set_height(self,n:i32)->Self{
        set_attr_int(self.w,"HEIGHT",n);
        self
    }
    fn set_width(self,n:i32)->Self{
        set_attr_int(self.w,"WIDTH",n);
        self
    }
    pub fn handle(self,s:&str)->Self{
        set_handle(s,self.w);
        self
    }
    fn autoscale(self,v:bool)->Self{
        set_attr_str(self.w,"AUTOSCALE",if v==true {"YES"} else {"NO"});
        self
    }
}

pub trait Control:Copy{
    fn ptr(self)->IUPPtr;

    fn is_enabled(self)->bool{
        if get_attr_str(self.ptr(),"ACTIVE")=="YES" {true} else {false}
    }
    fn set_enable(self,v:bool)->Self{
        set_attr_str(self.ptr(),"ACTIVE",if v==true {"YES"} else {"NO"});
        self
    }
    fn is_visible(self)->bool{
        if get_attr_str(self.ptr(),"VISIBLE")=="YES" {true} else {false}
    }
    fn set_visible(self,v:bool)->Self{
        set_attr_str(self.ptr(),"VISIBLE",if v==true {"YES"} else {"NO"});
        self
    }   
    fn font(self,s:&str)->Self{
        set_attr_str(self.ptr(), "FONT",s);
        self
    }
    fn handle(self,s:&str)->Self{
        set_handle(s,self.ptr());
        self
    }
    fn name(self,s:&str)->Self{
        set_attr_str(self.ptr(), "NAME",s);
        self
    }
}

pub trait Widget: Control {
    fn set_size(self, s: &str)->Self {
        set_attr_str(self.ptr(),"SIZE",s);
        self
    }    
     fn get_width(self)->i32 {
        let (x,y)=get_attr_int2(self.ptr(),"RASTERSIZE");
        x
    }
    fn set_width(self, v: i32)->Self {
        let (x,y)=get_attr_int2(self.ptr(),"RASTERSIZE");
        set_attr_str(self.ptr(),"RASTERSIZE",v.to_string()+"x"+&y.to_string());
        self
    }
    fn get_height(self)->i32 {
        let (x,y)=get_attr_int2(self.ptr(),"RASTERSIZE");
        y
    }
    fn set_height(self, v: i32)->Self {
        let (x,y)=get_attr_int2(self.ptr(),"RASTERSIZE");
        set_attr_str(self.ptr(),"RASTERSIZE",x.to_string()+"x"+&v.to_string());
        self
    }
    fn get_x(self)->i32 {
        get_attr_int(self.ptr(),"CX")
    }
    fn set_x(self, v: i32)->Self {
        set_attr_int(self.ptr(),"CX",v);
        self
    }
    fn get_y(self)->i32 {
        get_attr_int(self.ptr(),"CY")
    }
    fn set_y(self, v: i32)->Self {
        set_attr_int(self.ptr(),"CY",v);
        self
    }
    fn expand(self,s:&str)->Self{
        set_attr_str(self.ptr(), "EXPAND",s);
        self
    }
    fn bg_color(self,s:&str)->Self{
        set_attr_str(self.ptr(), "BGCOLOR",s);
        self
    }
    fn fg_color(self,s:&str)->Self{
        set_attr_str(self.ptr(), "FGCOLOR",s);
        self
    }
    fn set_focus(self)->Self{
        unsafe{IupSetFocus(self.ptr())};
        self
    }
}

//Containers
#[derive(Copy,Clone)]
pub struct Abox{w:IUPPtr }
impl Abox{
    pub fn new(mut v:Vec<IUPPtr>)->Abox{Abox{ w:{v.push(nil());unsafe{IupCboxv(v.as_mut_ptr())}}}}
}
impl Control for Abox{ fn ptr(self) ->IUPPtr{ self.w}}

#[derive(Copy,Clone)]
pub struct Fill{w:IUPPtr }
impl Fill{
    pub fn new()->Fill{Fill{ w:{unsafe{IupFill()}}}}
}
impl Control for Fill{ fn ptr(self) ->IUPPtr{ self.w}}

#[derive(Copy,Clone)]
pub struct Gbox{w:IUPPtr }
impl Gbox{
    pub fn new(mut v:Vec<IUPPtr>)->Gbox{Gbox{ w:{v.push(nil());unsafe{IupGridBoxv(v.as_mut_ptr())}}}}
    pub fn count(self,v:i32)->Self{
        set_attr_int(self.w, "NUMDIV", v);
        self
    }
    pub fn vertical(self,v:bool)->Self{
        set_attr_str(self.w, "ORIENTATION", if v==true {"VERTICAL"} else {"HORIZONTAL "});
        self
    }
    fn expand_child(self,v:bool)->Self{
        set_attr_str(self.w, "EXPANDCHILDREN", if v==true {"YES"} else {"NO"});
        self
    }
    pub fn gap_col(self, n: i32) -> Self {
        set_attr_int(self.w, "GAPCOL", n);
        self
    }
    pub fn gap_lin(self, n: i32) -> Self {
        set_attr_int(self.w, "GAPLIN", n);
        self
    }

}
impl Control for Gbox{fn ptr(self) ->IUPPtr{ self.w}}

#[derive(Copy,Clone)]
pub struct Hbox{w:IUPPtr }
impl Hbox{
    pub fn new(mut v:Vec<IUPPtr>)->Hbox{Hbox{ w:{v.push(nil());unsafe{IupHboxv(v.as_mut_ptr())}}}}
    pub fn expand_child(self,v:bool)->Self{
        set_attr_str(self.w, "EXPANDCHILDREN", if v==true {"YES"} else {"NO"});
        self
    }
    pub fn gap(self, n: i32) -> Self {
        set_attr_int(self.w, "GAP", n);
        self
    }
    pub fn margin(self, s: &str) -> Self {
        set_attr_str(self.w, "MARGIN", s);
        self
    }
}
impl Control for Hbox{fn ptr(self) ->IUPPtr{ self.w}}

#[derive(Copy,Clone)]
pub struct Vbox{w:IUPPtr }
impl Vbox{
    pub fn new(mut v:Vec<IUPPtr>)->Vbox{Vbox{ w:{v.push(nil());unsafe{IupVboxv(v.as_mut_ptr())}}}}
    pub fn expand_child(self,v:bool)->Self{
        set_attr_str(self.w, "EXPANDCHILDREN", if v==true {"YES"} else {"NO"});
        self
    }
    pub fn gap(self, n: i32) -> Self {
        set_attr_int(self.w, "GAP", n);
        self
    }
    pub fn margin(self, s: &str) -> Self {
        set_attr_str(self.w, "MARGIN", s);
        self
    }
}
impl Control for Vbox{fn ptr(self) ->IUPPtr{ self.w}}

#[derive(Copy,Clone)]
pub struct TabBox{w:IUPPtr }
impl TabBox{
    pub fn new(mut v:Vec<IUPPtr>)->TabBox{TabBox{ w:{v.push(nil());unsafe{IupTabsv(v.as_mut_ptr())}}}}
    pub fn get_tab(self) -> i32 {
        get_attr_int(self.w, "VALUEPOS")
    }
    pub fn set_tab(self, n: i32) -> Self {
        set_attr_int(self.w, "VALUEPOS", n);
        self
    }
    pub fn set_text(self, n: i32, s:&str) -> Self {
        set_attr_str(self.w, "TABTITLE".to_string()+&n.to_string(), s);
        self
    }
    pub fn on_change(self,f: Icallback)->Self{
        call_back(self.w,"TABCHANGEPOS_CB",f);
        self
    }
}
impl Control for TabBox{fn ptr(self) ->IUPPtr{ self.w}}

#[derive(Copy,Clone)]
pub struct Zbox{w:IUPPtr }
impl Zbox{
    pub fn new(mut v:Vec<IUPPtr>)->Zbox{Zbox{ w:{v.push(nil());unsafe{IupZboxv(v.as_mut_ptr())}}}}
    pub fn get_child_pos(self) -> i32 {
        get_attr_int(self.w, "VALUEPOS")
    }
    pub fn set_child_pos(self, n: i32) -> Self {
        set_attr_int(self.w, "VALUEPOS", n);
        self
    }
    pub fn get_child_name(self) -> String {
        get_attr_str(self.w, "VALUE")
    }
    pub fn set_child_name(self, s: &str) -> Self {
        set_attr_str(self.w, "VALUE", s);
        self
    }
}
impl Control for Zbox{fn ptr(self) ->IUPPtr{ self.w}}

#[derive(Copy,Clone)]
pub struct Expander{w:IUPPtr}
impl Expander{
    pub fn new(mut val:IUPPtr)->Expander{Expander{ w:{unsafe{IupExpander(val)}}}}
    pub fn get_text(self) -> String {
        get_attr_str(self.w, "TITLE")
    }
    pub fn set_text(self, s: &str) -> Self {
        set_attr_str(self.w, "TITLE", s);
        self
    }
    pub fn get_open(self)->bool{
        if get_attr_str(self.ptr(),"STATE")=="OPEN" {true} else {false}
    }
    pub fn set_open(self,v:bool)->Self{
        set_attr_str(self.ptr(),"STATE",if v==true {"OPEN"} else {"CLOSE"});
        self
    }
    pub fn on_change(self,f: Icallback)->Self{
        call_back(self.w,"ACTION",f);
        self
    }
}
impl Control for Expander{fn ptr(self) ->IUPPtr{ self.w}}

#[derive(Copy,Clone)]
pub struct Panel{w:IUPPtr}
impl Panel{
    pub fn new(mut val:IUPPtr)->Panel{Panel{ w:{unsafe{IupFrame(val)}}}}
    pub fn get_text(self) -> String {
        get_attr_str(self.w, "TITLE")
    }
    pub fn set_text(self, s: &str) -> Self {
        set_attr_str(self.w, "TITLE", s);
        self
    }
}
impl Control for Panel{fn ptr(self) ->IUPPtr{ self.w}}
impl Widget for Panel {}

#[derive(Copy,Clone)]
pub struct RadioBox{w:IUPPtr}
impl RadioBox{
    pub fn new(mut val:IUPPtr)->RadioBox{RadioBox{ w:{unsafe{IupRadio(val)}}}}
}
impl Control for RadioBox{fn ptr(self) ->IUPPtr{ self.w}}

#[derive(Copy,Clone)]
pub struct Resizer{w:IUPPtr}
impl Resizer{
    pub fn new(mut val:IUPPtr)->Resizer{Resizer{ w:{unsafe{IupSbox(val)}}}}
}
impl Control for Resizer{fn ptr(self) ->IUPPtr{ self.w}}

#[derive(Copy,Clone)]
pub struct ScrollBox{w:IUPPtr}
impl ScrollBox{
    pub fn new(mut val:IUPPtr)->ScrollBox{ScrollBox{ w:{unsafe{IupScrollBox(val)}}}}
}
impl Control for ScrollBox{fn ptr(self) ->IUPPtr{ self.w}}

#[derive(Copy,Clone)]
pub struct Spliter{w:IUPPtr}
impl Spliter{
    pub fn new(mut val1:IUPPtr,mut val2:IUPPtr)->Spliter{Spliter{ w:{unsafe{IupSplit(val1,val2)}}}}
}
impl Control for Spliter{fn ptr(self) ->IUPPtr{ self.w}}

//Munus
#[derive(Copy,Clone)]
pub struct Menu{ w: IUPPtr}
impl Menu {
    pub fn new(mut v:Vec<IUPPtr>)->Menu{Menu{ w:{v.push(nil());unsafe{IupMenuv(v.as_mut_ptr())}}}}
    pub fn set_radio(self,v:bool)->Self{
        set_attr_str(self.ptr(),"RADIO",if v==true {"ON"} else {"OF"});
        self
    }
}
impl Control for Menu{ fn ptr(self) ->IUPPtr{ self.w}}

#[derive(Copy,Clone)]
pub struct SubMenu{ w: IUPPtr}
impl SubMenu {
    pub fn new(s:&str,v:IUPPtr)->SubMenu{SubMenu{w:{unsafe{IupSubmenu(CString::new(s.to_string()).unwrap().as_ptr(),v)}}}}
}
impl Control for SubMenu{ fn ptr(self) ->IUPPtr{ self.w}}

extern fn _menu_item_click_(w:IUPPtr,_:i32,_:CBPtr,sp:CBPtr)->i32{
    MENU_ITEMS_MAP__.with(|c| {
        let  v = c.borrow();
        if let Some(x)=v.get(&w){
            x();
        }
    });
    0
}

#[derive(Copy,Clone)]
pub struct MenuItem{ w: IUPPtr}
impl MenuItem {
    pub fn new(s:&str)->Self{
        let p=unsafe{if s=="" {IupSeparator()} else {IupItem(CString::new(s.to_string()).unwrap().as_ptr(),ptr::null_mut())}};
        call_back(p,"ACTION",_menu_item_click_);
        Self{w:p}
    }

    pub fn set_text(self,s:&str)->Self{
        set_attr_str(self.w,"TITLE ",s);
        self
    }
    fn is_checked(self)->bool{
        if get_attr_str(self.ptr(),"VALUE")=="ON" {true} else {false}
    }

    pub fn set_check(self,v:bool)->Self{
        set_attr_str(self.ptr(),"VALUE",if v==true {"ON"} else {"OF"});
        self
    }
    pub fn set_autotoggle(self,v:bool)->Self{
        set_attr_str(self.ptr(),"AUTOTOGGLE",if v==true {"YES"} else {"NO"});
        self
    }

    pub fn set_image(self,s:&str)->Self{
        set_attr_str(self.w,"TITLEIMAGE",s);
        self
    }
    pub fn set_uncheck_image(self,s:&str)->Self{
        set_attr_str(self.w,"IMAGE",s);
        self
    }

    pub fn set_check_image(self,s:&str)->Self{
        set_attr_str(self.w,"IMPRESS",s);
        self
    }
    pub fn on_click(self, f:Box< dyn Fn()>)->Self{
        MENU_ITEMS_MAP__.with(|c| {
            let mut v = c.borrow_mut();
            v.insert(self.w, Box::new(f));
        });
        self
    }
}
impl Control for MenuItem{ fn ptr(self) ->IUPPtr{ self.w}}

//Widgets

#[derive(Copy,Clone)]
pub  struct AnyWidget{ w: IUPPtr}
impl AnyWidget {pub fn new(p:IUPPtr) -> AnyWidget { AnyWidget{w: p}}}
impl Control for AnyWidget{ fn ptr(self) ->IUPPtr{ self.w}}
impl Widget for AnyWidget {}


extern fn _button_click_(w:IUPPtr,_:i32,_:CBPtr,sp:CBPtr)->i32{
    BUTTONS_MAP__.with(|c| {
        let  v = c.borrow();
        if let Some(x)=v.get(&w){
            x();
        }
    });
    0
}

#[derive(Copy,Clone)]
pub  struct Button{ w: IUPPtr}
impl Button {
    pub fn new<T: Into<String>>(text: T) -> Button {
        let p={ unsafe { IupButton(CString::new(text.into()).unwrap().as_ptr(), ptr::null_mut()) } };
        call_back(p,"ACTION",_button_click_);
        Button { w:p  }
    }
    pub fn from(p: IUPPtr) -> Button { Button { w: p }}
    pub fn get_text(self)->String{
        get_attr_str(self.w,"TITLE")
    }
    pub fn set_align(self,s:&str)->Self{
        set_attr_str(self.w,"ALIGNMENT",s);
        self
    }
    pub fn set_text(self,s:&str)->Self{
        set_attr_str(self.w,"TITLE",s);
        self
    }
    pub fn set_image(self,s:&str)->Self{
        set_attr_str(self.w,"IMAGE",s);
        self
    }
    pub fn on_click(self, f:Box< dyn Fn()>)->Self{
        BUTTONS_MAP__.with(|c| {
            let mut v = c.borrow_mut();
            v.insert(self.w, Box::new(f));
        });
        self
    }
}
impl Control for Button{ fn ptr(self) ->IUPPtr{ self.w}}
impl Widget for Button {}

extern fn _flat_button_click_(w:IUPPtr,_:i32,_:CBPtr,sp:CBPtr)->i32{
    FLAT_BUTTONS_MAP__.with(|c| {
        let  v = c.borrow();
        if let Some(x)=v.get(&w){
            x();
        }
    });
    0
}

#[derive(Copy,Clone)]
pub  struct FlatButton{ w: IUPPtr}
impl FlatButton {
    pub fn new(image:&str,hint:&str) -> FlatButton {
        let p={ unsafe { IupFlatButton(CString::new("").unwrap().as_ptr(), ptr::null_mut()) } };
        set_attr_str(p,"TIP",hint);
        call_back(p,"FLAT_ACTION",_flat_button_click_);
        set_attr_handle(p,"IMAGE",load_image(image));
        FlatButton { w:p  }
    }
    pub fn from(p: IUPPtr) -> FlatButton { FlatButton { w: p }}
    pub fn on_click(self, f:Box< dyn Fn()>)->Self{
        FLAT_BUTTONS_MAP__.with(|c| {
            let mut v = c.borrow_mut();
            v.insert(self.w, Box::new(f));
        });
        self
    }
}
impl Control for FlatButton{ fn ptr(self) ->IUPPtr{ self.w}}
impl Widget for FlatButton {}

#[derive(Copy,Clone)]
pub  struct Calendar{ w: IUPPtr}
impl Calendar {
    pub fn new() -> Calendar { Calendar{w: { unsafe { IupCalendar()}}}}
    pub fn from(p: IUPPtr) -> Calendar { Calendar { w: p }}
    pub fn get_date(self)->String{
        get_attr_str(self.w,"VALUE")
    }
    pub fn set_date(self,s:&str)->Self{
        set_attr_str(self.w,"VALUE",s);
        self
    }
    pub fn on_change(self,f: Icallback)->Self{
        call_back(self.w,"VALUECHANGED_CB",f);
        self
    }
}
impl Control for Calendar{ fn ptr(self) ->IUPPtr{ self.w}}
impl Widget for Calendar {}

extern fn _date_change_(w:IUPPtr,_:i32,_:CBPtr,sp:CBPtr)->i32{
    DATES_MAP__.with(|c| {
        let  v = c.borrow();
        if let Some(x)=v.get(&w){
            x();
        }
    });
    0
}

#[derive(Copy,Clone)]
pub  struct DatePick{ w: IUPPtr}
impl DatePick {
    pub fn new() -> DatePick {
        let p={ unsafe { IupDatePick()}};
        set_attrs(p,"SEPARATOR=-,ZEROPRECED=yes,SIZE=x11");
        call_back(p,"VALUECHANGED_CB",_date_change_);
        DatePick{w: p}
    }
    pub fn from(p: IUPPtr) -> DatePick { DatePick { w: p }}
    pub fn get_text(self)->String{
        let s=get_attr_str(self.w,"VALUE");
        let mut v:Vec<&str>=s.split('/').collect();
        let s1="0".to_string()+v[1];
        let s2="0".to_string()+v[2];
        if v[1].len()<2 {
            v[1]=&s1;
        }
        if v[2].len()<2 {
            v[2]=&s2;
        }
        v[0].to_string()+"-"+v[1]+"-"+v[2]
    }
    pub fn set_text(self,s:&str)->Self{
        set_attr_str(self.w,"VALUE",s.replace("-","/"));
        self
    }
    pub fn on_change(self, f:Box< dyn Fn()>)->Self{
        DATES_MAP__.with(|c| {
            let mut v = c.borrow_mut();
            v.insert(self.w, Box::new(f));
        });
        self
    }
}
impl Control for DatePick{ fn ptr(self) ->IUPPtr{ self.w}}
impl Widget for DatePick {}

#[derive(Copy,Clone)]
pub  struct Dialog{ w: IUPPtr}
impl Dialog {
    pub fn new(w: IUPPtr) -> Dialog {Dialog {w: { unsafe { IupDialog(w)}}}}
    pub fn from(p:IUPPtr) -> Dialog { Dialog { w: p }}
    pub fn get_text(self)->String{
        get_attr_str(self.w,"TITLE")
    }
    pub fn set_text(self,s:&str)->Self{
        set_attr_str(self.w,"TITLE",s);
        self
    }
    pub fn set_image(self,s:&str)->Self{
        set_attr_handle(self.w,"ICON",load_image(s));
        self
    }
    pub fn menu(self,s:&str)->Self{
        set_attr_str(self.w,"MENU",s);
        self
    }
    pub fn get_child(self,s:&str)->IUPPtr{
        child_by_name(self.w,s)
    }
    pub fn max_button(self,v:bool)->Self{
        set_attr_str(self.ptr(),"MAXBOX",if v==true {"ON"} else {"OF"});
        self
    }
    pub fn min_button(self,v:bool)->Self{
        set_attr_str(self.ptr(),"MINBOX",if v==true {"ON"} else {"OF"});
        self
    }
    pub fn resize(self,v:bool)->Self{
        set_attr_str(self.ptr(),"RESIZE",if v==true {"ON"} else {"OF"});
        self
    }
    pub fn close(self){
        unsafe{IupDestroy(self.ptr())};
    }
}
impl Control for Dialog{ fn ptr(self) ->IUPPtr{ self.w}}
impl Widget for Dialog {}


#[derive(Copy,Clone)]
pub  struct Label{ w: IUPPtr}
impl Label {
    pub fn new<T: Into<String>>(text:T) -> Label {Label{w: { unsafe { IupLabel(CString::new(text.into()).unwrap().as_ptr())}}}}
    pub fn from(p: IUPPtr) -> Label { Label { w: p }}
    pub fn get_text(self)->String{
        get_attr_str(self.w,"TITLE")
    }
    pub fn set_align(self,s:&str)->Self{
        set_attr_str(self.w,"ALIGNMENT",s);
        self
    }
    pub fn set_text(self,s:&str)->Self{
        set_attr_str(self.w,"TITLE",s);
        self
    }
    pub fn set_image(self,s:&str)->Self{
        set_attr_str(self.w,"IMAGE",s);
        self
    }

}
impl Control for Label{ fn ptr(self) ->IUPPtr{ self.w}}
impl Widget for Label {}

extern fn _combo_change_(w:IUPPtr,_:i32,_:CBPtr,sp:CBPtr)->i32{
    COMBOS_MAP__.with(|c| {
        let  v = c.borrow();
        if let Some(x)=v.get(&w){
            x();
        }
    });
    0
}

#[derive(Copy,Clone)]
pub  struct Combo{ w: IUPPtr}
impl Combo {
    pub fn new() -> Combo {
        let p= { unsafe { IupList(ptr::null_mut())}};
        set_attr_str(p,"DROPDOWN","YES");
        call_back(p,"VALUECHANGED_CB",_combo_change_);
        Combo{w:p}
    }
    pub fn from(p: IUPPtr) -> Combo { Combo { w: p }}
    pub fn add(self,val:&str)->Self{
        set_attr_str(self.w,"APPENDITEM",val);
        self
    }
    pub fn count(self)->i32{
        get_attr_int(self.w,"COUNT")
    }
    pub fn dropdown(self,val:bool)->Self{
        set_attr_str(self.w,"DROPDOWN",if val==true {"YES"} else {"NO"});
        self
    }
    pub fn fill (self,v:Vec<String>)->Self{
        for (i,el) in v.iter().enumerate(){
            set_attr_str(self.w,(i+1).to_string(),el);
        }
        self.set_index(0);
        self
    }
    pub fn get_index(self)->i32{
        get_attr_int(self.w,"VALUE")-1
    }
    pub fn set_index(self,n:i32)->Self{
        set_attr_int(self.w,"VALUE",n+1);
        self
    }
    pub fn get_text(self)->String{
        get_attr_str(self.w,"VALUESTRING")
    }
    pub fn set_text(self,s:&str)->Self{
        set_attr_str(self.w,"VALUESTRING",s);
        self
    }
    pub fn show_image(self,v:bool)->Self{
        set_attr_str(self.w,"SHOWIMAGE",if v==true {"YES"} else {"NO"});
        self
    }
    pub fn set_image(self,n:u32,s:&str)->Self{
        set_attr_str(self.w,"IMAGE".to_string()+&n.to_string(),s);
        self
    }
    pub fn open(self,v:bool)->Self{
        set_attr_str(self.w,"SHOWDROPDOWN",if v==true {"YES"} else {"NO"});
        self
    }

    pub fn on_change(self, f:Box< dyn Fn()>)->Self{
        COMBOS_MAP__.with(|c| {
            let mut v = c.borrow_mut();
            v.insert(self.w, Box::new(f));
        });
        self
    }
    pub fn search(self,s:&str,n:usize){
        let ss:Vec<&str>=s.split(' ').collect();
        let l:i32=get_attr_str(self.w,"COUNT").parse().unwrap();
        for i in 0..l{
            let value=get_attr_str(self.w,&(i+1).to_string());
            let item=value.to_lowercase();
            let names:Vec<&str>=item.split(' ').collect();
            if names.len()==n+1 {
                if names[n].starts_with(s){
                    self.set_index(i);
                    break;
                }
            }else if ss.len()>1{
                if names[n].starts_with(ss[0]) && names[n+1].starts_with(ss[1]){
                    self.set_index(i);
                    break;
                }
            }else if s.len()>0 && ss.len()==1{
                if s.starts_with(" ") && names[n+1].starts_with(ss[0]){
                    self.set_index(i);
                    break;
                }else if !s.starts_with(" ") && names[n].starts_with(ss[0]){
                    self.set_index(i);
                    break;
                }
            }
        }
    }
}
impl Control for Combo{ fn ptr(self) ->IUPPtr{ self.w}}
impl Widget for Combo {}

#[derive(Copy,Clone)]
pub  struct ProgressBar{ w: IUPPtr}
impl ProgressBar {
    pub fn new() -> ProgressBar { ProgressBar { w: { unsafe { IupProgressBar() } } } }
    pub fn from(p: IUPPtr) -> ProgressBar { ProgressBar { w: p } }
    pub fn get_min(self) -> String {
        get_attr_str(self.w, "MIN")
    }
    pub fn set_min(self,s:&str)->Self{
        set_attr_str(self.w,"MIN",s);
        self
    }
    pub fn get_max(self) -> String {
        get_attr_str(self.w, "MAX")
    }
    pub fn set_max(self,s:&str)->Self{
        set_attr_str(self.w,"MAX",s);
        self
    }
    pub fn get_value(self) -> String {
        get_attr_str(self.w, "VALUE")
    }
    pub fn set_value(self,s:&str)->Self{
        set_attr_str(self.w,"VALUE",s);
        self
    }
}
impl Control for ProgressBar{ fn ptr(self) ->IUPPtr{ self.w}}
impl Widget for ProgressBar {}

#[derive(Copy,Clone)]
pub  struct Slider{ w: IUPPtr}
impl Slider {
    pub fn new<T: Into<String>>(text:T) -> Slider {Slider{w: { unsafe { IupVal(CString::new(text.into()).unwrap().as_ptr())}}}}
    pub fn from(p: IUPPtr) -> Slider { Slider { w: p } }
    pub fn get_min(self) -> String {
        get_attr_str(self.w, "MIN")
    }
    pub fn set_min(self,s:&str)->Self{
        set_attr_str(self.w,"MIN",s);
        self
    }
    pub fn get_max(self) -> String {
        get_attr_str(self.w, "MAX")
    }
    pub fn set_max(self,s:&str)->Self{
        set_attr_str(self.w,"MAX",s);
        self
    }
    pub fn get_step(self) -> String {
        get_attr_str(self.w, "STEP")
    }
    pub fn set_step(self,s:&str)->Self{
        set_attr_str(self.w,"STEP",s);
        self
    }
    pub fn get_value(self) -> String {
        get_attr_str(self.w, "VALUE")
    }
    pub fn set_value(self,s:&str)->Self{
        set_attr_str(self.w,"VALUE",s);
        self
    }
    pub fn on_change(self,f: Icallback)->Self{
        call_back(self.w,"VALUECHANGED_CB",f);
        self
    }
}
impl Control for Slider{ fn ptr(self) ->IUPPtr{ self.w}}
impl Widget for Slider {}

extern fn _text_change_(w:IUPPtr,_:i32,_:CBPtr,sp:CBPtr)->i32{
    TEXTS_MAP__.with(|c| {
        let  v = c.borrow();
        if let Some(x)=v.get(&w){
            x();
        }
    });
    0
}
extern fn _text_key_(w:IUPPtr,r:i32,_:CBPtr,sp:CBPtr)->i32{
    TEXTS_MAPK__.with(|c| {
        let  v = c.borrow();
        if let Some(x)=v.get(&w){
            x(r);
        }
    });
    0
}

#[derive(Copy,Clone)]
pub  struct Text{ w: IUPPtr} 
impl  Text  {
    pub fn new() -> Self {
        let p= { unsafe {  IupText(ptr::null_mut())}};
        call_back(p,"K_ANY",_text_key_);
        call_back(p,"VALUECHANGED_CB",_text_change_);
        Self {w:p}    
    }
    pub fn from(p: IUPPtr) -> Self { Self { w: p }}
    pub fn get_text(self)->String{
        get_attr_str(self.w,"VALUE")
    }
    pub fn set_text(self,s:&str)->Self{
        set_attr_str(self.w,"VALUE",s);
        self
    }
    pub fn multi(self,val:bool)->Self{
        set_attr_str(self.w,"MULTILINE",if val==true {"YES"} else {"NO"});
        self
    }
    pub fn password(self,val:bool)->Self{
        set_attr_str(self.w,"PASSWORD",if val==true {"YES"} else {"NO"});
        self
    }
    pub fn read_only(self,val:bool)->Self{
        set_attr_str(self.w,"READONLY",if val==true {"YES"} else {"NO"});
        self
    }
    pub fn select_all(self)->Self{
        set_attr_str(self.w,"SELECTION","ALL");
        self
    }
    pub fn spin(self,val:bool)->Self{
        set_attr_str(self.w,"SPIN",if val==true {"YES"} else {"NO"});
        self
    }
    pub fn get_spin_min(self) -> String {
        get_attr_str(self.w, "SPINMIN")
    }
    pub fn set_spin_min(self,s:&str)->Self{
        set_attr_str(self.w,"SPINMIN",s);
        self
    }
    pub fn get_spin_max(self) -> String {
        get_attr_str(self.w, "SPINMAX")
    }
    pub fn set_spin_max(self,s:&str)->Self{
        set_attr_str(self.w,"SPINMAX",s);
        self
    }
    pub fn get_spin_step(self) -> String {
        get_attr_str(self.w, "SPININC")
    }
    pub fn set_spin_step(self,s:&str)->Self{
        set_attr_str(self.w,"SPININC",s);
        self
    }
    pub fn get_spin_value(self) -> String {
        get_attr_str(self.w, "SPINVALUE ")
    }
    pub fn set_spin_value(self,s:&str)->Self{
        set_attr_str(self.w,"SPINVALUE",s);
        self
    }
    pub fn on_spin(self,f: Icallback)->Self{
        call_back(self.w,"SPIN_CB",f);
        self
    }
    pub fn on_change(self, f:Box< dyn Fn()>)->Self{
        TEXTS_MAP__.with(|c| {
            let mut v = c.borrow_mut();
            v.insert(self.w, Box::new(f));
        });
        self
    }
    pub fn on_key(self, f:Box< dyn Fn(i32)>)->Self{
        TEXTS_MAPK__.with(|c| {
            let mut v = c.borrow_mut();
            v.insert(self.w, Box::new(f));
        });
        self
    }
}
impl Control for Text{ fn ptr(self) ->IUPPtr{ self.w}}
impl Widget for Text {}

#[derive(Copy,Clone)]
pub  struct Toggle{ w: IUPPtr}
impl Toggle {
    pub fn new<T: Into<String>>(text: T) -> Toggle{ Toggle { w:{unsafe { IupToggle(CString::new(text.into()).unwrap().as_ptr(), ptr::null_mut())}}}}
    pub fn from(p: IUPPtr) -> Toggle { Toggle { w: p } }
    pub fn get_text(self) -> String {
        get_attr_str(self.w, "TITLE")
    }
    pub fn set_align(self,s:&str)->Self{
        set_attr_str(self.w,"ALIGNMENT",s);
        self
    }
    pub fn set_text(self, s: &str) -> Self {
        set_attr_str(self.w, "TITLE", s);
        self
    }
    pub fn is_checked(self) -> bool {
        if get_attr_str(self.w, "VALUE")=="ON" {true} else {false}
    }
    pub fn check(self,v:bool)->Self{
        set_attr_str(self.w,"VALUE",if v==true {"YES"} else {"NO"});
        self
    }
    pub fn set_image(self,s:&str)->Self{
        set_attr_str(self.w,"IMAGE",s);
        self
    }
    pub fn set_check_image(self,s:&str)->Self{
        set_attr_str(self.w,"IMPRESS",s);
        self
    }
    pub fn set_disable_image(self,s:&str)->Self{
        set_attr_str(self.w,"IMINACTIVE",s);
        self
    }
    pub fn on_change(self,f: Icallback)->Self{
        call_back(self.w,"VALUECHANGED_CB",f);
        self
    }
}
impl Control for Toggle{ fn ptr(self) ->IUPPtr{ self.w}}
impl Widget for Toggle {}


pub fn cursor(w: IUPPtr,l:i32,val:&str){
        let r = l.to_string() + ":*";
        set_attr_str(w, "BGCOLOR".to_string() + &r, val.to_string());
        set_attr_str(w, "REDRAW".to_string(), "L".to_string() + &l.to_string());
}


#[allow(unused)]
extern fn _table_click_(w:IUPPtr,r:i32,_:CBPtr,sp:CBPtr)->i32{
    if r>0 {
        let idx:i32=get_attr_str(w,"INDEX").parse().unwrap();
        if idx>0 {
            cursor(w,idx,"255 255 255");
        }
        cursor(w,r,CURSOR_COLOR);
        set_attr_str(w,"INDEX",&r.to_string());

        let s=p32_to_str(sp);
        let v= s.into_bytes();
        if v[5] as char =='D' {
            TABLES_MAP2__.with(|c| {
                let mut v = c.borrow();
                if let Some(x)=v.get(&w){
                    x(r-1);
                }
            });
        }else if v[4] as char =='3'{
            set_attr(w,"FOCUSCELL",&(r.to_string()+":1"));
            TABLES_MAPM__.with(|c| {
                let mut v = c.borrow();
                if let Some(x)=v.get(&w){
                    x(r-1);
                }
            });
        }else{
            TABLES_MAP__.with(|c| {
                let mut v = c.borrow();
                if let Some(x)=v.get(&w){
                    x(r-1);
                }
            });
        }
    }
    0
}

#[derive(Copy,Clone)]
pub  struct Table{ w: IUPPtr}
impl Table {
    pub fn new(h:Vec<&str>) -> Self {
        let p= {unsafe { IupMatrix(ptr::null_mut())}};
        set_attr_str(p,"INDEX","-1");
        set_attrs(p,"NUMLIN_VISIBLE=0,NUMCOL_VISIBLE=0,RESIZEMATRIX=YES,HEIGHT0=8,READONLY=YES,EXPAND=YES");
        set_attr(p,"NUMCOL",h.len().to_string());
        for (i,el) in h.iter().enumerate(){
           set_attr_str(p,"0:".to_string()+&(i+1).to_string(),el.to_string()); 
        }
        call_back(p,"CLICK_CB",_table_click_);

        Self{w:p}
    }
    pub fn from(p: IUPPtr) -> Self { Self { w: p }}
    
    pub fn get_index(self)->i32{
        let idx=get_attr_str(self.w,"INDEX");
        let r:i32=idx.parse().unwrap();
        if r<0 {-1} else{r-1}
    }

    pub fn set_index(self,r:i32)->Self{
        let idx:i32=get_attr_str(self.w,"INDEX").parse().unwrap();
        if idx>0 {
            cursor(self.w,idx,"255 255 255");
        }
        cursor(self.w,r+1,CURSOR_COLOR);
        set_attr_str(self.w,"INDEX",&((r+1).to_string()));
        set_attr(self.w,"FOCUSCELL",&((r+1).to_string()+":1"));
        self
    }

    pub fn on_click(self, f:Box< dyn Fn(i32)>)->Self{
        TABLES_MAP__.with(|c| {
            let mut v = c.borrow_mut();
            v.insert(self.w, Box::new(f));
        });
        self
    }
    pub fn on_dbl_click(self, f:Box< dyn Fn(i32)>)->Self{
        TABLES_MAP2__.with(|c| {
            let mut v = c.borrow_mut();
            v.insert(self.w, Box::new(f));
        });
        self
    }
    pub fn on_menu_click(self, f:Box< dyn Fn(i32)>)->Self{
        TABLES_MAPM__.with(|c| {
            let mut v = c.borrow_mut();
            v.insert(self.w, Box::new(f));
        });
        self
    }

    pub fn fill(self,s:String)->Self{
        let rows:Vec<&str>=s.split(';').collect();
        set_attr(self.w,"NUMLIN",rows.len().to_string());
        for (i,row) in rows.iter().enumerate(){
            let cols:Vec<&str>=row.split(',').collect();
            for (j,col) in cols.iter().enumerate(){
                set_attr_str(self.w,(i+1).to_string()+":"+ &(j+1).to_string(),col.to_string()); 
            }
        }
        self
    }
    pub fn fill_ext(self,s:String,funcs:Vec<fn(String)->String>)->Self{
        if s==""{
            set_attr(self.w,"NUMLIN","0");
            return self;
        }
        let rows:Vec<&str>=s.split(';').collect();
        set_attr(self.w,"NUMLIN",rows.len().to_string());
        for (i,row) in rows.iter().enumerate(){
            for (j,f) in funcs.iter().enumerate(){
                set_attr_str(self.w,(i+1).to_string()+":"+ &(j+1).to_string(),f(row.to_string())); 
            }
        }
        self
    }
    pub fn set_cols_width(self,wd:Vec<u32>)->Self{
        let mut s=String::from("");
        for(i,el) in wd.iter().enumerate(){
            let r=String::from(",WIDTH")+&(i+1).to_string() + "=" +&el.to_string();
            s.push_str(&r);
        }
        set_attrs(self.w,s);
        self
    }
    pub fn add(self,s:&str)->Self{
        let s_count=get_attr_str(self.w,"NUMLIN");
        let line:i32=s_count.parse().unwrap();
        set_attr(self.w,"ADDLIN",&s_count);
        let v_vals:Vec<&str>=s.split(',').collect();
        for (j,el) in v_vals.iter().enumerate(){
            set_attr_str(self.w,(line+1).to_string()+":"+ &(j+1).to_string(),el.to_string()); 
        };
        self.set_index(line);
        self.scroll_to(line);
        self
    }
    pub fn add_ext(self,s:&str,funcs:Vec<fn(String)->String>)->Self{
        let s_count=get_attr_str(self.w,"NUMLIN");
        let line:i32=s_count.parse().unwrap();
        set_attr(self.w,"ADDLIN",&s_count);
        for (j,f) in funcs.iter().enumerate(){
            set_attr_str(self.w,(line+1).to_string()+":"+ &(j+1).to_string(),f(s.to_string())); 
        };
        self.set_index(line);
        self.scroll_to(line);
        self
    }
    pub fn change(self,s:&str)->Self{
        let idx=get_attr_str(self.w,"INDEX");
        let line:i32=idx.parse().unwrap();
        let v_vals:Vec<&str>=s.split(',').collect();
        for (j,el) in v_vals.iter().enumerate(){
            set_attr_str(self.w,(line).to_string()+":"+ &(j+1).to_string(),el.to_string()); 
        };
        self.set_index(line-1);
        self
    }
    pub fn change_ext(self,s:&str,funcs:Vec<fn(String)->String>)->Self{
        let idx=get_attr_str(self.w,"INDEX");
        let line:i32=idx.parse().unwrap();
        for (j,f) in funcs.iter().enumerate(){
            set_attr_str(self.w,(line).to_string()+":"+ &(j+1).to_string(),f(s.to_string())); 
        };
        self.set_index(line-1);
        self
    }
    pub fn delete(self)->Self{
        let idx=self.get_index();
        set_attr_str(self.w,"DELLIN",&(idx+1).to_string());
        if idx > 0{
		    self.set_index(idx-1);
		} else {
			self.set_index(0);
		}            
        self
    }
    pub fn set_height(self,n:i32)->Self{
        set_attr(self.w,"EXPAND", "HORIZONTAL");
        set_attr(self.w,"SIZE", &("x".to_string()+&n.to_string()));
        self
    }
    pub fn set_header(self,n:i32,s:&str)->Self{
        set_attr(self.w,&("0:".to_string()+&(n+1).to_string()), s);
	    set_attr(self.w,"REDRAW", "L0");
        self
    }
    pub fn scroll_to(self,line:i32)->Self{
        let line_visible:i32 = get_attr_str(self.w,"NUMLIN_VISIBLE").parse().unwrap();
        let s_origin= get_attr_str(self.w,"ORIGIN");
        let v_origin:Vec<&str>=s_origin.split(':').collect();
        let origin:i32=v_origin[0].parse().unwrap();
        if line >= line_visible+origin-1 {
		    set_attr(self.w,"ORIGIN", &((line+2-line_visible).to_string()+"*"));
	    } else if line < origin {
		    set_attr(self.w,"ORIGIN", &((line+1).to_string()+"*"));
	    }
        self
    }
    pub fn search(self,store:&Vec<String>,s:String){
        if s.len()<1{
            return;
        };
        let ss:Vec<&str>=s.split(' ').collect();
        for (i,el) in store.iter().enumerate(){
            let it=el.to_lowercase();
            let names:Vec<&str>=it.split(' ').collect();
            if names.len() == 1 {
                if names[0].starts_with(&s){
                    self.set_index(i as i32);
                    self.scroll_to(i as i32);
                    break;
                }
            }else  if ss.len()>1{
                if names[0].starts_with(ss[0]) && names[1].starts_with(ss[1]){
                    self.set_index(i as i32);
                    self.scroll_to(i as i32);
                    break;
                }
            }else if s.len()>0 && ss.len()==1{
                if s.starts_with(" ") && names[1].starts_with(ss[0]){
                    self.set_index(i as i32);
                    self.scroll_to(i as i32);
                    break;
                }
                if names[0].starts_with(&s){
                    self.set_index(i as i32);
                    self.scroll_to(i as i32);
                    break;
                }
            }else if !s.starts_with(" ") && names[0].starts_with(ss[0]){
                self.set_index(i as i32);
                self.scroll_to(i as i32);
                break;
            };
        };
    }
}

impl Control for Table{ fn ptr(self) ->IUPPtr{ self.w}}
impl Widget for Table {}


