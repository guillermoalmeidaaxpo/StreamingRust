// Generated from grammar/OutboundAPIParser.g4 by ANTLR 4.13.2
#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]
use antlr4rust::PredictionContextCache;
use antlr4rust::parser::{Parser, BaseParser, ParserRecog, ParserNodeType};
use antlr4rust::token_stream::TokenStream;
use antlr4rust::TokenSource;
use antlr4rust::parser_atn_simulator::ParserATNSimulator;
use antlr4rust::errors::*;
use antlr4rust::rule_context::{BaseRuleContext, CustomRuleContext, RuleContext};
use antlr4rust::recognizer::{Recognizer,Actions};
use antlr4rust::atn_deserializer::ATNDeserializer;
use antlr4rust::dfa::DFA;
use antlr4rust::atn::{ATN, INVALID_ALT};
use antlr4rust::error_strategy::{ErrorStrategy, DefaultErrorStrategy};
use antlr4rust::parser_rule_context::{BaseParserRuleContext, ParserRuleContext,cast,cast_mut};
use antlr4rust::tree::*;
use antlr4rust::token::{TOKEN_EOF,OwningToken,Token};
use antlr4rust::int_stream::EOF;
use antlr4rust::vocabulary::{Vocabulary,VocabularyImpl};
use antlr4rust::token_factory::{CommonTokenFactory,TokenFactory, TokenAware};
use super::outboundapiparserlistener::*;
use super::outboundapiparservisitor::*;

use antlr4rust::lazy_static;
use antlr4rust::{TidAble,TidExt};

use std::marker::PhantomData;
use std::sync::Arc;
use std::rc::Rc;
use std::convert::TryFrom;
use std::cell::RefCell;
use std::ops::{DerefMut, Deref};
use std::borrow::{Borrow,BorrowMut};
use std::any::{Any,TypeId};

		pub const OutboundAPIParser_CI_VALIDITY_PERIOD_START:i32=1; 
		pub const OutboundAPIParser_CI_VALIDITY_PERIOD_END:i32=2; 
		pub const OutboundAPIParser_CI_INSTANCE_CODE:i32=3; 
		pub const OutboundAPIParser_CI_BIDID:i32=4; 
		pub const OutboundAPIParser_CI_ISP:i32=5; 
		pub const OutboundAPIParser_CI_DIRECTION:i32=6; 
		pub const OutboundAPIParser_CI_STATUS:i32=7; 
		pub const OutboundAPIParser_CI_OPTION_EXPIRY:i32=8; 
		pub const OutboundAPIParser_TIME_INTERVAL_FUNCTION_NAME:i32=9; 
		pub const OutboundAPIParser_TIME_INTERVAL_GAS_FUNCTION_NAME:i32=10; 
		pub const OutboundAPIParser_TIME_INTERVAL_EXPLICIT_FUNCTION_NAME:i32=11; 
		pub const OutboundAPIParser_LATEST_GLOBAL:i32=12; 
		pub const OutboundAPIParser_POINT_IN_TIME_FUNCTION_NAME:i32=13; 
		pub const OutboundAPIParser_POINT_IN_TIME_UTC_FUNCTION_NAME:i32=14; 
		pub const OutboundAPIParser_TIME_INTERVAL_TO_POINT_IN_TIME_FUNCTION:i32=15; 
		pub const OutboundAPIParser_RANK_OVER_FUNCTION_NAME:i32=16; 
		pub const OutboundAPIParser_LATEST_FUNCTION_NAME:i32=17; 
		pub const OutboundAPIParser_IN:i32=18; 
		pub const OutboundAPIParser_COMPARISON_OPERATOR:i32=19; 
		pub const OutboundAPIParser_SORT_ORDER:i32=20; 
		pub const OutboundAPIParser_OPEN_FILTER_INTERVAL_MARKER:i32=21; 
		pub const OutboundAPIParser_EQUAL:i32=22; 
		pub const OutboundAPIParser_GT:i32=23; 
		pub const OutboundAPIParser_LT:i32=24; 
		pub const OutboundAPIParser_LE:i32=25; 
		pub const OutboundAPIParser_GE:i32=26; 
		pub const OutboundAPIParser_ADD:i32=27; 
		pub const OutboundAPIParser_SUB:i32=28; 
		pub const OutboundAPIParser_MUL:i32=29; 
		pub const OutboundAPIParser_TIME_ZONE_IANA:i32=30; 
		pub const OutboundAPIParser_DATE:i32=31; 
		pub const OutboundAPIParser_TIME:i32=32; 
		pub const OutboundAPIParser_POINT_IN_TIME:i32=33; 
		pub const OutboundAPIParser_TIME_PERIOD:i32=34; 
		pub const OutboundAPIParser_LB:i32=35; 
		pub const OutboundAPIParser_RB:i32=36; 
		pub const OutboundAPIParser_LSB:i32=37; 
		pub const OutboundAPIParser_RSB:i32=38; 
		pub const OutboundAPIParser_ID:i32=39; 
		pub const OutboundAPIParser_SIGNED_INTEGER:i32=40; 
		pub const OutboundAPIParser_FLOAT:i32=41; 
		pub const OutboundAPIParser_WORD:i32=42; 
		pub const OutboundAPIParser_QUOTE:i32=43; 
		pub const OutboundAPIParser_COLON:i32=44; 
		pub const OutboundAPIParser_COMMA:i32=45; 
		pub const OutboundAPIParser_SEMICOLON:i32=46; 
		pub const OutboundAPIParser_DECIMAL_POINT:i32=47; 
		pub const OutboundAPIParser_WS:i32=48; 
		pub const OutboundAPIParser_ERRORCHAR:i32=49;
	pub const OutboundAPIParser_EOF:i32=EOF;
	pub const RULE_expressionsSection:usize = 0; 
	pub const RULE_keyFilterSection:usize = 1; 
	pub const RULE_keyComparison:usize = 2; 
	pub const RULE_keySurfaceColumn:usize = 3; 
	pub const RULE_textColumn:usize = 4; 
	pub const RULE_latestGlobalFunction:usize = 5; 
	pub const RULE_timeInterval:usize = 6; 
	pub const RULE_timeIntervalOrFunction:usize = 7; 
	pub const RULE_gasIntervalOrFunction:usize = 8; 
	pub const RULE_pointInTimeOrFunction:usize = 9; 
	pub const RULE_pointInTimeArithmetic:usize = 10; 
	pub const RULE_timeIntervalArithmetic:usize = 11; 
	pub const RULE_timeIntervalToPointInTime:usize = 12; 
	pub const RULE_rankOverFunction:usize = 13; 
	pub const RULE_rankOverFilter:usize = 14; 
	pub const RULE_latestFunction:usize = 15; 
	pub const RULE_latestExpression:usize = 16; 
	pub const RULE_genericValue:usize = 17;
	pub const ruleNames: [&'static str; 18] =  [
		"expressionsSection", "keyFilterSection", "keyComparison", "keySurfaceColumn", 
		"textColumn", "latestGlobalFunction", "timeInterval", "timeIntervalOrFunction", 
		"gasIntervalOrFunction", "pointInTimeOrFunction", "pointInTimeArithmetic", 
		"timeIntervalArithmetic", "timeIntervalToPointInTime", "rankOverFunction", 
		"rankOverFilter", "latestFunction", "latestExpression", "genericValue"
	];


	pub const _LITERAL_NAMES: [Option<&'static str>;48] = [
		None, None, None, None, None, None, None, None, None, None, None, None, 
		None, None, None, None, None, None, None, None, None, None, Some("'='"), 
		Some("'>'"), Some("'<'"), Some("'<='"), Some("'>='"), Some("'+'"), Some("'-'"), 
		Some("'*'"), None, None, None, None, None, Some("'('"), Some("')'"), Some("'['"), 
		Some("']'"), None, None, None, None, Some("'\"'"), Some("':'"), Some("','"), 
		Some("';'"), Some("'.'")
	];
	pub const _SYMBOLIC_NAMES: [Option<&'static str>;50]  = [
		None, Some("CI_VALIDITY_PERIOD_START"), Some("CI_VALIDITY_PERIOD_END"), 
		Some("CI_INSTANCE_CODE"), Some("CI_BIDID"), Some("CI_ISP"), Some("CI_DIRECTION"), 
		Some("CI_STATUS"), Some("CI_OPTION_EXPIRY"), Some("TIME_INTERVAL_FUNCTION_NAME"), 
		Some("TIME_INTERVAL_GAS_FUNCTION_NAME"), Some("TIME_INTERVAL_EXPLICIT_FUNCTION_NAME"), 
		Some("LATEST_GLOBAL"), Some("POINT_IN_TIME_FUNCTION_NAME"), Some("POINT_IN_TIME_UTC_FUNCTION_NAME"), 
		Some("TIME_INTERVAL_TO_POINT_IN_TIME_FUNCTION"), Some("RANK_OVER_FUNCTION_NAME"), 
		Some("LATEST_FUNCTION_NAME"), Some("IN"), Some("COMPARISON_OPERATOR"), 
		Some("SORT_ORDER"), Some("OPEN_FILTER_INTERVAL_MARKER"), Some("EQUAL"), 
		Some("GT"), Some("LT"), Some("LE"), Some("GE"), Some("ADD"), Some("SUB"), 
		Some("MUL"), Some("TIME_ZONE_IANA"), Some("DATE"), Some("TIME"), Some("POINT_IN_TIME"), 
		Some("TIME_PERIOD"), Some("LB"), Some("RB"), Some("LSB"), Some("RSB"), 
		Some("ID"), Some("SIGNED_INTEGER"), Some("FLOAT"), Some("WORD"), Some("QUOTE"), 
		Some("COLON"), Some("COMMA"), Some("SEMICOLON"), Some("DECIMAL_POINT"), 
		Some("WS"), Some("ERRORCHAR")
	];
	lazy_static!{
	    static ref _shared_context_cache: Arc<PredictionContextCache> = Arc::new(PredictionContextCache::new());
		static ref VOCABULARY: Box<dyn Vocabulary> = Box::new(VocabularyImpl::new(_LITERAL_NAMES.iter(), _SYMBOLIC_NAMES.iter(), None));
	}


type BaseParserType<'input, I> =
	BaseParser<'input,OutboundAPIParserExt<'input>, I, OutboundAPIParserContextType , dyn OutboundAPIParserListener<'input> + 'input >;

type TokenType<'input> = <LocalTokenFactory<'input> as TokenFactory<'input>>::Tok;
pub type LocalTokenFactory<'input> = CommonTokenFactory;

pub type OutboundAPIParserTreeWalker<'input,'a> =
	ParseTreeWalker<'input, 'a, OutboundAPIParserContextType , dyn OutboundAPIParserListener<'input> + 'a>;

/// Parser for OutboundAPIParser grammar
pub struct OutboundAPIParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	base:BaseParserType<'input,I>,
	interpreter:Arc<ParserATNSimulator>,
	_shared_context_cache: Box<PredictionContextCache>,
    pub err_handler: Box<dyn ErrorStrategy<'input,BaseParserType<'input,I> > >,
}

impl<'input, I> OutboundAPIParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
    pub fn set_error_strategy(&mut self, strategy: Box<dyn ErrorStrategy<'input,BaseParserType<'input,I> > >) {
        self.err_handler = strategy
    }

    pub fn with_strategy(input: I, strategy: Box<dyn ErrorStrategy<'input,BaseParserType<'input,I> > >) -> Self {
		antlr4rust::recognizer::check_version("0","5");
		let interpreter = Arc::new(ParserATNSimulator::new(
			_ATN.clone(),
			_decision_to_DFA.clone(),
			_shared_context_cache.clone(),
		));
		Self {
			base: BaseParser::new_base_parser(
				input,
				Arc::clone(&interpreter),
				OutboundAPIParserExt{
					_pd: Default::default(),
				}
			),
			interpreter,
            _shared_context_cache: Box::new(PredictionContextCache::new()),
            err_handler: strategy,
        }
    }

}

type DynStrategy<'input,I> = Box<dyn ErrorStrategy<'input,BaseParserType<'input,I>> + 'input>;

impl<'input, I> OutboundAPIParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
    pub fn with_dyn_strategy(input: I) -> Self{
    	Self::with_strategy(input,Box::new(DefaultErrorStrategy::new()))
    }
}

impl<'input, I> OutboundAPIParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
    pub fn new(input: I) -> Self{
    	Self::with_strategy(input,Box::new(DefaultErrorStrategy::new()))
    }
}

/// Trait for monomorphized trait object that corresponds to the nodes of parse tree generated for OutboundAPIParser
pub trait OutboundAPIParserContext<'input>:
	for<'x> Listenable<dyn OutboundAPIParserListener<'input> + 'x > + 
	for<'x> Visitable<dyn OutboundAPIParserVisitor<'input> + 'x > + 
	ParserRuleContext<'input, TF=LocalTokenFactory<'input>, Ctx=OutboundAPIParserContextType>
{}

antlr4rust::coerce_from!{ 'input : OutboundAPIParserContext<'input> }

impl<'input, 'x, T> VisitableDyn<T> for dyn OutboundAPIParserContext<'input> + 'input
where
    T: OutboundAPIParserVisitor<'input> + 'x,
{
    fn accept_dyn(&self, visitor: &mut T) {
        self.accept(visitor as &mut (dyn OutboundAPIParserVisitor<'input> + 'x))
    }
}

impl<'input> OutboundAPIParserContext<'input> for TerminalNode<'input,OutboundAPIParserContextType> {}
impl<'input> OutboundAPIParserContext<'input> for ErrorNode<'input,OutboundAPIParserContextType> {}

antlr4rust::tid! { impl<'input> TidAble<'input> for dyn OutboundAPIParserContext<'input> + 'input }

antlr4rust::tid! { impl<'input> TidAble<'input> for dyn OutboundAPIParserListener<'input> + 'input }

pub struct OutboundAPIParserContextType;
antlr4rust::tid!{OutboundAPIParserContextType}

impl<'input> ParserNodeType<'input> for OutboundAPIParserContextType{
	type TF = LocalTokenFactory<'input>;
	type Type = dyn OutboundAPIParserContext<'input> + 'input;
}

impl<'input, I> Deref for OutboundAPIParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
    type Target = BaseParserType<'input,I>;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl<'input, I> DerefMut for OutboundAPIParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

pub struct OutboundAPIParserExt<'input>{
	_pd: PhantomData<&'input str>,
}

impl<'input> OutboundAPIParserExt<'input>{
}
antlr4rust::tid! { OutboundAPIParserExt<'a> }

impl<'input> TokenAware<'input> for OutboundAPIParserExt<'input>{
	type TF = LocalTokenFactory<'input>;
}

impl<'input,I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>> ParserRecog<'input, BaseParserType<'input,I>> for OutboundAPIParserExt<'input>{}

impl<'input,I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>> Actions<'input, BaseParserType<'input,I>> for OutboundAPIParserExt<'input>{
	fn get_grammar_file_name(&self) -> & str{ "OutboundAPIParser.g4"}

   	fn get_rule_names(&self) -> &[& str] {&ruleNames}

   	fn get_vocabulary(&self) -> &dyn Vocabulary { &**VOCABULARY }
}
//------------------- expressionsSection ----------------
pub type ExpressionsSectionContextAll<'input> = ExpressionsSectionContext<'input>;


pub type ExpressionsSectionContext<'input> = BaseParserRuleContext<'input,ExpressionsSectionContextExt<'input>>;

#[derive(Clone)]
pub struct ExpressionsSectionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> OutboundAPIParserContext<'input> for ExpressionsSectionContext<'input>{}

