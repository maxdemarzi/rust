import codemap::span;
import ast::*;

export ast_fold_precursor;
export ast_fold;
export default_ast_fold;
export make_fold;
export noop_fold_crate;
export noop_fold_item;
export noop_fold_expr;
export noop_fold_pat;
export noop_fold_mod;
export noop_fold_ty;
export noop_fold_block;
export wrap;
export fold_ty_param;
export fold_ty_params;
export fold_fn_decl;

type ast_fold = @mut a_f;

// We may eventually want to be able to fold over type parameters, too

type ast_fold_precursor =
    //unlike the others, item_ is non-trivial
    {fold_crate: fn@(crate_, span, ast_fold) -> (crate_, span),
     fold_crate_directive: fn@(crate_directive_, span,
                               ast_fold) -> (crate_directive_, span),
     fold_view_item: fn@(view_item_, ast_fold) -> view_item_,
     fold_native_item: fn@(&&@native_item, ast_fold) -> @native_item,
     fold_item: fn@(&&@item, ast_fold) -> @item,
     fold_class_item: fn@(&&@class_member, ast_fold) -> @class_member,
     fold_item_underscore: fn@(item_, ast_fold) -> item_,
     fold_method: fn@(&&@method, ast_fold) -> @method,
     fold_block: fn@(blk_, span, ast_fold) -> (blk_, span),
     fold_stmt: fn@(stmt_, span, ast_fold) -> (stmt_, span),
     fold_arm: fn@(arm, ast_fold) -> arm,
     fold_pat: fn@(pat_, span, ast_fold) -> (pat_, span),
     fold_decl: fn@(decl_, span, ast_fold) -> (decl_, span),
     fold_expr: fn@(expr_, span, ast_fold) -> (expr_, span),
     fold_ty: fn@(ty_, span, ast_fold) -> (ty_, span),
     fold_constr: fn@(ast::constr_, span, ast_fold) -> (constr_, span),
     fold_ty_constr: fn@(ast::ty_constr_, span, ast_fold)
        -> (ty_constr_, span),
     fold_mod: fn@(_mod, ast_fold) -> _mod,
     fold_native_mod: fn@(native_mod, ast_fold) -> native_mod,
     fold_variant: fn@(variant_, span, ast_fold) -> (variant_, span),
     fold_ident: fn@(&&ident, ast_fold) -> ident,
     fold_path: fn@(path, ast_fold) -> path,
     fold_local: fn@(local_, span, ast_fold) -> (local_, span),
     map_exprs: fn@(fn@(&&@expr) -> @expr, [@expr]) -> [@expr],
     new_id: fn@(node_id) -> node_id,
     new_span: fn@(span) -> span};

type a_f =
    {fold_crate: fn@(crate) -> crate,
     fold_crate_directive: fn@(&&@crate_directive) -> @crate_directive,
     fold_view_item: fn@(&&@view_item) -> @view_item,
     fold_native_item: fn@(&&@native_item) -> @native_item,
     fold_item: fn@(&&@item) -> @item,
     fold_class_item: fn@(&&@class_member) -> @class_member,
     fold_item_underscore: fn@(item_) -> item_,
     fold_method: fn@(&&@method) -> @method,
     fold_block: fn@(blk) -> blk,
     fold_stmt: fn@(&&@stmt) -> @stmt,
     fold_arm: fn@(arm) -> arm,
     fold_pat: fn@(&&@pat) -> @pat,
     fold_decl: fn@(&&@decl) -> @decl,
     fold_expr: fn@(&&@expr) -> @expr,
     fold_ty: fn@(&&@ty) -> @ty,
     fold_constr: fn@(&&@constr) -> @constr,
     fold_ty_constr: fn@(&&@ty_constr) -> @ty_constr,
     fold_mod: fn@(_mod) -> _mod,
     fold_native_mod: fn@(native_mod) -> native_mod,
     fold_variant: fn@(variant) -> variant,
     fold_ident: fn@(&&ident) -> ident,
     fold_path: fn@(&&@path) -> @path,
     fold_local: fn@(&&@local) -> @local,
     map_exprs: fn@(fn@(&&@expr) -> @expr, [@expr]) -> [@expr],
     new_id: fn@(node_id) -> node_id,
     new_span: fn@(span) -> span};


