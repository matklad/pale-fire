use crate::palette::{ColorLightnessPreset, Palette};
use mottle::style::FontStyle;
use mottle::theme::Scope::*;
use mottle::theme::ThemeBuilder;
use tincture::{ColorSpace, Oklch};

pub(crate) fn add_rules(builder: &mut ThemeBuilder, palette: Palette) {
    workspace_colors(builder, &palette);
    syntax_highlighting(builder, &palette);
}

fn workspace_colors(builder: &mut ThemeBuilder, palette: &Palette) {
    builder.add_workspace_rule("activityBar.activeBorder", palette.fg());
    builder.add_workspace_rule("activityBar.background", palette.greyscale(-1));
    builder.add_workspace_rule("activityBar.foreground", palette.fg());
    builder.add_workspace_rule("activityBar.inactiveForeground", palette.greyscale(4));
    builder.add_workspace_rule("activityBarBadge.background", palette.blue(0));
    builder.add_workspace_rule("activityBarBadge.foreground", palette.greyscale(0));
    builder.add_workspace_rule("badge.background", palette.greyscale(3));
    builder.add_workspace_rule("badge.foreground", palette.fg());
    builder.add_workspace_rule("button.background", palette.blue(0));
    builder.add_workspace_rule("button.foreground", palette.greyscale(0));
    builder.add_workspace_rule("button.hoverBackground", palette.fg());
    builder.add_workspace_rule("checkbox.background", palette.greyscale(-2));
    builder.add_workspace_rule("checkbox.border", palette.greyscale(2));
    builder.add_workspace_rule("debugIcon.breakpointForeground", palette.red(0));
    builder.add_workspace_rule(
        "diffEditor.insertedTextBackground",
        (palette.green(ColorLightnessPreset::DiffBg), 0x33),
    );
    builder.add_workspace_rule(
        "diffEditor.removedTextBackground",
        (palette.red(ColorLightnessPreset::DiffBg), 0x33),
    );
    builder.add_workspace_rule("dropdown.border", palette.greyscale(2));
    builder.add_workspace_rule("editor.background", palette.greyscale(0));
    builder.add_workspace_rule("editor.findMatchBackground", (palette.blue(0), 0x66));
    builder.add_workspace_rule(
        "editor.findMatchHighlightBackground",
        (palette.blue(0), 0x44),
    );
    builder.add_workspace_rule("editor.foldBackground", (palette.blue(0), 0x22));
    builder.add_workspace_rules(&["editor.foreground", "foreground"], palette.fg());
    builder.add_workspace_rule("editor.hoverHighlightBackground", palette.greyscale(2));
    builder.add_workspace_rule("editor.lineHighlightBackground", palette.greyscale(-1));
    builder.add_workspace_rule("editor.rangeHighlightBackground", (palette.blue(0), 0x22));
    builder.add_workspace_rules(
        &[
            "editor.selectionBackground",
            "terminal.selectionBackground",
            "selection.background",
            "minimap.selectionHighlight",
        ],
        (palette.blue(0), 0x33),
    );
    builder.add_workspace_rules(
        &[
            "editor.selectionHighlightBackground",
            "editor.symbolHighlightBackground",
            "editor.wordHighlightBackground",
            "editor.wordHighlightStrongBackground",
        ],
        palette.greyscale(3),
    );
    builder.add_workspace_rule("editorCursor.foreground", palette.bright_fg());
    builder.add_workspace_rule("editorError.foreground", palette.red(0));
    builder.add_workspace_rule("editorGroup.dropBackground", (palette.blue(0), 0x22));
    builder.add_workspace_rule("editorGroupHeader.noTabsBackground", palette.greyscale(1));
    builder.add_workspace_rule("editorGroupHeader.tabsBackground", palette.greyscale(-2));
    builder.add_workspace_rule("editorGroup.border", palette.greyscale(3));
    builder.add_workspace_rule(
        "editorGutter.addedBackground",
        palette.green(ColorLightnessPreset::Gutter),
    );
    builder.add_workspace_rule(
        "editorGutter.deletedBackground",
        palette.red(ColorLightnessPreset::Gutter),
    );
    builder.add_workspace_rule(
        "editorGutter.modifiedBackground",
        palette.yellow(ColorLightnessPreset::Gutter),
    );
    builder.add_workspace_rule("editorIndentGuide.activeBackground", palette.greyscale(3));
    builder.add_workspace_rule("editorIndentGuide.background", palette.greyscale(2));
    builder.add_workspace_rule("editorLightBulb.foreground", palette.yellow(2));
    builder.add_workspace_rule("editorLineNumber.activeForeground", palette.greyscale(5));
    builder.add_workspace_rule("editorLineNumber.foreground", palette.greyscale(3));
    builder.add_workspace_rules(
        &[
            "editorLink.activeForeground",
            "textLink.foreground",
            "textLink.activeForeground",
        ],
        palette.blue(0),
    );
    builder.add_workspace_rule(
        "editorOverviewRuler.addedForeground",
        palette.green(ColorLightnessPreset::OverviewRuler),
    );
    builder.add_workspace_rule("editorOverviewRuler.border", palette.greyscale(3));
    builder.add_workspace_rule(
        "editorOverviewRuler.deletedForeground",
        palette.red(ColorLightnessPreset::OverviewRuler),
    );
    builder.add_workspace_rule(
        "editorOverviewRuler.errorForeground",
        palette.red(ColorLightnessPreset::OverviewRuler),
    );
    builder.add_workspace_rule(
        "editorOverviewRuler.findMatchForeground",
        (palette.blue(ColorLightnessPreset::OverviewRuler), 0x88),
    );
    builder.add_workspace_rule(
        "editorOverviewRuler.modifiedForeground",
        palette.yellow(ColorLightnessPreset::OverviewRuler),
    );
    builder.add_workspace_rule(
        "editorOverviewRuler.rangeHighlightForeground",
        (palette.blue(ColorLightnessPreset::OverviewRuler), 0x33),
    );
    builder.add_workspace_rule("editorWarning.foreground", palette.orange(0));
    builder.add_workspace_rule("editorWidget.background", palette.greyscale(-1));
    builder.add_workspace_rule("editorWidget.border", palette.greyscale(2));
    builder.add_workspace_rule(
        "errorLens.errorBackground",
        (palette.red(ColorLightnessPreset::ErrorLensBackground), 0x33),
    );
    builder.add_workspace_rule(
        "errorLens.errorForeground",
        palette.red(ColorLightnessPreset::ErrorLensForeground),
    );
    builder.add_workspace_rule(
        "errorLens.warningBackground",
        (
            palette.orange(ColorLightnessPreset::ErrorLensBackground),
            0x33,
        ),
    );
    builder.add_workspace_rule(
        "errorLens.warningForeground",
        palette.orange(ColorLightnessPreset::ErrorLensForeground),
    );
    builder.add_workspace_rule(
        "errorLens.infoBackground",
        (
            palette.blue(ColorLightnessPreset::ErrorLensBackground),
            0x33,
        ),
    );
    builder.add_workspace_rule(
        "errorLens.infoForeground",
        palette.blue(ColorLightnessPreset::ErrorLensForeground),
    );
    builder.add_workspace_rule(
        "errorLens.hintBackground",
        (
            palette.green(ColorLightnessPreset::ErrorLensBackground),
            0x33,
        ),
    );
    builder.add_workspace_rule(
        "errorLens.hintForeground",
        palette.green(ColorLightnessPreset::ErrorLensForeground),
    );
    builder.add_workspace_rule("focusBorder", palette.greyscale(3));
    builder.add_workspace_rule(
        "gitDecoration.ignoredResourceForeground",
        palette.greyscale(4),
    );
    builder.add_workspace_rule(
        "gitDecoration.modifiedResourceForeground",
        palette.yellow(ColorLightnessPreset::GitDecoration),
    );
    builder.add_workspace_rule(
        "gitDecoration.untrackedResourceForeground",
        palette.green(ColorLightnessPreset::GitDecoration),
    );
    builder.add_workspace_rule("input.background", palette.greyscale(-2));
    builder.add_workspace_rule("input.border", palette.greyscale(2));
    builder.add_workspace_rule("input.foreground", palette.fg());
    builder.add_workspace_rule("input.placeholderForeground", palette.greyscale(4));
    builder.add_workspace_rule("list.activeSelectionBackground", palette.greyscale(2));
    builder.add_workspace_rule("list.activeSelectionForeground", palette.fg());
    builder.add_workspace_rule("list.errorForeground", palette.red(0));
    builder.add_workspace_rule("list.focusBackground", palette.greyscale(2));
    builder.add_workspace_rule("list.highlightForeground", palette.blue(2));
    builder.add_workspace_rule("list.hoverBackground", palette.greyscale(0));
    builder.add_workspace_rule("list.inactiveSelectionBackground", palette.greyscale(1));
    builder.add_workspace_rule("list.warningForeground", palette.orange(0));
    builder.add_workspace_rule(
        "minimap.errorHighlight",
        palette.red(ColorLightnessPreset::Minimap),
    );
    builder.add_workspace_rule(
        "minimap.findMatchHighlight",
        (palette.blue(ColorLightnessPreset::Minimap), 0x66),
    );
    builder.add_workspace_rule("panel.background", palette.greyscale(1));
    builder.add_workspace_rule("panel.border", palette.greyscale(3));
    builder.add_workspace_rule("panelTitle.activeForeground", palette.fg());
    builder.add_workspace_rule("peekView.border", palette.greyscale(4));
    builder.add_workspace_rule("peekViewEditor.background", palette.greyscale(0));
    builder.add_workspace_rule(
        "peekViewEditor.matchHighlightBackground",
        (palette.blue(0), 0x66),
    );
    builder.add_workspace_rule("peekViewResult.background", palette.greyscale(-1));
    builder.add_workspace_rule("peekViewResult.fileForeground", palette.fg());
    builder.add_workspace_rule("peekViewResult.lineForeground", (palette.fg(), 0x99));
    builder.add_workspace_rule(
        "peekViewResult.matchHighlightBackground",
        (palette.blue(0), 0x44),
    );
    builder.add_workspace_rule("peekViewResult.selectionBackground", palette.greyscale(2));
    builder.add_workspace_rule("peekViewResult.selectionForeground", palette.fg());
    builder.add_workspace_rule("peekViewTitle.background", palette.greyscale(-1));
    builder.add_workspace_rule("peekViewTitleDescription.foreground", palette.blue(0));
    builder.add_workspace_rule("peekViewTitleLabel.foreground", palette.bright_fg());
    builder.add_workspace_rule("progressBar.background", palette.blue(0));
    builder.add_workspace_rule("rust_analyzer.inlayHints.foreground", palette.green(-2));
    builder.add_workspace_rule("scrollbar.shadow", (Oklch::BLACK, 0x88));
    builder.add_workspace_rule("settings.headerForeground", palette.bright_fg());
    builder.add_workspace_rule("settings.modifiedItemIndicator", palette.blue(0));
    builder.add_workspace_rule("sideBar.background", palette.greyscale(-1));
    builder.add_workspace_rule("sideBar.foreground", palette.fg());
    builder.add_workspace_rule("sideBarTitle.foreground", palette.bright_fg());
    builder.add_workspace_rules(
        &[
            "statusBar.background",
            "statusBar.debuggingBackground",
            "statusBar.noFolderBackground",
        ],
        palette.greyscale(-2),
    );
    builder.add_workspace_rule(
        "statusBar.foreground",
        palette.green(ColorLightnessPreset::StatusBar),
    );
    builder.add_workspace_rule(
        "statusBar.debuggingForeground",
        palette.orange(ColorLightnessPreset::StatusBar),
    );
    builder.add_workspace_rule("symbolIcon.keywordForeground", palette.keyword_color());
    builder.add_workspace_rule("symbolIcon.variableForeground", palette.variable_color());
    builder.add_workspace_rules(
        &[
            "symbolIcon.functionForeground",
            "symbolIcon.methodForeground",
        ],
        palette.function_color(),
    );
    builder.add_workspace_rules(
        &[
            "symbolIcon.classForeground",
            "symbolIcon.structForeground",
            "symbolIcon.enumeratorForeground",
            // we use same color for type parameters as class, struct, etc
            // instead of the actual type parameter color
            // since rust-analyzer emits this symbol kind for type aliases,
            // which we want to have the same color as other type symbol kinds
            "symbolIcon.typeParameterForeground",
        ],
        palette.type_color(),
    );
    builder.add_workspace_rule("symbolIcon.interfaceForeground", palette.interface_color());
    builder.add_workspace_rule("symbolIcon.constantForeground", palette.constant_color());
    builder.add_workspace_rule(
        "symbolIcon.enumeratorMemberForeground",
        palette.enum_member_color(),
    );
    builder.add_workspace_rules(
        &[
            "symbolIcon.fieldForeground",
            "symbolIcon.propertyForeground",
        ],
        palette.property_color(),
    );
    builder.add_workspace_rules(
        &[
            "symbolIcon.moduleForeground",
            "symbolIcon.namespaceForeground",
        ],
        palette.namespace_color(),
    );
    builder.add_workspace_rule("tab.activeForeground", palette.fg());
    builder.add_workspace_rule("tab.border", palette.greyscale(0));
    builder.add_workspace_rule("tab.inactiveBackground", palette.greyscale(-2));
    builder.add_workspace_rule("tab.inactiveForeground", palette.greyscale(4));
    builder.add_workspace_rule("terminal.ansiBlack", palette.greyscale(-2));
    builder.add_workspace_rule(
        "terminal.ansiBlue",
        palette.blue(ColorLightnessPreset::TerminalAnsi),
    );
    builder.add_workspace_rule("terminal.ansiBrightBlack", palette.greyscale(5));
    builder.add_workspace_rule(
        "terminal.ansiBrightBlue",
        palette.blue(ColorLightnessPreset::TerminalAnsiBright),
    );
    builder.add_workspace_rule(
        "terminal.ansiBrightCyan",
        palette.cyan(ColorLightnessPreset::TerminalAnsiBright),
    );
    builder.add_workspace_rule(
        "terminal.ansiBrightGreen",
        palette.green(ColorLightnessPreset::TerminalAnsiBright),
    );
    builder.add_workspace_rule(
        "terminal.ansiBrightMagenta",
        palette.purple(ColorLightnessPreset::TerminalAnsiBright),
    );
    builder.add_workspace_rule(
        "terminal.ansiBrightRed",
        palette.red(ColorLightnessPreset::TerminalAnsiBright),
    );
    builder.add_workspace_rule("terminal.ansiBrightWhite", palette.bright_fg());
    builder.add_workspace_rule(
        "terminal.ansiBrightYellow",
        palette.yellow(ColorLightnessPreset::TerminalAnsiBright),
    );
    builder.add_workspace_rule(
        "terminal.ansiCyan",
        palette.cyan(ColorLightnessPreset::TerminalAnsi),
    );
    builder.add_workspace_rule(
        "terminal.ansiGreen",
        palette.green(ColorLightnessPreset::TerminalAnsi),
    );
    builder.add_workspace_rule(
        "terminal.ansiMagenta",
        palette.purple(ColorLightnessPreset::TerminalAnsi),
    );
    builder.add_workspace_rule(
        "terminal.ansiRed",
        palette.red(ColorLightnessPreset::TerminalAnsi),
    );
    builder.add_workspace_rule("terminal.ansiWhite", palette.fg());
    builder.add_workspace_rule(
        "terminal.ansiYellow",
        palette.yellow(ColorLightnessPreset::TerminalAnsi),
    );
    builder.add_workspace_rule("terminal.foreground", palette.fg());
    builder.add_workspace_rule("terminalCursor.foreground", palette.bright_fg());
    builder.add_workspace_rule("textPreformat.foreground", palette.fg()); // inline code in e.g. Settings page
    builder.add_workspace_rule("titleBar.activeBackground", palette.greyscale(-1));
    builder.add_workspace_rule("titleBar.activeForeground", palette.fg());
    builder.add_workspace_rule("titleBar.inactiveBackground", palette.greyscale(-1));
    builder.add_workspace_rule("titleBar.inactiveForeground", palette.greyscale(4));
    builder.add_workspace_rule("widget.shadow", (Oklch::BLACK, 0x88));
}