impl<'input,'a> Listenable<dyn OutboundAPIParserListener<'input> + 'a> for ExpressionsSectionContext<'input>{
		fn enter(&self,listener: &mut (dyn OutboundAPIParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_expressionsSection(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn OutboundAPIParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_expressionsSection(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn OutboundAPIParserVisitor<'input> + 'a> for ExpressionsSectionContext<'input>{
	fn accept(&self,visitor: &mut (dyn OutboundAPIParserVisitor<'input> + 'a)) {
		visitor.visit_expressionsSection(self);
	}
}

impl<'input> CustomRuleContext<'input> for ExpressionsSectionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = OutboundAPIParserContextType;
	fn get_rule_index(&self) -> usize { RULE_expressionsSection }
	//fn type_rule_index() -> usize where Self: Sized { RULE_expressionsSection }
}
antlr4rust::tid!{ExpressionsSectionContextExt<'a>}

impl<'input> ExpressionsSectionContextExt<'input>{
	fn new(parent: Option<Rc<dyn OutboundAPIParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<ExpressionsSectionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ExpressionsSectionContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait ExpressionsSectionContextAttrs<'input>: OutboundAPIParserContext<'input> + BorrowMut<ExpressionsSectionContextExt<'input>>{

fn keyFilterSection_all(&self) ->  Vec<Rc<KeyFilterSectionContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn keyFilterSection(&self, i: usize) -> Option<Rc<KeyFilterSectionContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> ExpressionsSectionContextAttrs<'input> for ExpressionsSectionContext<'input>{}

impl<'input, I> OutboundAPIParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn expressionsSection(&mut self,)
	-> Result<Rc<ExpressionsSectionContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ExpressionsSectionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 0, RULE_expressionsSection);
        let mut _localctx: Rc<ExpressionsSectionContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(37); 
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			loop {
				{
				{
				/*InvokeRule keyFilterSection*/
				recog.base.set_state(36);
				recog.keyFilterSection()?;

				}
				}
				recog.base.set_state(39); 
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
				if !((((_la) & !0x3f) == 0 && ((1usize << _la) & 66046) != 0) || _la==OutboundAPIParser_ID) {break}
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- keyFilterSection ----------------
pub type KeyFilterSectionContextAll<'input> = KeyFilterSectionContext<'input>;


pub type KeyFilterSectionContext<'input> = BaseParserRuleContext<'input,KeyFilterSectionContextExt<'input>>;

#[derive(Clone)]
pub struct KeyFilterSectionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> OutboundAPIParserContext<'input> for KeyFilterSectionContext<'input>{}

impl<'input,'a> Listenable<dyn OutboundAPIParserListener<'input> + 'a> for KeyFilterSectionContext<'input>{
		fn enter(&self,listener: &mut (dyn OutboundAPIParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_keyFilterSection(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn OutboundAPIParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_keyFilterSection(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn OutboundAPIParserVisitor<'input> + 'a> for KeyFilterSectionContext<'input>{
	fn accept(&self,visitor: &mut (dyn OutboundAPIParserVisitor<'input> + 'a)) {
		visitor.visit_keyFilterSection(self);
	}
}

impl<'input> CustomRuleContext<'input> for KeyFilterSectionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = OutboundAPIParserContextType;
	fn get_rule_index(&self) -> usize { RULE_keyFilterSection }
	//fn type_rule_index() -> usize where Self: Sized { RULE_keyFilterSection }
}
antlr4rust::tid!{KeyFilterSectionContextExt<'a>}

impl<'input> KeyFilterSectionContextExt<'input>{
	fn new(parent: Option<Rc<dyn OutboundAPIParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<KeyFilterSectionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,KeyFilterSectionContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait KeyFilterSectionContextAttrs<'input>: OutboundAPIParserContext<'input> + BorrowMut<KeyFilterSectionContextExt<'input>>{

fn keyComparison_all(&self) ->  Vec<Rc<KeyComparisonContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn keyComparison(&self, i: usize) -> Option<Rc<KeyComparisonContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves first TerminalNode corresponding to token EOF
/// Returns `None` if there is no child corresponding to token EOF
fn EOF(&self) -> Option<Rc<TerminalNode<'input,OutboundAPIParserContextType>>> where Self:Sized{
	self.get_token(OutboundAPIParser_EOF, 0)
}
/// Retrieves all `TerminalNode`s corresponding to token SEMICOLON in current rule
fn SEMICOLON_all(&self) -> Vec<Rc<TerminalNode<'input,OutboundAPIParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token SEMICOLON, starting from 0.
/// Returns `None` if number of children corresponding to token SEMICOLON is less or equal than `i`.
fn SEMICOLON(&self, i: usize) -> Option<Rc<TerminalNode<'input,OutboundAPIParserContextType>>> where Self:Sized{
	self.get_token(OutboundAPIParser_SEMICOLON, i)
}

}

impl<'input> KeyFilterSectionContextAttrs<'input> for KeyFilterSectionContext<'input>{}

impl<'input, I> OutboundAPIParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn keyFilterSection(&mut self,)
	-> Result<Rc<KeyFilterSectionContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = KeyFilterSectionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 2, RULE_keyFilterSection);
        let mut _localctx: Rc<KeyFilterSectionContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			let mut _alt: i32;
			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			/*InvokeRule keyComparison*/
			recog.base.set_state(41);
			recog.keyComparison()?;

			recog.base.set_state(46);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = recog.interpreter.adaptive_predict(1,&mut recog.base)?;
			while { _alt!=2 && _alt!=INVALID_ALT } {
				if _alt==1 {
					{
					{
					recog.base.set_state(42);
					recog.base.match_token(OutboundAPIParser_SEMICOLON,&mut recog.err_handler)?;

					/*InvokeRule keyComparison*/
					recog.base.set_state(43);
					recog.keyComparison()?;

					}
					} 
				}
				recog.base.set_state(48);
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(1,&mut recog.base)?;
			}
			recog.base.set_state(50);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==OutboundAPIParser_SEMICOLON {
				{
				recog.base.set_state(49);
				recog.base.match_token(OutboundAPIParser_SEMICOLON,&mut recog.err_handler)?;

				}
			}

			recog.base.set_state(52);
			recog.base.match_token(OutboundAPIParser_EOF,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- keyComparison ----------------
#[derive(Debug)]
pub enum KeyComparisonContextAll<'input>{
	IdNumericComparisonContext(IdNumericComparisonContext<'input>),
	IdLatestGlobalComparisonContext(IdLatestGlobalComparisonContext<'input>),
	IdTimeIntervalToPointInTimeComparisonContext(IdTimeIntervalToPointInTimeComparisonContext<'input>),
	IdTimeIntervalInContext(IdTimeIntervalInContext<'input>),
	IdLatestComparisonContext(IdLatestComparisonContext<'input>),
	RankOverContext(RankOverContext<'input>),
	IdPointInTimeArithmeticComparisonContext(IdPointInTimeArithmeticComparisonContext<'input>),
	TextComparisonContext(TextComparisonContext<'input>),
Error(KeyComparisonContext<'input>)
}
antlr4rust::tid!{KeyComparisonContextAll<'a>}

impl<'input> antlr4rust::parser_rule_context::DerefSeal for KeyComparisonContextAll<'input>{}

impl<'input> OutboundAPIParserParserContext<'input> for KeyComparisonContextAll<'input>{}

impl<'input> Deref for KeyComparisonContextAll<'input>{
	type Target = dyn KeyComparisonContextAttrs<'input> + 'input;
	fn deref(&self) -> &Self::Target{
		use KeyComparisonContextAll::*;
		match self{
			IdNumericComparisonContext(inner) => inner,
			IdLatestGlobalComparisonContext(inner) => inner,
			IdTimeIntervalToPointInTimeComparisonContext(inner) => inner,
			IdTimeIntervalInContext(inner) => inner,
			IdLatestComparisonContext(inner) => inner,
			RankOverContext(inner) => inner,
			IdPointInTimeArithmeticComparisonContext(inner) => inner,
			TextComparisonContext(inner) => inner,
Error(inner) => inner
		}
	}
}
impl<'input,'a> Visitable<dyn OutboundAPIParserVisitor<'input> + 'a> for KeyComparisonContextAll<'input>{
	fn accept(&self, visitor: &mut (dyn OutboundAPIParserVisitor<'input> + 'a)) { self.deref().accept(visitor) }
}
impl<'input,'a> Listenable<dyn OutboundAPIParserListener<'input> + 'a> for KeyComparisonContextAll<'input>{
    fn enter(&self, listener: &mut (dyn OutboundAPIParserListener<'input> + 'a)) -> Result<(), ANTLRError> { self.deref().enter(listener) }
    fn exit(&self, listener: &mut (dyn OutboundAPIParserListener<'input> + 'a)) -> Result<(), ANTLRError> { self.deref().exit(listener) }
}



pub type KeyComparisonContext<'input> = BaseParserRuleContext<'input,KeyComparisonContextExt<'input>>;

#[derive(Clone)]
pub struct KeyComparisonContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> OutboundAPIParserContext<'input> for KeyComparisonContext<'input>{}

impl<'input,'a> Listenable<dyn OutboundAPIParserListener<'input> + 'a> for KeyComparisonContext<'input>{
}

impl<'input,'a> Visitable<dyn OutboundAPIParserVisitor<'input> + 'a> for KeyComparisonContext<'input>{
}

impl<'input> CustomRuleContext<'input> for KeyComparisonContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = OutboundAPIParserContextType;
	fn get_rule_index(&self) -> usize { RULE_keyComparison }
	//fn type_rule_index() -> usize where Self: Sized { RULE_keyComparison }
}
antlr4rust::tid!{KeyComparisonContextExt<'a>}

impl<'input> KeyComparisonContextExt<'input>{
	fn new(parent: Option<Rc<dyn OutboundAPIParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<KeyComparisonContextAll<'input>> {
		Rc::new(
		KeyComparisonContextAll::Error(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,KeyComparisonContextExt{

				ph:PhantomData
			}),
		)
		)
	}
}

pub trait KeyComparisonContextAttrs<'input>: OutboundAPIParserContext<'input> + BorrowMut<KeyComparisonContextExt<'input>>{


}

impl<'input> KeyComparisonContextAttrs<'input> for KeyComparisonContext<'input>{}

pub type IdNumericComparisonContext<'input> = BaseParserRuleContext<'input,IdNumericComparisonContextExt<'input>>;

pub trait IdNumericComparisonContextAttrs<'input>: OutboundAPIParserContext<'input>{
	/// Retrieves first TerminalNode corresponding to token ID
	/// Returns `None` if there is no child corresponding to token ID
	fn ID(&self) -> Option<Rc<TerminalNode<'input,OutboundAPIParserContextType>>> where Self:Sized{
		self.get_token(OutboundAPIParser_ID, 0)
	}
	/// Retrieves first TerminalNode corresponding to token COMPARISON_OPERATOR
	/// Returns `None` if there is no child corresponding to token COMPARISON_OPERATOR
	fn COMPARISON_OPERATOR(&self) -> Option<Rc<TerminalNode<'input,OutboundAPIParserContextType>>> where Self:Sized{
		self.get_token(OutboundAPIParser_COMPARISON_OPERATOR, 0)
	}
	/// Retrieves first TerminalNode corresponding to token SIGNED_INTEGER
	/// Returns `None` if there is no child corresponding to token SIGNED_INTEGER
	fn SIGNED_INTEGER(&self) -> Option<Rc<TerminalNode<'input,OutboundAPIParserContextType>>> where Self:Sized{
		self.get_token(OutboundAPIParser_SIGNED_INTEGER, 0)
	}
	/// Retrieves first TerminalNode corresponding to token FLOAT
	/// Returns `None` if there is no child corresponding to token FLOAT
	fn FLOAT(&self) -> Option<Rc<TerminalNode<'input,OutboundAPIParserContextType>>> where Self:Sized{
		self.get_token(OutboundAPIParser_FLOAT, 0)
	}
}

impl<'input> IdNumericComparisonContextAttrs<'input> for IdNumericComparisonContext<'input>{}

pub struct IdNumericComparisonContextExt<'input>{
	base:KeyComparisonContextExt<'input>,
	pub number: Option<TokenType<'input>>,
	ph:PhantomData<&'input str>
}

antlr4rust::tid!{IdNumericComparisonContextExt<'a>}

impl<'input> OutboundAPIParserContext<'input> for IdNumericComparisonContext<'input>{}

impl<'input,'a> Listenable<dyn OutboundAPIParserListener<'input> + 'a> for IdNumericComparisonContext<'input>{
	fn enter(&self,listener: &mut (dyn OutboundAPIParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.enter_every_rule(self)?;
		listener.enter_IdNumericComparison(self);
		Ok(())
	}
	fn exit(&self,listener: &mut (dyn OutboundAPIParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.exit_IdNumericComparison(self);
		listener.exit_every_rule(self)?;
		Ok(())
	}
}

impl<'input,'a> Visitable<dyn OutboundAPIParserVisitor<'input> + 'a> for IdNumericComparisonContext<'input>{
	fn accept(&self,visitor: &mut (dyn OutboundAPIParserVisitor<'input> + 'a)) {
		visitor.visit_IdNumericComparison(self);
	}
}

impl<'input> CustomRuleContext<'input> for IdNumericComparisonContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = OutboundAPIParserContextType;
	fn get_rule_index(&self) -> usize { RULE_keyComparison }
	//fn type_rule_index() -> usize where Self: Sized { RULE_keyComparison }
}

impl<'input> Borrow<KeyComparisonContextExt<'input>> for IdNumericComparisonContext<'input>{
	fn borrow(&self) -> &KeyComparisonContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<KeyComparisonContextExt<'input>> for IdNumericComparisonContext<'input>{
	fn borrow_mut(&mut self) -> &mut KeyComparisonContextExt<'input> { &mut self.base }
}

impl<'input> KeyComparisonContextAttrs<'input> for IdNumericComparisonContext<'input> {}

impl<'input> IdNumericComparisonContextExt<'input>{
	fn new(ctx: &dyn KeyComparisonContextAttrs<'input>) -> Rc<KeyComparisonContextAll<'input>>  {
		Rc::new(
			KeyComparisonContextAll::IdNumericComparisonContext(
				BaseParserRuleContext::copy_from(ctx,IdNumericComparisonContextExt{
					number:None, 
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type IdLatestGlobalComparisonContext<'input> = BaseParserRuleContext<'input,IdLatestGlobalComparisonContextExt<'input>>;

pub trait IdLatestGlobalComparisonContextAttrs<'input>: OutboundAPIParserContext<'input>{
	/// Retrieves first TerminalNode corresponding to token ID
	/// Returns `None` if there is no child corresponding to token ID
	fn ID(&self) -> Option<Rc<TerminalNode<'input,OutboundAPIParserContextType>>> where Self:Sized{
		self.get_token(OutboundAPIParser_ID, 0)
	}
	/// Retrieves first TerminalNode corresponding to token COMPARISON_OPERATOR
	/// Returns `None` if there is no child corresponding to token COMPARISON_OPERATOR
	fn COMPARISON_OPERATOR(&self) -> Option<Rc<TerminalNode<'input,OutboundAPIParserContextType>>> where Self:Sized{
		self.get_token(OutboundAPIParser_COMPARISON_OPERATOR, 0)
	}
	fn latestGlobalFunction(&self) -> Option<Rc<LatestGlobalFunctionContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
}

impl<'input> IdLatestGlobalComparisonContextAttrs<'input> for IdLatestGlobalComparisonContext<'input>{}

pub struct IdLatestGlobalComparisonContextExt<'input>{
	base:KeyComparisonContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr4rust::tid!{IdLatestGlobalComparisonContextExt<'a>}

impl<'input> OutboundAPIParserContext<'input> for IdLatestGlobalComparisonContext<'input>{}

impl<'input,'a> Listenable<dyn OutboundAPIParserListener<'input> + 'a> for IdLatestGlobalComparisonContext<'input>{
	fn enter(&self,listener: &mut (dyn OutboundAPIParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.enter_every_rule(self)?;
		listener.enter_IdLatestGlobalComparison(self);
		Ok(())
	}
	fn exit(&self,listener: &mut (dyn OutboundAPIParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.exit_IdLatestGlobalComparison(self);
		listener.exit_every_rule(self)?;
		Ok(())
	}
}

impl<'input,'a> Visitable<dyn OutboundAPIParserVisitor<'input> + 'a> for IdLatestGlobalComparisonContext<'input>{
	fn accept(&self,visitor: &mut (dyn OutboundAPIParserVisitor<'input> + 'a)) {
		visitor.visit_IdLatestGlobalComparison(self);
	}
}

impl<'input> CustomRuleContext<'input> for IdLatestGlobalComparisonContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = OutboundAPIParserContextType;
	fn get_rule_index(&self) -> usize { RULE_keyComparison }
	//fn type_rule_index() -> usize where Self: Sized { RULE_keyComparison }
}

impl<'input> Borrow<KeyComparisonContextExt<'input>> for IdLatestGlobalComparisonContext<'input>{
	fn borrow(&self) -> &KeyComparisonContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<KeyComparisonContextExt<'input>> for IdLatestGlobalComparisonContext<'input>{
	fn borrow_mut(&mut self) -> &mut KeyComparisonContextExt<'input> { &mut self.base }
}

impl<'input> KeyComparisonContextAttrs<'input> for IdLatestGlobalComparisonContext<'input> {}

impl<'input> IdLatestGlobalComparisonContextExt<'input>{
	fn new(ctx: &dyn KeyComparisonContextAttrs<'input>) -> Rc<KeyComparisonContextAll<'input>>  {
		Rc::new(
			KeyComparisonContextAll::IdLatestGlobalComparisonContext(
				BaseParserRuleContext::copy_from(ctx,IdLatestGlobalComparisonContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type IdTimeIntervalToPointInTimeComparisonContext<'input> = BaseParserRuleContext<'input,IdTimeIntervalToPointInTimeComparisonContextExt<'input>>;

pub trait IdTimeIntervalToPointInTimeComparisonContextAttrs<'input>: OutboundAPIParserContext<'input>{
	/// Retrieves first TerminalNode corresponding to token COMPARISON_OPERATOR
	/// Returns `None` if there is no child corresponding to token COMPARISON_OPERATOR
	fn COMPARISON_OPERATOR(&self) -> Option<Rc<TerminalNode<'input,OutboundAPIParserContextType>>> where Self:Sized{
		self.get_token(OutboundAPIParser_COMPARISON_OPERATOR, 0)
	}
	fn timeIntervalToPointInTime(&self) -> Option<Rc<TimeIntervalToPointInTimeContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
	/// Retrieves first TerminalNode corresponding to token ID
	/// Returns `None` if there is no child corresponding to token ID
	fn ID(&self) -> Option<Rc<TerminalNode<'input,OutboundAPIParserContextType>>> where Self:Sized{
		self.get_token(OutboundAPIParser_ID, 0)
	}
	fn keySurfaceColumn(&self) -> Option<Rc<KeySurfaceColumnContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
}

impl<'input> IdTimeIntervalToPointInTimeComparisonContextAttrs<'input> for IdTimeIntervalToPointInTimeComparisonContext<'input>{}

pub struct IdTimeIntervalToPointInTimeComparisonContextExt<'input>{
	base:KeyComparisonContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr4rust::tid!{IdTimeIntervalToPointInTimeComparisonContextExt<'a>}

impl<'input> OutboundAPIParserContext<'input> for IdTimeIntervalToPointInTimeComparisonContext<'input>{}

impl<'input,'a> Listenable<dyn OutboundAPIParserListener<'input> + 'a> for IdTimeIntervalToPointInTimeComparisonContext<'input>{
	fn enter(&self,listener: &mut (dyn OutboundAPIParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.enter_every_rule(self)?;
		listener.enter_IdTimeIntervalToPointInTimeComparison(self);
		Ok(())
	}
	fn exit(&self,listener: &mut (dyn OutboundAPIParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.exit_IdTimeIntervalToPointInTimeComparison(self);
		listener.exit_every_rule(self)?;
		Ok(())
	}
}

impl<'input,'a> Visitable<dyn OutboundAPIParserVisitor<'input> + 'a> for IdTimeIntervalToPointInTimeComparisonContext<'input>{
	fn accept(&self,visitor: &mut (dyn OutboundAPIParserVisitor<'input> + 'a)) {
		visitor.visit_IdTimeIntervalToPointInTimeComparison(self);
	}
}

impl<'input> CustomRuleContext<'input> for IdTimeIntervalToPointInTimeComparisonContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = OutboundAPIParserContextType;
	fn get_rule_index(&self) -> usize { RULE_keyComparison }
	//fn type_rule_index() -> usize where Self: Sized { RULE_keyComparison }
}

impl<'input> Borrow<KeyComparisonContextExt<'input>> for IdTimeIntervalToPointInTimeComparisonContext<'input>{
	fn borrow(&self) -> &KeyComparisonContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<KeyComparisonContextExt<'input>> for IdTimeIntervalToPointInTimeComparisonContext<'input>{
	fn borrow_mut(&mut self) -> &mut KeyComparisonContextExt<'input> { &mut self.base }
}

impl<'input> KeyComparisonContextAttrs<'input> for IdTimeIntervalToPointInTimeComparisonContext<'input> {}

impl<'input> IdTimeIntervalToPointInTimeComparisonContextExt<'input>{
	fn new(ctx: &dyn KeyComparisonContextAttrs<'input>) -> Rc<KeyComparisonContextAll<'input>>  {
		Rc::new(
			KeyComparisonContextAll::IdTimeIntervalToPointInTimeComparisonContext(
				BaseParserRuleContext::copy_from(ctx,IdTimeIntervalToPointInTimeComparisonContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type IdTimeIntervalInContext<'input> = BaseParserRuleContext<'input,IdTimeIntervalInContextExt<'input>>;

pub trait IdTimeIntervalInContextAttrs<'input>: OutboundAPIParserContext<'input>{
	/// Retrieves first TerminalNode corresponding to token IN
	/// Returns `None` if there is no child corresponding to token IN
	fn IN(&self) -> Option<Rc<TerminalNode<'input,OutboundAPIParserContextType>>> where Self:Sized{
		self.get_token(OutboundAPIParser_IN, 0)
	}
	fn timeIntervalArithmetic(&self) -> Option<Rc<TimeIntervalArithmeticContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
	/// Retrieves first TerminalNode corresponding to token ID
	/// Returns `None` if there is no child corresponding to token ID
	fn ID(&self) -> Option<Rc<TerminalNode<'input,OutboundAPIParserContextType>>> where Self:Sized{
		self.get_token(OutboundAPIParser_ID, 0)
	}
	fn keySurfaceColumn(&self) -> Option<Rc<KeySurfaceColumnContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
}

impl<'input> IdTimeIntervalInContextAttrs<'input> for IdTimeIntervalInContext<'input>{}

pub struct IdTimeIntervalInContextExt<'input>{
	base:KeyComparisonContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr4rust::tid!{IdTimeIntervalInContextExt<'a>}

impl<'input> OutboundAPIParserContext<'input> for IdTimeIntervalInContext<'input>{}

impl<'input,'a> Listenable<dyn OutboundAPIParserListener<'input> + 'a> for IdTimeIntervalInContext<'input>{
	fn enter(&self,listener: &mut (dyn OutboundAPIParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.enter_every_rule(self)?;
		listener.enter_IdTimeIntervalIn(self);
		Ok(())
	}
	fn exit(&self,listener: &mut (dyn OutboundAPIParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.exit_IdTimeIntervalIn(self);
		listener.exit_every_rule(self)?;
		Ok(())
	}
}

impl<'input,'a> Visitable<dyn OutboundAPIParserVisitor<'input> + 'a> for IdTimeIntervalInContext<'input>{
	fn accept(&self,visitor: &mut (dyn OutboundAPIParserVisitor<'input> + 'a)) {
		visitor.visit_IdTimeIntervalIn(self);
	}
}

impl<'input> CustomRuleContext<'input> for IdTimeIntervalInContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = OutboundAPIParserContextType;
	fn get_rule_index(&self) -> usize { RULE_keyComparison }
	//fn type_rule_index() -> usize where Self: Sized { RULE_keyComparison }
}

impl<'input> Borrow<KeyComparisonContextExt<'input>> for IdTimeIntervalInContext<'input>{
	fn borrow(&self) -> &KeyComparisonContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<KeyComparisonContextExt<'input>> for IdTimeIntervalInContext<'input>{
	fn borrow_mut(&mut self) -> &mut KeyComparisonContextExt<'input> { &mut self.base }
}

impl<'input> KeyComparisonContextAttrs<'input> for IdTimeIntervalInContext<'input> {}

impl<'input> IdTimeIntervalInContextExt<'input>{
	fn new(ctx: &dyn KeyComparisonContextAttrs<'input>) -> Rc<KeyComparisonContextAll<'input>>  {
		Rc::new(
			KeyComparisonContextAll::IdTimeIntervalInContext(
				BaseParserRuleContext::copy_from(ctx,IdTimeIntervalInContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type IdLatestComparisonContext<'input> = BaseParserRuleContext<'input,IdLatestComparisonContextExt<'input>>;

pub trait IdLatestComparisonContextAttrs<'input>: OutboundAPIParserContext<'input>{
	/// Retrieves first TerminalNode corresponding to token ID
	/// Returns `None` if there is no child corresponding to token ID
	fn ID(&self) -> Option<Rc<TerminalNode<'input,OutboundAPIParserContextType>>> where Self:Sized{
		self.get_token(OutboundAPIParser_ID, 0)
	}
	/// Retrieves first TerminalNode corresponding to token COMPARISON_OPERATOR
	/// Returns `None` if there is no child corresponding to token COMPARISON_OPERATOR
	fn COMPARISON_OPERATOR(&self) -> Option<Rc<TerminalNode<'input,OutboundAPIParserContextType>>> where Self:Sized{
		self.get_token(OutboundAPIParser_COMPARISON_OPERATOR, 0)
	}
	fn latestFunction(&self) -> Option<Rc<LatestFunctionContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
}

impl<'input> IdLatestComparisonContextAttrs<'input> for IdLatestComparisonContext<'input>{}

pub struct IdLatestComparisonContextExt<'input>{
	base:KeyComparisonContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr4rust::tid!{IdLatestComparisonContextExt<'a>}

impl<'input> OutboundAPIParserContext<'input> for IdLatestComparisonContext<'input>{}

impl<'input,'a> Listenable<dyn OutboundAPIParserListener<'input> + 'a> for IdLatestComparisonContext<'input>{
	fn enter(&self,listener: &mut (dyn OutboundAPIParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.enter_every_rule(self)?;
		listener.enter_IdLatestComparison(self);
		Ok(())
	}
	fn exit(&self,listener: &mut (dyn OutboundAPIParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.exit_IdLatestComparison(self);
		listener.exit_every_rule(self)?;
		Ok(())
	}
}

impl<'input,'a> Visitable<dyn OutboundAPIParserVisitor<'input> + 'a> for IdLatestComparisonContext<'input>{
	fn accept(&self,visitor: &mut (dyn OutboundAPIParserVisitor<'input> + 'a)) {
		visitor.visit_IdLatestComparison(self);
	}
}

impl<'input> CustomRuleContext<'input> for IdLatestComparisonContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = OutboundAPIParserContextType;
	fn get_rule_index(&self) -> usize { RULE_keyComparison }
	//fn type_rule_index() -> usize where Self: Sized { RULE_keyComparison }
}

impl<'input> Borrow<KeyComparisonContextExt<'input>> for IdLatestComparisonContext<'input>{
	fn borrow(&self) -> &KeyComparisonContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<KeyComparisonContextExt<'input>> for IdLatestComparisonContext<'input>{
	fn borrow_mut(&mut self) -> &mut KeyComparisonContextExt<'input> { &mut self.base }
}

impl<'input> KeyComparisonContextAttrs<'input> for IdLatestComparisonContext<'input> {}

impl<'input> IdLatestComparisonContextExt<'input>{
	fn new(ctx: &dyn KeyComparisonContextAttrs<'input>) -> Rc<KeyComparisonContextAll<'input>>  {
		Rc::new(
			KeyComparisonContextAll::IdLatestComparisonContext(
				BaseParserRuleContext::copy_from(ctx,IdLatestComparisonContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type RankOverContext<'input> = BaseParserRuleContext<'input,RankOverContextExt<'input>>;

pub trait RankOverContextAttrs<'input>: OutboundAPIParserContext<'input>{
	fn rankOverFunction(&self) -> Option<Rc<RankOverFunctionContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
}

impl<'input> RankOverContextAttrs<'input> for RankOverContext<'input>{}

pub struct RankOverContextExt<'input>{
	base:KeyComparisonContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr4rust::tid!{RankOverContextExt<'a>}

impl<'input> OutboundAPIParserContext<'input> for RankOverContext<'input>{}

impl<'input,'a> Listenable<dyn OutboundAPIParserListener<'input> + 'a> for RankOverContext<'input>{
	fn enter(&self,listener: &mut (dyn OutboundAPIParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.enter_every_rule(self)?;
		listener.enter_RankOver(self);
		Ok(())
	}
	fn exit(&self,listener: &mut (dyn OutboundAPIParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.exit_RankOver(self);
		listener.exit_every_rule(self)?;
		Ok(())
	}
}

impl<'input,'a> Visitable<dyn OutboundAPIParserVisitor<'input> + 'a> for RankOverContext<'input>{
	fn accept(&self,visitor: &mut (dyn OutboundAPIParserVisitor<'input> + 'a)) {
		visitor.visit_RankOver(self);
	}
}

impl<'input> CustomRuleContext<'input> for RankOverContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = OutboundAPIParserContextType;
	fn get_rule_index(&self) -> usize { RULE_keyComparison }
	//fn type_rule_index() -> usize where Self: Sized { RULE_keyComparison }
}

impl<'input> Borrow<KeyComparisonContextExt<'input>> for RankOverContext<'input>{
	fn borrow(&self) -> &KeyComparisonContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<KeyComparisonContextExt<'input>> for RankOverContext<'input>{
	fn borrow_mut(&mut self) -> &mut KeyComparisonContextExt<'input> { &mut self.base }
}

impl<'input> KeyComparisonContextAttrs<'input> for RankOverContext<'input> {}

impl<'input> RankOverContextExt<'input>{
	fn new(ctx: &dyn KeyComparisonContextAttrs<'input>) -> Rc<KeyComparisonContextAll<'input>>  {
		Rc::new(
			KeyComparisonContextAll::RankOverContext(
				BaseParserRuleContext::copy_from(ctx,RankOverContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type IdPointInTimeArithmeticComparisonContext<'input> = BaseParserRuleContext<'input,IdPointInTimeArithmeticComparisonContextExt<'input>>;

pub trait IdPointInTimeArithmeticComparisonContextAttrs<'input>: OutboundAPIParserContext<'input>{
	/// Retrieves first TerminalNode corresponding to token COMPARISON_OPERATOR
	/// Returns `None` if there is no child corresponding to token COMPARISON_OPERATOR
	fn COMPARISON_OPERATOR(&self) -> Option<Rc<TerminalNode<'input,OutboundAPIParserContextType>>> where Self:Sized{
		self.get_token(OutboundAPIParser_COMPARISON_OPERATOR, 0)
	}
	fn pointInTimeArithmetic(&self) -> Option<Rc<PointInTimeArithmeticContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
	/// Retrieves first TerminalNode corresponding to token ID
	/// Returns `None` if there is no child corresponding to token ID
	fn ID(&self) -> Option<Rc<TerminalNode<'input,OutboundAPIParserContextType>>> where Self:Sized{
		self.get_token(OutboundAPIParser_ID, 0)
	}
	fn keySurfaceColumn(&self) -> Option<Rc<KeySurfaceColumnContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
}

impl<'input> IdPointInTimeArithmeticComparisonContextAttrs<'input> for IdPointInTimeArithmeticComparisonContext<'input>{}

pub struct IdPointInTimeArithmeticComparisonContextExt<'input>{
	base:KeyComparisonContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr4rust::tid!{IdPointInTimeArithmeticComparisonContextExt<'a>}

impl<'input> OutboundAPIParserContext<'input> for IdPointInTimeArithmeticComparisonContext<'input>{}

impl<'input,'a> Listenable<dyn OutboundAPIParserListener<'input> + 'a> for IdPointInTimeArithmeticComparisonContext<'input>{
	fn enter(&self,listener: &mut (dyn OutboundAPIParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.enter_every_rule(self)?;
		listener.enter_IdPointInTimeArithmeticComparison(self);
		Ok(())
	}
	fn exit(&self,listener: &mut (dyn OutboundAPIParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.exit_IdPointInTimeArithmeticComparison(self);
		listener.exit_every_rule(self)?;
		Ok(())
	}
}

impl<'input,'a> Visitable<dyn OutboundAPIParserVisitor<'input> + 'a> for IdPointInTimeArithmeticComparisonContext<'input>{
	fn accept(&self,visitor: &mut (dyn OutboundAPIParserVisitor<'input> + 'a)) {
		visitor.visit_IdPointInTimeArithmeticComparison(self);
	}
}

impl<'input> CustomRuleContext<'input> for IdPointInTimeArithmeticComparisonContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = OutboundAPIParserContextType;
	fn get_rule_index(&self) -> usize { RULE_keyComparison }
	//fn type_rule_index() -> usize where Self: Sized { RULE_keyComparison }
}

impl<'input> Borrow<KeyComparisonContextExt<'input>> for IdPointInTimeArithmeticComparisonContext<'input>{
	fn borrow(&self) -> &KeyComparisonContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<KeyComparisonContextExt<'input>> for IdPointInTimeArithmeticComparisonContext<'input>{
	fn borrow_mut(&mut self) -> &mut KeyComparisonContextExt<'input> { &mut self.base }
}

impl<'input> KeyComparisonContextAttrs<'input> for IdPointInTimeArithmeticComparisonContext<'input> {}

impl<'input> IdPointInTimeArithmeticComparisonContextExt<'input>{
	fn new(ctx: &dyn KeyComparisonContextAttrs<'input>) -> Rc<KeyComparisonContextAll<'input>>  {
		Rc::new(
			KeyComparisonContextAll::IdPointInTimeArithmeticComparisonContext(
				BaseParserRuleContext::copy_from(ctx,IdPointInTimeArithmeticComparisonContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type TextComparisonContext<'input> = BaseParserRuleContext<'input,TextComparisonContextExt<'input>>;

pub trait TextComparisonContextAttrs<'input>: OutboundAPIParserContext<'input>{
	fn textColumn(&self) -> Option<Rc<TextColumnContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
	/// Retrieves first TerminalNode corresponding to token COMPARISON_OPERATOR
	/// Returns `None` if there is no child corresponding to token COMPARISON_OPERATOR
	fn COMPARISON_OPERATOR(&self) -> Option<Rc<TerminalNode<'input,OutboundAPIParserContextType>>> where Self:Sized{
		self.get_token(OutboundAPIParser_COMPARISON_OPERATOR, 0)
	}
	fn genericValue(&self) -> Option<Rc<GenericValueContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
}

impl<'input> TextComparisonContextAttrs<'input> for TextComparisonContext<'input>{}

pub struct TextComparisonContextExt<'input>{
	base:KeyComparisonContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr4rust::tid!{TextComparisonContextExt<'a>}

impl<'input> OutboundAPIParserContext<'input> for TextComparisonContext<'input>{}

impl<'input,'a> Listenable<dyn OutboundAPIParserListener<'input> + 'a> for TextComparisonContext<'input>{
	fn enter(&self,listener: &mut (dyn OutboundAPIParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.enter_every_rule(self)?;
		listener.enter_TextComparison(self);
		Ok(())
	}
	fn exit(&self,listener: &mut (dyn OutboundAPIParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.exit_TextComparison(self);
		listener.exit_every_rule(self)?;
		Ok(())
	}
}

impl<'input,'a> Visitable<dyn OutboundAPIParserVisitor<'input> + 'a> for TextComparisonContext<'input>{
	fn accept(&self,visitor: &mut (dyn OutboundAPIParserVisitor<'input> + 'a)) {
		visitor.visit_TextComparison(self);
	}
}

impl<'input> CustomRuleContext<'input> for TextComparisonContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = OutboundAPIParserContextType;
	fn get_rule_index(&self) -> usize { RULE_keyComparison }
	//fn type_rule_index() -> usize where Self: Sized { RULE_keyComparison }
}

impl<'input> Borrow<KeyComparisonContextExt<'input>> for TextComparisonContext<'input>{
	fn borrow(&self) -> &KeyComparisonContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<KeyComparisonContextExt<'input>> for TextComparisonContext<'input>{
	fn borrow_mut(&mut self) -> &mut KeyComparisonContextExt<'input> { &mut self.base }
}

impl<'input> KeyComparisonContextAttrs<'input> for TextComparisonContext<'input> {}

impl<'input> TextComparisonContextExt<'input>{
	fn new(ctx: &dyn KeyComparisonContextAttrs<'input>) -> Rc<KeyComparisonContextAll<'input>>  {
		Rc::new(
			KeyComparisonContextAll::TextComparisonContext(
				BaseParserRuleContext::copy_from(ctx,TextComparisonContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

impl<'input, I> OutboundAPIParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn keyComparison(&mut self,)
	-> Result<Rc<KeyComparisonContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = KeyComparisonContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 4, RULE_keyComparison);
        let mut _localctx: Rc<KeyComparisonContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(86);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(6,&mut recog.base)? {
				1 =>{
					let tmp = IdPointInTimeArithmeticComparisonContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 1)?;
					_localctx = tmp;
					{
					recog.base.set_state(56);
					recog.err_handler.sync(&mut recog.base)?;
					match recog.base.input.la(1) {
					OutboundAPIParser_ID 
						=> {
							{
							recog.base.set_state(54);
							recog.base.match_token(OutboundAPIParser_ID,&mut recog.err_handler)?;

							}
						}

					OutboundAPIParser_CI_VALIDITY_PERIOD_START |OutboundAPIParser_CI_VALIDITY_PERIOD_END |
					OutboundAPIParser_CI_INSTANCE_CODE |OutboundAPIParser_CI_ISP 
						=> {
							{
							/*InvokeRule keySurfaceColumn*/
							recog.base.set_state(55);
							recog.keySurfaceColumn()?;

							}
						}

						_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
					}
					recog.base.set_state(58);
					recog.base.match_token(OutboundAPIParser_COMPARISON_OPERATOR,&mut recog.err_handler)?;

					/*InvokeRule pointInTimeArithmetic*/
					recog.base.set_state(59);
					recog.pointInTimeArithmetic()?;

					}
				}
			,
				2 =>{
					let tmp = IdTimeIntervalInContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 2)?;
					_localctx = tmp;
					{
					recog.base.set_state(62);
					recog.err_handler.sync(&mut recog.base)?;
					match recog.base.input.la(1) {
					OutboundAPIParser_ID 
						=> {
							{
							recog.base.set_state(60);
							recog.base.match_token(OutboundAPIParser_ID,&mut recog.err_handler)?;

							}
						}

					OutboundAPIParser_CI_VALIDITY_PERIOD_START |OutboundAPIParser_CI_VALIDITY_PERIOD_END |
					OutboundAPIParser_CI_INSTANCE_CODE |OutboundAPIParser_CI_ISP 
						=> {
							{
							/*InvokeRule keySurfaceColumn*/
							recog.base.set_state(61);
							recog.keySurfaceColumn()?;

							}
						}

						_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
					}
					recog.base.set_state(64);
					recog.base.match_token(OutboundAPIParser_IN,&mut recog.err_handler)?;

					/*InvokeRule timeIntervalArithmetic*/
					recog.base.set_state(65);
					recog.timeIntervalArithmetic()?;

					}
				}
			,
				3 =>{
					let tmp = IdNumericComparisonContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 3)?;
					_localctx = tmp;
					{
					recog.base.set_state(66);
					recog.base.match_token(OutboundAPIParser_ID,&mut recog.err_handler)?;

					recog.base.set_state(67);
					recog.base.match_token(OutboundAPIParser_COMPARISON_OPERATOR,&mut recog.err_handler)?;

					recog.base.set_state(68);
					if let KeyComparisonContextAll::IdNumericComparisonContext(ctx) = cast_mut::<_,KeyComparisonContextAll >(&mut _localctx){
					ctx.number = recog.base.input.lt(1).cloned(); } else {unreachable!("cant cast");} 
					_la = recog.base.input.la(1);
					if { !(_la==OutboundAPIParser_SIGNED_INTEGER || _la==OutboundAPIParser_FLOAT) } {
						let tmp = recog.err_handler.recover_inline(&mut recog.base)?;
						if let KeyComparisonContextAll::IdNumericComparisonContext(ctx) = cast_mut::<_,KeyComparisonContextAll >(&mut _localctx){
						ctx.number = Some(tmp.clone()); } else {unreachable!("cant cast");}  

					}
					else {
						if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
						recog.err_handler.report_match(&mut recog.base);
						recog.base.consume(&mut recog.err_handler);
					}
					}
				}
			,
				4 =>{
					let tmp = IdLatestGlobalComparisonContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 4)?;
					_localctx = tmp;
					{
					recog.base.set_state(69);
					recog.base.match_token(OutboundAPIParser_ID,&mut recog.err_handler)?;

					recog.base.set_state(70);
					recog.base.match_token(OutboundAPIParser_COMPARISON_OPERATOR,&mut recog.err_handler)?;

					/*InvokeRule latestGlobalFunction*/
					recog.base.set_state(71);
					recog.latestGlobalFunction()?;

					}
				}
			,
				5 =>{
					let tmp = IdTimeIntervalToPointInTimeComparisonContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 5)?;
					_localctx = tmp;
					{
					recog.base.set_state(74);
					recog.err_handler.sync(&mut recog.base)?;
					match recog.base.input.la(1) {
					OutboundAPIParser_ID 
						=> {
							{
							recog.base.set_state(72);
							recog.base.match_token(OutboundAPIParser_ID,&mut recog.err_handler)?;

							}
						}

					OutboundAPIParser_CI_VALIDITY_PERIOD_START |OutboundAPIParser_CI_VALIDITY_PERIOD_END |
					OutboundAPIParser_CI_INSTANCE_CODE |OutboundAPIParser_CI_ISP 
						=> {
							{
							/*InvokeRule keySurfaceColumn*/
							recog.base.set_state(73);
							recog.keySurfaceColumn()?;

							}
						}

						_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
					}
					recog.base.set_state(76);
					recog.base.match_token(OutboundAPIParser_COMPARISON_OPERATOR,&mut recog.err_handler)?;

					/*InvokeRule timeIntervalToPointInTime*/
					recog.base.set_state(77);
					recog.timeIntervalToPointInTime()?;

					}
				}
			,
				6 =>{
					let tmp = IdLatestComparisonContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 6)?;
					_localctx = tmp;
					{
					recog.base.set_state(78);
					recog.base.match_token(OutboundAPIParser_ID,&mut recog.err_handler)?;

					recog.base.set_state(79);
					recog.base.match_token(OutboundAPIParser_COMPARISON_OPERATOR,&mut recog.err_handler)?;

					/*InvokeRule latestFunction*/
					recog.base.set_state(80);
					recog.latestFunction()?;

					}
				}
			,
				7 =>{
					let tmp = TextComparisonContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 7)?;
					_localctx = tmp;
					{
					/*InvokeRule textColumn*/
					recog.base.set_state(81);
					recog.textColumn()?;

					recog.base.set_state(82);
					recog.base.match_token(OutboundAPIParser_COMPARISON_OPERATOR,&mut recog.err_handler)?;

					/*InvokeRule genericValue*/
					recog.base.set_state(83);
					recog.genericValue()?;

					}
				}
			,
				8 =>{
					let tmp = RankOverContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 8)?;
					_localctx = tmp;
					{
					/*InvokeRule rankOverFunction*/
					recog.base.set_state(85);
					recog.rankOverFunction()?;

					}
				}

				_ => {}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- keySurfaceColumn ----------------
pub type KeySurfaceColumnContextAll<'input> = KeySurfaceColumnContext<'input>;


pub type KeySurfaceColumnContext<'input> = BaseParserRuleContext<'input,KeySurfaceColumnContextExt<'input>>;

#[derive(Clone)]
pub struct KeySurfaceColumnContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> OutboundAPIParserContext<'input> for KeySurfaceColumnContext<'input>{}

impl<'input,'a> Listenable<dyn OutboundAPIParserListener<'input> + 'a> for KeySurfaceColumnContext<'input>{
		fn enter(&self,listener: &mut (dyn OutboundAPIParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_keySurfaceColumn(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn OutboundAPIParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_keySurfaceColumn(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn OutboundAPIParserVisitor<'input> + 'a> for KeySurfaceColumnContext<'input>{
	fn accept(&self,visitor: &mut (dyn OutboundAPIParserVisitor<'input> + 'a)) {
		visitor.visit_keySurfaceColumn(self);
	}
}

impl<'input> CustomRuleContext<'input> for KeySurfaceColumnContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = OutboundAPIParserContextType;
	fn get_rule_index(&self) -> usize { RULE_keySurfaceColumn }
	//fn type_rule_index() -> usize where Self: Sized { RULE_keySurfaceColumn }
}
antlr4rust::tid!{KeySurfaceColumnContextExt<'a>}

impl<'input> KeySurfaceColumnContextExt<'input>{
	fn new(parent: Option<Rc<dyn OutboundAPIParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<KeySurfaceColumnContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,KeySurfaceColumnContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait KeySurfaceColumnContextAttrs<'input>: OutboundAPIParserContext<'input> + BorrowMut<KeySurfaceColumnContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token CI_VALIDITY_PERIOD_START
/// Returns `None` if there is no child corresponding to token CI_VALIDITY_PERIOD_START
fn CI_VALIDITY_PERIOD_START(&self) -> Option<Rc<TerminalNode<'input,OutboundAPIParserContextType>>> where Self:Sized{
	self.get_token(OutboundAPIParser_CI_VALIDITY_PERIOD_START, 0)
}
/// Retrieves first TerminalNode corresponding to token CI_VALIDITY_PERIOD_END
/// Returns `None` if there is no child corresponding to token CI_VALIDITY_PERIOD_END
fn CI_VALIDITY_PERIOD_END(&self) -> Option<Rc<TerminalNode<'input,OutboundAPIParserContextType>>> where Self:Sized{
	self.get_token(OutboundAPIParser_CI_VALIDITY_PERIOD_END, 0)
}
/// Retrieves first TerminalNode corresponding to token CI_INSTANCE_CODE
/// Returns `None` if there is no child corresponding to token CI_INSTANCE_CODE
fn CI_INSTANCE_CODE(&self) -> Option<Rc<TerminalNode<'input,OutboundAPIParserContextType>>> where Self:Sized{
	self.get_token(OutboundAPIParser_CI_INSTANCE_CODE, 0)
}
/// Retrieves first TerminalNode corresponding to token CI_ISP
/// Returns `None` if there is no child corresponding to token CI_ISP
fn CI_ISP(&self) -> Option<Rc<TerminalNode<'input,OutboundAPIParserContextType>>> where Self:Sized{
	self.get_token(OutboundAPIParser_CI_ISP, 0)
}

}

impl<'input> KeySurfaceColumnContextAttrs<'input> for KeySurfaceColumnContext<'input>{}

impl<'input, I> OutboundAPIParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn keySurfaceColumn(&mut self,)
	-> Result<Rc<KeySurfaceColumnContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = KeySurfaceColumnContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 6, RULE_keySurfaceColumn);
        let mut _localctx: Rc<KeySurfaceColumnContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(88);
			_la = recog.base.input.la(1);
			if { !((((_la) & !0x3f) == 0 && ((1usize << _la) & 46) != 0)) } {
				recog.err_handler.recover_inline(&mut recog.base)?;

			}
			else {
				if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
				recog.err_handler.report_match(&mut recog.base);
				recog.base.consume(&mut recog.err_handler);
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- textColumn ----------------
pub type TextColumnContextAll<'input> = TextColumnContext<'input>;


pub type TextColumnContext<'input> = BaseParserRuleContext<'input,TextColumnContextExt<'input>>;

#[derive(Clone)]
pub struct TextColumnContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> OutboundAPIParserContext<'input> for TextColumnContext<'input>{}

impl<'input,'a> Listenable<dyn OutboundAPIParserListener<'input> + 'a> for TextColumnContext<'input>{
		fn enter(&self,listener: &mut (dyn OutboundAPIParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_textColumn(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn OutboundAPIParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_textColumn(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn OutboundAPIParserVisitor<'input> + 'a> for TextColumnContext<'input>{
	fn accept(&self,visitor: &mut (dyn OutboundAPIParserVisitor<'input> + 'a)) {
		visitor.visit_textColumn(self);
	}
}

impl<'input> CustomRuleContext<'input> for TextColumnContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = OutboundAPIParserContextType;
	fn get_rule_index(&self) -> usize { RULE_textColumn }
	//fn type_rule_index() -> usize where Self: Sized { RULE_textColumn }
}
antlr4rust::tid!{TextColumnContextExt<'a>}

impl<'input> TextColumnContextExt<'input>{
	fn new(parent: Option<Rc<dyn OutboundAPIParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<TextColumnContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,TextColumnContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait TextColumnContextAttrs<'input>: OutboundAPIParserContext<'input> + BorrowMut<TextColumnContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token CI_BIDID
/// Returns `None` if there is no child corresponding to token CI_BIDID
fn CI_BIDID(&self) -> Option<Rc<TerminalNode<'input,OutboundAPIParserContextType>>> where Self:Sized{
	self.get_token(OutboundAPIParser_CI_BIDID, 0)
}
/// Retrieves first TerminalNode corresponding to token CI_DIRECTION
/// Returns `None` if there is no child corresponding to token CI_DIRECTION
fn CI_DIRECTION(&self) -> Option<Rc<TerminalNode<'input,OutboundAPIParserContextType>>> where Self:Sized{
	self.get_token(OutboundAPIParser_CI_DIRECTION, 0)
}
/// Retrieves first TerminalNode corresponding to token CI_STATUS
/// Returns `None` if there is no child corresponding to token CI_STATUS
fn CI_STATUS(&self) -> Option<Rc<TerminalNode<'input,OutboundAPIParserContextType>>> where Self:Sized{
	self.get_token(OutboundAPIParser_CI_STATUS, 0)
}
/// Retrieves first TerminalNode corresponding to token CI_OPTION_EXPIRY
/// Returns `None` if there is no child corresponding to token CI_OPTION_EXPIRY
fn CI_OPTION_EXPIRY(&self) -> Option<Rc<TerminalNode<'input,OutboundAPIParserContextType>>> where Self:Sized{
	self.get_token(OutboundAPIParser_CI_OPTION_EXPIRY, 0)
}

}

impl<'input> TextColumnContextAttrs<'input> for TextColumnContext<'input>{}

impl<'input, I> OutboundAPIParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn textColumn(&mut self,)
	-> Result<Rc<TextColumnContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = TextColumnContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 8, RULE_textColumn);
        let mut _localctx: Rc<TextColumnContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(90);
			_la = recog.base.input.la(1);
			if { !((((_la) & !0x3f) == 0 && ((1usize << _la) & 464) != 0)) } {
				recog.err_handler.recover_inline(&mut recog.base)?;

			}
			else {
				if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
				recog.err_handler.report_match(&mut recog.base);
				recog.base.consume(&mut recog.err_handler);
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- latestGlobalFunction ----------------
pub type LatestGlobalFunctionContextAll<'input> = LatestGlobalFunctionContext<'input>;


pub type LatestGlobalFunctionContext<'input> = BaseParserRuleContext<'input,LatestGlobalFunctionContextExt<'input>>;

#[derive(Clone)]
pub struct LatestGlobalFunctionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> OutboundAPIParserContext<'input> for LatestGlobalFunctionContext<'input>{}

impl<'input,'a> Listenable<dyn OutboundAPIParserListener<'input> + 'a> for LatestGlobalFunctionContext<'input>{
		fn enter(&self,listener: &mut (dyn OutboundAPIParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_latestGlobalFunction(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn OutboundAPIParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_latestGlobalFunction(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn OutboundAPIParserVisitor<'input> + 'a> for LatestGlobalFunctionContext<'input>{
	fn accept(&self,visitor: &mut (dyn OutboundAPIParserVisitor<'input> + 'a)) {
		visitor.visit_latestGlobalFunction(self);
	}
}

impl<'input> CustomRuleContext<'input> for LatestGlobalFunctionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = OutboundAPIParserContextType;
	fn get_rule_index(&self) -> usize { RULE_latestGlobalFunction }
	//fn type_rule_index() -> usize where Self: Sized { RULE_latestGlobalFunction }
}
antlr4rust::tid!{LatestGlobalFunctionContextExt<'a>}

impl<'input> LatestGlobalFunctionContextExt<'input>{
	fn new(parent: Option<Rc<dyn OutboundAPIParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<LatestGlobalFunctionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,LatestGlobalFunctionContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait LatestGlobalFunctionContextAttrs<'input>: OutboundAPIParserContext<'input> + BorrowMut<LatestGlobalFunctionContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token LATEST_GLOBAL
/// Returns `None` if there is no child corresponding to token LATEST_GLOBAL
fn LATEST_GLOBAL(&self) -> Option<Rc<TerminalNode<'input,OutboundAPIParserContextType>>> where Self:Sized{
	self.get_token(OutboundAPIParser_LATEST_GLOBAL, 0)
}
/// Retrieves first TerminalNode corresponding to token LB
/// Returns `None` if there is no child corresponding to token LB
fn LB(&self) -> Option<Rc<TerminalNode<'input,OutboundAPIParserContextType>>> where Self:Sized{
	self.get_token(OutboundAPIParser_LB, 0)
}
/// Retrieves first TerminalNode corresponding to token RB
/// Returns `None` if there is no child corresponding to token RB
fn RB(&self) -> Option<Rc<TerminalNode<'input,OutboundAPIParserContextType>>> where Self:Sized{
	self.get_token(OutboundAPIParser_RB, 0)
}

}

impl<'input> LatestGlobalFunctionContextAttrs<'input> for LatestGlobalFunctionContext<'input>{}

impl<'input, I> OutboundAPIParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn latestGlobalFunction(&mut self,)
	-> Result<Rc<LatestGlobalFunctionContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = LatestGlobalFunctionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 10, RULE_latestGlobalFunction);
        let mut _localctx: Rc<LatestGlobalFunctionContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(92);
			recog.base.match_token(OutboundAPIParser_LATEST_GLOBAL,&mut recog.err_handler)?;

			recog.base.set_state(93);
			recog.base.match_token(OutboundAPIParser_LB,&mut recog.err_handler)?;

			recog.base.set_state(94);
			recog.base.match_token(OutboundAPIParser_RB,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- timeInterval ----------------
pub type TimeIntervalContextAll<'input> = TimeIntervalContext<'input>;


pub type TimeIntervalContext<'input> = BaseParserRuleContext<'input,TimeIntervalContextExt<'input>>;

#[derive(Clone)]
pub struct TimeIntervalContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> OutboundAPIParserContext<'input> for TimeIntervalContext<'input>{}

impl<'input,'a> Listenable<dyn OutboundAPIParserListener<'input> + 'a> for TimeIntervalContext<'input>{
		fn enter(&self,listener: &mut (dyn OutboundAPIParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_timeInterval(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn OutboundAPIParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_timeInterval(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn OutboundAPIParserVisitor<'input> + 'a> for TimeIntervalContext<'input>{
	fn accept(&self,visitor: &mut (dyn OutboundAPIParserVisitor<'input> + 'a)) {
		visitor.visit_timeInterval(self);
	}
}

impl<'input> CustomRuleContext<'input> for TimeIntervalContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = OutboundAPIParserContextType;
	fn get_rule_index(&self) -> usize { RULE_timeInterval }
	//fn type_rule_index() -> usize where Self: Sized { RULE_timeInterval }
}
antlr4rust::tid!{TimeIntervalContextExt<'a>}

impl<'input> TimeIntervalContextExt<'input>{
	fn new(parent: Option<Rc<dyn OutboundAPIParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<TimeIntervalContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,TimeIntervalContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait TimeIntervalContextAttrs<'input>: OutboundAPIParserContext<'input> + BorrowMut<TimeIntervalContextExt<'input>>{

/// Retrieves all `TerminalNode`s corresponding to token POINT_IN_TIME in current rule
fn POINT_IN_TIME_all(&self) -> Vec<Rc<TerminalNode<'input,OutboundAPIParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token POINT_IN_TIME, starting from 0.
/// Returns `None` if number of children corresponding to token POINT_IN_TIME is less or equal than `i`.
fn POINT_IN_TIME(&self, i: usize) -> Option<Rc<TerminalNode<'input,OutboundAPIParserContextType>>> where Self:Sized{
	self.get_token(OutboundAPIParser_POINT_IN_TIME, i)
}
/// Retrieves first TerminalNode corresponding to token COMMA
/// Returns `None` if there is no child corresponding to token COMMA
fn COMMA(&self) -> Option<Rc<TerminalNode<'input,OutboundAPIParserContextType>>> where Self:Sized{
	self.get_token(OutboundAPIParser_COMMA, 0)
}

}

impl<'input> TimeIntervalContextAttrs<'input> for TimeIntervalContext<'input>{}

impl<'input, I> OutboundAPIParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn timeInterval(&mut self,)
	-> Result<Rc<TimeIntervalContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = TimeIntervalContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 12, RULE_timeInterval);
        let mut _localctx: Rc<TimeIntervalContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(96);
			recog.base.match_token(OutboundAPIParser_POINT_IN_TIME,&mut recog.err_handler)?;

			recog.base.set_state(97);
			recog.base.match_token(OutboundAPIParser_COMMA,&mut recog.err_handler)?;

			recog.base.set_state(98);
			recog.base.match_token(OutboundAPIParser_POINT_IN_TIME,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- timeIntervalOrFunction ----------------
pub type TimeIntervalOrFunctionContextAll<'input> = TimeIntervalOrFunctionContext<'input>;


pub type TimeIntervalOrFunctionContext<'input> = BaseParserRuleContext<'input,TimeIntervalOrFunctionContextExt<'input>>;

#[derive(Clone)]
pub struct TimeIntervalOrFunctionContextExt<'input>{
	pub expressionTimeZone: Option<TokenType<'input>>,
ph:PhantomData<&'input str>
}

impl<'input> OutboundAPIParserContext<'input> for TimeIntervalOrFunctionContext<'input>{}

impl<'input,'a> Listenable<dyn OutboundAPIParserListener<'input> + 'a> for TimeIntervalOrFunctionContext<'input>{
		fn enter(&self,listener: &mut (dyn OutboundAPIParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_timeIntervalOrFunction(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn OutboundAPIParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_timeIntervalOrFunction(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn OutboundAPIParserVisitor<'input> + 'a> for TimeIntervalOrFunctionContext<'input>{
	fn accept(&self,visitor: &mut (dyn OutboundAPIParserVisitor<'input> + 'a)) {
		visitor.visit_timeIntervalOrFunction(self);
	}
}

impl<'input> CustomRuleContext<'input> for TimeIntervalOrFunctionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = OutboundAPIParserContextType;
	fn get_rule_index(&self) -> usize { RULE_timeIntervalOrFunction }
	//fn type_rule_index() -> usize where Self: Sized { RULE_timeIntervalOrFunction }
}
antlr4rust::tid!{TimeIntervalOrFunctionContextExt<'a>}

impl<'input> TimeIntervalOrFunctionContextExt<'input>{
	fn new(parent: Option<Rc<dyn OutboundAPIParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<TimeIntervalOrFunctionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,TimeIntervalOrFunctionContextExt{
				expressionTimeZone: None, 

				ph:PhantomData
			}),
		)
	}
}

pub trait TimeIntervalOrFunctionContextAttrs<'input>: OutboundAPIParserContext<'input> + BorrowMut<TimeIntervalOrFunctionContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token TIME_INTERVAL_EXPLICIT_FUNCTION_NAME
/// Returns `None` if there is no child corresponding to token TIME_INTERVAL_EXPLICIT_FUNCTION_NAME
fn TIME_INTERVAL_EXPLICIT_FUNCTION_NAME(&self) -> Option<Rc<TerminalNode<'input,OutboundAPIParserContextType>>> where Self:Sized{
	self.get_token(OutboundAPIParser_TIME_INTERVAL_EXPLICIT_FUNCTION_NAME, 0)
}
/// Retrieves first TerminalNode corresponding to token LB
/// Returns `None` if there is no child corresponding to token LB
fn LB(&self) -> Option<Rc<TerminalNode<'input,OutboundAPIParserContextType>>> where Self:Sized{
	self.get_token(OutboundAPIParser_LB, 0)
}
fn timeInterval(&self) -> Option<Rc<TimeIntervalContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token RB
/// Returns `None` if there is no child corresponding to token RB
fn RB(&self) -> Option<Rc<TerminalNode<'input,OutboundAPIParserContextType>>> where Self:Sized{
	self.get_token(OutboundAPIParser_RB, 0)
}
/// Retrieves first TerminalNode corresponding to token TIME_INTERVAL_FUNCTION_NAME
/// Returns `None` if there is no child corresponding to token TIME_INTERVAL_FUNCTION_NAME
fn TIME_INTERVAL_FUNCTION_NAME(&self) -> Option<Rc<TerminalNode<'input,OutboundAPIParserContextType>>> where Self:Sized{
	self.get_token(OutboundAPIParser_TIME_INTERVAL_FUNCTION_NAME, 0)
}
fn pointInTimeArithmetic(&self) -> Option<Rc<PointInTimeArithmeticContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token COMMA
/// Returns `None` if there is no child corresponding to token COMMA
fn COMMA(&self) -> Option<Rc<TerminalNode<'input,OutboundAPIParserContextType>>> where Self:Sized{
	self.get_token(OutboundAPIParser_COMMA, 0)
}
/// Retrieves first TerminalNode corresponding to token TIME_ZONE_IANA
/// Returns `None` if there is no child corresponding to token TIME_ZONE_IANA
fn TIME_ZONE_IANA(&self) -> Option<Rc<TerminalNode<'input,OutboundAPIParserContextType>>> where Self:Sized{
	self.get_token(OutboundAPIParser_TIME_ZONE_IANA, 0)
}

}

impl<'input> TimeIntervalOrFunctionContextAttrs<'input> for TimeIntervalOrFunctionContext<'input>{}

impl<'input, I> OutboundAPIParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn timeIntervalOrFunction(&mut self,)
	-> Result<Rc<TimeIntervalOrFunctionContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = TimeIntervalOrFunctionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 14, RULE_timeIntervalOrFunction);
        let mut _localctx: Rc<TimeIntervalOrFunctionContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(114);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			OutboundAPIParser_TIME_INTERVAL_EXPLICIT_FUNCTION_NAME 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					recog.base.set_state(100);
					recog.base.match_token(OutboundAPIParser_TIME_INTERVAL_EXPLICIT_FUNCTION_NAME,&mut recog.err_handler)?;

					recog.base.set_state(101);
					recog.base.match_token(OutboundAPIParser_LB,&mut recog.err_handler)?;

					/*InvokeRule timeInterval*/
					recog.base.set_state(102);
					recog.timeInterval()?;

					recog.base.set_state(103);
					recog.base.match_token(OutboundAPIParser_RB,&mut recog.err_handler)?;

					}
				}

			OutboundAPIParser_TIME_INTERVAL_FUNCTION_NAME 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					recog.base.set_state(105);
					recog.base.match_token(OutboundAPIParser_TIME_INTERVAL_FUNCTION_NAME,&mut recog.err_handler)?;

					recog.base.set_state(106);
					recog.base.match_token(OutboundAPIParser_LB,&mut recog.err_handler)?;

					/*InvokeRule pointInTimeArithmetic*/
					recog.base.set_state(107);
					recog.pointInTimeArithmetic()?;

					recog.base.set_state(110);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==OutboundAPIParser_COMMA {
						{
						recog.base.set_state(108);
						recog.base.match_token(OutboundAPIParser_COMMA,&mut recog.err_handler)?;

						recog.base.set_state(109);
						let tmp = recog.base.match_token(OutboundAPIParser_TIME_ZONE_IANA,&mut recog.err_handler)?;
						 cast_mut::<_,TimeIntervalOrFunctionContext >(&mut _localctx).expressionTimeZone = Some(tmp.clone());
						  

						}
					}

					recog.base.set_state(112);
					recog.base.match_token(OutboundAPIParser_RB,&mut recog.err_handler)?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- gasIntervalOrFunction ----------------
pub type GasIntervalOrFunctionContextAll<'input> = GasIntervalOrFunctionContext<'input>;


pub type GasIntervalOrFunctionContext<'input> = BaseParserRuleContext<'input,GasIntervalOrFunctionContextExt<'input>>;

#[derive(Clone)]
pub struct GasIntervalOrFunctionContextExt<'input>{
	pub expressionTimeZone: Option<TokenType<'input>>,
ph:PhantomData<&'input str>
}

impl<'input> OutboundAPIParserContext<'input> for GasIntervalOrFunctionContext<'input>{}

impl<'input,'a> Listenable<dyn OutboundAPIParserListener<'input> + 'a> for GasIntervalOrFunctionContext<'input>{
		fn enter(&self,listener: &mut (dyn OutboundAPIParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_gasIntervalOrFunction(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn OutboundAPIParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_gasIntervalOrFunction(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn OutboundAPIParserVisitor<'input> + 'a> for GasIntervalOrFunctionContext<'input>{
	fn accept(&self,visitor: &mut (dyn OutboundAPIParserVisitor<'input> + 'a)) {
		visitor.visit_gasIntervalOrFunction(self);
	}
}

impl<'input> CustomRuleContext<'input> for GasIntervalOrFunctionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = OutboundAPIParserContextType;
	fn get_rule_index(&self) -> usize { RULE_gasIntervalOrFunction }
	//fn type_rule_index() -> usize where Self: Sized { RULE_gasIntervalOrFunction }
}
antlr4rust::tid!{GasIntervalOrFunctionContextExt<'a>}

impl<'input> GasIntervalOrFunctionContextExt<'input>{
	fn new(parent: Option<Rc<dyn OutboundAPIParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<GasIntervalOrFunctionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,GasIntervalOrFunctionContextExt{
				expressionTimeZone: None, 

				ph:PhantomData
			}),
		)
	}
}

pub trait GasIntervalOrFunctionContextAttrs<'input>: OutboundAPIParserContext<'input> + BorrowMut<GasIntervalOrFunctionContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token TIME_INTERVAL_GAS_FUNCTION_NAME
/// Returns `None` if there is no child corresponding to token TIME_INTERVAL_GAS_FUNCTION_NAME
fn TIME_INTERVAL_GAS_FUNCTION_NAME(&self) -> Option<Rc<TerminalNode<'input,OutboundAPIParserContextType>>> where Self:Sized{
	self.get_token(OutboundAPIParser_TIME_INTERVAL_GAS_FUNCTION_NAME, 0)
}
/// Retrieves first TerminalNode corresponding to token LB
/// Returns `None` if there is no child corresponding to token LB
fn LB(&self) -> Option<Rc<TerminalNode<'input,OutboundAPIParserContextType>>> where Self:Sized{
	self.get_token(OutboundAPIParser_LB, 0)
}
fn pointInTimeArithmetic(&self) -> Option<Rc<PointInTimeArithmeticContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token RB
/// Returns `None` if there is no child corresponding to token RB
fn RB(&self) -> Option<Rc<TerminalNode<'input,OutboundAPIParserContextType>>> where Self:Sized{
	self.get_token(OutboundAPIParser_RB, 0)
}
/// Retrieves first TerminalNode corresponding to token COMMA
/// Returns `None` if there is no child corresponding to token COMMA
fn COMMA(&self) -> Option<Rc<TerminalNode<'input,OutboundAPIParserContextType>>> where Self:Sized{
	self.get_token(OutboundAPIParser_COMMA, 0)
}
/// Retrieves first TerminalNode corresponding to token TIME_ZONE_IANA
/// Returns `None` if there is no child corresponding to token TIME_ZONE_IANA
fn TIME_ZONE_IANA(&self) -> Option<Rc<TerminalNode<'input,OutboundAPIParserContextType>>> where Self:Sized{
	self.get_token(OutboundAPIParser_TIME_ZONE_IANA, 0)
}

}

impl<'input> GasIntervalOrFunctionContextAttrs<'input> for GasIntervalOrFunctionContext<'input>{}

impl<'input, I> OutboundAPIParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn gasIntervalOrFunction(&mut self,)
	-> Result<Rc<GasIntervalOrFunctionContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = GasIntervalOrFunctionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 16, RULE_gasIntervalOrFunction);
        let mut _localctx: Rc<GasIntervalOrFunctionContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(116);
			recog.base.match_token(OutboundAPIParser_TIME_INTERVAL_GAS_FUNCTION_NAME,&mut recog.err_handler)?;

			recog.base.set_state(117);
			recog.base.match_token(OutboundAPIParser_LB,&mut recog.err_handler)?;

			/*InvokeRule pointInTimeArithmetic*/
			recog.base.set_state(118);
			recog.pointInTimeArithmetic()?;

			recog.base.set_state(121);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==OutboundAPIParser_COMMA {
				{
				recog.base.set_state(119);
				recog.base.match_token(OutboundAPIParser_COMMA,&mut recog.err_handler)?;

				recog.base.set_state(120);
				let tmp = recog.base.match_token(OutboundAPIParser_TIME_ZONE_IANA,&mut recog.err_handler)?;
				 cast_mut::<_,GasIntervalOrFunctionContext >(&mut _localctx).expressionTimeZone = Some(tmp.clone());
				  

				}
			}

			recog.base.set_state(123);
			recog.base.match_token(OutboundAPIParser_RB,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- pointInTimeOrFunction ----------------
pub type PointInTimeOrFunctionContextAll<'input> = PointInTimeOrFunctionContext<'input>;


pub type PointInTimeOrFunctionContext<'input> = BaseParserRuleContext<'input,PointInTimeOrFunctionContextExt<'input>>;

#[derive(Clone)]
pub struct PointInTimeOrFunctionContextExt<'input>{
	pub expressionTimeZone: Option<TokenType<'input>>,
ph:PhantomData<&'input str>
}

impl<'input> OutboundAPIParserContext<'input> for PointInTimeOrFunctionContext<'input>{}

impl<'input,'a> Listenable<dyn OutboundAPIParserListener<'input> + 'a> for PointInTimeOrFunctionContext<'input>{
		fn enter(&self,listener: &mut (dyn OutboundAPIParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_pointInTimeOrFunction(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn OutboundAPIParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_pointInTimeOrFunction(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn OutboundAPIParserVisitor<'input> + 'a> for PointInTimeOrFunctionContext<'input>{
	fn accept(&self,visitor: &mut (dyn OutboundAPIParserVisitor<'input> + 'a)) {
		visitor.visit_pointInTimeOrFunction(self);
	}
}

impl<'input> CustomRuleContext<'input> for PointInTimeOrFunctionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = OutboundAPIParserContextType;
	fn get_rule_index(&self) -> usize { RULE_pointInTimeOrFunction }
	//fn type_rule_index() -> usize where Self: Sized { RULE_pointInTimeOrFunction }
}
antlr4rust::tid!{PointInTimeOrFunctionContextExt<'a>}

impl<'input> PointInTimeOrFunctionContextExt<'input>{
	fn new(parent: Option<Rc<dyn OutboundAPIParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<PointInTimeOrFunctionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,PointInTimeOrFunctionContextExt{
				expressionTimeZone: None, 

				ph:PhantomData
			}),
		)
	}
}

pub trait PointInTimeOrFunctionContextAttrs<'input>: OutboundAPIParserContext<'input> + BorrowMut<PointInTimeOrFunctionContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token POINT_IN_TIME
/// Returns `None` if there is no child corresponding to token POINT_IN_TIME
fn POINT_IN_TIME(&self) -> Option<Rc<TerminalNode<'input,OutboundAPIParserContextType>>> where Self:Sized{
	self.get_token(OutboundAPIParser_POINT_IN_TIME, 0)
}
/// Retrieves first TerminalNode corresponding to token POINT_IN_TIME_FUNCTION_NAME
/// Returns `None` if there is no child corresponding to token POINT_IN_TIME_FUNCTION_NAME
fn POINT_IN_TIME_FUNCTION_NAME(&self) -> Option<Rc<TerminalNode<'input,OutboundAPIParserContextType>>> where Self:Sized{
	self.get_token(OutboundAPIParser_POINT_IN_TIME_FUNCTION_NAME, 0)
}
/// Retrieves all `TerminalNode`s corresponding to token LB in current rule
fn LB_all(&self) -> Vec<Rc<TerminalNode<'input,OutboundAPIParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token LB, starting from 0.
/// Returns `None` if number of children corresponding to token LB is less or equal than `i`.
fn LB(&self, i: usize) -> Option<Rc<TerminalNode<'input,OutboundAPIParserContextType>>> where Self:Sized{
	self.get_token(OutboundAPIParser_LB, i)
}
/// Retrieves all `TerminalNode`s corresponding to token RB in current rule
fn RB_all(&self) -> Vec<Rc<TerminalNode<'input,OutboundAPIParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token RB, starting from 0.
/// Returns `None` if number of children corresponding to token RB is less or equal than `i`.
fn RB(&self, i: usize) -> Option<Rc<TerminalNode<'input,OutboundAPIParserContextType>>> where Self:Sized{
	self.get_token(OutboundAPIParser_RB, i)
}
/// Retrieves first TerminalNode corresponding to token POINT_IN_TIME_UTC_FUNCTION_NAME
/// Returns `None` if there is no child corresponding to token POINT_IN_TIME_UTC_FUNCTION_NAME
fn POINT_IN_TIME_UTC_FUNCTION_NAME(&self) -> Option<Rc<TerminalNode<'input,OutboundAPIParserContextType>>> where Self:Sized{
	self.get_token(OutboundAPIParser_POINT_IN_TIME_UTC_FUNCTION_NAME, 0)
}
/// Retrieves first TerminalNode corresponding to token COMMA
/// Returns `None` if there is no child corresponding to token COMMA
fn COMMA(&self) -> Option<Rc<TerminalNode<'input,OutboundAPIParserContextType>>> where Self:Sized{
	self.get_token(OutboundAPIParser_COMMA, 0)
}
/// Retrieves first TerminalNode corresponding to token TIME_ZONE_IANA
/// Returns `None` if there is no child corresponding to token TIME_ZONE_IANA
fn TIME_ZONE_IANA(&self) -> Option<Rc<TerminalNode<'input,OutboundAPIParserContextType>>> where Self:Sized{
	self.get_token(OutboundAPIParser_TIME_ZONE_IANA, 0)
}

}

impl<'input> PointInTimeOrFunctionContextAttrs<'input> for PointInTimeOrFunctionContext<'input>{}

impl<'input, I> OutboundAPIParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn pointInTimeOrFunction(&mut self,)
	-> Result<Rc<PointInTimeOrFunctionContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = PointInTimeOrFunctionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 18, RULE_pointInTimeOrFunction);
        let mut _localctx: Rc<PointInTimeOrFunctionContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(140);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			OutboundAPIParser_POINT_IN_TIME 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					recog.base.set_state(125);
					recog.base.match_token(OutboundAPIParser_POINT_IN_TIME,&mut recog.err_handler)?;

					}
				}

			OutboundAPIParser_POINT_IN_TIME_FUNCTION_NAME 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					recog.base.set_state(126);
					recog.base.match_token(OutboundAPIParser_POINT_IN_TIME_FUNCTION_NAME,&mut recog.err_handler)?;

					recog.base.set_state(127);
					recog.base.match_token(OutboundAPIParser_LB,&mut recog.err_handler)?;

					recog.base.set_state(128);
					recog.base.match_token(OutboundAPIParser_RB,&mut recog.err_handler)?;

					}
				}

			OutboundAPIParser_POINT_IN_TIME_UTC_FUNCTION_NAME 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 3)?;
					recog.base.enter_outer_alt(None, 3)?;
					{
					recog.base.set_state(129);
					recog.base.match_token(OutboundAPIParser_POINT_IN_TIME_UTC_FUNCTION_NAME,&mut recog.err_handler)?;

					recog.base.set_state(130);
					recog.base.match_token(OutboundAPIParser_LB,&mut recog.err_handler)?;

					recog.base.set_state(135);
					recog.err_handler.sync(&mut recog.base)?;
					match recog.base.input.la(1) {
					OutboundAPIParser_POINT_IN_TIME 
						=> {
							{
							recog.base.set_state(131);
							recog.base.match_token(OutboundAPIParser_POINT_IN_TIME,&mut recog.err_handler)?;

							}
						}

					OutboundAPIParser_POINT_IN_TIME_FUNCTION_NAME 
						=> {
							{
							recog.base.set_state(132);
							recog.base.match_token(OutboundAPIParser_POINT_IN_TIME_FUNCTION_NAME,&mut recog.err_handler)?;

							recog.base.set_state(133);
							recog.base.match_token(OutboundAPIParser_LB,&mut recog.err_handler)?;

							recog.base.set_state(134);
							recog.base.match_token(OutboundAPIParser_RB,&mut recog.err_handler)?;

							}
						}

						_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
					}
					recog.base.set_state(137);
					recog.base.match_token(OutboundAPIParser_COMMA,&mut recog.err_handler)?;

					recog.base.set_state(138);
					let tmp = recog.base.match_token(OutboundAPIParser_TIME_ZONE_IANA,&mut recog.err_handler)?;
					 cast_mut::<_,PointInTimeOrFunctionContext >(&mut _localctx).expressionTimeZone = Some(tmp.clone());
					  

					recog.base.set_state(139);
					recog.base.match_token(OutboundAPIParser_RB,&mut recog.err_handler)?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- pointInTimeArithmetic ----------------
pub type PointInTimeArithmeticContextAll<'input> = PointInTimeArithmeticContext<'input>;


pub type PointInTimeArithmeticContext<'input> = BaseParserRuleContext<'input,PointInTimeArithmeticContextExt<'input>>;

#[derive(Clone)]
pub struct PointInTimeArithmeticContextExt<'input>{
	pub ArithmeticOperator: Option<TokenType<'input>>,
	pub TimePeriod: Option<TokenType<'input>>,
ph:PhantomData<&'input str>
}

impl<'input> OutboundAPIParserContext<'input> for PointInTimeArithmeticContext<'input>{}

impl<'input,'a> Listenable<dyn OutboundAPIParserListener<'input> + 'a> for PointInTimeArithmeticContext<'input>{
		fn enter(&self,listener: &mut (dyn OutboundAPIParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_pointInTimeArithmetic(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn OutboundAPIParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_pointInTimeArithmetic(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn OutboundAPIParserVisitor<'input> + 'a> for PointInTimeArithmeticContext<'input>{
	fn accept(&self,visitor: &mut (dyn OutboundAPIParserVisitor<'input> + 'a)) {
		visitor.visit_pointInTimeArithmetic(self);
	}
}

impl<'input> CustomRuleContext<'input> for PointInTimeArithmeticContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = OutboundAPIParserContextType;
	fn get_rule_index(&self) -> usize { RULE_pointInTimeArithmetic }
	//fn type_rule_index() -> usize where Self: Sized { RULE_pointInTimeArithmetic }
}
antlr4rust::tid!{PointInTimeArithmeticContextExt<'a>}

impl<'input> PointInTimeArithmeticContextExt<'input>{
	fn new(parent: Option<Rc<dyn OutboundAPIParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<PointInTimeArithmeticContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,PointInTimeArithmeticContextExt{
				ArithmeticOperator: None, TimePeriod: None, 

				ph:PhantomData
			}),
		)
	}
}

pub trait PointInTimeArithmeticContextAttrs<'input>: OutboundAPIParserContext<'input> + BorrowMut<PointInTimeArithmeticContextExt<'input>>{

fn pointInTimeOrFunction(&self) -> Option<Rc<PointInTimeOrFunctionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token TIME_PERIOD
/// Returns `None` if there is no child corresponding to token TIME_PERIOD
fn TIME_PERIOD(&self) -> Option<Rc<TerminalNode<'input,OutboundAPIParserContextType>>> where Self:Sized{
	self.get_token(OutboundAPIParser_TIME_PERIOD, 0)
}
/// Retrieves first TerminalNode corresponding to token ADD
/// Returns `None` if there is no child corresponding to token ADD
fn ADD(&self) -> Option<Rc<TerminalNode<'input,OutboundAPIParserContextType>>> where Self:Sized{
	self.get_token(OutboundAPIParser_ADD, 0)
}
/// Retrieves first TerminalNode corresponding to token SUB
/// Returns `None` if there is no child corresponding to token SUB
fn SUB(&self) -> Option<Rc<TerminalNode<'input,OutboundAPIParserContextType>>> where Self:Sized{
	self.get_token(OutboundAPIParser_SUB, 0)
}

}

impl<'input> PointInTimeArithmeticContextAttrs<'input> for PointInTimeArithmeticContext<'input>{}

impl<'input, I> OutboundAPIParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn pointInTimeArithmetic(&mut self,)
	-> Result<Rc<PointInTimeArithmeticContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = PointInTimeArithmeticContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 20, RULE_pointInTimeArithmetic);
        let mut _localctx: Rc<PointInTimeArithmeticContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			{
			/*InvokeRule pointInTimeOrFunction*/
			recog.base.set_state(142);
			recog.pointInTimeOrFunction()?;

			}
			recog.base.set_state(145);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==OutboundAPIParser_ADD || _la==OutboundAPIParser_SUB {
				{
				recog.base.set_state(143);
				 cast_mut::<_,PointInTimeArithmeticContext >(&mut _localctx).ArithmeticOperator = recog.base.input.lt(1).cloned();
				 
				_la = recog.base.input.la(1);
				if { !(_la==OutboundAPIParser_ADD || _la==OutboundAPIParser_SUB) } {
					let tmp = recog.err_handler.recover_inline(&mut recog.base)?;
					 cast_mut::<_,PointInTimeArithmeticContext >(&mut _localctx).ArithmeticOperator = Some(tmp.clone());
					  

				}
				else {
					if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
					recog.err_handler.report_match(&mut recog.base);
					recog.base.consume(&mut recog.err_handler);
				}
				recog.base.set_state(144);
				let tmp = recog.base.match_token(OutboundAPIParser_TIME_PERIOD,&mut recog.err_handler)?;
				 cast_mut::<_,PointInTimeArithmeticContext >(&mut _localctx).TimePeriod = Some(tmp.clone());
				  

				}
			}

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- timeIntervalArithmetic ----------------
pub type TimeIntervalArithmeticContextAll<'input> = TimeIntervalArithmeticContext<'input>;


pub type TimeIntervalArithmeticContext<'input> = BaseParserRuleContext<'input,TimeIntervalArithmeticContextExt<'input>>;

#[derive(Clone)]
pub struct TimeIntervalArithmeticContextExt<'input>{
	pub ArithmeticOperator: Option<TokenType<'input>>,
	pub TimePeriod: Option<TokenType<'input>>,
ph:PhantomData<&'input str>
}

impl<'input> OutboundAPIParserContext<'input> for TimeIntervalArithmeticContext<'input>{}

impl<'input,'a> Listenable<dyn OutboundAPIParserListener<'input> + 'a> for TimeIntervalArithmeticContext<'input>{
		fn enter(&self,listener: &mut (dyn OutboundAPIParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_timeIntervalArithmetic(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn OutboundAPIParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_timeIntervalArithmetic(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn OutboundAPIParserVisitor<'input> + 'a> for TimeIntervalArithmeticContext<'input>{
	fn accept(&self,visitor: &mut (dyn OutboundAPIParserVisitor<'input> + 'a)) {
		visitor.visit_timeIntervalArithmetic(self);
	}
}

impl<'input> CustomRuleContext<'input> for TimeIntervalArithmeticContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = OutboundAPIParserContextType;
	fn get_rule_index(&self) -> usize { RULE_timeIntervalArithmetic }
	//fn type_rule_index() -> usize where Self: Sized { RULE_timeIntervalArithmetic }
}
antlr4rust::tid!{TimeIntervalArithmeticContextExt<'a>}

impl<'input> TimeIntervalArithmeticContextExt<'input>{
	fn new(parent: Option<Rc<dyn OutboundAPIParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<TimeIntervalArithmeticContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,TimeIntervalArithmeticContextExt{
				ArithmeticOperator: None, TimePeriod: None, 

				ph:PhantomData
			}),
		)
	}
}

pub trait TimeIntervalArithmeticContextAttrs<'input>: OutboundAPIParserContext<'input> + BorrowMut<TimeIntervalArithmeticContextExt<'input>>{

fn timeIntervalOrFunction(&self) -> Option<Rc<TimeIntervalOrFunctionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token TIME_PERIOD
/// Returns `None` if there is no child corresponding to token TIME_PERIOD
fn TIME_PERIOD(&self) -> Option<Rc<TerminalNode<'input,OutboundAPIParserContextType>>> where Self:Sized{
	self.get_token(OutboundAPIParser_TIME_PERIOD, 0)
}
/// Retrieves first TerminalNode corresponding to token ADD
/// Returns `None` if there is no child corresponding to token ADD
fn ADD(&self) -> Option<Rc<TerminalNode<'input,OutboundAPIParserContextType>>> where Self:Sized{
	self.get_token(OutboundAPIParser_ADD, 0)
}
/// Retrieves first TerminalNode corresponding to token SUB
/// Returns `None` if there is no child corresponding to token SUB
fn SUB(&self) -> Option<Rc<TerminalNode<'input,OutboundAPIParserContextType>>> where Self:Sized{
	self.get_token(OutboundAPIParser_SUB, 0)
}
fn gasIntervalOrFunction(&self) -> Option<Rc<GasIntervalOrFunctionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> TimeIntervalArithmeticContextAttrs<'input> for TimeIntervalArithmeticContext<'input>{}

impl<'input, I> OutboundAPIParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn timeIntervalArithmetic(&mut self,)
	-> Result<Rc<TimeIntervalArithmeticContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = TimeIntervalArithmeticContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 22, RULE_timeIntervalArithmetic);
        let mut _localctx: Rc<TimeIntervalArithmeticContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(153);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			OutboundAPIParser_TIME_INTERVAL_FUNCTION_NAME |OutboundAPIParser_TIME_INTERVAL_EXPLICIT_FUNCTION_NAME 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					{
					/*InvokeRule timeIntervalOrFunction*/
					recog.base.set_state(147);
					recog.timeIntervalOrFunction()?;

					}
					recog.base.set_state(150);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==OutboundAPIParser_ADD || _la==OutboundAPIParser_SUB {
						{
						recog.base.set_state(148);
						 cast_mut::<_,TimeIntervalArithmeticContext >(&mut _localctx).ArithmeticOperator = recog.base.input.lt(1).cloned();
						 
						_la = recog.base.input.la(1);
						if { !(_la==OutboundAPIParser_ADD || _la==OutboundAPIParser_SUB) } {
							let tmp = recog.err_handler.recover_inline(&mut recog.base)?;
							 cast_mut::<_,TimeIntervalArithmeticContext >(&mut _localctx).ArithmeticOperator = Some(tmp.clone());
							  

						}
						else {
							if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
							recog.err_handler.report_match(&mut recog.base);
							recog.base.consume(&mut recog.err_handler);
						}
						recog.base.set_state(149);
						let tmp = recog.base.match_token(OutboundAPIParser_TIME_PERIOD,&mut recog.err_handler)?;
						 cast_mut::<_,TimeIntervalArithmeticContext >(&mut _localctx).TimePeriod = Some(tmp.clone());
						  

						}
					}

					}
				}

			OutboundAPIParser_TIME_INTERVAL_GAS_FUNCTION_NAME 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					/*InvokeRule gasIntervalOrFunction*/
					recog.base.set_state(152);
					recog.gasIntervalOrFunction()?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- timeIntervalToPointInTime ----------------
