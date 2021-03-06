use its = syntax::parse::token::ident_to_str;

use rustc::metadata::{csearch,decoder,cstore};
use syntax;
use syntax::ast;

use doctree;
use visit;
use std::local_data;

pub trait Clean<T> {
    pub fn clean(&self) -> T;
}

impl<T: Clean<U>, U> Clean<~[U]> for ~[T] {
    pub fn clean(&self) -> ~[U] {
        self.iter().transform(|x| x.clean()).collect()
    }
}
impl<T: Clean<U>, U> Clean<U> for @T {
    pub fn clean(&self) -> U {
        (**self).clean()
    }
}

impl<T: Clean<U>, U> Clean<Option<U>> for Option<T> {
    pub fn clean(&self) -> Option<U> {
        match self {
            &None => None,
            &Some(ref v) => Some(v.clean())
        }
    }
}

impl<T, U: Clean<T>> Clean<T> for syntax::codemap::spanned<U> {
    pub fn clean(&self) -> T {
        self.node.clean()
    }
}

impl<T: Clean<U>, U> Clean<~[U]> for syntax::opt_vec::OptVec<T> {
    pub fn clean(&self) -> ~[U] {
        match self {
            &syntax::opt_vec::Empty => ~[],
            &syntax::opt_vec::Vec(ref v) => v.clean()
        }
    }
}

pub struct Crate {
    name: ~str,
    attrs: ~[Attribute],
    mods: ~[Module],
}

impl Clean<Crate> for visit::RustdocVisitor {
    pub fn clean(&self) -> Crate {
        use syntax::attr::{find_linkage_metas, last_meta_item_value_str_by_name};
        let maybe_meta = last_meta_item_value_str_by_name(find_linkage_metas(self.attrs), "name");

        Crate {
            name: match maybe_meta {
                Some(x) => x.to_owned(),
                None => fail!("rustdoc_ng requires a #[link(name=\"foo\")] crate attribute"),
            },
            mods: self.mods.clean(),
            attrs: self.attrs.clean(),
        }
    }
}

pub struct Module {
    name: ~str,
    attrs: ~[Attribute],
    structs: ~[Struct],
    enums: ~[Enum],
    fns: ~[Function],
    mods: ~[Module],
    typedefs: ~[Typedef],
    statics: ~[Static],
    traits: ~[Trait],
    impls: ~[Impl],
    view_items: ~[ViewItem],
}

impl Clean<Module> for doctree::Module {
    pub fn clean(&self) -> Module {
        let name = if self.name.is_some() {
            self.name.unwrap().clean()
        } else {
            ~""
        };
        Module {
            name       : name,
            attrs      : self.attrs.clean(),
            structs    : self.structs.clean(),
            enums      : self.enums.clean(),
            fns        : self.fns.clean(),
            mods       : self.mods.clean(),
            typedefs   : self.typedefs.clean(),
            statics    : self.statics.clean(),
            traits     : self.traits.clean(),
            impls      : self.impls.clean(),
            view_items : self.view_items.clean(),
        }
    }
}

#[deriving(Clone)]
pub enum Attribute {
    Word(~str),
    List(~str, ~[Attribute]),
    NameValue(~str, ~str)
}

impl Clean<Attribute> for ast::MetaItem_ {
    pub fn clean(&self) -> Attribute {
        match *self {
            ast::MetaWord(s) => Word(remove_comment_tags(s)),
            ast::MetaList(ref s, ref l) => List(remove_comment_tags(*s), l.iter()
                                         .transform(|x| x.node.clean()).collect()),
            ast::MetaNameValue(s, ref v) => NameValue(remove_comment_tags(s),
                                         remove_comment_tags(lit_to_str(v)))
        }
    }
}

impl Clean<Attribute> for ast::Attribute_ {
    pub fn clean(&self) -> Attribute {
        self.value.clean()
    }
}

#[deriving(Clone)]
pub struct TyParam {
    name: ~str,
    node: ast::NodeId,
    bounds: ~[TyParamBound]
}

impl Clean<TyParam> for ast::TyParam {
    pub fn clean(&self) -> TyParam {
        TyParam {
            name: self.ident.clean(),
            node: self.id,
            bounds: self.bounds.iter().transform(|x| x.clean()).collect()
        }
    }
}