//fn nf_dummy<T>(&T node) -> T { fail; }
fn nf_crate_dummy(_c: crate) -> crate { fail; }
fn nf_crate_directive_dummy(&&_c: @crate_directive) -> @crate_directive {
    fail;
}
fn nf_view_item_dummy(&&_v: @view_item) -> @view_item { fail; }
fn nf_native_item_dummy(&&_n: @native_item) -> @native_item { fail; }
fn nf_item_dummy(&&_i: @item) -> @item { fail; }
fn nf_class_item_dummy(&&_ci: @class_member) -> @class_member { fail; }
fn nf_item_underscore_dummy(_i: item_) -> item_ { fail; }
fn nf_method_dummy(&&_m: @method) -> @method { fail; }
fn nf_blk_dummy(_b: blk) -> blk { fail; }
fn nf_stmt_dummy(&&_s: @stmt) -> @stmt { fail; }
fn nf_arm_dummy(_a: arm) -> arm { fail; }
fn nf_pat_dummy(&&_p: @pat) -> @pat { fail; }
fn nf_decl_dummy(&&_d: @decl) -> @decl { fail; }
fn nf_expr_dummy(&&_e: @expr) -> @expr { fail; }
fn nf_ty_dummy(&&_t: @ty) -> @ty { fail; }
fn nf_constr_dummy(&&_c: @constr) -> @constr { fail; }
fn nf_ty_constr_dummy(&&_c: @ty_constr) -> @ty_constr { fail; }
fn nf_mod_dummy(_m: _mod) -> _mod { fail; }
fn nf_native_mod_dummy(_n: native_mod) -> native_mod { fail; }
fn nf_variant_dummy(_v: variant) -> variant { fail; }
fn nf_ident_dummy(&&_i: ident) -> ident { fail; }
fn nf_path_dummy(&&_p: @path) -> @path { fail; }
fn nf_local_dummy(&&_o: @local) -> @local { fail; }

/* some little folds that probably aren't useful to have in ast_fold itself*/

//used in noop_fold_item and noop_fold_crate and noop_fold_crate_directive
fn fold_meta_item_(&&mi: @meta_item, fld: ast_fold) -> @meta_item {
    ret @{node:
              alt mi.node {
                meta_word(id) { meta_word(fld.fold_ident(id)) }
                meta_list(id, mis) {
                  let fold_meta_item = bind fold_meta_item_(_, fld);
                  meta_list(id, vec::map(mis, fold_meta_item))
                }
                meta_name_value(id, s) {
                  meta_name_value(fld.fold_ident(id), s)
                }
              },
          span: fld.new_span(mi.span)};
}
//used in noop_fold_item and noop_fold_crate
fn fold_attribute_(at: attribute, fld: ast_fold) ->
   attribute {
    ret {node: {style: at.node.style,
                value: *fold_meta_item_(@at.node.value, fld)},
         span: fld.new_span(at.span)};
}
//used in noop_fold_native_item and noop_fold_fn_decl
fn fold_arg_(a: arg, fld: ast_fold) -> arg {
    ret {mode: a.mode,
         ty: fld.fold_ty(a.ty),
         ident: fld.fold_ident(a.ident),
         id: fld.new_id(a.id)};
}
//used in noop_fold_expr, and possibly elsewhere in the future
fn fold_mac_(m: mac, fld: ast_fold) -> mac {
    ret {node:
             alt m.node {
               mac_invoc(pth, arg, body) {
                 mac_invoc(fld.fold_path(pth),
                           option::map(arg, fld.fold_expr), body)
               }
               mac_embed_type(ty) { mac_embed_type(fld.fold_ty(ty)) }
               mac_embed_block(blk) { mac_embed_block(fld.fold_block(blk)) }
               mac_ellipsis { mac_ellipsis }
               mac_aq(_,_) { /* fixme */ m.node }
               mac_var(_) { /* fixme */ m.node }
             },
         span: fld.new_span(m.span)};
}

fn fold_fn_decl(decl: ast::fn_decl, fld: ast_fold) -> ast::fn_decl {
    ret {inputs: vec::map(decl.inputs, bind fold_arg_(_, fld)),
         output: fld.fold_ty(decl.output),
         purity: decl.purity,
         cf: decl.cf,
         constraints: vec::map(decl.constraints, fld.fold_constr)}
}

fn fold_ty_param_bound(tpb: ty_param_bound, fld: ast_fold) -> ty_param_bound {
    alt tpb {
      bound_copy | bound_send { tpb }
      bound_iface(ty) { bound_iface(fld.fold_ty(ty)) }
    }
}

fn fold_ty_param(tp: ty_param, fld: ast_fold) -> ty_param {
    {ident: tp.ident,
     id: fld.new_id(tp.id),
     bounds: @vec::map(*tp.bounds, fold_ty_param_bound(_, fld))}
}

fn fold_ty_params(tps: [ty_param], fld: ast_fold) -> [ty_param] {
    vec::map(tps, fold_ty_param(_, fld))
}