pub type TimeIntervalToPointInTimeContextAll<'input> = TimeIntervalToPointInTimeContext<'input>;


pub type TimeIntervalToPointInTimeContext<'input> = BaseParserRuleContext<'input,TimeIntervalToPointInTimeContextExt<'input>>;

#[derive(Clone)]
pub struct TimeIntervalToPointInTimeContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> OutboundAPIParserContext<'input> for TimeIntervalToPointInTimeContext<'input>{}

impl<'input,'a> Listenable<dyn OutboundAPIParserListener<'input> + 'a> for TimeIntervalToPointInTimeContext<'input>{
		fn enter(&self,listener: &mut (dyn OutboundAPIParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_timeIntervalToPointInTime(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn OutboundAPIParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_timeIntervalToPointInTime(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn OutboundAPIParserVisitor<'input> + 'a> for TimeIntervalToPointInTimeContext<'input>{
	fn accept(&self,visitor: &mut (dyn OutboundAPIParserVisitor<'input> + 'a)) {
		visitor.visit_timeIntervalToPointInTime(self);
	}
}

impl<'input> CustomRuleContext<'input> for TimeIntervalToPointInTimeContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = OutboundAPIParserContextType;
	fn get_rule_index(&self) -> usize { RULE_timeIntervalToPointInTime }
	//fn type_rule_index() -> usize where Self: Sized { RULE_timeIntervalToPointInTime }
}
antlr4rust::tid!{TimeIntervalToPointInTimeContextExt<'a>}

impl<'input> TimeIntervalToPointInTimeContextExt<'input>{
	fn new(parent: Option<Rc<dyn OutboundAPIParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<TimeIntervalToPointInTimeContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,TimeIntervalToPointInTimeContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait TimeIntervalToPointInTimeContextAttrs<'input>: OutboundAPIParserContext<'input> + BorrowMut<TimeIntervalToPointInTimeContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token TIME_INTERVAL_TO_POINT_IN_TIME_FUNCTION
/// Returns `None` if there is no child corresponding to token TIME_INTERVAL_TO_POINT_IN_TIME_FUNCTION
fn TIME_INTERVAL_TO_POINT_IN_TIME_FUNCTION(&self) -> Option<Rc<TerminalNode<'input,OutboundAPIParserContextType>>> where Self:Sized{
	self.get_token(OutboundAPIParser_TIME_INTERVAL_TO_POINT_IN_TIME_FUNCTION, 0)
}
/// Retrieves first TerminalNode corresponding to token LB
/// Returns `None` if there is no child corresponding to token LB
fn LB(&self) -> Option<Rc<TerminalNode<'input,OutboundAPIParserContextType>>> where Self:Sized{
	self.get_token(OutboundAPIParser_LB, 0)
}
fn timeIntervalArithmetic(&self) -> Option<Rc<TimeIntervalArithmeticContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token RB
/// Returns `None` if there is no child corresponding to token RB
fn RB(&self) -> Option<Rc<TerminalNode<'input,OutboundAPIParserContextType>>> where Self:Sized{
	self.get_token(OutboundAPIParser_RB, 0)
}

}

impl<'input> TimeIntervalToPointInTimeContextAttrs<'input> for TimeIntervalToPointInTimeContext<'input>{}

impl<'input, I> OutboundAPIParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn timeIntervalToPointInTime(&mut self,)
	-> Result<Rc<TimeIntervalToPointInTimeContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = TimeIntervalToPointInTimeContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 24, RULE_timeIntervalToPointInTime);
        let mut _localctx: Rc<TimeIntervalToPointInTimeContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(155);
			recog.base.match_token(OutboundAPIParser_TIME_INTERVAL_TO_POINT_IN_TIME_FUNCTION,&mut recog.err_handler)?;

			recog.base.set_state(156);
			recog.base.match_token(OutboundAPIParser_LB,&mut recog.err_handler)?;

			/*InvokeRule timeIntervalArithmetic*/
			recog.base.set_state(157);
			recog.timeIntervalArithmetic()?;

			recog.base.set_state(158);
			recog.base.match_token(OutboundAPIParser_RB,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- rankOverFunction ----------------
pub type RankOverFunctionContextAll<'input> = RankOverFunctionContext<'input>;


pub type RankOverFunctionContext<'input> = BaseParserRuleContext<'input,RankOverFunctionContextExt<'input>>;

#[derive(Clone)]
pub struct RankOverFunctionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> OutboundAPIParserContext<'input> for RankOverFunctionContext<'input>{}

impl<'input,'a> Listenable<dyn OutboundAPIParserListener<'input> + 'a> for RankOverFunctionContext<'input>{
		fn enter(&self,listener: &mut (dyn OutboundAPIParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_rankOverFunction(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn OutboundAPIParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_rankOverFunction(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn OutboundAPIParserVisitor<'input> + 'a> for RankOverFunctionContext<'input>{
	fn accept(&self,visitor: &mut (dyn OutboundAPIParserVisitor<'input> + 'a)) {
		visitor.visit_rankOverFunction(self);
	}
}

impl<'input> CustomRuleContext<'input> for RankOverFunctionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = OutboundAPIParserContextType;
	fn get_rule_index(&self) -> usize { RULE_rankOverFunction }
	//fn type_rule_index() -> usize where Self: Sized { RULE_rankOverFunction }
}
antlr4rust::tid!{RankOverFunctionContextExt<'a>}

impl<'input> RankOverFunctionContextExt<'input>{
	fn new(parent: Option<Rc<dyn OutboundAPIParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<RankOverFunctionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,RankOverFunctionContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait RankOverFunctionContextAttrs<'input>: OutboundAPIParserContext<'input> + BorrowMut<RankOverFunctionContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token RANK_OVER_FUNCTION_NAME
/// Returns `None` if there is no child corresponding to token RANK_OVER_FUNCTION_NAME
fn RANK_OVER_FUNCTION_NAME(&self) -> Option<Rc<TerminalNode<'input,OutboundAPIParserContextType>>> where Self:Sized{
	self.get_token(OutboundAPIParser_RANK_OVER_FUNCTION_NAME, 0)
}
/// Retrieves first TerminalNode corresponding to token LB
/// Returns `None` if there is no child corresponding to token LB
fn LB(&self) -> Option<Rc<TerminalNode<'input,OutboundAPIParserContextType>>> where Self:Sized{
	self.get_token(OutboundAPIParser_LB, 0)
}
/// Retrieves all `TerminalNode`s corresponding to token LSB in current rule
fn LSB_all(&self) -> Vec<Rc<TerminalNode<'input,OutboundAPIParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token LSB, starting from 0.
/// Returns `None` if number of children corresponding to token LSB is less or equal than `i`.
fn LSB(&self, i: usize) -> Option<Rc<TerminalNode<'input,OutboundAPIParserContextType>>> where Self:Sized{
	self.get_token(OutboundAPIParser_LSB, i)
}
/// Retrieves all `TerminalNode`s corresponding to token ID in current rule
fn ID_all(&self) -> Vec<Rc<TerminalNode<'input,OutboundAPIParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token ID, starting from 0.
/// Returns `None` if number of children corresponding to token ID is less or equal than `i`.
fn ID(&self, i: usize) -> Option<Rc<TerminalNode<'input,OutboundAPIParserContextType>>> where Self:Sized{
	self.get_token(OutboundAPIParser_ID, i)
}
/// Retrieves all `TerminalNode`s corresponding to token RSB in current rule
fn RSB_all(&self) -> Vec<Rc<TerminalNode<'input,OutboundAPIParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token RSB, starting from 0.
/// Returns `None` if number of children corresponding to token RSB is less or equal than `i`.
fn RSB(&self, i: usize) -> Option<Rc<TerminalNode<'input,OutboundAPIParserContextType>>> where Self:Sized{
	self.get_token(OutboundAPIParser_RSB, i)
}
/// Retrieves all `TerminalNode`s corresponding to token COMMA in current rule
fn COMMA_all(&self) -> Vec<Rc<TerminalNode<'input,OutboundAPIParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token COMMA, starting from 0.
/// Returns `None` if number of children corresponding to token COMMA is less or equal than `i`.
fn COMMA(&self, i: usize) -> Option<Rc<TerminalNode<'input,OutboundAPIParserContextType>>> where Self:Sized{
	self.get_token(OutboundAPIParser_COMMA, i)
}
/// Retrieves all `TerminalNode`s corresponding to token SORT_ORDER in current rule
fn SORT_ORDER_all(&self) -> Vec<Rc<TerminalNode<'input,OutboundAPIParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token SORT_ORDER, starting from 0.
/// Returns `None` if number of children corresponding to token SORT_ORDER is less or equal than `i`.
fn SORT_ORDER(&self, i: usize) -> Option<Rc<TerminalNode<'input,OutboundAPIParserContextType>>> where Self:Sized{
	self.get_token(OutboundAPIParser_SORT_ORDER, i)
}
/// Retrieves first TerminalNode corresponding to token RB
/// Returns `None` if there is no child corresponding to token RB
fn RB(&self) -> Option<Rc<TerminalNode<'input,OutboundAPIParserContextType>>> where Self:Sized{
	self.get_token(OutboundAPIParser_RB, 0)
}
fn rankOverFilter_all(&self) ->  Vec<Rc<RankOverFilterContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn rankOverFilter(&self, i: usize) -> Option<Rc<RankOverFilterContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> RankOverFunctionContextAttrs<'input> for RankOverFunctionContext<'input>{}

