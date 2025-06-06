use super::builder::*;
use super::types::*;
use dprint_core::configuration::*;

/// Resolves configuration from a collection of key value strings.
///
/// # Example
///
/// ```
/// use dprint_core::configuration::ConfigKeyMap;
/// use dprint_core::configuration::resolve_global_config;
/// use dprint_plugin_typescript::configuration::resolve_config;
///
/// let mut config_map = ConfigKeyMap::new(); // get a collection of key value pairs from somewhere
/// let global_config_result = resolve_global_config(&mut config_map);
///
/// // check global_config_result.diagnostics here...
///
/// let typescript_config_map = ConfigKeyMap::new(); // get a collection of k/v pairs from somewhere
/// let config_result = resolve_config(
///     typescript_config_map,
///     &global_config_result.config
/// );
///
/// // check config_result.diagnostics here and use config_result.config
/// ```
pub fn resolve_config(config: ConfigKeyMap, global_config: &GlobalConfiguration) -> ResolveConfigurationResult<Configuration> {
  let mut diagnostics = Vec::new();
  let mut config = config;

  if get_value(&mut config, "deno", false, &mut diagnostics) {
    fill_deno_config(&mut config);
  }

  // show diagnostics for renaming this property
  handle_renamed_config_property(
    &mut config,
    "jsxElement.spaceBeforeSelfClosingTagSlash",
    "jsxSelfClosingElement.spaceBeforeSlash",
    &mut diagnostics,
  );
  handle_renamed_config_property(
    &mut config,
    "jsx.spaceBeforeSelfClosingTagSlash",
    "jsxSelfClosingElement.spaceBeforeSlash",
    &mut diagnostics,
  );

  let semi_colons = get_value(&mut config, "semiColons", SemiColons::Prefer, &mut diagnostics);
  let brace_position = get_value(&mut config, "bracePosition", BracePosition::SameLineUnlessHanging, &mut diagnostics);
  let next_control_flow_position = get_value(&mut config, "nextControlFlowPosition", NextControlFlowPosition::SameLine, &mut diagnostics);
  let operator_position = get_value(&mut config, "operatorPosition", OperatorPosition::NextLine, &mut diagnostics);
  let single_body_position = get_value(&mut config, "singleBodyPosition", SameOrNextLinePosition::Maintain, &mut diagnostics);
  let trailing_commas = get_value(&mut config, "trailingCommas", TrailingCommas::OnlyMultiLine, &mut diagnostics);
  let use_braces = get_value(&mut config, "useBraces", UseBraces::WhenNotSingleLine, &mut diagnostics);
  let prefer_hanging = get_value(&mut config, "preferHanging", false, &mut diagnostics);
  let prefer_hanging_granular = if prefer_hanging { PreferHanging::Always } else { PreferHanging::Never };
  let prefer_single_line_nullable = get_nullable_value(&mut config, "preferSingleLine", &mut diagnostics);
  let prefer_single_line = prefer_single_line_nullable.unwrap_or(false);
  let space_surrounding_properties = get_value(&mut config, "spaceSurroundingProperties", true, &mut diagnostics);
  let type_literal_separator_kind = get_value(&mut config, "typeLiteral.separatorKind", SemiColonOrComma::SemiColon, &mut diagnostics);
  let quote_style = get_value(&mut config, "quoteStyle", QuoteStyle::AlwaysDouble, &mut diagnostics);
  let quote_props = get_value(&mut config, "quoteProps", QuoteProps::Preserve, &mut diagnostics);
  let space_around = get_value(&mut config, "spaceAround", false, &mut diagnostics);
  let jsx_bracket_position = get_value(&mut config, "jsx.bracketPosition", SameOrNextLinePosition::NextLine, &mut diagnostics);

  let resolved_config = Configuration {
    line_width: get_value(
      &mut config,
      "lineWidth",
      global_config.line_width.unwrap_or(RECOMMENDED_GLOBAL_CONFIGURATION.line_width),
      &mut diagnostics,
    ),
    use_tabs: get_value(
      &mut config,
      "useTabs",
      global_config.use_tabs.unwrap_or(RECOMMENDED_GLOBAL_CONFIGURATION.use_tabs),
      &mut diagnostics,
    ),
    indent_width: get_value(
      &mut config,
      "indentWidth",
      global_config.indent_width.unwrap_or(RECOMMENDED_GLOBAL_CONFIGURATION.indent_width),
      &mut diagnostics,
    ),
    new_line_kind: get_value(
      &mut config,
      "newLineKind",
      global_config.new_line_kind.unwrap_or(RECOMMENDED_GLOBAL_CONFIGURATION.new_line_kind),
      &mut diagnostics,
    ),
    quote_style,
    quote_props,
    semi_colons,
    file_indent_level: get_value(&mut config, "fileIndentLevel", 0, &mut diagnostics),
    /* situational */
    arrow_function_use_parentheses: get_value(&mut config, "arrowFunction.useParentheses", UseParentheses::Maintain, &mut diagnostics),
    binary_expression_line_per_expression: get_value(&mut config, "binaryExpression.linePerExpression", false, &mut diagnostics),
    conditional_expression_line_per_expression: get_value(&mut config, "conditionalExpression.linePerExpression", true, &mut diagnostics),
    jsx_quote_style: get_value(&mut config, "jsx.quoteStyle", quote_style.to_jsx_quote_style(), &mut diagnostics),
    jsx_multi_line_parens: get_value(&mut config, "jsx.multiLineParens", JsxMultiLineParens::Prefer, &mut diagnostics),
    jsx_force_new_lines_surrounding_content: get_value(&mut config, "jsx.forceNewLinesSurroundingContent", false, &mut diagnostics),
    jsx_opening_element_bracket_position: get_value(&mut config, "jsxOpeningElement.bracketPosition", jsx_bracket_position, &mut diagnostics),
    jsx_self_closing_element_bracket_position: get_value(&mut config, "jsxSelfClosingElement.bracketPosition", jsx_bracket_position, &mut diagnostics),
    member_expression_line_per_expression: get_value(&mut config, "memberExpression.linePerExpression", false, &mut diagnostics),
    type_literal_separator_kind_single_line: get_value(
      &mut config,
      "typeLiteral.separatorKind.singleLine",
      type_literal_separator_kind,
      &mut diagnostics,
    ),
    type_literal_separator_kind_multi_line: get_value(
      &mut config,
      "typeLiteral.separatorKind.multiLine",
      type_literal_separator_kind,
      &mut diagnostics,
    ),
    /* sorting */
    module_sort_import_declarations: get_value(&mut config, "module.sortImportDeclarations", SortOrder::CaseInsensitive, &mut diagnostics),
    module_sort_export_declarations: get_value(&mut config, "module.sortExportDeclarations", SortOrder::CaseInsensitive, &mut diagnostics),
    import_declaration_sort_named_imports: get_value(&mut config, "importDeclaration.sortNamedImports", SortOrder::CaseInsensitive, &mut diagnostics),
    export_declaration_sort_named_exports: get_value(&mut config, "exportDeclaration.sortNamedExports", SortOrder::CaseInsensitive, &mut diagnostics),
    import_declaration_sort_type_only_imports: get_value(
      &mut config,
      "importDeclaration.sortTypeOnlyImports",
      NamedTypeImportsExportsOrder::None,
      &mut diagnostics,
    ),
    export_declaration_sort_type_only_exports: get_value(
      &mut config,
      "exportDeclaration.sortTypeOnlyExports",
      NamedTypeImportsExportsOrder::None,
      &mut diagnostics,
    ),
    /* ignore comments */
    ignore_node_comment_text: get_value(&mut config, "ignoreNodeCommentText", String::from("dprint-ignore"), &mut diagnostics),
    ignore_file_comment_text: get_value(&mut config, "ignoreFileCommentText", String::from("dprint-ignore-file"), &mut diagnostics),
    /* brace position */
    arrow_function_brace_position: get_value(&mut config, "arrowFunction.bracePosition", brace_position, &mut diagnostics),
    class_declaration_brace_position: get_value(&mut config, "classDeclaration.bracePosition", brace_position, &mut diagnostics),
    class_expression_brace_position: get_value(&mut config, "classExpression.bracePosition", brace_position, &mut diagnostics),
    constructor_brace_position: get_value(&mut config, "constructor.bracePosition", brace_position, &mut diagnostics),
    do_while_statement_brace_position: get_value(&mut config, "doWhileStatement.bracePosition", brace_position, &mut diagnostics),
    enum_declaration_brace_position: get_value(&mut config, "enumDeclaration.bracePosition", brace_position, &mut diagnostics),
    for_statement_brace_position: get_value(&mut config, "forStatement.bracePosition", brace_position, &mut diagnostics),
    for_in_statement_brace_position: get_value(&mut config, "forInStatement.bracePosition", brace_position, &mut diagnostics),
    for_of_statement_brace_position: get_value(&mut config, "forOfStatement.bracePosition", brace_position, &mut diagnostics),
    get_accessor_brace_position: get_value(&mut config, "getAccessor.bracePosition", brace_position, &mut diagnostics),
    if_statement_brace_position: get_value(&mut config, "ifStatement.bracePosition", brace_position, &mut diagnostics),
    interface_declaration_brace_position: get_value(&mut config, "interfaceDeclaration.bracePosition", brace_position, &mut diagnostics),
    function_declaration_brace_position: get_value(&mut config, "functionDeclaration.bracePosition", brace_position, &mut diagnostics),
    function_expression_brace_position: get_value(&mut config, "functionExpression.bracePosition", brace_position, &mut diagnostics),
    method_brace_position: get_value(&mut config, "method.bracePosition", brace_position, &mut diagnostics),
    module_declaration_brace_position: get_value(&mut config, "moduleDeclaration.bracePosition", brace_position, &mut diagnostics),
    set_accessor_brace_position: get_value(&mut config, "setAccessor.bracePosition", brace_position, &mut diagnostics),
    static_block_brace_position: get_value(&mut config, "staticBlock.bracePosition", brace_position, &mut diagnostics),
    switch_case_brace_position: get_value(&mut config, "switchCase.bracePosition", brace_position, &mut diagnostics),
    switch_statement_brace_position: get_value(&mut config, "switchStatement.bracePosition", brace_position, &mut diagnostics),
    try_statement_brace_position: get_value(&mut config, "tryStatement.bracePosition", brace_position, &mut diagnostics),
    while_statement_brace_position: get_value(&mut config, "whileStatement.bracePosition", brace_position, &mut diagnostics),
    /* prefer hanging */
    arguments_prefer_hanging: get_value(&mut config, "arguments.preferHanging", prefer_hanging_granular, &mut diagnostics),
    array_expression_prefer_hanging: get_value(&mut config, "arrayExpression.preferHanging", prefer_hanging_granular, &mut diagnostics),
    array_pattern_prefer_hanging: get_value(&mut config, "arrayPattern.preferHanging", prefer_hanging, &mut diagnostics),
    do_while_statement_prefer_hanging: get_value(&mut config, "doWhileStatement.preferHanging", prefer_hanging, &mut diagnostics),
    export_declaration_prefer_hanging: get_value(&mut config, "exportDeclaration.preferHanging", prefer_hanging, &mut diagnostics),
    extends_clause_prefer_hanging: get_value(&mut config, "extendsClause.preferHanging", prefer_hanging, &mut diagnostics),
    for_in_statement_prefer_hanging: get_value(&mut config, "forInStatement.preferHanging", prefer_hanging, &mut diagnostics),
    for_of_statement_prefer_hanging: get_value(&mut config, "forOfStatement.preferHanging", prefer_hanging, &mut diagnostics),
    for_statement_prefer_hanging: get_value(&mut config, "forStatement.preferHanging", prefer_hanging, &mut diagnostics),
    if_statement_prefer_hanging: get_value(&mut config, "ifStatement.preferHanging", prefer_hanging, &mut diagnostics),
    implements_clause_prefer_hanging: get_value(&mut config, "implementsClause.preferHanging", prefer_hanging, &mut diagnostics),
    import_declaration_prefer_hanging: get_value(&mut config, "importDeclaration.preferHanging", prefer_hanging, &mut diagnostics),
    jsx_attributes_prefer_hanging: get_value(&mut config, "jsxAttributes.preferHanging", prefer_hanging, &mut diagnostics),
    object_expression_prefer_hanging: get_value(&mut config, "objectExpression.preferHanging", prefer_hanging, &mut diagnostics),
    object_pattern_prefer_hanging: get_value(&mut config, "objectPattern.preferHanging", prefer_hanging, &mut diagnostics),
    parameters_prefer_hanging: get_value(&mut config, "parameters.preferHanging", prefer_hanging_granular, &mut diagnostics),
    sequence_expression_prefer_hanging: get_value(&mut config, "sequenceExpression.preferHanging", prefer_hanging, &mut diagnostics),
    switch_statement_prefer_hanging: get_value(&mut config, "switchStatement.preferHanging", prefer_hanging, &mut diagnostics),
    tuple_type_prefer_hanging: get_value(&mut config, "tupleType.preferHanging", prefer_hanging_granular, &mut diagnostics),
    type_literal_prefer_hanging: get_value(&mut config, "typeLiteral.preferHanging", prefer_hanging, &mut diagnostics),
    type_parameters_prefer_hanging: get_value(&mut config, "typeParameters.preferHanging", prefer_hanging_granular, &mut diagnostics),
    union_and_intersection_type_prefer_hanging: get_value(&mut config, "unionAndIntersectionType.preferHanging", prefer_hanging, &mut diagnostics),
    variable_statement_prefer_hanging: get_value(&mut config, "variableStatement.preferHanging", prefer_hanging, &mut diagnostics),
    while_statement_prefer_hanging: get_value(&mut config, "whileStatement.preferHanging", prefer_hanging, &mut diagnostics),
    /* member spacing */
    enum_declaration_member_spacing: get_value(&mut config, "enumDeclaration.memberSpacing", MemberSpacing::Maintain, &mut diagnostics),
    /* next control flow position */
    if_statement_next_control_flow_position: get_value(&mut config, "ifStatement.nextControlFlowPosition", next_control_flow_position, &mut diagnostics),
    try_statement_next_control_flow_position: get_value(
      &mut config,
      "tryStatement.nextControlFlowPosition",
      next_control_flow_position,
      &mut diagnostics,
    ),
    do_while_statement_next_control_flow_position: get_value(
      &mut config,
      "doWhileStatement.nextControlFlowPosition",
      next_control_flow_position,
      &mut diagnostics,
    ),
    /* operator position */
    binary_expression_operator_position: get_value(&mut config, "binaryExpression.operatorPosition", operator_position, &mut diagnostics),
    conditional_expression_operator_position: get_value(&mut config, "conditionalExpression.operatorPosition", operator_position, &mut diagnostics),
    conditional_type_operator_position: get_value(&mut config, "conditionalType.operatorPosition", operator_position, &mut diagnostics),
    /* single body position */
    if_statement_single_body_position: get_value(&mut config, "ifStatement.singleBodyPosition", single_body_position, &mut diagnostics),
    for_statement_single_body_position: get_value(&mut config, "forStatement.singleBodyPosition", single_body_position, &mut diagnostics),
    for_in_statement_single_body_position: get_value(&mut config, "forInStatement.singleBodyPosition", single_body_position, &mut diagnostics),
    for_of_statement_single_body_position: get_value(&mut config, "forOfStatement.singleBodyPosition", single_body_position, &mut diagnostics),
    while_statement_single_body_position: get_value(&mut config, "whileStatement.singleBodyPosition", single_body_position, &mut diagnostics),
    /* trailing commas */
    arguments_trailing_commas: get_value(&mut config, "arguments.trailingCommas", trailing_commas, &mut diagnostics),
    parameters_trailing_commas: get_value(&mut config, "parameters.trailingCommas", trailing_commas, &mut diagnostics),
    array_expression_trailing_commas: get_value(&mut config, "arrayExpression.trailingCommas", trailing_commas, &mut diagnostics),
    array_pattern_trailing_commas: get_value(&mut config, "arrayPattern.trailingCommas", trailing_commas, &mut diagnostics),
    enum_declaration_trailing_commas: get_value(&mut config, "enumDeclaration.trailingCommas", trailing_commas, &mut diagnostics),
    export_declaration_trailing_commas: get_value(&mut config, "exportDeclaration.trailingCommas", trailing_commas, &mut diagnostics),
    import_declaration_trailing_commas: get_value(&mut config, "importDeclaration.trailingCommas", trailing_commas, &mut diagnostics),
    object_expression_trailing_commas: get_value(&mut config, "objectExpression.trailingCommas", trailing_commas, &mut diagnostics),
    object_pattern_trailing_commas: get_value(&mut config, "objectPattern.trailingCommas", trailing_commas, &mut diagnostics),
    tuple_type_trailing_commas: get_value(&mut config, "tupleType.trailingCommas", trailing_commas, &mut diagnostics),
    type_literal_trailing_commas: get_value(&mut config, "typeLiteral.trailingCommas", trailing_commas, &mut diagnostics),
    type_parameters_trailing_commas: get_value(&mut config, "typeParameters.trailingCommas", trailing_commas, &mut diagnostics),
    /* use braces */
    if_statement_use_braces: get_value(&mut config, "ifStatement.useBraces", use_braces, &mut diagnostics),
    for_statement_use_braces: get_value(&mut config, "forStatement.useBraces", use_braces, &mut diagnostics),
    for_in_statement_use_braces: get_value(&mut config, "forInStatement.useBraces", use_braces, &mut diagnostics),
    for_of_statement_use_braces: get_value(&mut config, "forOfStatement.useBraces", use_braces, &mut diagnostics),
    while_statement_use_braces: get_value(&mut config, "whileStatement.useBraces", use_braces, &mut diagnostics),
    /* prefer single line */
    array_expression_prefer_single_line: get_value(&mut config, "arrayExpression.preferSingleLine", prefer_single_line, &mut diagnostics),
    array_pattern_prefer_single_line: get_value(&mut config, "arrayPattern.preferSingleLine", prefer_single_line, &mut diagnostics),
    arguments_prefer_single_line: get_value(&mut config, "arguments.preferSingleLine", prefer_single_line, &mut diagnostics),
    binary_expression_prefer_single_line: get_value(&mut config, "binaryExpression.preferSingleLine", prefer_single_line, &mut diagnostics),
    computed_prefer_single_line: get_value(&mut config, "computed.preferSingleLine", prefer_single_line, &mut diagnostics),
    conditional_expression_prefer_single_line: get_value(&mut config, "conditionalExpression.preferSingleLine", prefer_single_line, &mut diagnostics),
    conditional_type_prefer_single_line: get_value(&mut config, "conditionalType.preferSingleLine", prefer_single_line, &mut diagnostics),
    decorators_prefer_single_line: get_value(&mut config, "decorators.preferSingleLine", prefer_single_line, &mut diagnostics),
    export_declaration_prefer_single_line: get_value(
      &mut config,
      "exportDeclaration.preferSingleLine",
      prefer_single_line_nullable.unwrap_or(true),
      &mut diagnostics,
    ),
    for_statement_prefer_single_line: get_value(&mut config, "forStatement.preferSingleLine", prefer_single_line, &mut diagnostics),
    import_declaration_prefer_single_line: get_value(
      &mut config,
      "importDeclaration.preferSingleLine",
      prefer_single_line_nullable.unwrap_or(true),
      &mut diagnostics,
    ),
    jsx_attributes_prefer_single_line: get_value(&mut config, "jsxAttributes.preferSingleLine", prefer_single_line, &mut diagnostics),
    jsx_element_prefer_single_line: get_value(&mut config, "jsxElement.preferSingleLine", prefer_single_line, &mut diagnostics),
    mapped_type_prefer_single_line: get_value(&mut config, "mappedType.preferSingleLine", prefer_single_line, &mut diagnostics),
    member_expression_prefer_single_line: get_value(&mut config, "memberExpression.preferSingleLine", prefer_single_line, &mut diagnostics),
    object_expression_prefer_single_line: get_value(&mut config, "objectExpression.preferSingleLine", prefer_single_line, &mut diagnostics),
    object_pattern_prefer_single_line: get_value(&mut config, "objectPattern.preferSingleLine", prefer_single_line, &mut diagnostics),
    parameters_prefer_single_line: get_value(&mut config, "parameters.preferSingleLine", prefer_single_line, &mut diagnostics),
    parentheses_prefer_single_line: get_value(&mut config, "parentheses.preferSingleLine", prefer_single_line, &mut diagnostics),
    tuple_type_prefer_single_line: get_value(&mut config, "tupleType.preferSingleLine", prefer_single_line, &mut diagnostics),
    type_literal_prefer_single_line: get_value(&mut config, "typeLiteral.preferSingleLine", prefer_single_line, &mut diagnostics),
    type_parameters_prefer_single_line: get_value(&mut config, "typeParameters.preferSingleLine", prefer_single_line, &mut diagnostics),
    union_and_intersection_type_prefer_single_line: get_value(&mut config, "unionAndIntersectionType.preferSingleLine", prefer_single_line, &mut diagnostics),
    variable_statement_prefer_single_line: get_value(&mut config, "variableStatement.preferSingleLine", prefer_single_line, &mut diagnostics),
    /* force single line */
    import_declaration_force_single_line: get_value(&mut config, "importDeclaration.forceSingleLine", false, &mut diagnostics),
    export_declaration_force_single_line: get_value(&mut config, "exportDeclaration.forceSingleLine", false, &mut diagnostics),
    /* force multi line specifiers */
    import_declaration_force_multi_line: get_value(&mut config, "importDeclaration.forceMultiLine", ForceMultiLine::Never, &mut diagnostics),
    export_declaration_force_multi_line: get_value(&mut config, "exportDeclaration.forceMultiLine", ForceMultiLine::Never, &mut diagnostics),
    /* space settings */
    binary_expression_space_surrounding_bitwise_and_arithmetic_operator: get_value(
      &mut config,
      "binaryExpression.spaceSurroundingBitwiseAndArithmeticOperator",
      true,
      &mut diagnostics,
    ),
    comment_line_force_space_after_slashes: get_value(&mut config, "commentLine.forceSpaceAfterSlashes", true, &mut diagnostics),
    construct_signature_space_after_new_keyword: get_value(&mut config, "constructSignature.spaceAfterNewKeyword", false, &mut diagnostics),
    constructor_space_before_parentheses: get_value(&mut config, "constructor.spaceBeforeParentheses", false, &mut diagnostics),
    constructor_type_space_after_new_keyword: get_value(&mut config, "constructorType.spaceAfterNewKeyword", false, &mut diagnostics),
    do_while_statement_space_after_while_keyword: get_value(&mut config, "doWhileStatement.spaceAfterWhileKeyword", true, &mut diagnostics),
    export_declaration_space_surrounding_named_exports: get_value(&mut config, "exportDeclaration.spaceSurroundingNamedExports", true, &mut diagnostics),
    for_statement_space_after_for_keyword: get_value(&mut config, "forStatement.spaceAfterForKeyword", true, &mut diagnostics),
    for_statement_space_after_semi_colons: get_value(&mut config, "forStatement.spaceAfterSemiColons", true, &mut diagnostics),
    for_in_statement_space_after_for_keyword: get_value(&mut config, "forInStatement.spaceAfterForKeyword", true, &mut diagnostics),
    for_of_statement_space_after_for_keyword: get_value(&mut config, "forOfStatement.spaceAfterForKeyword", true, &mut diagnostics),
    function_declaration_space_before_parentheses: get_value(&mut config, "functionDeclaration.spaceBeforeParentheses", false, &mut diagnostics),
    function_expression_space_before_parentheses: get_value(&mut config, "functionExpression.spaceBeforeParentheses", false, &mut diagnostics),
    function_expression_space_after_function_keyword: get_value(&mut config, "functionExpression.spaceAfterFunctionKeyword", false, &mut diagnostics),
    get_accessor_space_before_parentheses: get_value(&mut config, "getAccessor.spaceBeforeParentheses", false, &mut diagnostics),
    if_statement_space_after_if_keyword: get_value(&mut config, "ifStatement.spaceAfterIfKeyword", true, &mut diagnostics),
    import_declaration_space_surrounding_named_imports: get_value(&mut config, "importDeclaration.spaceSurroundingNamedImports", true, &mut diagnostics),
    jsx_expression_container_space_surrounding_expression: get_value(&mut config, "jsxExpressionContainer.spaceSurroundingExpression", false, &mut diagnostics),
    jsx_self_closing_element_space_before_slash: get_value(&mut config, "jsxSelfClosingElement.spaceBeforeSlash", true, &mut diagnostics),
    method_space_before_parentheses: get_value(&mut config, "method.spaceBeforeParentheses", false, &mut diagnostics),
    object_expression_space_surrounding_properties: get_value(
      &mut config,
      "objectExpression.spaceSurroundingProperties",
      space_surrounding_properties,
      &mut diagnostics,
    ),
    object_pattern_space_surrounding_properties: get_value(
      &mut config,
      "objectPattern.spaceSurroundingProperties",
      space_surrounding_properties,
      &mut diagnostics,
    ),
    set_accessor_space_before_parentheses: get_value(&mut config, "setAccessor.spaceBeforeParentheses", false, &mut diagnostics),
    space_surrounding_properties,
    tagged_template_space_before_literal: get_value(&mut config, "taggedTemplate.spaceBeforeLiteral", false, &mut diagnostics),
    type_annotation_space_before_colon: get_value(&mut config, "typeAnnotation.spaceBeforeColon", false, &mut diagnostics),
    type_assertion_space_before_expression: get_value(&mut config, "typeAssertion.spaceBeforeExpression", true, &mut diagnostics),
    type_literal_space_surrounding_properties: get_value(
      &mut config,
      "typeLiteral.spaceSurroundingProperties",
      space_surrounding_properties,
      &mut diagnostics,
    ),
    while_statement_space_after_while_keyword: get_value(&mut config, "whileStatement.spaceAfterWhileKeyword", true, &mut diagnostics),
    arguments_space_around: get_value(&mut config, "arguments.spaceAround", space_around, &mut diagnostics),
    array_expression_space_around: get_value(&mut config, "arrayExpression.spaceAround", space_around, &mut diagnostics),
    array_pattern_space_around: get_value(&mut config, "arrayPattern.spaceAround", space_around, &mut diagnostics),
    catch_clause_space_around: get_value(&mut config, "catchClause.spaceAround", space_around, &mut diagnostics),
    do_while_statement_space_around: get_value(&mut config, "doWhileStatement.spaceAround", space_around, &mut diagnostics),
    for_in_statement_space_around: get_value(&mut config, "forInStatement.spaceAround", space_around, &mut diagnostics),
    for_of_statement_space_around: get_value(&mut config, "forOfStatement.spaceAround", space_around, &mut diagnostics),
    for_statement_space_around: get_value(&mut config, "forStatement.spaceAround", space_around, &mut diagnostics),
    if_statement_space_around: get_value(&mut config, "ifStatement.spaceAround", space_around, &mut diagnostics),
    parameters_space_around: get_value(&mut config, "parameters.spaceAround", space_around, &mut diagnostics),
    paren_expression_space_around: get_value(&mut config, "parenExpression.spaceAround", space_around, &mut diagnostics),
    switch_statement_space_around: get_value(&mut config, "switchStatement.spaceAround", space_around, &mut diagnostics),
    tuple_type_space_around: get_value(&mut config, "tupleType.spaceAround", space_around, &mut diagnostics),
    while_statement_space_around: get_value(&mut config, "whileStatement.spaceAround", space_around, &mut diagnostics),
  };

  diagnostics.extend(get_unknown_property_diagnostics(config));

  return ResolveConfigurationResult {
    config: resolved_config,
    diagnostics,
  };

  fn fill_deno_config(config: &mut ConfigKeyMap) {
    for (key, value) in ConfigurationBuilder::new().deno().config.iter() {
      if !config.contains_key(key) {
        config.insert(key.clone(), value.clone());
      }
    }
  }
}