pub enum TyParamBound {
    RegionBound,
    TraitBound(TraitRef)
}

#[doc = "Automatically derived."]
impl ::std::clone::Clone for TyParamBound {
    pub fn clone(&self) -> TyParamBound {
        match *self {
            RegionBound => RegionBound,
            TraitBound(ref __self_0) => TraitBound((*__self_0).clone())
        }
    }
}

impl Clean<TyParamBound> for ast::TyParamBound {
    pub fn clean(&self) -> TyParamBound {
        match *self {
            ast::RegionTyParamBound => RegionBound,
            ast::TraitTyParamBound(ref t) => TraitBound(t.clean()),
        }
    }
}

#[deriving(Clone)]
pub struct Lifetime(~str);

impl Clean<Lifetime> for ast::Lifetime {
    pub fn clean(&self) -> Lifetime {
        Lifetime(self.ident.clean())
    }
}

// maybe use a Generic enum and use ~[Generic]?
#[deriving(Clone)]
pub struct Generics {
    lifetimes: ~[Lifetime],
    type_params: ~[TyParam]
}

impl Generics {
    pub fn new() -> Generics {
        Generics {
            lifetimes: ~[],
            type_params: ~[]
        }
    }
}

impl Clean<Generics> for ast::Generics {
    pub fn clean(&self) -> Generics {
        Generics {
            lifetimes: self.lifetimes.iter().transform(|x| x.clean()).collect(),
            type_params: self.ty_params.iter().transform(|x| x.clean()).collect()
        }
    }
}

pub struct Method {
    name: ~str,
    attrs: ~[Attribute],
    generics: Generics,
    self_: SelfTy,
    purity: ast::purity,
    decl: FnDecl,
    where: ~str,
    id: ast::NodeId,
    vis: Visibility,
}

impl ::std::clone::Clone for Method {
     pub fn clone(&self) -> Method {
         match *self {
             Method{name: ref __self_0_0,
                    attrs: ref __self_0_1,
                    generics: ref __self_0_2,
                    self_: ref __self_0_3,
                    purity: ref __self_0_4,
                    decl: ref __self_0_5,
                    where: ref __self_0_6,
                    id: ref __self_0_7,
                    vis: ref __self_0_8} =>
             Method{name: __self_0_0.clone(),
                    attrs: __self_0_1.clone(),
                    generics: __self_0_2.clone(),
                    self_: (*__self_0_3).clone(),
                    purity: __self_0_4.clone(),
                    decl: __self_0_5.clone(),
                    where: __self_0_6.clone(),
                    id: __self_0_7.clone(),
                    vis: __self_0_8.clone(),}
         }
     }
 }

impl Clean<Method> for ast::method {
    pub fn clean(&self) -> Method {
        Method {
            name: self.ident.clean(),
            attrs: self.attrs.clean(),
            generics: self.generics.clean(),
            self_: self.explicit_self.clean(),
            purity: self.purity.clone(),
            decl: self.decl.clean(),
            where: self.span.clean(),
            id: self.self_id.clone(),
            vis: self.vis,
        }
    }
}

#[cfg(ignore)]
impl Clean<Method> for @ast::method {
    pub fn clean(&self) -> Method {
        Method {
            name: self.ident.clean(),
            attrs: self.attrs.clean(),
            generics: self.generics.clean(),
            self_: self.explicit_self.clean(),
            purity: self.purity.clone(),
            decl: self.decl.clean(),
            where: self.span.clean(),
            id: self.self_id.clone(),
            vis: self.vis,
        }
    }
}

#[deriving(Clone)]
pub struct TyMethod {
    name: ~str,
    attrs: ~[Attribute],
    purity: ast::purity,
    decl: FnDecl,
    generics: Generics,
    id: ast::NodeId,
    self_: SelfTy,
    where: ~str
}

impl Clean<TyMethod> for ast::TypeMethod {
    pub fn clean(&self) -> TyMethod {
        TyMethod {
            name: self.ident.clean(),
            attrs: self.attrs.clean(),
            purity: self.purity.clone(),
            decl: self.decl.clean(),
            self_: self.explicit_self.clean(),
            generics: self.generics.clean(),
            id: self.id,
            where: self.span.clean()
        }
    }
}