impl<'input, I> OutboundAPIParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn rankOverFunction(&mut self,)
	-> Result<Rc<RankOverFunctionContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = RankOverFunctionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 26, RULE_rankOverFunction);
        let mut _localctx: Rc<RankOverFunctionContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(160);
			recog.base.match_token(OutboundAPIParser_RANK_OVER_FUNCTION_NAME,&mut recog.err_handler)?;

			recog.base.set_state(161);
			recog.base.match_token(OutboundAPIParser_LB,&mut recog.err_handler)?;

			recog.base.set_state(162);
			recog.base.match_token(OutboundAPIParser_LSB,&mut recog.err_handler)?;

			recog.base.set_state(163);
			recog.base.match_token(OutboundAPIParser_ID,&mut recog.err_handler)?;

			recog.base.set_state(168);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==OutboundAPIParser_COMMA {
				{
				{
				recog.base.set_state(164);
				recog.base.match_token(OutboundAPIParser_COMMA,&mut recog.err_handler)?;

				recog.base.set_state(165);
				recog.base.match_token(OutboundAPIParser_ID,&mut recog.err_handler)?;

				}
				}
				recog.base.set_state(170);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			recog.base.set_state(171);
			recog.base.match_token(OutboundAPIParser_RSB,&mut recog.err_handler)?;

			recog.base.set_state(172);
			recog.base.match_token(OutboundAPIParser_COMMA,&mut recog.err_handler)?;

			recog.base.set_state(173);
			recog.base.match_token(OutboundAPIParser_LSB,&mut recog.err_handler)?;

			recog.base.set_state(174);
			recog.base.match_token(OutboundAPIParser_ID,&mut recog.err_handler)?;

			recog.base.set_state(175);
			recog.base.match_token(OutboundAPIParser_SORT_ORDER,&mut recog.err_handler)?;

			recog.base.set_state(181);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==OutboundAPIParser_COMMA {
				{
				{
				recog.base.set_state(176);
				recog.base.match_token(OutboundAPIParser_COMMA,&mut recog.err_handler)?;

				recog.base.set_state(177);
				recog.base.match_token(OutboundAPIParser_ID,&mut recog.err_handler)?;

				recog.base.set_state(178);
				recog.base.match_token(OutboundAPIParser_SORT_ORDER,&mut recog.err_handler)?;

				}
				}
				recog.base.set_state(183);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			recog.base.set_state(184);
			recog.base.match_token(OutboundAPIParser_RSB,&mut recog.err_handler)?;

			recog.base.set_state(189);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==OutboundAPIParser_COMMA {
				{
				{
				recog.base.set_state(185);
				recog.base.match_token(OutboundAPIParser_COMMA,&mut recog.err_handler)?;

				/*InvokeRule rankOverFilter*/
				recog.base.set_state(186);
				recog.rankOverFilter()?;

				}
				}
				recog.base.set_state(191);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			recog.base.set_state(192);
			recog.base.match_token(OutboundAPIParser_RB,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- rankOverFilter ----------------
pub type RankOverFilterContextAll<'input> = RankOverFilterContext<'input>;


pub type RankOverFilterContext<'input> = BaseParserRuleContext<'input,RankOverFilterContextExt<'input>>;

#[derive(Clone)]
pub struct RankOverFilterContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> OutboundAPIParserContext<'input> for RankOverFilterContext<'input>{}

impl<'input,'a> Listenable<dyn OutboundAPIParserListener<'input> + 'a> for RankOverFilterContext<'input>{
		fn enter(&self,listener: &mut (dyn OutboundAPIParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_rankOverFilter(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn OutboundAPIParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_rankOverFilter(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn OutboundAPIParserVisitor<'input> + 'a> for RankOverFilterContext<'input>{
	fn accept(&self,visitor: &mut (dyn OutboundAPIParserVisitor<'input> + 'a)) {
		visitor.visit_rankOverFilter(self);
	}
}

impl<'input> CustomRuleContext<'input> for RankOverFilterContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = OutboundAPIParserContextType;
	fn get_rule_index(&self) -> usize { RULE_rankOverFilter }
	//fn type_rule_index() -> usize where Self: Sized { RULE_rankOverFilter }
}
antlr4rust::tid!{RankOverFilterContextExt<'a>}

impl<'input> RankOverFilterContextExt<'input>{
	fn new(parent: Option<Rc<dyn OutboundAPIParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<RankOverFilterContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,RankOverFilterContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait RankOverFilterContextAttrs<'input>: OutboundAPIParserContext<'input> + BorrowMut<RankOverFilterContextExt<'input>>{

/// Retrieves all `TerminalNode`s corresponding to token SIGNED_INTEGER in current rule
fn SIGNED_INTEGER_all(&self) -> Vec<Rc<TerminalNode<'input,OutboundAPIParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token SIGNED_INTEGER, starting from 0.
/// Returns `None` if number of children corresponding to token SIGNED_INTEGER is less or equal than `i`.
fn SIGNED_INTEGER(&self, i: usize) -> Option<Rc<TerminalNode<'input,OutboundAPIParserContextType>>> where Self:Sized{
	self.get_token(OutboundAPIParser_SIGNED_INTEGER, i)
}
/// Retrieves first TerminalNode corresponding to token LSB
/// Returns `None` if there is no child corresponding to token LSB
fn LSB(&self) -> Option<Rc<TerminalNode<'input,OutboundAPIParserContextType>>> where Self:Sized{
	self.get_token(OutboundAPIParser_LSB, 0)
}
/// Retrieves first TerminalNode corresponding to token COMMA
/// Returns `None` if there is no child corresponding to token COMMA
fn COMMA(&self) -> Option<Rc<TerminalNode<'input,OutboundAPIParserContextType>>> where Self:Sized{
	self.get_token(OutboundAPIParser_COMMA, 0)
}
/// Retrieves first TerminalNode corresponding to token RSB
/// Returns `None` if there is no child corresponding to token RSB
fn RSB(&self) -> Option<Rc<TerminalNode<'input,OutboundAPIParserContextType>>> where Self:Sized{
	self.get_token(OutboundAPIParser_RSB, 0)
}
/// Retrieves first TerminalNode corresponding to token OPEN_FILTER_INTERVAL_MARKER
/// Returns `None` if there is no child corresponding to token OPEN_FILTER_INTERVAL_MARKER
fn OPEN_FILTER_INTERVAL_MARKER(&self) -> Option<Rc<TerminalNode<'input,OutboundAPIParserContextType>>> where Self:Sized{
	self.get_token(OutboundAPIParser_OPEN_FILTER_INTERVAL_MARKER, 0)
}

}

impl<'input> RankOverFilterContextAttrs<'input> for RankOverFilterContext<'input>{}

impl<'input, I> OutboundAPIParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn rankOverFilter(&mut self,)
	-> Result<Rc<RankOverFilterContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = RankOverFilterContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 28, RULE_rankOverFilter);
        let mut _localctx: Rc<RankOverFilterContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(205);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(18,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					recog.base.set_state(194);
					recog.base.match_token(OutboundAPIParser_SIGNED_INTEGER,&mut recog.err_handler)?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					recog.base.set_state(195);
					recog.base.match_token(OutboundAPIParser_LSB,&mut recog.err_handler)?;

					recog.base.set_state(196);
					recog.base.match_token(OutboundAPIParser_SIGNED_INTEGER,&mut recog.err_handler)?;

					recog.base.set_state(197);
					recog.base.match_token(OutboundAPIParser_COMMA,&mut recog.err_handler)?;

					recog.base.set_state(198);
					recog.base.match_token(OutboundAPIParser_SIGNED_INTEGER,&mut recog.err_handler)?;

					recog.base.set_state(199);
					recog.base.match_token(OutboundAPIParser_RSB,&mut recog.err_handler)?;

					}
				}
			,
				3 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 3)?;
					recog.base.enter_outer_alt(None, 3)?;
					{
					recog.base.set_state(200);
					recog.base.match_token(OutboundAPIParser_LSB,&mut recog.err_handler)?;

					recog.base.set_state(201);
					recog.base.match_token(OutboundAPIParser_SIGNED_INTEGER,&mut recog.err_handler)?;

					recog.base.set_state(202);
					recog.base.match_token(OutboundAPIParser_COMMA,&mut recog.err_handler)?;

					recog.base.set_state(203);
					recog.base.match_token(OutboundAPIParser_OPEN_FILTER_INTERVAL_MARKER,&mut recog.err_handler)?;

					recog.base.set_state(204);
					recog.base.match_token(OutboundAPIParser_RSB,&mut recog.err_handler)?;

					}
				}

				_ => {}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- latestFunction ----------------