fn noop_fold_crate(c: crate_, fld: ast_fold) -> crate_ {
    let fold_meta_item = bind fold_meta_item_(_, fld);
    let fold_attribute = bind fold_attribute_(_, fld);

    ret {directives: vec::map(c.directives, fld.fold_crate_directive),
         module: fld.fold_mod(c.module),
         attrs: vec::map(c.attrs, fold_attribute),
         config: vec::map(c.config, fold_meta_item)};
}

fn noop_fold_crate_directive(cd: crate_directive_, fld: ast_fold) ->
   crate_directive_ {
    ret alt cd {
          cdir_src_mod(id, attrs) {
            cdir_src_mod(fld.fold_ident(id), attrs)
          }
          cdir_dir_mod(id, cds, attrs) {
            cdir_dir_mod(fld.fold_ident(id),
                         vec::map(cds, fld.fold_crate_directive), attrs)
          }
          cdir_view_item(vi) { cdir_view_item(fld.fold_view_item(vi)) }
          cdir_syntax(_) { cd }
        }
}

fn noop_fold_view_item(vi: view_item_, _fld: ast_fold) -> view_item_ {
    ret vi;
}


fn noop_fold_native_item(&&ni: @native_item, fld: ast_fold) -> @native_item {
    let fold_arg = bind fold_arg_(_, fld);
    let fold_attribute = bind fold_attribute_(_, fld);

    ret @{ident: fld.fold_ident(ni.ident),
          attrs: vec::map(ni.attrs, fold_attribute),
          node:
              alt ni.node {
                native_item_fn(fdec, typms) {
                  native_item_fn({inputs: vec::map(fdec.inputs, fold_arg),
                                  output: fld.fold_ty(fdec.output),
                                  purity: fdec.purity,
                                  cf: fdec.cf,
                                  constraints:
                                      vec::map(fdec.constraints,
                                               fld.fold_constr)},
                                 fold_ty_params(typms, fld))
                }
              },
          id: fld.new_id(ni.id),
          span: fld.new_span(ni.span)};
}

fn noop_fold_item(&&i: @item, fld: ast_fold) -> @item {
    let fold_attribute = bind fold_attribute_(_, fld);

    ret @{ident: fld.fold_ident(i.ident),
          attrs: vec::map(i.attrs, fold_attribute),
          id: fld.new_id(i.id),
          node: fld.fold_item_underscore(i.node),
          span: fld.new_span(i.span)};
}

fn noop_fold_class_item(&&ci: @class_member, fld: ast_fold)
    -> @class_member {
    @{node: alt ci.node {
        instance_var(ident, t, cm, id, p) {
           instance_var(ident, fld.fold_ty(t), cm, id, p)
        }
        class_method(m) { class_method(fld.fold_method(m)) }
      },
      span: ci.span}
}

fn noop_fold_item_underscore(i: item_, fld: ast_fold) -> item_ {
    ret alt i {
          item_const(t, e) { item_const(fld.fold_ty(t), fld.fold_expr(e)) }
          item_fn(decl, typms, body) {
              item_fn(fold_fn_decl(decl, fld),
                      fold_ty_params(typms, fld),
                      fld.fold_block(body))
          }
          item_mod(m) { item_mod(fld.fold_mod(m)) }
          item_native_mod(nm) { item_native_mod(fld.fold_native_mod(nm)) }
          item_ty(t, typms, rp) { item_ty(fld.fold_ty(t),
                                          fold_ty_params(typms, fld),
                                          rp) }
          item_enum(variants, typms, r) {
            item_enum(vec::map(variants, fld.fold_variant),
                      fold_ty_params(typms, fld),
                      r)
          }
          item_class(typms, ifaces, items, ctor, rp) {
              let ctor_body = fld.fold_block(ctor.node.body);
              let ctor_decl = fold_fn_decl(ctor.node.dec, fld);
              let ctor_id   = fld.new_id(ctor.node.id);
              item_class(
                  typms,
                  vec::map(ifaces, {|p| fold_iface_ref(p, fld) }),
                  vec::map(items, fld.fold_class_item),
                  {node: {body: ctor_body,
                          dec: ctor_decl,
                          id: ctor_id with ctor.node}
                   with ctor},
                  rp)
          }
          item_impl(tps, ifce, ty, methods) {
              item_impl(tps, option::map(ifce, {|p| fold_iface_ref(p, fld)}),
                        fld.fold_ty(ty),
                      vec::map(methods, fld.fold_method))
          }
          item_iface(tps, methods) { item_iface(tps, methods) }
          item_res(decl, typms, body, did, cid, rp) {
            item_res(fold_fn_decl(decl, fld),
                     fold_ty_params(typms, fld),
                     fld.fold_block(body),
                     fld.new_id(did),
                     fld.new_id(cid),
                     rp)
          }
        };
}