#[deriving(Clone)]
pub enum SelfTy {
    SelfStatic,
    SelfValue,
    SelfBorrowed(Option<Lifetime>, Mutability),
    SelfManaged(Mutability),
    SelfOwned,
}

impl Clean<SelfTy> for ast::explicit_self_ {
    pub fn clean(&self) -> SelfTy {
        match *self {
            ast::sty_static => SelfStatic,
            ast::sty_value => SelfValue,
            ast::sty_uniq => SelfOwned,
            ast::sty_region(lt, mt) => SelfBorrowed(lt.clean(), mt.clean()),
            ast::sty_box(mt) => SelfManaged(mt.clean()),
        }
    }
}

pub struct Function {
    decl: FnDecl,
    name: ~str,
    visibility: Visibility,
    where: ~str,
    generics: Generics,
    //body: Block,
    id: ast::NodeId,
    attrs: ~[Attribute]
}

impl Clean<Function> for doctree::Function {
    pub fn clean(&self) -> Function {
        Function {
            decl: self.decl.clean(),
            name: self.name.clean(),
            id: self.id,
            attrs: collapse_docs(self.attrs.clean()),
            where: self.where.clean(),
            visibility: self.visibility,
            generics: self.generics.clean(),
        }
    }
}

#[deriving(Clone)]
pub struct ClosureDecl {
    sigil: ast::Sigil,
    region: Option<Lifetime>,
    lifetimes: ~[Lifetime],
    decl: FnDecl,
    onceness: ast::Onceness,
    purity: ast::purity,
    bounds: ~[TyParamBound]
}

impl Clean<ClosureDecl> for ast::TyClosure {
    pub fn clean(&self) -> ClosureDecl {
        ClosureDecl {
            sigil: self.sigil,
            region: self.region.clean(),
            lifetimes: self.lifetimes.clean(),
            decl: self.decl.clean(),
            onceness: self.onceness,
            purity: self.purity,
            bounds: match self.bounds {
                Some(ref x) => x.clean(),
                None        => ~[]
            },
        }
    }
}

pub struct FnDecl {
    inputs: ~[Argument],
    output: Type,
    cf: RetStyle,
    attrs: ~[Attribute]
}

#[doc = "Automatically derived."]
impl ::std::clone::Clone for FnDecl {
    pub fn clone(&self) -> FnDecl {
        match *self {
            FnDecl{inputs: ref __self_0_0,
            output: ref __self_0_1,
            cf: ref __self_0_2,
            attrs: ref __self_0_3} => FnDecl{
                inputs: __self_0_0.clone(),
                output: __self_0_1.clone(),
                cf: (*__self_0_2).clone(),
                attrs: __self_0_3.clone(),
            }
        }
    }
}

impl Clean<FnDecl> for ast::fn_decl {
    pub fn clean(&self) -> FnDecl {
        FnDecl {
            inputs: self.inputs.iter().transform(|x| x.clean()).collect(),
            output: (self.output.clean()),
            cf: self.cf.clean(),
            attrs: ~[]
        }
    }
}

#[deriving(Clone)]
pub struct Argument {
    ty: Type,
    name: ~str,
    id: ast::NodeId
}

impl Clean<Argument> for ast::arg {
    pub fn clean(&self) -> Argument {
        Argument {
            name: name_from_pat(self.pat),
            ty: (self.ty.clean()),
            id: self.id
        }
    }
}

#[deriving(Clone)]
pub enum RetStyle {
    NoReturn,
    Return
}

impl Clean<RetStyle> for ast::ret_style {
    pub fn clean(&self) -> RetStyle {
        match *self {
            ast::return_val => Return,
            ast::noreturn => NoReturn
        }
    }
}

#[deriving(Clone)]
pub struct Trait {
    name: ~str,
    methods: ~[TraitMethod],
    generics: Generics,
    where: ~str,
    attrs: ~[Attribute],
    parents: ~[TraitRef],
    id: ast::NodeId,
}