pub type LatestFunctionContextAll<'input> = LatestFunctionContext<'input>;


pub type LatestFunctionContext<'input> = BaseParserRuleContext<'input,LatestFunctionContextExt<'input>>;

#[derive(Clone)]
pub struct LatestFunctionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> OutboundAPIParserContext<'input> for LatestFunctionContext<'input>{}

impl<'input,'a> Listenable<dyn OutboundAPIParserListener<'input> + 'a> for LatestFunctionContext<'input>{
		fn enter(&self,listener: &mut (dyn OutboundAPIParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_latestFunction(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn OutboundAPIParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_latestFunction(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn OutboundAPIParserVisitor<'input> + 'a> for LatestFunctionContext<'input>{
	fn accept(&self,visitor: &mut (dyn OutboundAPIParserVisitor<'input> + 'a)) {
		visitor.visit_latestFunction(self);
	}
}

impl<'input> CustomRuleContext<'input> for LatestFunctionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = OutboundAPIParserContextType;
	fn get_rule_index(&self) -> usize { RULE_latestFunction }
	//fn type_rule_index() -> usize where Self: Sized { RULE_latestFunction }
}
antlr4rust::tid!{LatestFunctionContextExt<'a>}

impl<'input> LatestFunctionContextExt<'input>{
	fn new(parent: Option<Rc<dyn OutboundAPIParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<LatestFunctionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,LatestFunctionContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait LatestFunctionContextAttrs<'input>: OutboundAPIParserContext<'input> + BorrowMut<LatestFunctionContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token LATEST_FUNCTION_NAME
/// Returns `None` if there is no child corresponding to token LATEST_FUNCTION_NAME
fn LATEST_FUNCTION_NAME(&self) -> Option<Rc<TerminalNode<'input,OutboundAPIParserContextType>>> where Self:Sized{
	self.get_token(OutboundAPIParser_LATEST_FUNCTION_NAME, 0)
}
/// Retrieves first TerminalNode corresponding to token LB
/// Returns `None` if there is no child corresponding to token LB
fn LB(&self) -> Option<Rc<TerminalNode<'input,OutboundAPIParserContextType>>> where Self:Sized{
	self.get_token(OutboundAPIParser_LB, 0)
}
fn latestExpression_all(&self) ->  Vec<Rc<LatestExpressionContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn latestExpression(&self, i: usize) -> Option<Rc<LatestExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves first TerminalNode corresponding to token RB
/// Returns `None` if there is no child corresponding to token RB
fn RB(&self) -> Option<Rc<TerminalNode<'input,OutboundAPIParserContextType>>> where Self:Sized{
	self.get_token(OutboundAPIParser_RB, 0)
}
/// Retrieves all `TerminalNode`s corresponding to token COMMA in current rule
fn COMMA_all(&self) -> Vec<Rc<TerminalNode<'input,OutboundAPIParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token COMMA, starting from 0.
/// Returns `None` if number of children corresponding to token COMMA is less or equal than `i`.
fn COMMA(&self, i: usize) -> Option<Rc<TerminalNode<'input,OutboundAPIParserContextType>>> where Self:Sized{
	self.get_token(OutboundAPIParser_COMMA, i)
}

}

