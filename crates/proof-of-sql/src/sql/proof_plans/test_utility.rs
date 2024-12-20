use super::{DynProofPlan, FilterExec, GroupByExec, ProjectionExec};
use crate::sql::proof_exprs::{AliasedDynProofExpr, ColumnExpr, DynProofExpr, TableExpr};

pub fn projection(results: Vec<AliasedDynProofExpr>, table: TableExpr) -> DynProofPlan {
    DynProofPlan::Projection(ProjectionExec::new(results, table))
}

pub fn filter(
    results: Vec<AliasedDynProofExpr>,
    table: TableExpr,
    where_clause: DynProofExpr,
) -> DynProofPlan {
    DynProofPlan::Filter(FilterExec::new(results, table, where_clause))
}

/// # Panics
///
/// Will panic if `count_alias` cannot be parsed as a valid identifier.
pub fn group_by(
    group_by_exprs: Vec<ColumnExpr>,
    sum_expr: Vec<AliasedDynProofExpr>,
    count_alias: &str,
    table: TableExpr,
    where_clause: DynProofExpr,
) -> DynProofPlan {
    DynProofPlan::GroupBy(GroupByExec::new(
        group_by_exprs,
        sum_expr,
        count_alias.parse().unwrap(),
        table,
        where_clause,
    ))
}