fn fold_iface_ref(&&p: @iface_ref, fld: ast_fold) -> @iface_ref {
    @{path: fld.fold_path(p.path), id: fld.new_id(p.id)}
}

fn noop_fold_method(&&m: @method, fld: ast_fold) -> @method {
    ret @{ident: fld.fold_ident(m.ident),
          attrs: m.attrs,
          tps: fold_ty_params(m.tps, fld),
          decl: fold_fn_decl(m.decl, fld),
          body: fld.fold_block(m.body),
          id: fld.new_id(m.id),
          span: fld.new_span(m.span),
          self_id: fld.new_id(m.self_id),
          privacy: m.privacy};
}


fn noop_fold_block(b: blk_, fld: ast_fold) -> blk_ {
    ret {view_items: vec::map(b.view_items, fld.fold_view_item),
         stmts: vec::map(b.stmts, fld.fold_stmt),
         expr: option::map(b.expr, fld.fold_expr),
         id: fld.new_id(b.id),
         rules: b.rules};
}

fn noop_fold_stmt(s: stmt_, fld: ast_fold) -> stmt_ {
    ret alt s {
      stmt_decl(d, nid) { stmt_decl(fld.fold_decl(d), fld.new_id(nid)) }
      stmt_expr(e, nid) { stmt_expr(fld.fold_expr(e), fld.new_id(nid)) }
      stmt_semi(e, nid) { stmt_semi(fld.fold_expr(e), fld.new_id(nid)) }
    };
}

fn noop_fold_arm(a: arm, fld: ast_fold) -> arm {
    ret {pats: vec::map(a.pats, fld.fold_pat),
         guard: option::map(a.guard, fld.fold_expr),
         body: fld.fold_block(a.body)};
}

fn noop_fold_pat(p: pat_, fld: ast_fold) -> pat_ {
    ret alt p {
          pat_wild { p }
          pat_ident(pth, sub) {
            pat_ident(fld.fold_path(pth), option::map(sub, fld.fold_pat))
          }
          pat_lit(e) { pat_lit(fld.fold_expr(e)) }
          pat_enum(pth, pats) {
              pat_enum(fld.fold_path(pth), option::map(pats)
                       {|pats| vec::map(pats, fld.fold_pat)})
          }
          pat_rec(fields, etc) {
            let mut fs = [];
            for fields.each {|f|
                fs += [{ident: f.ident, pat: fld.fold_pat(f.pat)}];
            }
            pat_rec(fs, etc)
          }
          pat_tup(elts) { pat_tup(vec::map(elts, fld.fold_pat)) }
          pat_box(inner) { pat_box(fld.fold_pat(inner)) }
          pat_uniq(inner) { pat_uniq(fld.fold_pat(inner)) }
          pat_range(e1, e2) {
            pat_range(fld.fold_expr(e1), fld.fold_expr(e2))
          }
        };
}

fn noop_fold_decl(d: decl_, fld: ast_fold) -> decl_ {
    alt d {
      decl_local(ls) { decl_local(vec::map(ls, fld.fold_local)) }
      decl_item(it) { decl_item(fld.fold_item(it)) }
    }
}

fn wrap<T>(f: fn@(T, ast_fold) -> T)
    -> fn@(T, span, ast_fold) -> (T, span)
{
    ret fn@(x: T, s: span, fld: ast_fold) -> (T, span) {
        (f(x, fld), s)
    }
}

