#![allow(nonstandard_style)]
// Generated from grammar/OutboundAPIParser.g4 by ANTLR 4.13.2
use antlr4rust::tree::ParseTreeListener;
use super::outboundapiparser::*;

pub trait OutboundAPIParserListener<'input> : ParseTreeListener<'input,OutboundAPIParserContextType>{
/**
 * Enter a parse tree produced by {@link OutboundAPIParser#expressionsSection}.
 * @param ctx the parse tree
 */
fn enter_expressionsSection(&mut self, _ctx: &ExpressionsSectionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link OutboundAPIParser#expressionsSection}.
 * @param ctx the parse tree
 */
fn exit_expressionsSection(&mut self, _ctx: &ExpressionsSectionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link OutboundAPIParser#keyFilterSection}.
 * @param ctx the parse tree
 */
fn enter_keyFilterSection(&mut self, _ctx: &KeyFilterSectionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link OutboundAPIParser#keyFilterSection}.
 * @param ctx the parse tree
 */
fn exit_keyFilterSection(&mut self, _ctx: &KeyFilterSectionContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code IdPointInTimeArithmeticComparison}
 * labeled alternative in {@link OutboundAPIParser#keyComparison}.
 * @param ctx the parse tree
 */
fn enter_IdPointInTimeArithmeticComparison(&mut self, _ctx: &IdPointInTimeArithmeticComparisonContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code IdPointInTimeArithmeticComparison}
 * labeled alternative in {@link OutboundAPIParser#keyComparison}.
 * @param ctx the parse tree
 */
fn exit_IdPointInTimeArithmeticComparison(&mut self, _ctx: &IdPointInTimeArithmeticComparisonContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code IdTimeIntervalIn}
 * labeled alternative in {@link OutboundAPIParser#keyComparison}.
 * @param ctx the parse tree
 */
fn enter_IdTimeIntervalIn(&mut self, _ctx: &IdTimeIntervalInContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code IdTimeIntervalIn}
 * labeled alternative in {@link OutboundAPIParser#keyComparison}.
 * @param ctx the parse tree
 */
fn exit_IdTimeIntervalIn(&mut self, _ctx: &IdTimeIntervalInContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code IdNumericComparison}
 * labeled alternative in {@link OutboundAPIParser#keyComparison}.
 * @param ctx the parse tree
 */
fn enter_IdNumericComparison(&mut self, _ctx: &IdNumericComparisonContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code IdNumericComparison}
 * labeled alternative in {@link OutboundAPIParser#keyComparison}.
 * @param ctx the parse tree
 */
fn exit_IdNumericComparison(&mut self, _ctx: &IdNumericComparisonContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code IdLatestGlobalComparison}
 * labeled alternative in {@link OutboundAPIParser#keyComparison}.
 * @param ctx the parse tree
 */
fn enter_IdLatestGlobalComparison(&mut self, _ctx: &IdLatestGlobalComparisonContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code IdLatestGlobalComparison}
 * labeled alternative in {@link OutboundAPIParser#keyComparison}.
 * @param ctx the parse tree
 */
fn exit_IdLatestGlobalComparison(&mut self, _ctx: &IdLatestGlobalComparisonContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code IdTimeIntervalToPointInTimeComparison}
 * labeled alternative in {@link OutboundAPIParser#keyComparison}.
 * @param ctx the parse tree
 */
fn enter_IdTimeIntervalToPointInTimeComparison(&mut self, _ctx: &IdTimeIntervalToPointInTimeComparisonContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code IdTimeIntervalToPointInTimeComparison}
 * labeled alternative in {@link OutboundAPIParser#keyComparison}.
 * @param ctx the parse tree
 */
fn exit_IdTimeIntervalToPointInTimeComparison(&mut self, _ctx: &IdTimeIntervalToPointInTimeComparisonContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code IdLatestComparison}
 * labeled alternative in {@link OutboundAPIParser#keyComparison}.
 * @param ctx the parse tree
 */
fn enter_IdLatestComparison(&mut self, _ctx: &IdLatestComparisonContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code IdLatestComparison}
 * labeled alternative in {@link OutboundAPIParser#keyComparison}.
 * @param ctx the parse tree
 */
fn exit_IdLatestComparison(&mut self, _ctx: &IdLatestComparisonContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code TextComparison}
 * labeled alternative in {@link OutboundAPIParser#keyComparison}.
 * @param ctx the parse tree
 */
fn enter_TextComparison(&mut self, _ctx: &TextComparisonContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code TextComparison}
 * labeled alternative in {@link OutboundAPIParser#keyComparison}.
 * @param ctx the parse tree
 */
fn exit_TextComparison(&mut self, _ctx: &TextComparisonContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code RankOver}
 * labeled alternative in {@link OutboundAPIParser#keyComparison}.
 * @param ctx the parse tree
 */
fn enter_RankOver(&mut self, _ctx: &RankOverContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code RankOver}
 * labeled alternative in {@link OutboundAPIParser#keyComparison}.
 * @param ctx the parse tree
 */
fn exit_RankOver(&mut self, _ctx: &RankOverContext<'input>) { }
/**
 * Enter a parse tree produced by {@link OutboundAPIParser#keySurfaceColumn}.
 * @param ctx the parse tree
 */
fn enter_keySurfaceColumn(&mut self, _ctx: &KeySurfaceColumnContext<'input>) { }
/**
 * Exit a parse tree produced by {@link OutboundAPIParser#keySurfaceColumn}.
 * @param ctx the parse tree
 */
fn exit_keySurfaceColumn(&mut self, _ctx: &KeySurfaceColumnContext<'input>) { }
/**
 * Enter a parse tree produced by {@link OutboundAPIParser#textColumn}.
 * @param ctx the parse tree
 */
fn enter_textColumn(&mut self, _ctx: &TextColumnContext<'input>) { }
/**
 * Exit a parse tree produced by {@link OutboundAPIParser#textColumn}.
 * @param ctx the parse tree
 */
fn exit_textColumn(&mut self, _ctx: &TextColumnContext<'input>) { }
/**
 * Enter a parse tree produced by {@link OutboundAPIParser#latestGlobalFunction}.
 * @param ctx the parse tree
 */
fn enter_latestGlobalFunction(&mut self, _ctx: &LatestGlobalFunctionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link OutboundAPIParser#latestGlobalFunction}.
 * @param ctx the parse tree
 */
fn exit_latestGlobalFunction(&mut self, _ctx: &LatestGlobalFunctionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link OutboundAPIParser#timeInterval}.
 * @param ctx the parse tree
 */
fn enter_timeInterval(&mut self, _ctx: &TimeIntervalContext<'input>) { }
/**
 * Exit a parse tree produced by {@link OutboundAPIParser#timeInterval}.
 * @param ctx the parse tree
 */
fn exit_timeInterval(&mut self, _ctx: &TimeIntervalContext<'input>) { }
/**
 * Enter a parse tree produced by {@link OutboundAPIParser#timeIntervalOrFunction}.
 * @param ctx the parse tree
 */
fn enter_timeIntervalOrFunction(&mut self, _ctx: &TimeIntervalOrFunctionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link OutboundAPIParser#timeIntervalOrFunction}.
 * @param ctx the parse tree
 */
fn exit_timeIntervalOrFunction(&mut self, _ctx: &TimeIntervalOrFunctionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link OutboundAPIParser#gasIntervalOrFunction}.
 * @param ctx the parse tree
 */
fn enter_gasIntervalOrFunction(&mut self, _ctx: &GasIntervalOrFunctionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link OutboundAPIParser#gasIntervalOrFunction}.
 * @param ctx the parse tree
 */
fn exit_gasIntervalOrFunction(&mut self, _ctx: &GasIntervalOrFunctionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link OutboundAPIParser#pointInTimeOrFunction}.
 * @param ctx the parse tree
 */
fn enter_pointInTimeOrFunction(&mut self, _ctx: &PointInTimeOrFunctionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link OutboundAPIParser#pointInTimeOrFunction}.
 * @param ctx the parse tree
 */
fn exit_pointInTimeOrFunction(&mut self, _ctx: &PointInTimeOrFunctionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link OutboundAPIParser#pointInTimeArithmetic}.
 * @param ctx the parse tree
 */
fn enter_pointInTimeArithmetic(&mut self, _ctx: &PointInTimeArithmeticContext<'input>) { }
/**
 * Exit a parse tree produced by {@link OutboundAPIParser#pointInTimeArithmetic}.
 * @param ctx the parse tree
 */
fn exit_pointInTimeArithmetic(&mut self, _ctx: &PointInTimeArithmeticContext<'input>) { }
/**
 * Enter a parse tree produced by {@link OutboundAPIParser#timeIntervalArithmetic}.
 * @param ctx the parse tree
 */
fn enter_timeIntervalArithmetic(&mut self, _ctx: &TimeIntervalArithmeticContext<'input>) { }
/**
 * Exit a parse tree produced by {@link OutboundAPIParser#timeIntervalArithmetic}.
 * @param ctx the parse tree
 */
fn exit_timeIntervalArithmetic(&mut self, _ctx: &TimeIntervalArithmeticContext<'input>) { }
/**
 * Enter a parse tree produced by {@link OutboundAPIParser#timeIntervalToPointInTime}.
 * @param ctx the parse tree
 */
fn enter_timeIntervalToPointInTime(&mut self, _ctx: &TimeIntervalToPointInTimeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link OutboundAPIParser#timeIntervalToPointInTime}.
 * @param ctx the parse tree
 */
fn exit_timeIntervalToPointInTime(&mut self, _ctx: &TimeIntervalToPointInTimeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link OutboundAPIParser#rankOverFunction}.
 * @param ctx the parse tree
 */
fn enter_rankOverFunction(&mut self, _ctx: &RankOverFunctionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link OutboundAPIParser#rankOverFunction}.
 * @param ctx the parse tree
 */
fn exit_rankOverFunction(&mut self, _ctx: &RankOverFunctionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link OutboundAPIParser#rankOverFilter}.
 * @param ctx the parse tree
 */
fn enter_rankOverFilter(&mut self, _ctx: &RankOverFilterContext<'input>) { }
/**
 * Exit a parse tree produced by {@link OutboundAPIParser#rankOverFilter}.
 * @param ctx the parse tree
 */
fn exit_rankOverFilter(&mut self, _ctx: &RankOverFilterContext<'input>) { }
/**
 * Enter a parse tree produced by {@link OutboundAPIParser#latestFunction}.
 * @param ctx the parse tree
 */
fn enter_latestFunction(&mut self, _ctx: &LatestFunctionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link OutboundAPIParser#latestFunction}.
 * @param ctx the parse tree
 */
fn exit_latestFunction(&mut self, _ctx: &LatestFunctionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link OutboundAPIParser#latestExpression}.
 * @param ctx the parse tree
 */
fn enter_latestExpression(&mut self, _ctx: &LatestExpressionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link OutboundAPIParser#latestExpression}.
 * @param ctx the parse tree
 */
fn exit_latestExpression(&mut self, _ctx: &LatestExpressionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link OutboundAPIParser#genericValue}.
 * @param ctx the parse tree
 */
fn enter_genericValue(&mut self, _ctx: &GenericValueContext<'input>) { }
/**
 * Exit a parse tree produced by {@link OutboundAPIParser#genericValue}.
 * @param ctx the parse tree
 */
fn exit_genericValue(&mut self, _ctx: &GenericValueContext<'input>) { }

}

antlr4rust::coerce_from!{ 'input : OutboundAPIParserListener<'input> }