impl Clean<Trait> for doctree::Trait {
    pub fn clean(&self) -> Trait {
        Trait {
            name: self.name.clean(),
            methods: self.methods.clean(),
            generics: self.generics.clean(),
            parents: self.parents.clean(),
            where: self.where.clean(),
            attrs: self.attrs.clean(),
            id: self.id
        }
    }
}

#[deriving(Clone)]
pub struct TraitRef {
    path: ~str,
    id: ast::NodeId,
}

impl Clean<TraitRef> for ast::trait_ref {
    pub fn clean(&self) -> TraitRef {
        TraitRef {
            path: self.path.clean(),
            id: self.ref_id,
        }
    }
}

pub enum TraitMethod {
    Required(TyMethod),
    Provided(Method),
}

impl TraitMethod {
    pub fn is_req(&self) -> bool {
        match self {
            &Required(*) => true,
            _ => false,
        }
    }
    pub fn is_def(&self) -> bool {
        match self {
            &Provided(*) => true,
            _ => false,
        }
    }
}
#[doc = "Automatically derived."]
impl ::std::clone::Clone for TraitMethod {
    pub fn clone(&self) -> TraitMethod {
        match *self {
            Required(ref __self_0) => Required((*__self_0).clone()),
            Provided(ref __self_0) => Provided(__self_0.clone())
        }
    }
}

impl Clean<TraitMethod> for ast::trait_method {
    pub fn clean(&self) -> TraitMethod {
        match self {
            &ast::required(ref t) => Required(t.clean()),
            &ast::provided(ref t) => Provided(t.clean()),
        }
    }
}

/// A representation of a Type suitable for hyperlinking purposes. Ideally one can get the original
/// type out of the AST/ty::ctxt given one of these, if more information is needed. Most importantly
/// it does not preserve mutability or boxes.
#[deriving(Clone)]
pub enum Type {
    /// Most types start out as "Unresolved". It serves as an intermediate stage between cleaning
    /// and type resolution.
    Unresolved(ast::NodeId),
    /// structs/enums/traits (anything that'd be an ast::ty_path)
    Resolved(ast::NodeId),
    /// Reference to an item in an external crate (fully qualified path)
    External(~str, ~str),
    /// For parameterized types, so the consumer of the JSON don't go looking
    /// for types which don't exist anywhere.
    Generic(ast::NodeId),
    /// For references to self
    Self(ast::NodeId),
    /// Primitives are just the fixed-size numeric types (plus int/uint/float), and char.
    Primitive(ast::prim_ty),
    Closure(~ClosureDecl),
    /// extern "ABI" fn
    BareFunction(~BareFunctionDecl),
    Tuple(~[Type]),
    Vector(~Type),
    String,
    Bool,
    /// aka ty_nil
    Unit,
    /// aka ty_bot
    Bottom,
    Unique(~Type),
    Managed(~Type),
    RawPointer(~Type),
    BorrowedRef(~Type),
    // region, raw, other boxes, mutable
}

impl Clean<Type> for ast::Ty {
    pub fn clean(&self) -> Type {
        use syntax::ast::*;
        debug!("cleaning type `%?`", self);
        let codemap = local_data::get(super::ctxtkey, |x| *x.unwrap()).sess.codemap;
        debug!("span corresponds to `%s`", codemap.span_to_str(self.span));
        let t = match self.node {
            ty_nil => Unit,
            ty_ptr(ref m) =>  RawPointer(~resolve_type(&m.ty.clean())),
            ty_rptr(_, ref m) => BorrowedRef(~resolve_type(&m.ty.clean())),
            ty_box(ref m) => Managed(~resolve_type(&m.ty.clean())),
            ty_uniq(ref m) => Unique(~resolve_type(&m.ty.clean())),
            ty_vec(ref m) | ty_fixed_length_vec(ref m, _) => Vector(~resolve_type(&m.ty.clean())),
            ty_tup(ref tys) => Tuple(tys.iter().transform(|x| resolve_type(&x.clean())).collect()),
            ty_path(_, _, id) => Unresolved(id),
            ty_closure(ref c) => Closure(~c.clean()),
            ty_bare_fn(ref barefn) => BareFunction(~barefn.clean()),
            ty_bot => Bottom,
            ref x => fail!("Unimplemented type %?", x),
        };
        resolve_type(&t)
    }
}

