use clippy_utils::path_def_id;
use clippy_utils::ty::is_type_diagnostic_item;
use regex_syntax::hir::Literal;
use rustc_lint::{LateContext, LateLintPass};
use clippy_utils::consts::{constant_context, Constant};
use clippy_utils::diagnostics::span_lint_and_sugg;
use clippy_utils::source::snippet;
use rustc_hir::{Expr, ExprKind};
use rustc_session::{declare_lint_pass, declare_tool_lint};
use if_chain::if_chain;
use rustc_middle::ty;
use rustc_span::sym;
use rustc_span::source_map::Spanned;
use rustc_errors::Applicability;
use rustc_ast::ast::LitKind;
use rustc_ast::ast::Path;
use rustc_ast::ast::TyKind;
use rustc_hir::QPath::Resolved;
use clippy_utils::{is_trait_method, match_trait_method, paths};

declare_clippy_lint! {
    /// ### What it does
    ///
    /// ### Why is this bad?
    ///
    /// ### Example
    /// ```rust
    /// // example code where clippy issues a warning
    /// ```
    /// Use instead:
    /// ```rust
    /// // example code which does not raise clippy warning
    /// ```
    #[clippy::version = "1.64.0"]
    pub SEEK_INSTEAD_OF_REWIND,
    complexity,
    "default lint description"
}
declare_lint_pass!(SeekInsteadOfRewind => [SEEK_INSTEAD_OF_REWIND]);

impl <'tcx>LateLintPass<'tcx> for SeekInsteadOfRewind {
    fn check_expr(&mut self, cx: &LateContext<'_>, expr: &'tcx Expr<'_>) {
        if_chain! {
            if let ExprKind::MethodCall(path, [receiver, count], _) = &expr.kind;
            let ty = cx.typeck_results().expr_ty(count);

            if let Expr { kind: ExprKind::Call ( Expr { kind, .. }, [arg]), ..} = count;

            // Check if an argument for StartSeek is zero
            if let Expr { kind: ExprKind::Lit (Spanned { node: LitKind::Int(0, ..), .. }), ..} = arg;

            //if let ExprKind::Path( Resolved (None, Path { segments, .. } ) ) = kind;
            if let ExprKind::Path( Resolved( None, rustc_hir::Path{ segments, ..} ) ) = kind;

            if path.ident.name == sym!(seek);
            if !receiver.span.from_expansion();
            then {
                if is_trait_method(cx, expr, sym::Iterator) {
                    eprintln!("YES SEEK \n");
                } else {
                    eprintln!("NO SEEK\n");
                }
                eprintln!("RECEIVER{:?}\n \n", receiver);
                eprintln!("COUNT START{:?}\n COUNTEND\n", count);
                eprintln!("ARG START{:?}\n ARG\n", arg);
                eprintln!("SEGMENT START{:?}\n SEGMENT\n", segments);
                eprintln!("\n\nAAAAAAAAAAAAAAA{:?}", ty);
 
                eprintln!("\n\nexpr{:?}", constant_context(cx, cx.typeck_results()).expr(count));

                //if ty == "std::io::Start" {
                    // The type is an `Option`
                //}

                span_lint_and_sugg(
                    cx,
                    SEEK_INSTEAD_OF_REWIND,
                    expr.span,
                    "using `Seek::from::Start(0)` with `seek()`",
                    "consider using `.rewind()` instead",
                    format!("{}.seek()", snippet(cx, receiver.span, r#""...""#)),
                    Applicability::MachineApplicable,
                );
            }
        }
    }
}