fn noop_fold_expr(e: expr_, fld: ast_fold) -> expr_ {
    fn fold_field_(field: field, fld: ast_fold) -> field {
        ret {node:
                 {mutbl: field.node.mutbl,
                  ident: fld.fold_ident(field.node.ident),
                  expr: fld.fold_expr(field.node.expr)},
             span: fld.new_span(field.span)};
    }
    let fold_field = bind fold_field_(_, fld);

    let fold_mac = bind fold_mac_(_, fld);

    ret alt e {
          expr_new(p, i, v) {
            expr_new(fld.fold_expr(p),
                     fld.new_id(i),
                     fld.fold_expr(v))
          }
          expr_vstore(e, v) {
            expr_vstore(fld.fold_expr(e), v)
          }
          expr_vec(exprs, mutt) {
            expr_vec(fld.map_exprs(fld.fold_expr, exprs), mutt)
          }
          expr_rec(fields, maybe_expr) {
            expr_rec(vec::map(fields, fold_field),
                     option::map(maybe_expr, fld.fold_expr))
          }
          expr_tup(elts) { expr_tup(vec::map(elts, fld.fold_expr)) }
          expr_call(f, args, blk) {
            expr_call(fld.fold_expr(f), fld.map_exprs(fld.fold_expr, args),
                      blk)
          }
          expr_bind(f, args) {
            let opt_map_se = bind option::map(_, fld.fold_expr);
            expr_bind(fld.fold_expr(f), vec::map(args, opt_map_se))
          }
          expr_binary(binop, lhs, rhs) {
            expr_binary(binop, fld.fold_expr(lhs), fld.fold_expr(rhs))
          }
          expr_unary(binop, ohs) { expr_unary(binop, fld.fold_expr(ohs)) }
          expr_loop_body(f) { expr_loop_body(fld.fold_expr(f)) }
          expr_lit(_) { e }
          expr_cast(expr, ty) { expr_cast(fld.fold_expr(expr), ty) }
          expr_addr_of(m, ohs) { expr_addr_of(m, fld.fold_expr(ohs)) }
          expr_if(cond, tr, fl) {
            expr_if(fld.fold_expr(cond), fld.fold_block(tr),
                    option::map(fl, fld.fold_expr))
          }
          expr_while(cond, body) {
            expr_while(fld.fold_expr(cond), fld.fold_block(body))
          }
          expr_do_while(blk, expr) {
            expr_do_while(fld.fold_block(blk), fld.fold_expr(expr))
          }
          expr_loop(body) {
              expr_loop(fld.fold_block(body))
          }
          expr_alt(expr, arms, mode) {
            expr_alt(fld.fold_expr(expr), vec::map(arms, fld.fold_arm), mode)
          }
          expr_fn(proto, decl, body, captures) {
              expr_fn(proto, fold_fn_decl(decl, fld),
                      fld.fold_block(body), captures)
          }
          expr_fn_block(decl, body) {
            expr_fn_block(fold_fn_decl(decl, fld), fld.fold_block(body))
          }
          expr_block(blk) { expr_block(fld.fold_block(blk)) }
          expr_move(el, er) {
            expr_move(fld.fold_expr(el), fld.fold_expr(er))
          }
          expr_copy(e) { expr_copy(fld.fold_expr(e)) }
          expr_assign(el, er) {
            expr_assign(fld.fold_expr(el), fld.fold_expr(er))
          }
          expr_swap(el, er) {
            expr_swap(fld.fold_expr(el), fld.fold_expr(er))
          }
          expr_assign_op(op, el, er) {
            expr_assign_op(op, fld.fold_expr(el), fld.fold_expr(er))
          }
          expr_field(el, id, tys) {
            expr_field(fld.fold_expr(el), fld.fold_ident(id),
                       vec::map(tys, fld.fold_ty))
          }
          expr_index(el, er) {
            expr_index(fld.fold_expr(el), fld.fold_expr(er))
          }
          expr_path(pth) { expr_path(fld.fold_path(pth)) }
          expr_fail(e) { expr_fail(option::map(e, fld.fold_expr)) }
          expr_break | expr_cont { e }
          expr_ret(e) { expr_ret(option::map(e, fld.fold_expr)) }
          expr_be(e) { expr_be(fld.fold_expr(e)) }
          expr_log(i, lv, e) { expr_log(i, fld.fold_expr(lv),
                                        fld.fold_expr(e)) }
          expr_assert(e) { expr_assert(fld.fold_expr(e)) }
          expr_check(m, e) { expr_check(m, fld.fold_expr(e)) }
          expr_if_check(cond, tr, fl) {
            expr_if_check(fld.fold_expr(cond), fld.fold_block(tr),
                          option::map(fl, fld.fold_expr))
          }
          expr_mac(mac) { expr_mac(fold_mac(mac)) }
        }
}

fn noop_fold_ty(t: ty_, fld: ast_fold) -> ty_ {
    let fold_mac = bind fold_mac_(_, fld);
    fn fold_mt(mt: mt, fld: ast_fold) -> mt {
        {ty: fld.fold_ty(mt.ty), mutbl: mt.mutbl}
    }
    fn fold_field(f: ty_field, fld: ast_fold) -> ty_field {
        {node: {ident: fld.fold_ident(f.node.ident),
                mt: fold_mt(f.node.mt, fld)},
         span: fld.new_span(f.span)}
    }
    alt t {
      ty_nil | ty_bot {t}
      ty_box(mt) {ty_box(fold_mt(mt, fld))}
      ty_uniq(mt) {ty_uniq(fold_mt(mt, fld))}
      ty_vec(mt) {ty_vec(fold_mt(mt, fld))}
      ty_ptr(mt) {ty_ptr(fold_mt(mt, fld))}
      ty_rptr(region, mt) {ty_rptr(region, fold_mt(mt, fld))}
      ty_rec(fields) {ty_rec(vec::map(fields) {|f| fold_field(f, fld)})}
      ty_fn(proto, decl) {ty_fn(proto, fold_fn_decl(decl, fld))}
      ty_tup(tys) {ty_tup(vec::map(tys) {|ty| fld.fold_ty(ty)})}
      ty_path(path, id) {ty_path(fld.fold_path(path), fld.new_id(id))}
      ty_constr(ty, constrs) {ty_constr(fld.fold_ty(ty),
                                vec::map(constrs, fld.fold_ty_constr))}
      ty_vstore(t, vs) {ty_vstore(fld.fold_ty(t), vs)}
      ty_mac(mac) {ty_mac(fold_mac(mac))}
      ty_infer {t}
    }
}