fn syntax_highlighting(builder: &mut ThemeBuilder, palette: &Palette) {
    builder.add_rules(
        &[
            Semantic("keyword"),
            Semantic("selfParameter"),
            Textmate("entity.name.tag"),
            Textmate("keyword.operator.expression"),
            Textmate("keyword.operator.new"),
            Textmate("keyword.operator.wordlike"),
            Textmate("keyword.type.elm"),
            Textmate("keyword.type.go"),
            Textmate("keyword"),
            Textmate("keyword.operator.in"), // in keyword
            Textmate("punctuation.definition.heading"),
            Textmate("storage.modifier"),
            Textmate("storage.type.class"),
            Textmate("storage.type.enum"),
            Textmate("storage.type.function.python"),
            Textmate("storage.type.function.ts"),
            Textmate("storage.type.function"),
            Textmate("storage.type.js"),
            Textmate("storage.type.local.java"), // var keyword
            Textmate("storage.type.def.groovy"), // def keyword
            Textmate("storage.type.property"),
            Textmate("storage.type.rust"),
            Textmate("storage.type.struct"),
            Textmate("storage.type.ts"),
            Textmate("variable.language.self"),
            Textmate("variable.language.special.self"),
            Textmate("variable.language.this"),
        ],
        (palette.keyword_color(), FontStyle::Bold),
    );

    builder.add_rules(
        &[
            Semantic("number"),
            Textmate("constant.numeric"),
            Textmate("keyword.other.unit"),
        ],
        palette.green(1),
    );

    builder.add_rules(
        &[
            Semantic("string"),
            Textmate("constant.character"),
            Textmate("punctuation.definition.char"),
            Textmate("punctuation.definition.string"),
            Textmate("string"),
        ],
        palette.red(-1),
    );

    builder.add_rules(
        &[Semantic("variable"), Textmate("variable")],
        palette.variable_color(),
    );

    builder.add_rules(
        &[
            Semantic("boolean"),
            Semantic("enumMember"),
            Textmate("variable.other.enummember"),
        ],
        palette.enum_member_color(),
    );

    builder.add_rules(
        &[
            Semantic("variable.static"),
            Textmate("constant"),
            Textmate("entity.name.constant"),
            Textmate("variable.other.metavariable"),
            Textmate("support.constant"),
        ],
        (palette.constant_color(), FontStyle::Inherit),
    );

    builder.add_rules(
        &[
            Semantic("function"),
            Semantic("method"),
            Textmate("entity.name.function"),
            Textmate("entity.name.function-call"),
            Textmate("meta.function-call.generic.python"),
            Textmate("support.function"),
            Textmate("entity.other.attribute-name.table.toml"),
            Textmate("entity.other.attribute-name.table.array.toml"),
        ],
        palette.function_color(),
    );
    builder.add_rules(
        &[
            Semantic("function.public.declaration"),
            Semantic("method.public.declaration"),
        ],
        (palette.purple(1), FontStyle::Inherit),
    );

    builder.add_rules(
        &[
            Semantic("type"),
            Semantic("class"),
            Semantic("struct"),
            Semantic("enum"),
            Semantic("union"),
            Semantic("typeAlias"),
            Textmate("entity.name.type"),
            Textmate("storage.type"),
            Textmate("support.class"),
            Textmate("support.type"),
        ],
        palette.type_color(),
    );
    builder.add_rules(
        &[
            Semantic("type.public.declaration"),
            Semantic("class.public.declaration"),
            Semantic("struct.public.declaration"),
            Semantic("enum.public.declaration"),
            Semantic("union.public.declaration"),
            Semantic("typeAlias.public.declaration"),
        ],
        (palette.purple(-1), FontStyle::Inherit),
    );

    builder.add_rules(
        &[
            Semantic("builtinType"),
            Textmate("keyword.type"),
            Textmate("storage.type.boolean.go"),
            Textmate("storage.type.built-in"),
            Textmate("storage.type.byte.go"),
            Textmate("storage.type.error.go"),
            Textmate("storage.type.numeric.go"),
            Textmate("storage.type.primitive"),
            Textmate("storage.type.rune.go"),
            Textmate("storage.type.string.go"),
            Textmate("storage.type.uintptr.go"),
            Textmate("support.type"),
            Textmate("variable.other.metavariable.specifier"),
        ],
        palette.cyan(0),
    );

    builder.add_rules(
        &[
            Semantic("typeParameter"),
            Textmate("entity.name.type.parameter"),
            Textmate("variable.type"),
        ],
        palette.orange(1),
    );

    builder.add_rules(
        &[
            Semantic("property"),
            Textmate("entity.name.field"),
            Textmate("entity.name.record.field"),
            Textmate("entity.name.variable.field"),
            Textmate("meta.attribute.python"),
            Textmate("punctuation.support.type.property-name"),
            Textmate("support.type.property-name"),
            Textmate("support.type.vendored.property-name"),
            Textmate("variable.other.member"),
            Textmate("variable.other.object.property"),
            Textmate("variable.other.property"),
            Textmate("entity.name.tag.toml"),
            Textmate("entity.name.tag.yaml"),
        ],
        palette.property_color(),
    );

    builder.add_rule(
        Semantic("interface"),
        (palette.interface_color(), FontStyle::Italic),
    );
    builder.add_rule(
        Semantic("interface.public.declaration"),
        (palette.purple(0), FontStyle::Inherit),
    );
    builder.add_rule(Semantic("*.trait"), FontStyle::Italic);

    builder.add_rules(
        &[
            Semantic("namespace"),
            Textmate("entity.name.module"),
            Textmate("entity.name.namespace"),
            Textmate("entity.name.type.namespace"),
            Textmate("storage.modifier.import"),
            Textmate("storage.modifier.package"),
            Textmate("support.module"),
            Textmate("entity.name.type.module"),
            Textmate("variable.other.constant.elixir"),
        ],
        palette.namespace_color(),
    );

    builder.add_rule(Semantic("namespace.crateRoot"), palette.green(2));

    builder.add_rules(
        &[
            Semantic("macro"),
            Textmate("entity.name.function.macro"),
            Textmate("entity.name.macro"),
            Textmate("entity.name.other.preprocessor.macro"),
            Textmate("variable.other.readwrite.module.elixir"),
            Textmate("punctuation.definition.variable.elixir"),
        ],
        palette.blue(0),
    );

    builder.add_rules(
        &[
            Semantic("lifetime"),
            Textmate("storage.modifier.lifetime.rust"),
            Textmate("entity.name.lifetime.rust"),
            Textmate("entity.name.type.lifetime"),
            Textmate("punctuation.definition.lifetime"),
        ],
        (palette.orange(0), FontStyle::Italic),
    );

    builder.add_rules(
        &[
            Semantic("escapeSequence"),
            Textmate("constant.character.escape"),
        ],
        palette.blue(0),
    );

    builder.add_rules(
        &[
            Semantic("formatSpecifier"),
            Textmate("constant.character.format.placeholder"),
            Textmate("constant.other.placeholder"),
            Textmate("punctuation.section.embedded"),
            Textmate("punctuation.definition.template-expression"),
        ],
        palette.blue(0),
    );

    builder.add_rules(
        &[
            Semantic("comment"),
            Textmate("comment"),
            Textmate("punctuation.definition.comment"),
        ],
        palette.green(-2),
    );

    builder.add_rules(
        &[
            Semantic("comment.documentation"),
            Textmate("comment.line.documentation"),
        ],
        palette.green(-1),
    );

    builder.add_rules(
        &[
            Semantic("*.attribute"),
            Textmate("entity.name.function.decorator"),
            Textmate("meta.attribute"),
            Textmate("punctuation.brackets.attribute"),
            Textmate("punctuation.definition.annotation"),
            Textmate("punctuation.definition.attribute"),
            Textmate("punctuation.definition.decorator"),
            Textmate("storage.modifier.attribute"),
            Textmate("storage.type.annotation"),
        ],
        palette.blue(0),
    );

    // CSS classes and IDs.
    builder.add_rules(
        &[
            Textmate("entity.other.attribute-name.class"),
            Textmate("entity.other.attribute-name.id"),
        ],
        palette.cyan(-1),
    );

    builder.add_rules(
        &[
            Semantic("*.unsafe"),
            Semantic("function.unsafe"),
            Semantic("operator.unsafe"),
            Textmate("keyword.other.unsafe"),
        ],
        (palette.red(-2), FontStyle::Inherit),
    );

    builder.add_rules(
        &[
            Semantic("punctuation"),
            Textmate("keyword.operator.logical.rust"),
            Textmate("keyword.operator"),
            Textmate("storage.modifier.pointer"),
            Textmate("storage.type.function.arrow"),
            Textmate("punctuation"),
            Textmate("keyword.control.flow.block-scalar.literal.yaml"),
        ],
        palette.fg(),
    );

    builder.add_rule(Textmate("markup.italic"), FontStyle::Italic);
    builder.add_rule(Textmate("markup.bold"), FontStyle::Bold);
    builder.add_rule(Textmate("markup.heading"), FontStyle::Bold);
    builder.add_rules(
        &[
            Textmate("punctuation.definition.markdown"),
            Textmate("punctuation.definition.heading.markdown"),
            Textmate("punctuation.definition.metadata.markdown"),
            Textmate("punctuation.definition.raw.markdown"),
            Textmate("punctuation.definition.constant.markdown"),
            Textmate("punctuation.definition.constant.begin.markdown"),
            Textmate("punctuation.definition.constant.end.markdown"),
            Textmate("punctuation.definition.string.begin.markdown"),
            Textmate("punctuation.definition.string.end.markdown"),
            Textmate("punctuation.definition.list.begin.markdown"),
            Textmate("punctuation.definition.quote.begin.markdown"),
            Textmate("punctuation.definition.bold.markdown"),
            Textmate("punctuation.definition.italic.markdown"),
            Textmate("punctuation.separator.key-value.markdown"),
            Textmate("punctuation.separator.key-value.markdown"),
            Textmate("fenced_code.block.language.markdown"),
            Textmate("constant.other.reference.link.markdown"),
            Textmate("meta.link.inline.markdown"),
            Textmate("meta.link.reference.def.markdown"),
            Textmate("punctuation.definition.asciidoc"),
            Textmate("punctuation.separator.asciidoc"),
            Textmate("support.asciidoc"),
            Textmate("markup.heading.asciidoc"),
            Textmate("markup.heading.marker.asciidoc"),
            Textmate("markup.list.bullet.asciidoc"),
            Textmate("markup.link.asciidoc"),
            Textmate("markup.other.url.asciidoc"),
            Textmate("markup.other.anchor.asciidoc"),
            Textmate("support.constant.asciidoc"),
            Textmate("constant.asciidoc"),
            Textmate("entity.name.function.asciidoc"),
        ],
        palette.green(-1),
    );
    builder.add_rules(
        &[
            Textmate("string.other.link.title.markdown"),
            Textmate("string.other.link.description.markdown"),
            Textmate("string.unquoted.asciidoc"),
        ],
        palette.fg(),
    );

    builder.add_rules(
        &[
            Textmate("markup.inserted"),
            Textmate("punctuation.definition.inserted.diff"),
        ],
        palette.green(ColorLightnessPreset::DiffFg),
    );
    builder.add_rules(
        &[
            Textmate("markup.deleted"),
            Textmate("punctuation.definition.deleted.diff"),
        ],
        palette.red(ColorLightnessPreset::DiffFg),
    );
    builder.add_rules(
        &[Textmate("markup.changed")],
        palette.orange(ColorLightnessPreset::DiffFg),
    );

    builder.add_rules(
        &[
            Textmate("punctuation.definition.range.diff"),
            Textmate("meta.diff.range"),
        ],
        palette.blue(0),
    );
    builder.add_rules(
        &[
            Textmate("comment.line.number-sign.git-commit"),
            Textmate("punctuation.definition.comment.git-commit"),
            Textmate("meta.diff.index"),
            Textmate("meta.diff.header"),
        ],
        palette.greyscale(4),
    );
    builder.add_rules(
        &[
            Textmate("meta.diff.header.to-file"),
            Textmate("meta.diff.header.from-file"),
        ],
        // we really want this to stand out
        // because it’s easily lost among noisy diff output
        (palette.bright_fg(), FontStyle::Bold),
    );
    builder.add_rules(
        &[
            Textmate("punctuation.definition.from-file.diff"),
            Textmate("punctuation.definition.to-file.diff"),
        ],
        palette.cyan(0),
    );

    builder.add_rules(
        &[Semantic("*.mutable"), Textmate("meta.mutable")],
        FontStyle::Underline,
    );
    builder.add_rule(
        Semantic("*.public.declaration"),
        (palette.purple(1), FontStyle::Inherit),
    );

    builder.add_rule(
        Semantic("unresolvedReference"),
        (palette.red(-1), FontStyle::Underline),
    );

    builder.add_rule(
        Semantic("magit-ref-name"),
        (palette.cyan(1), FontStyle::Bold),
    );
    builder.add_rule(
        Semantic("magit-remote-ref-name"),
        (palette.green(-2), FontStyle::Bold),
    );
    builder.add_rule(
        Textmate("magit.header"),
        (palette.yellow(2), FontStyle::Bold),
    );
    builder.add_rule(Textmate("magit.subheader"), FontStyle::Bold);
    builder.add_rule(Textmate("magit.entity"), palette.greyscale(5));

    // Over 50 characters, the recommended limit.
    builder.add_rule(
        Textmate("invalid.deprecated.line-too-long.git-commit"),
        palette.orange(0),
    );

    // Over 72 characters, the hard limit.
    builder.add_rule(
        Textmate("invalid.illegal.line-too-long.git-commit"),
        palette.red(0),
    );
}