pub struct StructField {
    name: ~str,
    type_: Type,
    attrs: ~[Attribute],
    visibility: Option<Visibility>,
}

impl Clean<StructField> for doctree::StructField {
    pub fn clean(&self) -> StructField {
        StructField {
            name: if self.name.is_some() { self.name.unwrap().clean() } else { ~"" },
            type_: self.type_.clean(),
            attrs: collapse_docs(self.attrs.iter().transform(|x| x.clean()).collect()),
            visibility: self.visibility
        }
    }
}

pub type Visibility = ast::visibility;

pub struct Struct {
    name: ~str,
    where: ~str,
    node: ast::NodeId,
    struct_type: doctree::StructType,
    attrs: ~[Attribute],
    generics: Generics,
    fields: ~[StructField],
}

impl Clean<Struct> for doctree::Struct {
    pub fn clean(&self) -> Struct {
        Struct {
            name: self.name.clean(),
            node: self.id,
            struct_type: self.struct_type,
            attrs: collapse_docs(self.attrs.iter().transform(|x| x.clean()).collect()),
            generics: self.generics.clean(),
            fields: self.fields.iter().transform(|x| x.clean()).collect(),
            where: self.where.clean(),
        }
    }
}

/// This is a more limited form of the standard Struct, different in that it
/// it lacks the things most items have (name, id, parameterization). Found
/// only as a variant in an enum.
pub struct VariantStruct {
    struct_type: doctree::StructType,
    fields: ~[StructField],
}

impl Clean<VariantStruct> for syntax::ast::struct_def {
    pub fn clean(&self) -> VariantStruct {
        VariantStruct {
            struct_type: doctree::struct_type_from_def(self),
            fields: self.fields.iter().transform(
                                       |x| doctree::StructField::new(&x.node).clean()).collect()
        }
    }
}

pub struct Enum {
    variants: ~[Variant],
    generics: Generics,
    attrs: ~[Attribute],
    name: ~str,
    node: ast::NodeId,
    where: ~str,
}

impl Clean<Enum> for doctree::Enum {
    pub fn clean(&self) -> Enum {
        Enum {
            variants: self.variants.iter().transform(|x| x.clean()).collect(),
            generics: self.generics.clean(),
            attrs: collapse_docs(self.attrs.iter().transform(|x| x.clean()).collect()),
            name: self.name.clean(),
            where: self.where.clean(),
            node: self.id
        }
    }
}

pub struct Variant {
    name: ~str,
    attrs: ~[Attribute],
    kind: VariantKind,
    visibility: Visibility,
}

impl Clean<Variant> for doctree::Variant {
    pub fn clean(&self) -> Variant {
        Variant {
            name: self.name.clean(),
            attrs: collapse_docs(self.attrs.iter().transform(|x| x.clean()).collect()),
            kind: self.kind.clean(),
            visibility: self.visibility
        }
    }
}

pub enum VariantKind {
    CLikeVariant,
    TupleVariant(~[Type]),
    StructVariant(VariantStruct),
}

impl Clean<VariantKind> for ast::variant_kind {
    pub fn clean(&self) -> VariantKind {
        match self {
            &ast::tuple_variant_kind(ref args) => {
                if args.len() == 0 {
                    CLikeVariant
                } else {
                    TupleVariant(args.iter().transform(|x| x.ty.clean()).collect())
                }
            },
            &ast::struct_variant_kind(ref sd) => StructVariant(sd.clean()),
        }
    }
}

impl Clean<~str> for syntax::codemap::span {
    pub fn clean(&self) -> ~str {
        let cm = local_data::get(super::ctxtkey, |x| x.unwrap().clone()).sess.codemap;
        cm.span_to_str(*self)
    }
}

impl Clean<~str> for ast::Path {
    pub fn clean(&self) -> ~str {
        use syntax::parse::token::interner_get;

        let mut s = ~"";
        let mut first = true;
        for i in self.idents.iter().transform(|x| interner_get(x.name)) {
            if !first {
                s.push_str("::");
            } else {
                first = false;
            }
            s.push_str(i);
        }
        s
    }
}