impl<'input> LatestFunctionContextAttrs<'input> for LatestFunctionContext<'input>{}

impl<'input, I> OutboundAPIParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn latestFunction(&mut self,)
	-> Result<Rc<LatestFunctionContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = LatestFunctionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 30, RULE_latestFunction);
        let mut _localctx: Rc<LatestFunctionContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(207);
			recog.base.match_token(OutboundAPIParser_LATEST_FUNCTION_NAME,&mut recog.err_handler)?;

			recog.base.set_state(208);
			recog.base.match_token(OutboundAPIParser_LB,&mut recog.err_handler)?;

			/*InvokeRule latestExpression*/
			recog.base.set_state(209);
			recog.latestExpression()?;

			recog.base.set_state(214);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==OutboundAPIParser_COMMA {
				{
				{
				recog.base.set_state(210);
				recog.base.match_token(OutboundAPIParser_COMMA,&mut recog.err_handler)?;

				/*InvokeRule latestExpression*/
				recog.base.set_state(211);
				recog.latestExpression()?;

				}
				}
				recog.base.set_state(216);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			recog.base.set_state(217);
			recog.base.match_token(OutboundAPIParser_RB,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- latestExpression ----------------
pub type LatestExpressionContextAll<'input> = LatestExpressionContext<'input>;


pub type LatestExpressionContext<'input> = BaseParserRuleContext<'input,LatestExpressionContextExt<'input>>;

#[derive(Clone)]
pub struct LatestExpressionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> OutboundAPIParserContext<'input> for LatestExpressionContext<'input>{}

impl<'input,'a> Listenable<dyn OutboundAPIParserListener<'input> + 'a> for LatestExpressionContext<'input>{
		fn enter(&self,listener: &mut (dyn OutboundAPIParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_latestExpression(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn OutboundAPIParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_latestExpression(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn OutboundAPIParserVisitor<'input> + 'a> for LatestExpressionContext<'input>{
	fn accept(&self,visitor: &mut (dyn OutboundAPIParserVisitor<'input> + 'a)) {
		visitor.visit_latestExpression(self);
	}
}

impl<'input> CustomRuleContext<'input> for LatestExpressionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = OutboundAPIParserContextType;
	fn get_rule_index(&self) -> usize { RULE_latestExpression }
	//fn type_rule_index() -> usize where Self: Sized { RULE_latestExpression }
}
antlr4rust::tid!{LatestExpressionContextExt<'a>}

impl<'input> LatestExpressionContextExt<'input>{
	fn new(parent: Option<Rc<dyn OutboundAPIParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<LatestExpressionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,LatestExpressionContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait LatestExpressionContextAttrs<'input>: OutboundAPIParserContext<'input> + BorrowMut<LatestExpressionContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token ID
/// Returns `None` if there is no child corresponding to token ID
fn ID(&self) -> Option<Rc<TerminalNode<'input,OutboundAPIParserContextType>>> where Self:Sized{
	self.get_token(OutboundAPIParser_ID, 0)
}
/// Retrieves first TerminalNode corresponding to token COMPARISON_OPERATOR
/// Returns `None` if there is no child corresponding to token COMPARISON_OPERATOR
fn COMPARISON_OPERATOR(&self) -> Option<Rc<TerminalNode<'input,OutboundAPIParserContextType>>> where Self:Sized{
	self.get_token(OutboundAPIParser_COMPARISON_OPERATOR, 0)
}
fn pointInTimeArithmetic(&self) -> Option<Rc<PointInTimeArithmeticContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token IN
/// Returns `None` if there is no child corresponding to token IN
fn IN(&self) -> Option<Rc<TerminalNode<'input,OutboundAPIParserContextType>>> where Self:Sized{
	self.get_token(OutboundAPIParser_IN, 0)
}
fn timeIntervalArithmetic(&self) -> Option<Rc<TimeIntervalArithmeticContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn timeIntervalToPointInTime(&self) -> Option<Rc<TimeIntervalToPointInTimeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token SIGNED_INTEGER
/// Returns `None` if there is no child corresponding to token SIGNED_INTEGER
fn SIGNED_INTEGER(&self) -> Option<Rc<TerminalNode<'input,OutboundAPIParserContextType>>> where Self:Sized{
	self.get_token(OutboundAPIParser_SIGNED_INTEGER, 0)
}
/// Retrieves first TerminalNode corresponding to token FLOAT
/// Returns `None` if there is no child corresponding to token FLOAT
fn FLOAT(&self) -> Option<Rc<TerminalNode<'input,OutboundAPIParserContextType>>> where Self:Sized{
	self.get_token(OutboundAPIParser_FLOAT, 0)
}

}

impl<'input> LatestExpressionContextAttrs<'input> for LatestExpressionContext<'input>{}

impl<'input, I> OutboundAPIParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn latestExpression(&mut self,)
	-> Result<Rc<LatestExpressionContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = LatestExpressionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 32, RULE_latestExpression);
        let mut _localctx: Rc<LatestExpressionContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(231);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(20,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					recog.base.set_state(219);
					recog.base.match_token(OutboundAPIParser_ID,&mut recog.err_handler)?;

					recog.base.set_state(220);
					recog.base.match_token(OutboundAPIParser_COMPARISON_OPERATOR,&mut recog.err_handler)?;

					/*InvokeRule pointInTimeArithmetic*/
					recog.base.set_state(221);
					recog.pointInTimeArithmetic()?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					recog.base.set_state(222);
					recog.base.match_token(OutboundAPIParser_ID,&mut recog.err_handler)?;

					recog.base.set_state(223);
					recog.base.match_token(OutboundAPIParser_IN,&mut recog.err_handler)?;

					/*InvokeRule timeIntervalArithmetic*/
					recog.base.set_state(224);
					recog.timeIntervalArithmetic()?;

					}
				}
			,
				3 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 3)?;
					recog.base.enter_outer_alt(None, 3)?;
					{
					recog.base.set_state(225);
					recog.base.match_token(OutboundAPIParser_ID,&mut recog.err_handler)?;

					recog.base.set_state(226);
					recog.base.match_token(OutboundAPIParser_COMPARISON_OPERATOR,&mut recog.err_handler)?;

					/*InvokeRule timeIntervalToPointInTime*/
					recog.base.set_state(227);
					recog.timeIntervalToPointInTime()?;

					}
				}
			,
				4 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 4)?;
					recog.base.enter_outer_alt(None, 4)?;
					{
					recog.base.set_state(228);
					recog.base.match_token(OutboundAPIParser_ID,&mut recog.err_handler)?;

					recog.base.set_state(229);
					recog.base.match_token(OutboundAPIParser_COMPARISON_OPERATOR,&mut recog.err_handler)?;

					recog.base.set_state(230);
					_la = recog.base.input.la(1);
					if { !(_la==OutboundAPIParser_SIGNED_INTEGER || _la==OutboundAPIParser_FLOAT) } {
						recog.err_handler.recover_inline(&mut recog.base)?;

					}
					else {
						if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
						recog.err_handler.report_match(&mut recog.base);
						recog.base.consume(&mut recog.err_handler);
					}
					}
				}

				_ => {}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- genericValue ----------------
