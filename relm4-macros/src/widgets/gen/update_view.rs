use proc_macro2::TokenStream as TokenStream2;
use quote::{quote, quote_spanned};
use syn::spanned::Spanned;
use syn::{Ident, Path};

use crate::widgets::{
    AssignProperty, AssignPropertyAttr, ConditionalBranches, ConditionalWidget, MatchArm,
    Properties, Property, PropertyName, PropertyType, ReturnedWidget, Widget,
};

impl Property {
    fn update_view_stream(
        &self,
        stream: &mut TokenStream2,
        w_name: &Ident,
        model_name: &Ident,
        conditional_branch: bool,
        relm4_path: &Path,
    ) {
        match &self.ty {
            PropertyType::Assign(assign) => assign.update_view_stream(
                stream,
                &self.name,
                w_name,
                model_name,
                conditional_branch,
                relm4_path,
            ),
            PropertyType::Widget(widget) => {
                widget.update_view_stream(stream, model_name, conditional_branch, relm4_path)
            }
            PropertyType::ConditionalWidget(cond_widget) => {
                cond_widget.update_view_stream(stream, model_name, relm4_path)
            }
            PropertyType::SignalHandler(_) | PropertyType::ParseError(_) => (),
        }
    }
}

impl Properties {
    fn update_view_stream(
        &self,
        stream: &mut TokenStream2,
        w_name: &Ident,
        model_name: &Ident,
        conditional_branch: bool,
        relm4_path: &Path,
    ) {
        for prop in &self.properties {
            prop.update_view_stream(stream, w_name, model_name, conditional_branch, relm4_path);
        }
    }
}

impl Widget {
    pub fn init_update_view_stream(
        &self,
        stream: &mut TokenStream2,
        model_name: &Ident,
        relm4_path: &Path,
    ) {
        self.update_view_stream(stream, model_name, false, relm4_path);
    }

    fn update_view_stream(
        &self,
        stream: &mut TokenStream2,
        model_name: &Ident,
        conditional_branch: bool,
        relm4_path: &Path,
    ) {
        let w_name = &self.name;
        self.properties.update_view_stream(
            stream,
            w_name,
            model_name,
            conditional_branch,
            relm4_path,
        );
        if let Some(returned_widget) = &self.returned_widget {
            returned_widget.update_view_stream(stream, model_name, conditional_branch, relm4_path);
        }
    }
}

impl ConditionalWidget {
    fn update_view_stream(&self, stream: &mut TokenStream2, model_name: &Ident, relm4_path: &Path) {
        let brach_stream = match &self.branches {
            ConditionalBranches::If(if_branches) => {
                let mut stream = TokenStream2::new();

                for (index, branch) in if_branches.iter().enumerate() {
                    let mut inner_update_stream = TokenStream2::new();
                    branch.widget.update_view_stream(
                        &mut inner_update_stream,
                        model_name,
                        true,
                        relm4_path,
                    );
                    branch.update_stream(&mut stream, inner_update_stream, index);
                }
                stream
            }
            ConditionalBranches::Match((match_token, expr, match_arms)) => {
                let mut inner_tokens = TokenStream2::new();
                for (index, match_arm) in match_arms.iter().enumerate() {
                    let mut inner_update_stream = TokenStream2::new();
                    match_arm.widget.update_view_stream(
                        &mut inner_update_stream,
                        model_name,
                        true,
                        relm4_path,
                    );
                    let MatchArm {
                        pattern,
                        guard,
                        arrow,
                        ..
                    } = match_arm;
                    let (guard_if, guard_expr) = if let Some((guard_if, guard_expr)) = guard {
                        (Some(guard_if), Some(guard_expr))
                    } else {
                        (None, None)
                    };

                    let index = index.to_string();
                    inner_tokens.extend(quote! {
                        #pattern #guard_if #guard_expr #arrow {
                            let __page_active: bool = (__current_page == #index);
                            #inner_update_stream
                            #index
                        },
                    })
                }
                quote! {
                    #match_token #expr {
                        #inner_tokens
                    }
                }
            }
        };

        let w_name = &self.name;
        stream.extend(quote_spanned! {
            w_name.span() =>
            let __current_page = #w_name.visible_child_name().map_or("".to_string(), |s| s.as_str().to_string());
            #w_name.set_visible_child_name(#brach_stream);
        });
    }
}

impl ReturnedWidget {
    fn update_view_stream(
        &self,
        stream: &mut TokenStream2,
        model_name: &Ident,
        conditional_branch: bool,
        relm4_path: &Path,
    ) {
        let w_name = &self.name;
        self.properties.update_view_stream(
            stream,
            w_name,
            model_name,
            conditional_branch,
            relm4_path,
        );
    }
}

impl AssignProperty {
    fn update_view_stream(
        &self,
        stream: &mut TokenStream2,
        p_name: &PropertyName,
        w_name: &Ident,
        model_name: &Ident,
        conditional_branch: bool,
        relm4_path: &Path,
    ) {
        match &self.attr {
            AssignPropertyAttr::None => (),
            AssignPropertyAttr::Watch => {
                self.assign_stream(stream, p_name, w_name, relm4_path);
            }
            AssignPropertyAttr::Track((track_stream, paste_model)) => {
                let mut assign_stream = TokenStream2::new();
                self.assign_stream(&mut assign_stream, p_name, w_name, relm4_path);
                let model = paste_model.then(|| model_name);
                let page_switch = conditional_branch.then(|| {
                    quote_spanned! {
                        p_name.span() =>
                            !__page_active ||
                    }
                });

                stream.extend(quote! {
                    if #page_switch (#model #track_stream) {
                        #assign_stream
                    }
                });
            }
        }
    }
}