fn noop_fold_constr(c: constr_, fld: ast_fold) -> constr_ {
    {path: fld.fold_path(c.path), args: c.args, id: fld.new_id(c.id)}
}

fn noop_fold_ty_constr(c: ty_constr_, fld: ast_fold) -> ty_constr_ {
    let rslt: ty_constr_ =
        {path: fld.fold_path(c.path), args: c.args, id: fld.new_id(c.id)};
    rslt
}
// ...nor do modules
fn noop_fold_mod(m: _mod, fld: ast_fold) -> _mod {
    ret {view_items: vec::map(m.view_items, fld.fold_view_item),
         items: vec::map(m.items, fld.fold_item)};
}

fn noop_fold_native_mod(nm: native_mod, fld: ast_fold) -> native_mod {
    ret {view_items: vec::map(nm.view_items, fld.fold_view_item),
         items: vec::map(nm.items, fld.fold_native_item)}
}

fn noop_fold_variant(v: variant_, fld: ast_fold) -> variant_ {
    fn fold_variant_arg_(va: variant_arg, fld: ast_fold) -> variant_arg {
        ret {ty: fld.fold_ty(va.ty), id: fld.new_id(va.id)};
    }
    let fold_variant_arg = bind fold_variant_arg_(_, fld);
    let args = vec::map(v.args, fold_variant_arg);

    let fold_attribute = bind fold_attribute_(_, fld);
    let attrs = vec::map(v.attrs, fold_attribute);

    let de = alt v.disr_expr {
      some(e) {some(fld.fold_expr(e))}
      none {none}
    };
    ret {name: v.name,
         attrs: attrs,
         args: args, id: fld.new_id(v.id),
         disr_expr: de};
}

fn noop_fold_ident(&&i: ident, _fld: ast_fold) -> ident { ret i; }

fn noop_fold_path(&&p: path, fld: ast_fold) -> path {
    ret {span: fld.new_span(p.span), global: p.global,
         idents: vec::map(p.idents, fld.fold_ident),
         types: vec::map(p.types, fld.fold_ty)};
}

fn noop_fold_local(l: local_, fld: ast_fold) -> local_ {
    ret {is_mutbl: l.is_mutbl,
         ty: fld.fold_ty(l.ty),
         pat: fld.fold_pat(l.pat),
         init:
             alt l.init {
               option::none::<initializer> { l.init }
               option::some::<initializer>(init) {
                 option::some::<initializer>({op: init.op,
                                              expr: fld.fold_expr(init.expr)})
               }
             },
         id: fld.new_id(l.id)};
}

/* temporarily eta-expand because of a compiler bug with using `fn<T>` as a
   value */
fn noop_map_exprs(f: fn@(&&@expr) -> @expr, es: [@expr]) -> [@expr] {
    ret vec::map(es, f);
}

fn noop_id(i: node_id) -> node_id { ret i; }

fn noop_span(sp: span) -> span { ret sp; }


fn default_ast_fold() -> @ast_fold_precursor {
    ret @{fold_crate: wrap(noop_fold_crate),
          fold_crate_directive: wrap(noop_fold_crate_directive),
          fold_view_item: noop_fold_view_item,
          fold_native_item: noop_fold_native_item,
          fold_item: noop_fold_item,
          fold_class_item: noop_fold_class_item,
          fold_item_underscore: noop_fold_item_underscore,
          fold_method: noop_fold_method,
          fold_block: wrap(noop_fold_block),
          fold_stmt: wrap(noop_fold_stmt),
          fold_arm: noop_fold_arm,
          fold_pat: wrap(noop_fold_pat),
          fold_decl: wrap(noop_fold_decl),
          fold_expr: wrap(noop_fold_expr),
          fold_ty: wrap(noop_fold_ty),
          fold_constr: wrap(noop_fold_constr),
          fold_ty_constr: wrap(noop_fold_ty_constr),
          fold_mod: noop_fold_mod,
          fold_native_mod: noop_fold_native_mod,
          fold_variant: wrap(noop_fold_variant),
          fold_ident: noop_fold_ident,
          fold_path: noop_fold_path,
          fold_local: wrap(noop_fold_local),
          map_exprs: noop_map_exprs,
          new_id: noop_id,
          new_span: noop_span};
}