pub type GenericValueContextAll<'input> = GenericValueContext<'input>;


pub type GenericValueContext<'input> = BaseParserRuleContext<'input,GenericValueContextExt<'input>>;

#[derive(Clone)]
pub struct GenericValueContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> OutboundAPIParserContext<'input> for GenericValueContext<'input>{}

impl<'input,'a> Listenable<dyn OutboundAPIParserListener<'input> + 'a> for GenericValueContext<'input>{
		fn enter(&self,listener: &mut (dyn OutboundAPIParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_genericValue(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn OutboundAPIParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_genericValue(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn OutboundAPIParserVisitor<'input> + 'a> for GenericValueContext<'input>{
	fn accept(&self,visitor: &mut (dyn OutboundAPIParserVisitor<'input> + 'a)) {
		visitor.visit_genericValue(self);
	}
}

impl<'input> CustomRuleContext<'input> for GenericValueContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = OutboundAPIParserContextType;
	fn get_rule_index(&self) -> usize { RULE_genericValue }
	//fn type_rule_index() -> usize where Self: Sized { RULE_genericValue }
}
antlr4rust::tid!{GenericValueContextExt<'a>}

impl<'input> GenericValueContextExt<'input>{
	fn new(parent: Option<Rc<dyn OutboundAPIParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<GenericValueContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,GenericValueContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait GenericValueContextAttrs<'input>: OutboundAPIParserContext<'input> + BorrowMut<GenericValueContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token IN
/// Returns `None` if there is no child corresponding to token IN
fn IN(&self) -> Option<Rc<TerminalNode<'input,OutboundAPIParserContextType>>> where Self:Sized{
	self.get_token(OutboundAPIParser_IN, 0)
}
/// Retrieves first TerminalNode corresponding to token SORT_ORDER
/// Returns `None` if there is no child corresponding to token SORT_ORDER
fn SORT_ORDER(&self) -> Option<Rc<TerminalNode<'input,OutboundAPIParserContextType>>> where Self:Sized{
	self.get_token(OutboundAPIParser_SORT_ORDER, 0)
}
/// Retrieves first TerminalNode corresponding to token OPEN_FILTER_INTERVAL_MARKER
/// Returns `None` if there is no child corresponding to token OPEN_FILTER_INTERVAL_MARKER
fn OPEN_FILTER_INTERVAL_MARKER(&self) -> Option<Rc<TerminalNode<'input,OutboundAPIParserContextType>>> where Self:Sized{
	self.get_token(OutboundAPIParser_OPEN_FILTER_INTERVAL_MARKER, 0)
}
/// Retrieves first TerminalNode corresponding to token DATE
/// Returns `None` if there is no child corresponding to token DATE
fn DATE(&self) -> Option<Rc<TerminalNode<'input,OutboundAPIParserContextType>>> where Self:Sized{
	self.get_token(OutboundAPIParser_DATE, 0)
}
/// Retrieves first TerminalNode corresponding to token TIME
/// Returns `None` if there is no child corresponding to token TIME
fn TIME(&self) -> Option<Rc<TerminalNode<'input,OutboundAPIParserContextType>>> where Self:Sized{
	self.get_token(OutboundAPIParser_TIME, 0)
}
/// Retrieves first TerminalNode corresponding to token POINT_IN_TIME
/// Returns `None` if there is no child corresponding to token POINT_IN_TIME
fn POINT_IN_TIME(&self) -> Option<Rc<TerminalNode<'input,OutboundAPIParserContextType>>> where Self:Sized{
	self.get_token(OutboundAPIParser_POINT_IN_TIME, 0)
}
/// Retrieves first TerminalNode corresponding to token TIME_PERIOD
/// Returns `None` if there is no child corresponding to token TIME_PERIOD
fn TIME_PERIOD(&self) -> Option<Rc<TerminalNode<'input,OutboundAPIParserContextType>>> where Self:Sized{
	self.get_token(OutboundAPIParser_TIME_PERIOD, 0)
}
/// Retrieves first TerminalNode corresponding to token TIME_ZONE_IANA
/// Returns `None` if there is no child corresponding to token TIME_ZONE_IANA
fn TIME_ZONE_IANA(&self) -> Option<Rc<TerminalNode<'input,OutboundAPIParserContextType>>> where Self:Sized{
	self.get_token(OutboundAPIParser_TIME_ZONE_IANA, 0)
}
/// Retrieves first TerminalNode corresponding to token SIGNED_INTEGER
/// Returns `None` if there is no child corresponding to token SIGNED_INTEGER
fn SIGNED_INTEGER(&self) -> Option<Rc<TerminalNode<'input,OutboundAPIParserContextType>>> where Self:Sized{
	self.get_token(OutboundAPIParser_SIGNED_INTEGER, 0)
}
/// Retrieves first TerminalNode corresponding to token FLOAT
/// Returns `None` if there is no child corresponding to token FLOAT
fn FLOAT(&self) -> Option<Rc<TerminalNode<'input,OutboundAPIParserContextType>>> where Self:Sized{
	self.get_token(OutboundAPIParser_FLOAT, 0)
}
/// Retrieves first TerminalNode corresponding to token ID
/// Returns `None` if there is no child corresponding to token ID
fn ID(&self) -> Option<Rc<TerminalNode<'input,OutboundAPIParserContextType>>> where Self:Sized{
	self.get_token(OutboundAPIParser_ID, 0)
}
/// Retrieves first TerminalNode corresponding to token WORD
/// Returns `None` if there is no child corresponding to token WORD
fn WORD(&self) -> Option<Rc<TerminalNode<'input,OutboundAPIParserContextType>>> where Self:Sized{
	self.get_token(OutboundAPIParser_WORD, 0)
}

}