impl Clean<~str> for ast::ident {
    pub fn clean(&self) -> ~str {
        its(self).to_owned()
    }
}

pub struct Typedef {
    name: ~str,
    type_: Type,
    generics: Generics,
    where: ~str,
    id: ast::NodeId,
    attrs: ~[Attribute],
}

impl Clean<Typedef> for doctree::Typedef {
    pub fn clean(&self) -> Typedef {
        Typedef {
            type_: self.ty.clean(),
            generics: self.gen.clean(),
            name: self.name.clean(),
            id: self.id.clone(),
            attrs: self.attrs.clean(),
            where: self.where.clean(),
        }
    }
}

#[deriving(Clone)]
pub struct BareFunctionDecl {
    purity: ast::purity,
    generics: Generics,
    decl: FnDecl,
    abi: ~str
}

impl Clean<BareFunctionDecl> for ast::TyBareFn {
    pub fn clean(&self) -> BareFunctionDecl {
        BareFunctionDecl {
            purity: self.purity,
            generics: Generics {
                lifetimes: self.lifetimes.clean(),
                type_params: ~[],
            },
            decl: self.decl.clean(),
            abi: self.abis.to_str(),
        }
    }
}

pub struct Static {
    name: ~str,
    type_: Type,
    mutability: Mutability,
    where: ~str,
    /// It's useful to have the value of a static documented, but I have no
    /// desire to represent expressions (that'd basically be all of the AST,
    /// which is huge!). So, have a string.
    expr: ~str,
    attrs: ~[Attribute],
}

impl Clean<Static> for doctree::Static {
    pub fn clean(&self) -> Static {
        debug!("claning static %s: %?", self.name.clean(), self);
        Static {
            type_: self.type_.clean(),
            mutability: self.mutability.clean(),
            expr: self.expr.span.to_src(),
            name: self.name.clean(),
            attrs: self.attrs.clean(),
            where: self.where.clean(),
        }
    }
}

#[deriving(ToStr, Clone)]
pub enum Mutability {
    Mutable,
    Immutable,
    Const,
}

impl Clean<Mutability> for ast::mutability {
    pub fn clean(&self) -> Mutability {
        match self {
            &ast::m_mutbl => Mutable,
            &ast::m_imm => Immutable,
            &ast::m_const => Const
        }
    }
}

#[deriving(Clone)]
pub struct Impl {
    generics: Generics,
    trait_: Option<TraitRef>,
    for_: Type,
    methods: ~[Method],
    attrs: ~[Attribute],
    where: ~str,
}

impl Clean<Impl> for doctree::Impl {
    pub fn clean(&self) -> Impl {
        Impl {
            generics: self.generics.clean(),
            trait_: self.trait_.clean(),
            for_: self.for_.clean(),
            methods: self.methods.clean(),
            attrs: self.attrs.clean(),
            where: self.where.clean(),
        }
    }
}

pub struct ViewItem {
    attrs: ~[Attribute],
    where: ~str,
    vis: Visibility,
    inner: ViewItemInner
}

impl Clean<ViewItem> for ast::view_item {
    pub fn clean(&self) -> ViewItem {
        ViewItem {
            attrs: self.attrs.clean(),
            where: self.span.clean(),
            vis: self.vis,
            inner: self.node.clean()
        }
    }
}

pub enum ViewItemInner {
    ExternMod(~str, ~[Attribute], ast::NodeId),
    Import(~[ViewPath])
}

impl Clean<ViewItemInner> for ast::view_item_ {
    pub fn clean(&self) -> ViewItemInner {
        match self {
            &ast::view_item_extern_mod(ref i, ref mi, ref id) =>
                ExternMod(i.clean(), mi.clean(), *id),
            &ast::view_item_use(ref vp) => Import(vp.clean())
        }
    }
}

pub enum ViewPath {
    SimpleImport(~str, ~str, ast::NodeId),
    GlobImport(~str, ast::NodeId),
    ImportList(~str, ~[ViewListIdent], ast::NodeId)
}

