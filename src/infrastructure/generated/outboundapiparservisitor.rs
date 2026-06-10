#![allow(nonstandard_style)]
// Generated from grammar/OutboundAPIParser.g4 by ANTLR 4.13.2
use antlr4rust::tree::{ParseTreeVisitor,ParseTreeVisitorCompat};
use super::outboundapiparser::*;

/**
 * This interface defines a complete generic visitor for a parse tree produced
 * by {@link OutboundAPIParser}.
 */
pub trait OutboundAPIParserVisitor<'input>: ParseTreeVisitor<'input,OutboundAPIParserContextType>{
	/**
	 * Visit a parse tree produced by {@link OutboundAPIParser#expressionsSection}.
	 * @param ctx the parse tree
	 */
	fn visit_expressionsSection(&mut self, ctx: &ExpressionsSectionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link OutboundAPIParser#keyFilterSection}.
	 * @param ctx the parse tree
	 */
	fn visit_keyFilterSection(&mut self, ctx: &KeyFilterSectionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code IdPointInTimeArithmeticComparison}
	 * labeled alternative in {@link OutboundAPIParser#keyComparison}.
	 * @param ctx the parse tree
	 */
	fn visit_IdPointInTimeArithmeticComparison(&mut self, ctx: &IdPointInTimeArithmeticComparisonContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code IdTimeIntervalIn}
	 * labeled alternative in {@link OutboundAPIParser#keyComparison}.
	 * @param ctx the parse tree
	 */
	fn visit_IdTimeIntervalIn(&mut self, ctx: &IdTimeIntervalInContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code IdNumericComparison}
	 * labeled alternative in {@link OutboundAPIParser#keyComparison}.
	 * @param ctx the parse tree
	 */
	fn visit_IdNumericComparison(&mut self, ctx: &IdNumericComparisonContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code IdLatestGlobalComparison}
	 * labeled alternative in {@link OutboundAPIParser#keyComparison}.
	 * @param ctx the parse tree
	 */
	fn visit_IdLatestGlobalComparison(&mut self, ctx: &IdLatestGlobalComparisonContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code IdTimeIntervalToPointInTimeComparison}
	 * labeled alternative in {@link OutboundAPIParser#keyComparison}.
	 * @param ctx the parse tree
	 */
	fn visit_IdTimeIntervalToPointInTimeComparison(&mut self, ctx: &IdTimeIntervalToPointInTimeComparisonContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code IdLatestComparison}
	 * labeled alternative in {@link OutboundAPIParser#keyComparison}.
	 * @param ctx the parse tree
	 */
	fn visit_IdLatestComparison(&mut self, ctx: &IdLatestComparisonContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code TextComparison}
	 * labeled alternative in {@link OutboundAPIParser#keyComparison}.
	 * @param ctx the parse tree
	 */
	fn visit_TextComparison(&mut self, ctx: &TextComparisonContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code RankOver}
	 * labeled alternative in {@link OutboundAPIParser#keyComparison}.
	 * @param ctx the parse tree
	 */
	fn visit_RankOver(&mut self, ctx: &RankOverContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link OutboundAPIParser#keySurfaceColumn}.
	 * @param ctx the parse tree
	 */
	fn visit_keySurfaceColumn(&mut self, ctx: &KeySurfaceColumnContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link OutboundAPIParser#textColumn}.
	 * @param ctx the parse tree
	 */
	fn visit_textColumn(&mut self, ctx: &TextColumnContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link OutboundAPIParser#latestGlobalFunction}.
	 * @param ctx the parse tree
	 */
	fn visit_latestGlobalFunction(&mut self, ctx: &LatestGlobalFunctionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link OutboundAPIParser#timeInterval}.
	 * @param ctx the parse tree
	 */
	fn visit_timeInterval(&mut self, ctx: &TimeIntervalContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link OutboundAPIParser#timeIntervalOrFunction}.
	 * @param ctx the parse tree
	 */
	fn visit_timeIntervalOrFunction(&mut self, ctx: &TimeIntervalOrFunctionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link OutboundAPIParser#gasIntervalOrFunction}.
	 * @param ctx the parse tree
	 */
	fn visit_gasIntervalOrFunction(&mut self, ctx: &GasIntervalOrFunctionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link OutboundAPIParser#pointInTimeOrFunction}.
	 * @param ctx the parse tree
	 */
	fn visit_pointInTimeOrFunction(&mut self, ctx: &PointInTimeOrFunctionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link OutboundAPIParser#pointInTimeArithmetic}.
	 * @param ctx the parse tree
	 */
	fn visit_pointInTimeArithmetic(&mut self, ctx: &PointInTimeArithmeticContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link OutboundAPIParser#timeIntervalArithmetic}.
	 * @param ctx the parse tree
	 */
	fn visit_timeIntervalArithmetic(&mut self, ctx: &TimeIntervalArithmeticContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link OutboundAPIParser#timeIntervalToPointInTime}.
	 * @param ctx the parse tree
	 */
	fn visit_timeIntervalToPointInTime(&mut self, ctx: &TimeIntervalToPointInTimeContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link OutboundAPIParser#rankOverFunction}.
	 * @param ctx the parse tree
	 */
	fn visit_rankOverFunction(&mut self, ctx: &RankOverFunctionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link OutboundAPIParser#rankOverFilter}.
	 * @param ctx the parse tree
	 */
	fn visit_rankOverFilter(&mut self, ctx: &RankOverFilterContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link OutboundAPIParser#latestFunction}.
	 * @param ctx the parse tree
	 */
	fn visit_latestFunction(&mut self, ctx: &LatestFunctionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link OutboundAPIParser#latestExpression}.
	 * @param ctx the parse tree
	 */
	fn visit_latestExpression(&mut self, ctx: &LatestExpressionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link OutboundAPIParser#genericValue}.
	 * @param ctx the parse tree
	 */
	fn visit_genericValue(&mut self, ctx: &GenericValueContext<'input>) { self.visit_children(ctx) }

}

pub trait OutboundAPIParserVisitorCompat<'input>:ParseTreeVisitorCompat<'input, Node= OutboundAPIParserContextType>{
	/**
	 * Visit a parse tree produced by {@link OutboundAPIParser#expressionsSection}.
	 * @param ctx the parse tree
	 */
		fn visit_expressionsSection(&mut self, ctx: &ExpressionsSectionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link OutboundAPIParser#keyFilterSection}.
	 * @param ctx the parse tree
	 */
		fn visit_keyFilterSection(&mut self, ctx: &KeyFilterSectionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code IdPointInTimeArithmeticComparison}
	 * labeled alternative in {@link OutboundAPIParser#keyComparison}.
	 * @param ctx the parse tree
	 */
		fn visit_IdPointInTimeArithmeticComparison(&mut self, ctx: &IdPointInTimeArithmeticComparisonContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code IdTimeIntervalIn}
	 * labeled alternative in {@link OutboundAPIParser#keyComparison}.
	 * @param ctx the parse tree
	 */
		fn visit_IdTimeIntervalIn(&mut self, ctx: &IdTimeIntervalInContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code IdNumericComparison}
	 * labeled alternative in {@link OutboundAPIParser#keyComparison}.
	 * @param ctx the parse tree
	 */
		fn visit_IdNumericComparison(&mut self, ctx: &IdNumericComparisonContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code IdLatestGlobalComparison}
	 * labeled alternative in {@link OutboundAPIParser#keyComparison}.
	 * @param ctx the parse tree
	 */
		fn visit_IdLatestGlobalComparison(&mut self, ctx: &IdLatestGlobalComparisonContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code IdTimeIntervalToPointInTimeComparison}
	 * labeled alternative in {@link OutboundAPIParser#keyComparison}.
	 * @param ctx the parse tree
	 */
		fn visit_IdTimeIntervalToPointInTimeComparison(&mut self, ctx: &IdTimeIntervalToPointInTimeComparisonContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code IdLatestComparison}
	 * labeled alternative in {@link OutboundAPIParser#keyComparison}.
	 * @param ctx the parse tree
	 */
		fn visit_IdLatestComparison(&mut self, ctx: &IdLatestComparisonContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code TextComparison}
	 * labeled alternative in {@link OutboundAPIParser#keyComparison}.
	 * @param ctx the parse tree
	 */
		fn visit_TextComparison(&mut self, ctx: &TextComparisonContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code RankOver}
	 * labeled alternative in {@link OutboundAPIParser#keyComparison}.
	 * @param ctx the parse tree
	 */
		fn visit_RankOver(&mut self, ctx: &RankOverContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link OutboundAPIParser#keySurfaceColumn}.
	 * @param ctx the parse tree
	 */
		fn visit_keySurfaceColumn(&mut self, ctx: &KeySurfaceColumnContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link OutboundAPIParser#textColumn}.
	 * @param ctx the parse tree
	 */
		fn visit_textColumn(&mut self, ctx: &TextColumnContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link OutboundAPIParser#latestGlobalFunction}.
	 * @param ctx the parse tree
	 */
		fn visit_latestGlobalFunction(&mut self, ctx: &LatestGlobalFunctionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link OutboundAPIParser#timeInterval}.
	 * @param ctx the parse tree
	 */
		fn visit_timeInterval(&mut self, ctx: &TimeIntervalContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link OutboundAPIParser#timeIntervalOrFunction}.
	 * @param ctx the parse tree
	 */
		fn visit_timeIntervalOrFunction(&mut self, ctx: &TimeIntervalOrFunctionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link OutboundAPIParser#gasIntervalOrFunction}.
	 * @param ctx the parse tree
	 */
		fn visit_gasIntervalOrFunction(&mut self, ctx: &GasIntervalOrFunctionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link OutboundAPIParser#pointInTimeOrFunction}.
	 * @param ctx the parse tree
	 */
		fn visit_pointInTimeOrFunction(&mut self, ctx: &PointInTimeOrFunctionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link OutboundAPIParser#pointInTimeArithmetic}.
	 * @param ctx the parse tree
	 */
		fn visit_pointInTimeArithmetic(&mut self, ctx: &PointInTimeArithmeticContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link OutboundAPIParser#timeIntervalArithmetic}.
	 * @param ctx the parse tree
	 */
		fn visit_timeIntervalArithmetic(&mut self, ctx: &TimeIntervalArithmeticContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link OutboundAPIParser#timeIntervalToPointInTime}.
	 * @param ctx the parse tree
	 */
		fn visit_timeIntervalToPointInTime(&mut self, ctx: &TimeIntervalToPointInTimeContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link OutboundAPIParser#rankOverFunction}.
	 * @param ctx the parse tree
	 */
		fn visit_rankOverFunction(&mut self, ctx: &RankOverFunctionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link OutboundAPIParser#rankOverFilter}.
	 * @param ctx the parse tree
	 */
		fn visit_rankOverFilter(&mut self, ctx: &RankOverFilterContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link OutboundAPIParser#latestFunction}.
	 * @param ctx the parse tree
	 */
		fn visit_latestFunction(&mut self, ctx: &LatestFunctionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link OutboundAPIParser#latestExpression}.
	 * @param ctx the parse tree
	 */
		fn visit_latestExpression(&mut self, ctx: &LatestExpressionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link OutboundAPIParser#genericValue}.
	 * @param ctx the parse tree
	 */
		fn visit_genericValue(&mut self, ctx: &GenericValueContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

}

impl<'input,T> OutboundAPIParserVisitor<'input> for T
where
	T: OutboundAPIParserVisitorCompat<'input>
{
	fn visit_expressionsSection(&mut self, ctx: &ExpressionsSectionContext<'input>){
		let result = <Self as OutboundAPIParserVisitorCompat>::visit_expressionsSection(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_keyFilterSection(&mut self, ctx: &KeyFilterSectionContext<'input>){
		let result = <Self as OutboundAPIParserVisitorCompat>::visit_keyFilterSection(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_IdPointInTimeArithmeticComparison(&mut self, ctx: &IdPointInTimeArithmeticComparisonContext<'input>){
		let result = <Self as OutboundAPIParserVisitorCompat>::visit_IdPointInTimeArithmeticComparison(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_IdTimeIntervalIn(&mut self, ctx: &IdTimeIntervalInContext<'input>){
		let result = <Self as OutboundAPIParserVisitorCompat>::visit_IdTimeIntervalIn(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_IdNumericComparison(&mut self, ctx: &IdNumericComparisonContext<'input>){
		let result = <Self as OutboundAPIParserVisitorCompat>::visit_IdNumericComparison(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_IdLatestGlobalComparison(&mut self, ctx: &IdLatestGlobalComparisonContext<'input>){
		let result = <Self as OutboundAPIParserVisitorCompat>::visit_IdLatestGlobalComparison(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_IdTimeIntervalToPointInTimeComparison(&mut self, ctx: &IdTimeIntervalToPointInTimeComparisonContext<'input>){
		let result = <Self as OutboundAPIParserVisitorCompat>::visit_IdTimeIntervalToPointInTimeComparison(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_IdLatestComparison(&mut self, ctx: &IdLatestComparisonContext<'input>){
		let result = <Self as OutboundAPIParserVisitorCompat>::visit_IdLatestComparison(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_TextComparison(&mut self, ctx: &TextComparisonContext<'input>){
		let result = <Self as OutboundAPIParserVisitorCompat>::visit_TextComparison(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_RankOver(&mut self, ctx: &RankOverContext<'input>){
		let result = <Self as OutboundAPIParserVisitorCompat>::visit_RankOver(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_keySurfaceColumn(&mut self, ctx: &KeySurfaceColumnContext<'input>){
		let result = <Self as OutboundAPIParserVisitorCompat>::visit_keySurfaceColumn(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_textColumn(&mut self, ctx: &TextColumnContext<'input>){
		let result = <Self as OutboundAPIParserVisitorCompat>::visit_textColumn(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_latestGlobalFunction(&mut self, ctx: &LatestGlobalFunctionContext<'input>){
		let result = <Self as OutboundAPIParserVisitorCompat>::visit_latestGlobalFunction(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_timeInterval(&mut self, ctx: &TimeIntervalContext<'input>){
		let result = <Self as OutboundAPIParserVisitorCompat>::visit_timeInterval(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_timeIntervalOrFunction(&mut self, ctx: &TimeIntervalOrFunctionContext<'input>){
		let result = <Self as OutboundAPIParserVisitorCompat>::visit_timeIntervalOrFunction(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_gasIntervalOrFunction(&mut self, ctx: &GasIntervalOrFunctionContext<'input>){
		let result = <Self as OutboundAPIParserVisitorCompat>::visit_gasIntervalOrFunction(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_pointInTimeOrFunction(&mut self, ctx: &PointInTimeOrFunctionContext<'input>){
		let result = <Self as OutboundAPIParserVisitorCompat>::visit_pointInTimeOrFunction(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_pointInTimeArithmetic(&mut self, ctx: &PointInTimeArithmeticContext<'input>){
		let result = <Self as OutboundAPIParserVisitorCompat>::visit_pointInTimeArithmetic(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_timeIntervalArithmetic(&mut self, ctx: &TimeIntervalArithmeticContext<'input>){
		let result = <Self as OutboundAPIParserVisitorCompat>::visit_timeIntervalArithmetic(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_timeIntervalToPointInTime(&mut self, ctx: &TimeIntervalToPointInTimeContext<'input>){
		let result = <Self as OutboundAPIParserVisitorCompat>::visit_timeIntervalToPointInTime(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_rankOverFunction(&mut self, ctx: &RankOverFunctionContext<'input>){
		let result = <Self as OutboundAPIParserVisitorCompat>::visit_rankOverFunction(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_rankOverFilter(&mut self, ctx: &RankOverFilterContext<'input>){
		let result = <Self as OutboundAPIParserVisitorCompat>::visit_rankOverFilter(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_latestFunction(&mut self, ctx: &LatestFunctionContext<'input>){
		let result = <Self as OutboundAPIParserVisitorCompat>::visit_latestFunction(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_latestExpression(&mut self, ctx: &LatestExpressionContext<'input>){
		let result = <Self as OutboundAPIParserVisitorCompat>::visit_latestExpression(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_genericValue(&mut self, ctx: &GenericValueContext<'input>){
		let result = <Self as OutboundAPIParserVisitorCompat>::visit_genericValue(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

}