impl<'input> GenericValueContextAttrs<'input> for GenericValueContext<'input>{}

impl<'input, I> OutboundAPIParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn genericValue(&mut self,)
	-> Result<Rc<GenericValueContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = GenericValueContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 34, RULE_genericValue);
        let mut _localctx: Rc<GenericValueContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(233);
			_la = recog.base.input.la(1);
			if { !(((((_la - 18)) & !0x3f) == 0 && ((1usize << (_la - 18)) & 31584269) != 0)) } {
				recog.err_handler.recover_inline(&mut recog.base)?;

			}
			else {
				if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
				recog.err_handler.report_match(&mut recog.base);
				recog.base.consume(&mut recog.err_handler);
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
	lazy_static!{
    static ref _ATN: Arc<ATN> =
        Arc::new(ATNDeserializer::new(None).deserialize(&mut _serializedATN.iter()));
    static ref _decision_to_DFA: Arc<Vec<antlr4rust::RwLock<DFA>>> = {
        let mut dfa = Vec::new();
        let size = _ATN.decision_to_state.len() as i32;
        for i in 0..size {
            dfa.push(DFA::new(
                _ATN.clone(),
                _ATN.get_decision_state(i),
                i,
            ).into())
        }
        Arc::new(dfa)
    };
	static ref _serializedATN: Vec<i32> = vec![
		4, 1, 49, 236, 2, 0, 7, 0, 2, 1, 7, 1, 2, 2, 7, 2, 2, 3, 7, 3, 2, 4, 7, 
		4, 2, 5, 7, 5, 2, 6, 7, 6, 2, 7, 7, 7, 2, 8, 7, 8, 2, 9, 7, 9, 2, 10, 
		7, 10, 2, 11, 7, 11, 2, 12, 7, 12, 2, 13, 7, 13, 2, 14, 7, 14, 2, 15, 
		7, 15, 2, 16, 7, 16, 2, 17, 7, 17, 1, 0, 4, 0, 38, 8, 0, 11, 0, 12, 0, 
		39, 1, 1, 1, 1, 1, 1, 5, 1, 45, 8, 1, 10, 1, 12, 1, 48, 9, 1, 1, 1, 3, 
		1, 51, 8, 1, 1, 1, 1, 1, 1, 2, 1, 2, 3, 2, 57, 8, 2, 1, 2, 1, 2, 1, 2, 
		1, 2, 3, 2, 63, 8, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 
		1, 2, 1, 2, 3, 2, 75, 8, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 
		1, 2, 1, 2, 1, 2, 3, 2, 87, 8, 2, 1, 3, 1, 3, 1, 4, 1, 4, 1, 5, 1, 5, 
		1, 5, 1, 5, 1, 6, 1, 6, 1, 6, 1, 6, 1, 7, 1, 7, 1, 7, 1, 7, 1, 7, 1, 7, 
		1, 7, 1, 7, 1, 7, 1, 7, 3, 7, 111, 8, 7, 1, 7, 1, 7, 3, 7, 115, 8, 7, 
		1, 8, 1, 8, 1, 8, 1, 8, 1, 8, 3, 8, 122, 8, 8, 1, 8, 1, 8, 1, 9, 1, 9, 
		1, 9, 1, 9, 1, 9, 1, 9, 1, 9, 1, 9, 1, 9, 1, 9, 3, 9, 136, 8, 9, 1, 9, 
		1, 9, 1, 9, 3, 9, 141, 8, 9, 1, 10, 1, 10, 1, 10, 3, 10, 146, 8, 10, 1, 
		11, 1, 11, 1, 11, 3, 11, 151, 8, 11, 1, 11, 3, 11, 154, 8, 11, 1, 12, 
		1, 12, 1, 12, 1, 12, 1, 12, 1, 13, 1, 13, 1, 13, 1, 13, 1, 13, 1, 13, 
		5, 13, 167, 8, 13, 10, 13, 12, 13, 170, 9, 13, 1, 13, 1, 13, 1, 13, 1, 
		13, 1, 13, 1, 13, 1, 13, 1, 13, 5, 13, 180, 8, 13, 10, 13, 12, 13, 183, 
		9, 13, 1, 13, 1, 13, 1, 13, 5, 13, 188, 8, 13, 10, 13, 12, 13, 191, 9, 
		13, 1, 13, 1, 13, 1, 14, 1, 14, 1, 14, 1, 14, 1, 14, 1, 14, 1, 14, 1, 
		14, 1, 14, 1, 14, 1, 14, 3, 14, 206, 8, 14, 1, 15, 1, 15, 1, 15, 1, 15, 
		1, 15, 5, 15, 213, 8, 15, 10, 15, 12, 15, 216, 9, 15, 1, 15, 1, 15, 1, 
		16, 1, 16, 1, 16, 1, 16, 1, 16, 1, 16, 1, 16, 1, 16, 1, 16, 1, 16, 1, 
		16, 1, 16, 3, 16, 232, 8, 16, 1, 17, 1, 17, 1, 17, 0, 0, 18, 0, 2, 4, 
		6, 8, 10, 12, 14, 16, 18, 20, 22, 24, 26, 28, 30, 32, 34, 0, 5, 1, 0, 
		40, 41, 2, 0, 1, 3, 5, 5, 2, 0, 4, 4, 6, 8, 1, 0, 27, 28, 4, 0, 18, 18, 
		20, 21, 30, 34, 39, 42, 248, 0, 37, 1, 0, 0, 0, 2, 41, 1, 0, 0, 0, 4, 
		86, 1, 0, 0, 0, 6, 88, 1, 0, 0, 0, 8, 90, 1, 0, 0, 0, 10, 92, 1, 0, 0, 
		0, 12, 96, 1, 0, 0, 0, 14, 114, 1, 0, 0, 0, 16, 116, 1, 0, 0, 0, 18, 140, 
		1, 0, 0, 0, 20, 142, 1, 0, 0, 0, 22, 153, 1, 0, 0, 0, 24, 155, 1, 0, 0, 
		0, 26, 160, 1, 0, 0, 0, 28, 205, 1, 0, 0, 0, 30, 207, 1, 0, 0, 0, 32, 
		231, 1, 0, 0, 0, 34, 233, 1, 0, 0, 0, 36, 38, 3, 2, 1, 0, 37, 36, 1, 0, 
		0, 0, 38, 39, 1, 0, 0, 0, 39, 37, 1, 0, 0, 0, 39, 40, 1, 0, 0, 0, 40, 
		1, 1, 0, 0, 0, 41, 46, 3, 4, 2, 0, 42, 43, 5, 46, 0, 0, 43, 45, 3, 4, 
		2, 0, 44, 42, 1, 0, 0, 0, 45, 48, 1, 0, 0, 0, 46, 44, 1, 0, 0, 0, 46, 
		47, 1, 0, 0, 0, 47, 50, 1, 0, 0, 0, 48, 46, 1, 0, 0, 0, 49, 51, 5, 46, 
		0, 0, 50, 49, 1, 0, 0, 0, 50, 51, 1, 0, 0, 0, 51, 52, 1, 0, 0, 0, 52, 
		53, 5, 0, 0, 1, 53, 3, 1, 0, 0, 0, 54, 57, 5, 39, 0, 0, 55, 57, 3, 6, 
		3, 0, 56, 54, 1, 0, 0, 0, 56, 55, 1, 0, 0, 0, 57, 58, 1, 0, 0, 0, 58, 
		59, 5, 19, 0, 0, 59, 87, 3, 20, 10, 0, 60, 63, 5, 39, 0, 0, 61, 63, 3, 
		6, 3, 0, 62, 60, 1, 0, 0, 0, 62, 61, 1, 0, 0, 0, 63, 64, 1, 0, 0, 0, 64, 
		65, 5, 18, 0, 0, 65, 87, 3, 22, 11, 0, 66, 67, 5, 39, 0, 0, 67, 68, 5, 
		19, 0, 0, 68, 87, 7, 0, 0, 0, 69, 70, 5, 39, 0, 0, 70, 71, 5, 19, 0, 0, 
		71, 87, 3, 10, 5, 0, 72, 75, 5, 39, 0, 0, 73, 75, 3, 6, 3, 0, 74, 72, 
		1, 0, 0, 0, 74, 73, 1, 0, 0, 0, 75, 76, 1, 0, 0, 0, 76, 77, 5, 19, 0, 
		0, 77, 87, 3, 24, 12, 0, 78, 79, 5, 39, 0, 0, 79, 80, 5, 19, 0, 0, 80, 
		87, 3, 30, 15, 0, 81, 82, 3, 8, 4, 0, 82, 83, 5, 19, 0, 0, 83, 84, 3, 
		34, 17, 0, 84, 87, 1, 0, 0, 0, 85, 87, 3, 26, 13, 0, 86, 56, 1, 0, 0, 
		0, 86, 62, 1, 0, 0, 0, 86, 66, 1, 0, 0, 0, 86, 69, 1, 0, 0, 0, 86, 74, 
		1, 0, 0, 0, 86, 78, 1, 0, 0, 0, 86, 81, 1, 0, 0, 0, 86, 85, 1, 0, 0, 0, 
		87, 5, 1, 0, 0, 0, 88, 89, 7, 1, 0, 0, 89, 7, 1, 0, 0, 0, 90, 91, 7, 2, 
		0, 0, 91, 9, 1, 0, 0, 0, 92, 93, 5, 12, 0, 0, 93, 94, 5, 35, 0, 0, 94, 
		95, 5, 36, 0, 0, 95, 11, 1, 0, 0, 0, 96, 97, 5, 33, 0, 0, 97, 98, 5, 45, 
		0, 0, 98, 99, 5, 33, 0, 0, 99, 13, 1, 0, 0, 0, 100, 101, 5, 11, 0, 0, 
		101, 102, 5, 35, 0, 0, 102, 103, 3, 12, 6, 0, 103, 104, 5, 36, 0, 0, 104, 
		115, 1, 0, 0, 0, 105, 106, 5, 9, 0, 0, 106, 107, 5, 35, 0, 0, 107, 110, 
		3, 20, 10, 0, 108, 109, 5, 45, 0, 0, 109, 111, 5, 30, 0, 0, 110, 108, 
		1, 0, 0, 0, 110, 111, 1, 0, 0, 0, 111, 112, 1, 0, 0, 0, 112, 113, 5, 36, 
		0, 0, 113, 115, 1, 0, 0, 0, 114, 100, 1, 0, 0, 0, 114, 105, 1, 0, 0, 0, 
		115, 15, 1, 0, 0, 0, 116, 117, 5, 10, 0, 0, 117, 118, 5, 35, 0, 0, 118, 
		121, 3, 20, 10, 0, 119, 120, 5, 45, 0, 0, 120, 122, 5, 30, 0, 0, 121, 
		119, 1, 0, 0, 0, 121, 122, 1, 0, 0, 0, 122, 123, 1, 0, 0, 0, 123, 124, 
		5, 36, 0, 0, 124, 17, 1, 0, 0, 0, 125, 141, 5, 33, 0, 0, 126, 127, 5, 
		13, 0, 0, 127, 128, 5, 35, 0, 0, 128, 141, 5, 36, 0, 0, 129, 130, 5, 14, 
		0, 0, 130, 135, 5, 35, 0, 0, 131, 136, 5, 33, 0, 0, 132, 133, 5, 13, 0, 
		0, 133, 134, 5, 35, 0, 0, 134, 136, 5, 36, 0, 0, 135, 131, 1, 0, 0, 0, 
		135, 132, 1, 0, 0, 0, 136, 137, 1, 0, 0, 0, 137, 138, 5, 45, 0, 0, 138, 
		139, 5, 30, 0, 0, 139, 141, 5, 36, 0, 0, 140, 125, 1, 0, 0, 0, 140, 126, 
		1, 0, 0, 0, 140, 129, 1, 0, 0, 0, 141, 19, 1, 0, 0, 0, 142, 145, 3, 18, 
		9, 0, 143, 144, 7, 3, 0, 0, 144, 146, 5, 34, 0, 0, 145, 143, 1, 0, 0, 
		0, 145, 146, 1, 0, 0, 0, 146, 21, 1, 0, 0, 0, 147, 150, 3, 14, 7, 0, 148, 
		149, 7, 3, 0, 0, 149, 151, 5, 34, 0, 0, 150, 148, 1, 0, 0, 0, 150, 151, 
		1, 0, 0, 0, 151, 154, 1, 0, 0, 0, 152, 154, 3, 16, 8, 0, 153, 147, 1, 
		0, 0, 0, 153, 152, 1, 0, 0, 0, 154, 23, 1, 0, 0, 0, 155, 156, 5, 15, 0, 
		0, 156, 157, 5, 35, 0, 0, 157, 158, 3, 22, 11, 0, 158, 159, 5, 36, 0, 
		0, 159, 25, 1, 0, 0, 0, 160, 161, 5, 16, 0, 0, 161, 162, 5, 35, 0, 0, 
		162, 163, 5, 37, 0, 0, 163, 168, 5, 39, 0, 0, 164, 165, 5, 45, 0, 0, 165, 
		167, 5, 39, 0, 0, 166, 164, 1, 0, 0, 0, 167, 170, 1, 0, 0, 0, 168, 166, 
		1, 0, 0, 0, 168, 169, 1, 0, 0, 0, 169, 171, 1, 0, 0, 0, 170, 168, 1, 0, 
		0, 0, 171, 172, 5, 38, 0, 0, 172, 173, 5, 45, 0, 0, 173, 174, 5, 37, 0, 
		0, 174, 175, 5, 39, 0, 0, 175, 181, 5, 20, 0, 0, 176, 177, 5, 45, 0, 0, 
		177, 178, 5, 39, 0, 0, 178, 180, 5, 20, 0, 0, 179, 176, 1, 0, 0, 0, 180, 
		183, 1, 0, 0, 0, 181, 179, 1, 0, 0, 0, 181, 182, 1, 0, 0, 0, 182, 184, 
		1, 0, 0, 0, 183, 181, 1, 0, 0, 0, 184, 189, 5, 38, 0, 0, 185, 186, 5, 
		45, 0, 0, 186, 188, 3, 28, 14, 0, 187, 185, 1, 0, 0, 0, 188, 191, 1, 0, 
		0, 0, 189, 187, 1, 0, 0, 0, 189, 190, 1, 0, 0, 0, 190, 192, 1, 0, 0, 0, 
		191, 189, 1, 0, 0, 0, 192, 193, 5, 36, 0, 0, 193, 27, 1, 0, 0, 0, 194, 
		206, 5, 40, 0, 0, 195, 196, 5, 37, 0, 0, 196, 197, 5, 40, 0, 0, 197, 198, 
		5, 45, 0, 0, 198, 199, 5, 40, 0, 0, 199, 206, 5, 38, 0, 0, 200, 201, 5, 
		37, 0, 0, 201, 202, 5, 40, 0, 0, 202, 203, 5, 45, 0, 0, 203, 204, 5, 21, 
		0, 0, 204, 206, 5, 38, 0, 0, 205, 194, 1, 0, 0, 0, 205, 195, 1, 0, 0, 
		0, 205, 200, 1, 0, 0, 0, 206, 29, 1, 0, 0, 0, 207, 208, 5, 17, 0, 0, 208, 
		209, 5, 35, 0, 0, 209, 214, 3, 32, 16, 0, 210, 211, 5, 45, 0, 0, 211, 
		213, 3, 32, 16, 0, 212, 210, 1, 0, 0, 0, 213, 216, 1, 0, 0, 0, 214, 212, 
		1, 0, 0, 0, 214, 215, 1, 0, 0, 0, 215, 217, 1, 0, 0, 0, 216, 214, 1, 0, 
		0, 0, 217, 218, 5, 36, 0, 0, 218, 31, 1, 0, 0, 0, 219, 220, 5, 39, 0, 
		0, 220, 221, 5, 19, 0, 0, 221, 232, 3, 20, 10, 0, 222, 223, 5, 39, 0, 
		0, 223, 224, 5, 18, 0, 0, 224, 232, 3, 22, 11, 0, 225, 226, 5, 39, 0, 
		0, 226, 227, 5, 19, 0, 0, 227, 232, 3, 24, 12, 0, 228, 229, 5, 39, 0, 
		0, 229, 230, 5, 19, 0, 0, 230, 232, 7, 0, 0, 0, 231, 219, 1, 0, 0, 0, 
		231, 222, 1, 0, 0, 0, 231, 225, 1, 0, 0, 0, 231, 228, 1, 0, 0, 0, 232, 
		33, 1, 0, 0, 0, 233, 234, 7, 4, 0, 0, 234, 35, 1, 0, 0, 0, 21, 39, 46, 
		50, 56, 62, 74, 86, 110, 114, 121, 135, 140, 145, 150, 153, 168, 181, 
		189, 205, 214, 231
	];
}