fn make_fold(afp: ast_fold_precursor) -> ast_fold {
    // FIXME: Have to bind all the bare functions into shared functions
    // because @mut is invariant with respect to its contents
    // I assume this has something to do with Issue #1973 - tjc
    let result: ast_fold =
        @mut {fold_crate: bind nf_crate_dummy(_),
                  fold_crate_directive: bind nf_crate_directive_dummy(_),
                  fold_view_item: bind nf_view_item_dummy(_),
                  fold_native_item: bind nf_native_item_dummy(_),
                  fold_item: bind nf_item_dummy(_),
                  fold_class_item: bind nf_class_item_dummy(_),
                  fold_item_underscore: bind nf_item_underscore_dummy(_),
                  fold_method: bind nf_method_dummy(_),
                  fold_block: bind nf_blk_dummy(_),
                  fold_stmt: bind nf_stmt_dummy(_),
                  fold_arm: bind nf_arm_dummy(_),
                  fold_pat: bind nf_pat_dummy(_),
                  fold_decl: bind nf_decl_dummy(_),
                  fold_expr: bind nf_expr_dummy(_),
                  fold_ty: bind nf_ty_dummy(_),
                  fold_constr: bind nf_constr_dummy(_),
                  fold_ty_constr: bind nf_ty_constr_dummy(_),
                  fold_mod: bind nf_mod_dummy(_),
                  fold_native_mod: bind nf_native_mod_dummy(_),
                  fold_variant: bind nf_variant_dummy(_),
                  fold_ident: bind nf_ident_dummy(_),
                  fold_path: bind nf_path_dummy(_),
                  fold_local: bind nf_local_dummy(_),
                  map_exprs: bind noop_map_exprs(_, _),
                  new_id: bind noop_id(_),
                  new_span: bind noop_span(_)};

    /* naturally, a macro to write these would be nice */
    fn f_crate(afp: ast_fold_precursor, f: ast_fold, c: crate) -> crate {
        let (n, s) = afp.fold_crate(c.node, c.span, f);
        ret {node: n, span: afp.new_span(s)};
    }
    fn f_crate_directive(afp: ast_fold_precursor, f: ast_fold,
                         &&c: @crate_directive) -> @crate_directive {
        let (n, s) = afp.fold_crate_directive(c.node, c.span, f);
        ret @{node: n,
              span: afp.new_span(s)};
    }
    fn f_view_item(afp: ast_fold_precursor, f: ast_fold, &&x: @view_item) ->
       @view_item {
        ret @{node: afp.fold_view_item(x.node, f),
              span: afp.new_span(x.span)};
    }
    fn f_native_item(afp: ast_fold_precursor, f: ast_fold, &&x: @native_item)
        -> @native_item {
        ret afp.fold_native_item(x, f);
    }
    fn f_item(afp: ast_fold_precursor, f: ast_fold, &&i: @item) -> @item {
        ret afp.fold_item(i, f);
    }
    fn f_class_item(afp: ast_fold_precursor, f: ast_fold,
                      &&ci: @class_member) -> @class_member {
        @{node: alt ci.node {
           instance_var(nm, t, mt, id, p) {
               instance_var(nm, f_ty(afp, f, t),
                            mt, id, p)
           }
           class_method(m) {
               class_method(afp.fold_method(m, f))
           }
          }, span: afp.new_span(ci.span)}
    }
    fn f_item_underscore(afp: ast_fold_precursor, f: ast_fold, i: item_) ->
       item_ {
        ret afp.fold_item_underscore(i, f);
    }
    fn f_method(afp: ast_fold_precursor, f: ast_fold, &&x: @method)
        -> @method {
        ret afp.fold_method(x, f);
    }
    fn f_block(afp: ast_fold_precursor, f: ast_fold, x: blk) -> blk {
        let (n, s) = afp.fold_block(x.node, x.span, f);
        ret {node: n, span: afp.new_span(s)};
    }
    fn f_stmt(afp: ast_fold_precursor, f: ast_fold, &&x: @stmt) -> @stmt {
        let (n, s) = afp.fold_stmt(x.node, x.span, f);
        ret @{node: n, span: afp.new_span(s)};
    }
    fn f_arm(afp: ast_fold_precursor, f: ast_fold, x: arm) -> arm {
        ret afp.fold_arm(x, f);
    }
    fn f_pat(afp: ast_fold_precursor, f: ast_fold, &&x: @pat) -> @pat {
        let (n, s) =  afp.fold_pat(x.node, x.span, f);
        ret @{id: afp.new_id(x.id),
              node: n,
              span: afp.new_span(s)};
    }
    fn f_decl(afp: ast_fold_precursor, f: ast_fold, &&x: @decl) -> @decl {
        let (n, s) = afp.fold_decl(x.node, x.span, f);
        ret @{node: n, span: afp.new_span(s)};
    }
    fn f_expr(afp: ast_fold_precursor, f: ast_fold, &&x: @expr) -> @expr {
        let (n, s) = afp.fold_expr(x.node, x.span, f);
        ret @{id: afp.new_id(x.id),
              node: n,
              span: afp.new_span(s)};
    }
    fn f_ty(afp: ast_fold_precursor, f: ast_fold, &&x: @ty) -> @ty {
        let (n, s) = afp.fold_ty(x.node, x.span, f);
        ret @{id: afp.new_id(x.id), node: n, span: afp.new_span(s)};
    }
    fn f_constr(afp: ast_fold_precursor, f: ast_fold, &&x: @ast::constr) ->
       @ast::constr {
        let (n, s) = afp.fold_constr(x.node, x.span, f);
        ret @{node: n, span: afp.new_span(s)};
    }
    fn f_ty_constr(afp: ast_fold_precursor, f: ast_fold,
                   &&x: @ast::ty_constr) ->
       @ast::ty_constr {
        let (n, s) : (ty_constr_, span) =
            afp.fold_ty_constr(x.node, x.span, f);
        ret @{node: n, span: afp.new_span(s)};
    }
    fn f_mod(afp: ast_fold_precursor, f: ast_fold, x: _mod) -> _mod {
        ret afp.fold_mod(x, f);
    }
    fn f_native_mod(afp: ast_fold_precursor, f: ast_fold, x: native_mod) ->
       native_mod {
        ret afp.fold_native_mod(x, f);
    }
    fn f_variant(afp: ast_fold_precursor, f: ast_fold, x: variant) ->
       variant {
        let (n, s) = afp.fold_variant(x.node, x.span, f);
        ret {node: n, span: afp.new_span(s)};
    }
    fn f_ident(afp: ast_fold_precursor, f: ast_fold, &&x: ident) -> ident {
        ret afp.fold_ident(x, f);
    }
    fn f_path(afp: ast_fold_precursor, f: ast_fold, &&x: @path) -> @path {
        @afp.fold_path(*x, f)
    }
    fn f_local(afp: ast_fold_precursor, f: ast_fold, &&x: @local) -> @local {
        let (n, s) = afp.fold_local(x.node, x.span, f);
        ret @{node: n, span: afp.new_span(s)};
    }

    *result =
        {fold_crate: bind f_crate(afp, result, _),
         fold_crate_directive: bind f_crate_directive(afp, result, _),
         fold_view_item: bind f_view_item(afp, result, _),
         fold_native_item: bind f_native_item(afp, result, _),
         fold_item: bind f_item(afp, result, _),
         fold_class_item: bind f_class_item(afp, result, _),
         fold_item_underscore: bind f_item_underscore(afp, result, _),
         fold_method: bind f_method(afp, result, _),
         fold_block: bind f_block(afp, result, _),
         fold_stmt: bind f_stmt(afp, result, _),
         fold_arm: bind f_arm(afp, result, _),
         fold_pat: bind f_pat(afp, result, _),
         fold_decl: bind f_decl(afp, result, _),
         fold_expr: bind f_expr(afp, result, _),
         fold_ty: bind f_ty(afp, result, _),
         fold_constr: bind f_constr(afp, result, _),
         fold_ty_constr: bind f_ty_constr(afp, result, _),
         fold_mod: bind f_mod(afp, result, _),
         fold_native_mod: bind f_native_mod(afp, result, _),
         fold_variant: bind f_variant(afp, result, _),
         fold_ident: bind f_ident(afp, result, _),
         fold_path: bind f_path(afp, result, _),
         fold_local: bind f_local(afp, result, _),
         map_exprs: afp.map_exprs,
         new_id: afp.new_id,
         new_span: afp.new_span};
    ret result;
}

//
// Local Variables:
// mode: rust
// fill-column: 78;
// indent-tabs-mode: nil
// c-basic-offset: 4
// buffer-file-coding-system: utf-8-unix
// End:
//