#[cfg(test)]
mod tests {
  use dprint_core::configuration::resolve_global_config;
  use dprint_core::configuration::NewLineKind;

  use super::super::builder::ConfigurationBuilder;
  use super::*;

  #[test]
  fn handle_global_config_default() {
    let config_builder = ConfigurationBuilder::new();
    let config = config_builder.build();
    assert_eq!(config.line_width, 120);
    assert_eq!(config.indent_width, 2);
    assert_eq!(config.new_line_kind, NewLineKind::LineFeed);
    assert!(!config.use_tabs);
  }

  #[test]
  fn handle_global_config() {
    let mut global_config = ConfigKeyMap::new();
    global_config.insert(String::from("lineWidth"), ConfigKeyValue::from_i32(80));
    global_config.insert(String::from("indentWidth"), ConfigKeyValue::from_i32(8));
    global_config.insert(String::from("newLineKind"), ConfigKeyValue::from_str("crlf"));
    global_config.insert(String::from("useTabs"), ConfigKeyValue::from_bool(true));
    let global_config = resolve_global_config(&mut global_config).config;
    let mut config_builder = ConfigurationBuilder::new();
    let config = config_builder.global_config(global_config).build();
    assert_eq!(config.line_width, 80);
    assert_eq!(config.indent_width, 8);
    assert_eq!(config.new_line_kind, NewLineKind::CarriageReturnLineFeed);
    assert!(config.use_tabs);
  }

  #[test]
  fn handle_deno_config() {
    let mut config = ConfigKeyMap::new();
    config.insert(String::from("deno"), ConfigKeyValue::from_bool(true));
    let global_config = GlobalConfiguration::default();
    let result = resolve_config(config, &global_config);
    let expected_config = ConfigurationBuilder::new().deno().build();
    // todo: test that both objects equal each other
    assert_eq!(result.config.indent_width, expected_config.indent_width);
    assert_eq!(result.config.line_width, expected_config.line_width);
    assert_eq!(result.diagnostics.len(), 0);
  }

  #[test]
  fn handle_deno_config_with_overwrites() {
    let mut config = ConfigKeyMap::new();
    config.insert(String::from("deno"), ConfigKeyValue::from_bool(true));
    config.insert(String::from("indentWidth"), ConfigKeyValue::from_i32(8));
    let global_config = GlobalConfiguration::default();
    let result = resolve_config(config, &global_config);
    let expected_config = ConfigurationBuilder::new().deno().build();
    assert_eq!(result.config.indent_width, 8);
    assert_eq!(result.config.line_width, expected_config.line_width);
    assert_eq!(result.diagnostics.len(), 0);
  }
}