impl Clean<ViewPath> for ast::view_path_ {
    pub fn clean(&self) -> ViewPath {
        match self {
            &ast::view_path_simple(ref i, ref p, ref id) => SimpleImport(i.clean(), p.clean(), *id),
            &ast::view_path_glob(ref p, ref id) => GlobImport(p.clean(), *id),
            &ast::view_path_list(ref p, ref pl, ref id) => ImportList(p.clean(), pl.clean(), *id),
        }
    }
}

pub type ViewListIdent = ~str;

impl Clean<ViewListIdent> for ast::path_list_ident_ {
    pub fn clean(&self) -> ViewListIdent {
        self.name.clean()
    }
}

// Utilities

trait ToSource {
    pub fn to_src(&self) -> ~str;
}

impl ToSource for syntax::codemap::span {
    pub fn to_src(&self) -> ~str {
        let cm = local_data::get(super::ctxtkey, |x| x.unwrap().clone()).sess.codemap.clone();
        match cm.span_to_snippet(*self) {
            Some(x) => x,
            None    => ~""
        }
    }
}

fn lit_to_str(lit: &ast::lit) -> ~str {
    match lit.node {
        ast::lit_str(st) => st.to_owned(),
        ast::lit_int(ch, ast::ty_char) => ~"'" + ch.to_str() + "'",
        ast::lit_int(i, _t) => i.to_str(),
        ast::lit_uint(u, _t) => u.to_str(),
        ast::lit_int_unsuffixed(i) => i.to_str(),
        ast::lit_float(f, _t) => f.to_str(),
        ast::lit_float_unsuffixed(f) => f.to_str(),
        ast::lit_bool(b) => b.to_str(),
        ast::lit_nil => ~"",
    }
}

fn name_from_pat(p: &ast::pat) -> ~str {
    use syntax::ast::*;
    match p.node {
        pat_wild => ~"_",
        pat_ident(_, ref p, _) => p.clean(),
        pat_enum(ref p, _) => p.clean(),
        pat_struct(*) => fail!("tried to get argument name from pat_struct, \
                                 which is not allowed in function arguments"),
        pat_tup(*) => ~"(tuple arg NYI)",
        pat_box(p) => name_from_pat(p),
        pat_uniq(p) => name_from_pat(p),
        pat_region(p) => name_from_pat(p),
        pat_lit(*) => fail!("tried to get argument name from pat_lit, \
                             which is not allowed in function arguments"),
        pat_range(*) => fail!("tried to get argument name from pat_range, \
                               which is not allowed in function arguments"),
        pat_vec(*) => fail!("tried to get argument name from pat_vec, \
                             which is not allowed in function arguments")
    }
}

fn remove_comment_tags(s: &str) -> ~str {
    if s.starts_with("/") {
        match s.slice(0,3) {
            &"///" => return s.slice(3, s.len()).trim().to_owned(),
            &"/**" | &"/*!" => return s.slice(3, s.len() - 2).trim().to_owned(),
            _ => return s.trim().to_owned()
        }
    } else {
        return s.to_owned();
    }
}

enum CleanCommentStates {
    Collect,
    Strip,
    Stripped,
}

fn clean_comment_body(s: ~str) -> ~str {
    let mut res = ~"";
    let mut state = Strip;

    for char in s.iter() {
        match (state, char) {
            (Strip, '*') => state = Stripped,
            (Strip, '/') => state = Stripped,
            (Stripped, '/') => state = Stripped,
            (Strip, ' ') => (),
            (Strip, '\t') => (),
            (Stripped, ' ') => state = Collect,
            (Stripped, '\t') => state = Collect,
            (_, '\n') => { res.push_char(char); state = Strip; }
            (_, char) => res.push_char(char)
        }
    }

    res = res.trim().to_owned();
    res.push_char('\n');
    res
}

pub fn collapse_docs(attrs: ~[Attribute]) -> ~[Attribute] {
    let mut docstr = ~"";
    for at in attrs.iter() {
        match *at {
            //XXX how should these be separated?
            NameValue(~"doc", ref s) => docstr.push_str(fmt!("%s ", clean_comment_body(s.clone()))),
            _ => ()
        }
    }
    let mut a = attrs.iter().filter(|&a| match a {
        &NameValue(~"doc", _) => false,
        _ => true
    }).transform(|x| x.clone()).collect::<~[Attribute]>();
    a.push(NameValue(~"doc", docstr.trim().to_owned()));
    a
}

