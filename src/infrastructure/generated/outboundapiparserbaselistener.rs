// Generated from grammar/OutboundAPIParser.g4 by ANTLR 4.13.2

use super::outboundapiparser::*;
use antlr4rust::tree::ParseTreeListener;

// A complete Visitor for a parse tree produced by OutboundAPIParser.

pub trait OutboundAPIParserBaseListener<'input>:
    ParseTreeListener<'input, OutboundAPIParserContextType> {

    /**
     * Enter a parse tree produced by \{@link OutboundAPIParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_expressionssection(&mut self, _ctx: &ExpressionsSectionContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  OutboundAPIParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_expressionssection(&mut self, _ctx: &ExpressionsSectionContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link OutboundAPIParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_keyfiltersection(&mut self, _ctx: &KeyFilterSectionContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  OutboundAPIParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_keyfiltersection(&mut self, _ctx: &KeyFilterSectionContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link OutboundAPIParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_idpointintimearithmeticcomparison(&mut self, _ctx: &IdPointInTimeArithmeticComparisonContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  OutboundAPIParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_idpointintimearithmeticcomparison(&mut self, _ctx: &IdPointInTimeArithmeticComparisonContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link OutboundAPIParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_idtimeintervalin(&mut self, _ctx: &IdTimeIntervalInContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  OutboundAPIParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_idtimeintervalin(&mut self, _ctx: &IdTimeIntervalInContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link OutboundAPIParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_idnumericcomparison(&mut self, _ctx: &IdNumericComparisonContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  OutboundAPIParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_idnumericcomparison(&mut self, _ctx: &IdNumericComparisonContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link OutboundAPIParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_idlatestglobalcomparison(&mut self, _ctx: &IdLatestGlobalComparisonContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  OutboundAPIParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_idlatestglobalcomparison(&mut self, _ctx: &IdLatestGlobalComparisonContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link OutboundAPIParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_idtimeintervaltopointintimecomparison(&mut self, _ctx: &IdTimeIntervalToPointInTimeComparisonContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  OutboundAPIParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_idtimeintervaltopointintimecomparison(&mut self, _ctx: &IdTimeIntervalToPointInTimeComparisonContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link OutboundAPIParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_idlatestcomparison(&mut self, _ctx: &IdLatestComparisonContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  OutboundAPIParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_idlatestcomparison(&mut self, _ctx: &IdLatestComparisonContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link OutboundAPIParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_textcomparison(&mut self, _ctx: &TextComparisonContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  OutboundAPIParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_textcomparison(&mut self, _ctx: &TextComparisonContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link OutboundAPIParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_rankover(&mut self, _ctx: &RankOverContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  OutboundAPIParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_rankover(&mut self, _ctx: &RankOverContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link OutboundAPIParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_keysurfacecolumn(&mut self, _ctx: &KeySurfaceColumnContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  OutboundAPIParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_keysurfacecolumn(&mut self, _ctx: &KeySurfaceColumnContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link OutboundAPIParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_textcolumn(&mut self, _ctx: &TextColumnContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  OutboundAPIParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_textcolumn(&mut self, _ctx: &TextColumnContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link OutboundAPIParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_latestglobalfunction(&mut self, _ctx: &LatestGlobalFunctionContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  OutboundAPIParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_latestglobalfunction(&mut self, _ctx: &LatestGlobalFunctionContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link OutboundAPIParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_timeinterval(&mut self, _ctx: &TimeIntervalContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  OutboundAPIParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_timeinterval(&mut self, _ctx: &TimeIntervalContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link OutboundAPIParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_timeintervalorfunction(&mut self, _ctx: &TimeIntervalOrFunctionContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  OutboundAPIParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_timeintervalorfunction(&mut self, _ctx: &TimeIntervalOrFunctionContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link OutboundAPIParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_gasintervalorfunction(&mut self, _ctx: &GasIntervalOrFunctionContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  OutboundAPIParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_gasintervalorfunction(&mut self, _ctx: &GasIntervalOrFunctionContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link OutboundAPIParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_pointintimeorfunction(&mut self, _ctx: &PointInTimeOrFunctionContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  OutboundAPIParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_pointintimeorfunction(&mut self, _ctx: &PointInTimeOrFunctionContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link OutboundAPIParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_pointintimearithmetic(&mut self, _ctx: &PointInTimeArithmeticContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  OutboundAPIParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_pointintimearithmetic(&mut self, _ctx: &PointInTimeArithmeticContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link OutboundAPIParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_timeintervalarithmetic(&mut self, _ctx: &TimeIntervalArithmeticContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  OutboundAPIParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_timeintervalarithmetic(&mut self, _ctx: &TimeIntervalArithmeticContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link OutboundAPIParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_timeintervaltopointintime(&mut self, _ctx: &TimeIntervalToPointInTimeContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  OutboundAPIParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_timeintervaltopointintime(&mut self, _ctx: &TimeIntervalToPointInTimeContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link OutboundAPIParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_rankoverfunction(&mut self, _ctx: &RankOverFunctionContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  OutboundAPIParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_rankoverfunction(&mut self, _ctx: &RankOverFunctionContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link OutboundAPIParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_rankoverfilter(&mut self, _ctx: &RankOverFilterContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  OutboundAPIParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_rankoverfilter(&mut self, _ctx: &RankOverFilterContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link OutboundAPIParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_latestfunction(&mut self, _ctx: &LatestFunctionContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  OutboundAPIParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_latestfunction(&mut self, _ctx: &LatestFunctionContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link OutboundAPIParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_latestexpression(&mut self, _ctx: &LatestExpressionContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  OutboundAPIParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_latestexpression(&mut self, _ctx: &LatestExpressionContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link OutboundAPIParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_genericvalue(&mut self, _ctx: &GenericValueContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  OutboundAPIParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_genericvalue(&mut self, _ctx: &GenericValueContext<'input>) {}


}