
// Generated from grammar/OutboundAPIParser.g4 by ANTLR 4.13.2

use antlr4rust::tree::ParseTreeVisitor;
use super::outboundapiparser::*;

// A complete Visitor for a parse tree produced by OutboundAPIParser.

pub trait OutboundAPIParserBaseVisitor<'input>:
    ParseTreeVisitor<'input, OutboundAPIParserContextType> {
	// Visit a parse tree produced by OutboundAPIParser#expressionsSection.
	fn visit_expressionssection(&mut self, ctx: &ExpressionsSectionContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by OutboundAPIParser#keyFilterSection.
	fn visit_keyfiltersection(&mut self, ctx: &KeyFilterSectionContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by OutboundAPIParser#IdPointInTimeArithmeticComparison.
	fn visit_idpointintimearithmeticcomparison(&mut self, ctx: &IdPointInTimeArithmeticComparisonContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by OutboundAPIParser#IdTimeIntervalIn.
	fn visit_idtimeintervalin(&mut self, ctx: &IdTimeIntervalInContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by OutboundAPIParser#IdNumericComparison.
	fn visit_idnumericcomparison(&mut self, ctx: &IdNumericComparisonContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by OutboundAPIParser#IdLatestGlobalComparison.
	fn visit_idlatestglobalcomparison(&mut self, ctx: &IdLatestGlobalComparisonContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by OutboundAPIParser#IdTimeIntervalToPointInTimeComparison.
	fn visit_idtimeintervaltopointintimecomparison(&mut self, ctx: &IdTimeIntervalToPointInTimeComparisonContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by OutboundAPIParser#IdLatestComparison.
	fn visit_idlatestcomparison(&mut self, ctx: &IdLatestComparisonContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by OutboundAPIParser#TextComparison.
	fn visit_textcomparison(&mut self, ctx: &TextComparisonContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by OutboundAPIParser#RankOver.
	fn visit_rankover(&mut self, ctx: &RankOverContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by OutboundAPIParser#keySurfaceColumn.
	fn visit_keysurfacecolumn(&mut self, ctx: &KeySurfaceColumnContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by OutboundAPIParser#textColumn.
	fn visit_textcolumn(&mut self, ctx: &TextColumnContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by OutboundAPIParser#latestGlobalFunction.
	fn visit_latestglobalfunction(&mut self, ctx: &LatestGlobalFunctionContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by OutboundAPIParser#timeInterval.
	fn visit_timeinterval(&mut self, ctx: &TimeIntervalContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by OutboundAPIParser#timeIntervalOrFunction.
	fn visit_timeintervalorfunction(&mut self, ctx: &TimeIntervalOrFunctionContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by OutboundAPIParser#gasIntervalOrFunction.
	fn visit_gasintervalorfunction(&mut self, ctx: &GasIntervalOrFunctionContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by OutboundAPIParser#pointInTimeOrFunction.
	fn visit_pointintimeorfunction(&mut self, ctx: &PointInTimeOrFunctionContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by OutboundAPIParser#pointInTimeArithmetic.
	fn visit_pointintimearithmetic(&mut self, ctx: &PointInTimeArithmeticContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by OutboundAPIParser#timeIntervalArithmetic.
	fn visit_timeintervalarithmetic(&mut self, ctx: &TimeIntervalArithmeticContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by OutboundAPIParser#timeIntervalToPointInTime.
	fn visit_timeintervaltopointintime(&mut self, ctx: &TimeIntervalToPointInTimeContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by OutboundAPIParser#rankOverFunction.
	fn visit_rankoverfunction(&mut self, ctx: &RankOverFunctionContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by OutboundAPIParser#rankOverFilter.
	fn visit_rankoverfilter(&mut self, ctx: &RankOverFilterContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by OutboundAPIParser#latestFunction.
	fn visit_latestfunction(&mut self, ctx: &LatestFunctionContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by OutboundAPIParser#latestExpression.
	fn visit_latestexpression(&mut self, ctx: &LatestExpressionContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by OutboundAPIParser#genericValue.
	fn visit_genericvalue(&mut self, ctx: &GenericValueContext<'input>) {
            self.visit_children(ctx)
        }

}