/// Given a Type, resolve it using the def_map
fn resolve_type(t: &Type) -> Type {
    use syntax::ast::*;

    let id = match t {
        &Unresolved(id) => id,
        _ => return (*t).clone(),
    };

    let dm = local_data::get(super::ctxtkey, |x| *x.unwrap()).tycx.def_map;
    debug!("searching for %? in defmap", id);
    let d = match dm.find(&id) {
        Some(k) => k,
        None => {
            let ctxt = local_data::get(super::ctxtkey, |x| *x.unwrap());
            debug!("could not find %? in defmap (`%s`)", id,
                   syntax::ast_map::node_id_to_str(ctxt.tycx.items, id, ctxt.sess.intr()));
            fail!("Unexpected failure: unresolved id not in defmap (this is a bug!)")
        }
    };

    let def_id = match *d {
        def_fn(i, _) => i,
        def_self(i, _) | def_self_ty(i) => return Self(i),
        def_ty(i) => i,
        def_trait(i) => {
            debug!("saw def_trait in def_to_id");
            i
        },
        def_prim_ty(p) => match p {
            ty_str => return String,
            ty_bool => return Bool,
            _ => return Primitive(p)
        },
        def_ty_param(i, _) => return Generic(i.node),
        def_struct(i) => i,
        def_typaram_binder(i) => return Resolved(i),
        x => fail!("resolved type maps to a weird def %?", x),
    };

    if def_id.crate != ast::CRATE_NODE_ID {
        let sess = local_data::get(super::ctxtkey, |x| *x.unwrap()).sess;
        let mut path = ~"";
        let mut ty = ~"";
        do csearch::each_path(sess.cstore, def_id.crate) |pathstr, deflike, _vis| {
            match deflike {
                decoder::dl_def(di) => {
                    let d2 = match di {
                        def_fn(i, _) | def_ty(i) | def_trait(i) |
                            def_struct(i) | def_mod(i) => Some(i),
                        _ => None,
                    };
                    if d2.is_some() {
                        let d2 = d2.unwrap();
                        if def_id.node == d2.node {
                            debug!("found external def: %?", di);
                            path = pathstr.to_owned();
                            ty = match di {
                                def_fn(*) => ~"fn", 
                                def_ty(*) => ~"enum",
                                def_trait(*) => ~"trait",
                                def_prim_ty(p) => match p {
                                    ty_str => ~"str",
                                    ty_bool => ~"bool",
                                    _ => Primitive(p).to_json().to_str(),
                                },
                                def_ty_param(*) => ~"generic",
                                def_struct(*) => ~"struct",
                                def_typaram_binder(*) => ~"typaram_binder",
                                x => fail!("resolved external maps to a weird def %?", x),
                            };

                        }
                    }
                },
                _ => (),
            };
            true
        };
        let cname = cstore::get_crate_data(sess.cstore, def_id.crate).name.to_owned();
        External(cname + "::" + path, ty)
    } else {
        Resolved(def_id.node)
    }
}

#[cfg(test)]
mod tests {
    use super::NameValue;

    #[test]
    fn test_doc_collapsing() {
        assert_eq!(collapse_docs(~"// Foo\n//Bar\n // Baz\n"), ~"Foo\nBar\nBaz");
        assert_eq!(collapse_docs(~"* Foo\n *  Bar\n *Baz\n"), ~"Foo\n Bar\nBaz");
        assert_eq!(collapse_docs(~"* Short desc\n *\n * Bar\n *Baz\n"), ~"Short desc\n\nBar\nBaz");
        assert_eq!(collapse_docs(~" * Foo"), ~"Foo");
        assert_eq!(collapse_docs(~"\n *\n *\n * Foo"), ~"Foo");
    }

    fn collapse_docs(input: ~str) -> ~str {
        let attrs = ~[NameValue(~"doc", input)];
        let attrs_clean = super::collapse_docs(attrs);

        match attrs_clean[0] {
            NameValue(~"doc", s) => s,
            _ => (fail!("dude where's my doc?"))
        }
    }
}
